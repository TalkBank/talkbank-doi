# Media Server (media.talkbank.org)

**Status:** Current
**Last updated:** 2026-03-17

## Overview

`media.talkbank.org` hosts TalkBank's research media files (audio, video)
for 19 corpus banks. It is served by a Node.js HTTPS server running on
CMU Campus Cloud Plus infrastructure. Media files are synced from net
via `sync-media` (rclone over SFTP).

## Server Details

| Field | Value |
|-------|-------|
| Hostname | `talkbank-02.andrew.cmu.edu` |
| Public DNS | `media.talkbank.org` (CNAME → `talkbank-02.andrew.cmu.edu`) |
| IP | 128.237.92.37 |
| OS | Red Hat Enterprise Linux 8.10 (Ootpa) |
| Kernel | 4.18.0-553.97.1.el8_10.x86_64 |
| CPU | 2 vCPUs |
| RAM | 3.5 GB |
| Swap | 2 GB |
| Data volume | 20 TB (`/dev/mapper/datavg-data` mounted at `/data`), 13 TB used |
| Cloud tier | CMU Campus Cloud Plus (restricted — CMU IT configures services) |
| Node.js | v20.19.5 |

## Accounts

| Account | Purpose |
|---------|---------|
| `psych-tb-svc` | Service account. Owns the Node.js media server and `/home/psych-tb-svc/mediaServer/`. Used by `sync-media` for SFTP file transfer. |
| `macw` | Admin account. Has sudo. Used for manual maintenance. |

SSH keys for both accounts are on net (`/Users/macw/.ssh/`).

## Network Access

The server sits behind the CMU Campus Cloud Plus network segment. Access
from net currently requires Cisco AnyConnect VPN connected to `vpn.cmu.edu`
with CMU credentials + Duo 2FA.

**Firewall exception requested (2026-03-17):** Direct SSH (TCP 22) from
net (172.24.73.5) to talkbank-02 (128.237.92.37) to eliminate the VPN
requirement. See `docs/media-server-firewall-request.md`.

## Node.js Media Server

John set up a Node.js HTTPS server that serves media files to authorized
users via the browser:

- **Systemd unit:** `mediaServer.service` (runs as `psych-tb-svc`)
- **Working directory:** `/home/psych-tb-svc/mediaServer/`
- **Entry point:** `serve.js` (Express + HTTPS)
- **Port:** 443 (uses `cap_net_bind_service` to bind as non-root)
- **SSL certs:** `/etc/pki/tls/private/localhost.key` + `/etc/pki/tls/certs/localhost.crt` + chain
- **Auth backend:** `https://sla2.talkbank.org:443` (John's auth service)
- **Public banks (no auth):** `open`, `psyling`
- **Restart policy:** Always, 500ms delay, no rate limit

The server builds a path tree of `/data/` at startup and serves files
with path traversal protection. It excludes `lost+found`, `users`, and
`confs` directories.

## Data Layout

`/data/` contains one directory per bank:

```
/data/
├── aphasia/    ├── dementia/   ├── motor/      ├── samtale/
├── asd/        ├── fluency/    ├── open/       ├── slabank/
├── biling/     ├── homebank/   ├── phon/       ├── tbi/
├── ca/         ├── lost+found/ ├── psyling/
├── childes/    ├── class/      ├── psychosis/
│               │               ├── rhd/
```

## Media Sync

Media files are synced from net to the media server using `sync-media`
(a Python tool wrapping rclone). See the `sync-media/` repo.

**Source:** `/Users/macw/media/<bank>/` on net
**Destination:** `/data/<bank>/` on talkbank-02 (via SFTP as `psych-tb-svc`)
**Tool:** rclone sync (4 parallel transfers, 8 checkers, follows symlinks)
**Config on net:** `~/.sync-media/config`

```bash
# Run from net:
sync-media childes          # sync one bank
sync-media childes --dry-run  # preview changes
```

Sync uses `rclone sync` which **deletes** files on the destination that
don't exist in the source.

## Known Issues

### VPN overhead

Every media sync requires a team member to manually authenticate with
Cisco AnyConnect (CMU credentials + Duo 2FA), run the sync, and
disconnect. This cannot be automated. A firewall exception has been
requested to allow direct SSH from net to talkbank-02 — see
`docs/media-server-firewall-request.md`.

### Permission problems

Brian stages media files on his local machine before syncing. Incorrect
permissions on the source side propagate to the media server and can
break browser access via the Node.js server.

### Campus Cloud Plus admin changes

CMU IT has occasionally changed permissions on `/etc/httpd/conf.d/` on
the media server without notice, requiring manual intervention to restore
the configuration.
