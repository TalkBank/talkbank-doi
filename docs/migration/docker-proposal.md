# Docker Proposal for talkbank.org

**Status:** Draft
**Last updated:** 2026-03-21

## Why Docker

When TalkBank transitions to a successor, they need to be able to stand up the
entire web presence from scratch at whatever institution they're at. Today that
means: get a Linux VM, install 10+ packages, clone 40+ repos, configure nginx,
set up certbot, install a GitHub Actions runner, configure mergerfs, deploy John's
app, set up systemd services. Miss a step and something breaks silently.

With Docker: `git clone talkbank-deploy && docker compose up`. Done.

The host OS becomes irrelevant — RHEL, Ubuntu, Debian, whatever the new institution
provides. The entire web stack runs identically everywhere.

## Architecture

```
                        ┌─────────────────────────────────┐
                        │         Host (any Linux)        │
                        │                                 │
  Internet ──────────── │ ┌─────────────────────────────┐ │
        :443/:80        │ │  nginx container            │ │
                        │ │  - static bank sites        │ │
                        │ │  - reverse proxy → node     │ │
                        │ │  - reverse proxy → cgi      │ │
                        │ │  - certbot webroot          │ │
                        │ └──────┬──────────┬───────────┘ │
                        │        │          │             │
                        │  ┌─────▼───┐ ┌────▼──────────┐ │
                        │  │  node   │ │  cgi          │ │
                        │  │  :4000  │ │  (morgra2jpg) │ │
                        │  └─────────┘ └───────────────┘ │
                        │                                 │
                        │  ┌─────────────────────────────┐│
                        │  │  certbot (sidecar, periodic) ││
                        │  └─────────────────────────────┘│
                        │                                 │
                        │  Volumes:                       │
                        │    /data/repos     (git clones) │
                        │    /data/view      (mergerfs)   │
                        │    /web            (bank sites) │
                        │    /certs          (letsencrypt) │
                        └─────────────────────────────────┘
```

## docker-compose.yml (sketch)

```yaml
services:

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
      - "443:443"
    volumes:
      - ./nginx/talkbank.conf:/etc/nginx/conf.d/default.conf:ro
      - web-content:/var/www/web:ro
      - data-view:/var/data/view:ro
      - certbot-webroot:/var/www/letsencrypt:ro
      - certbot-certs:/etc/letsencrypt:ro
    depends_on:
      - node
      - cgi
    restart: unless-stopped

  node:
    build: ./node
    # Or: image: node:22-alpine + command
    volumes:
      - data-view:/var/data/view:ro
    environment:
      - PORT=4000
      - DATA_ROOT=/var/data/view
    expose:
      - "4000"
    restart: unless-stopped

  cgi:
    build: ./cgi
    # Minimal image: perl + graphviz + fcgiwrap
    # Exposes a FastCGI socket or TCP port for nginx to proxy to
    expose:
      - "9000"
    restart: unless-stopped

  certbot:
    image: certbot/certbot
    volumes:
      - certbot-webroot:/var/www/letsencrypt
      - certbot-certs:/etc/letsencrypt
    # Run renewal check daily
    entrypoint: sh -c "while true; do certbot renew --webroot -w /var/www/letsencrypt; sleep 86400; done"
    restart: unless-stopped

volumes:
  web-content:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: /home/macw/web
  data-view:
    driver: local
    driver_opts:
      type: none
      o: bind
      device: /var/data/view
  certbot-webroot:
  certbot-certs:
```

## Container details

### nginx

The simplest container. Official `nginx:alpine` image, our generated `talkbank.conf`
mounted in. All static content (bank websites, downloads) is a bind-mounted volume
from the host. nginx proxies:

- `/{bank}/data/` and `/{bank}/data-orig/` → `node:4000`
- `/cgi-bin/` → `cgi:9000` (FastCGI)
- Everything else → static files from `/var/www/web/`

### node (John's app)

John's data browsing/ZIP app. Reads from `/var/data/view/{bank}/` (the mergerfs
merged view, bind-mounted from the host). Listens on port 4000, only exposed
internally to nginx.

**Dockerfile sketch:**
```dockerfile
FROM node:22-alpine
WORKDIR /app
COPY package*.json ./
RUN npm ci --production
COPY . .
ENV PORT=4000 DATA_ROOT=/var/data/view
EXPOSE 4000
USER node
CMD ["node", "server.js"]
```

### cgi (morgra2jpg)

This is the ugliest container. `morgra2jpg.cgi` is a Perl script that shells out to
`graphviz` (the `dot` command) to render dependency trees as JPEGs. Options:

**Option A: Keep CGI (minimal change)**

Build a small image with perl, graphviz, and fcgiwrap. fcgiwrap listens on TCP 9000
inside the container, nginx proxies FastCGI to it.

```dockerfile
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y --no-install-recommends \
    perl graphviz fcgiwrap && rm -rf /var/lib/apt/lists/*
COPY morgra2jpg.cgi /usr/lib/cgi-bin/
RUN chmod +x /usr/lib/cgi-bin/morgra2jpg.cgi
# fcgiwrap listening on TCP instead of unix socket
CMD ["fcgiwrap", "-s", "tcp:0.0.0.0:9000"]
```

**Option B: Rewrite as HTTP service (cleaner)**

Replace the CGI script with a small HTTP server (Python, Rust, or even Node) that
accepts the same query parameters and returns the same JPEG. This eliminates
fcgiwrap entirely and makes the container trivial. The CGI script is ~200 lines of
Perl — a rewrite is a few hours of work.

Recommendation: Option A for now (it works, no rewrite needed), Option B later when
someone has time.

### certbot

Standard pattern: the `certbot/certbot` image runs periodic renewal checks. It
writes to a shared volume that nginx reads from. After renewal, nginx needs to
reload — this can be done via a `docker compose exec nginx nginx -s reload` in
the renewal hook, or by using `inotifywait` on the cert files.

The HTTP-01 challenge works because nginx serves the webroot. No DNS-01 needed.

## What stays on the host

Docker doesn't containerize everything. These remain on the host:

| Component | Why |
|-----------|-----|
| **Git repo clones** (`/home/macw/data/`, `/home/macw/web/`) | 72+ GB of data, updated by `git pull`. Can't bake into images. Bind-mounted as volumes. |
| **mergerfs** | FUSE filesystem. Running inside a container requires `--privileged` which defeats the purpose. Runs on host, containers see the merged view via bind mount. |
| **GitHub Actions runner** | Needs host access to run `git pull` on repos. Runs on host, not in a container. |
| **Tailscale** | System-level VPN. Runs on host. |
| **CrowdStrike** | CMU-mandated. Runs on host. |

## Migration path

We don't have to go all-at-once. Containers can coexist with host services:

**Phase 1: Containerize nginx only.**
Replace the host nginx with a containerized one. Same config, same volumes. Proves
the Docker approach works without touching John's app or CGI.

**Phase 2: Add John's Node app as a container.**
Move from host systemd service to Docker container. John's app is already stateless
(reads from a volume), so this is straightforward.

**Phase 3: Add CGI container.**
Either wrap the existing Perl script or rewrite it.

**Phase 4: docker-compose.yml is the deployment artifact.**
The entire web stack is `docker compose up`. New deployment = `git pull && docker compose up -d`.

## What this gives a successor

1. Clone the deploy repo
2. Get a Linux VM with Docker (any cloud, any OS)
3. Clone the data and web repos
4. Set up mergerfs on the host (one script)
5. `docker compose up -d`
6. Configure DNS to point `talkbank.org` at the new VM

No package installation, no nginx tuning, no systemd services, no "ask Franklin
how he set it up." The `docker-compose.yml` IS the documentation.

## What this costs us

- Learning Docker (Franklin already knows it; Brian and John don't need to)
- Maintaining Dockerfiles (minor — they change rarely)
- Slight operational complexity (`docker compose logs` instead of `journalctl`)
- mergerfs still runs on the host (can't fully containerize FUSE)

## Open questions

1. **Domain and DNS:** A successor would need to transfer or re-point the `talkbank.org`
   domain. Document the registrar and DNS setup.
2. **SSL at a new institution:** Let's Encrypt works anywhere. The only requirement is
   DNS pointing to the new server before `certbot` runs.
3. **GitHub Actions runner:** Could switch to GitHub-hosted runners if the data repos
   are small enough (they're not — 72 GB). Self-hosted runner on the new host is the
   realistic answer.
4. **morgra2jpg rewrite:** Is it worth doing now, or wait until Docker migration forces
   the question?
