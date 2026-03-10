# TalkBank Legacy Documentation

Transferred from `~/Dropbox/documentation/` on 2026-03-10.
Originally maintained 2020–2025 as the central documentation for TalkBank infrastructure.

Reviewed interactively on 2026-03-10. Files with no remaining value were deleted
(60 of 84 original files removed). Credentials extracted to `~/.talkbank-secrets/`.

**What remains:** provisioning traces, active infrastructure docs, and reference
material needed for succession planning and future automation.

---

## Servers & Infrastructure (provisioning traces)

| Document | Notes |
|----------|-------|
| [servers.md](servers.md) | Vhost structure, password protection via `0access.txt` (removed), samtale-users (removed) |
| [servers.txt](servers.txt) | Vhost→machine mapping inventory (18 on talkbank.org, 3 on git, sla/sla2, dali) |
| [server-setup.md](server-setup.md) | Step-by-step: adding a new bank (repos, config.py, vhosts, deploy). Needs modernization. |
| [cloud.md](cloud.md) | CMU Campus Cloud: vSphere console, IaaS plan, Cloud Plus for media |
| [homebank.md](homebank.md) | Full from-scratch server build log (Ubuntu 20.04, Apache, certs, Java, sudoers, deploys) |
| [media-server.md](media-server.md) | media.talkbank.org: Red Hat, Cloud Plus, sync-media, permission fixes |
| [ubuntu.md](ubuntu.md) | Server OS: user setup, SSH keys, firewall, upgrade history (18.04→20.04), DNS fixes |
| [macos.md](macos.md) | macOS version notes, CLAN compatibility history |
| [sudoers.txt](sudoers.txt) | Sudoers config for git.talkbank.org + DNS resolution debugging |
| [resize.md](resize.md) | LVM disk resize procedure for cloud VMs |
| [development-environment.md](development-environment.md) | Gaps in standardized dev environment (being addressed by `make clone`) |

## SSL & Certificates

| Document | Notes |
|----------|-------|
| [certbot.md](certbot.md) | Let's Encrypt setup for 20 vhosts, symlink trick, Nginx renewal, **known port 80 renewal failure** |
| [certbot.sh](certbot.sh) | Renewal script (credentials redacted → `~/.talkbank-secrets/acme-certbot.env`) |
| [certificates.md](certificates.md) | Certificate repo pointer (credential redacted → `~/.talkbank-secrets/certificates.txt`) |

## Git & Version Control

| Document | Notes |
|----------|-------|
| [gitlab.md](gitlab.md) | GitLab EE setup, API, upgrade log. Keep until GitLab→GitHub migration complete (~1 week). Tokens redacted. |

## Security & Operations

| Document | Notes |
|----------|-------|
| [security.md](security.md) | No formal system yet. Vault investigation TODO. Shared accounts. |
| [credentials.md](credentials.md) | SSH key setup, keychain `.bashrc` hack, authorized keys |

## Data Management & Metadata

| Document | Notes |
|----------|-------|
| [development.md](development.md) | Data pipeline overview: `NAME-data`/`NAME-site` repos, @PID, @Types, media workflow |
| [datacite.md](datacite.md) | DOI management via `cdcs-to-csv`, DataCite API, consistency issues |
| [metadata.txt](metadata.txt) | `0metadata.cdc` field mapping for DataCite DOIs |
| [pid.md](pid.md) | PID generation: Leonid's Handle server process on dali + "Mac Zee" |
| [xml-schema.md](xml-schema.md) | `talkbank.xsd` history (since 2002), xsddoc generation. Unknown external dependents. |

## Validation

| Document | Notes |
|----------|-------|
| [checkers.md](checkers.md) | Inventory of all checkers: Chatter, CHECK, file names, passwords, check_chat_html, @Types |

---

## Deleted Files (2026-03-10)

60 files removed across two commits. Categories:

- **Empty stubs** (7): talkbank-db, cgi, logging, slack, monitoring, r, red-hat
- **Superseded technologies** (18): apache-httpd, tomcat, oai, scons, shibboleth, gitbucket, bitbucket, php, mirrors, ssh, remote-access, git-server, automated-build/deployment, manual-deployment, continuous-*
- **Failed experiments / unused** (6): gitlab-docker, gitlab-no-docker-ce, gitlab-ci, kubernetes, virtualbox, vagrant
- **Outdated stubs** (14): chatter, talkbank-api, send2clan, update-chat-types, docker, homebrew, documentation-formats, graphql, java, python, rust, typescript, stack, new-machine-setup
- **Absorbed elsewhere** (10): git, github, clan, clan-info, cdcs, cmdi, gandalf, non-cloud-servers, talkbank-browser, browser-clan
- **Other** (5): testing, external-contributions, transforming-data, web-site-checkers, ssl

## Credential Locations

All credentials extracted from these docs are at `~/.talkbank-secrets/` (chmod 700, not in git).
See `~/.talkbank-secrets/README.md` for inventory.
