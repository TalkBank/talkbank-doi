# TalkBank Legacy Documentation

Transferred from `~/Dropbox/documentation/` on 2026-03-10.
Originally maintained 2020–2025 as the central documentation for TalkBank infrastructure.

**Status:** These docs are being reviewed for accuracy and integrated into the
talkbank-dev workspace. Each document is annotated below with its current status.

| Status Tag | Meaning |
|------------|---------|
| **current** | Still accurate, needs minor updates |
| **outdated** | Partially accurate, needs significant revision |
| **obsolete** | Superseded or no longer applicable |
| **stub** | Placeholder with minimal content |
| **review** | Needs audit to determine accuracy |

---

## CHAT Format & Tools

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [chatter.md](chatter.md) | 203 | **obsolete** | Describes Java/ANTLR 3 chatter; superseded by Rust `chatter` in talkbank-tools |
| [clan.md](clan.md) | 14 | **stub** | Links to CLAN info |
| [clan-info.md](clan-info.md) | 8 | **stub** | Points to clan-info repo |
| [xml-schema.md](xml-schema.md) | 45 | **review** | References Oxygen xsddoc generation |
| [talkbank-api.md](talkbank-api.md) | 17 | **obsolete** | Empty npm package; never completed |
| [send2clan.md](send2clan.md) | 12 | **outdated** | Now consolidated in talkbank-tools (send2clan-sys crate) |

## Servers & Infrastructure

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [servers.md](servers.md) | 60 | **review** | Overview of cloud/non-cloud architecture |
| [server-setup.md](server-setup.md) | 60 | **review** | Adding new corpus servers (Git + Apache vhost) |
| [cloud.md](cloud.md) | 67 | **review** | DNS, IP, CDN for cloud servers |
| [non-cloud-servers.md](non-cloud-servers.md) | 11 | **stub** | |
| [gandalf.md](gandalf.md) | 12 | **review** | Brian's Mac — Apache + Shiny (Vagrant) |
| [media-server.md](media-server.md) | 66 | **review** | Media file sync between gandalf and media server |
| [homebank.md](homebank.md) | 209 | **outdated** | Ubuntu 20.04 setup; may need update for current OS |
| [apache-httpd.md](apache-httpd.md) | 496 | **current** | Comprehensive Apache 2.4 docs; TODO mentions Nginx |
| [tomcat.md](tomcat.md) | 213 | **review** | Apache Tomcat for OAI server |
| [new-machine-setup.md](new-machine-setup.md) | 8 | **stub** | |

## SSL & Certificates

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [certbot.md](certbot.md) | 171 | **current** | Let's Encrypt automation; updated Feb 2025 |
| [certificates.md](certificates.md) | 11 | **stub** | |
| [ssl.md](ssl.md) | 122 | **obsolete** | Replaced by certbot |
| [certbot.sh](certbot.sh) | 10 | **current** | Renewal script |

## Git & Version Control

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [git.md](git.md) | 82 | **review** | Workflow, branching, credentials |
| [git-server.md](git-server.md) | 44 | **outdated** | Internal Git server (GitBucket era) |
| [github.md](github.md) | 190 | **outdated** | Needs update for current 54-repo org |
| [gitlab.md](gitlab.md) | 315 | **current** | GitLab EE setup — needed until decommissioned |
| [gitlab-ci.md](gitlab-ci.md) | 112 | **review** | CI pipeline syntax |
| [gitlab-docker.md](gitlab-docker.md) | 90 | **review** | GitLab via Docker |
| [gitlab-no-docker-ce.md](gitlab-no-docker-ce.md) | 175 | **review** | GitLab CE without Docker |
| [gitbucket.md](gitbucket.md) | 377 | **obsolete** | Migrated away |
| [bitbucket.md](bitbucket.md) | 66 | **obsolete** | Migrated away |

## CI/CD & Deployment

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [continuous-delivery.md](continuous-delivery.md) | 21 | **review** | Automation philosophy |
| [continuous-integration.md](continuous-integration.md) | 10 | **stub** | |
| [continuous-deployment.md](continuous-deployment.md) | 12 | **stub** | |
| [automated-build.md](automated-build.md) | 49 | **outdated** | References Chatter XML gen, SCons ZIPs |
| [automated-deployment.md](automated-deployment.md) | 53 | **outdated** | Git hooks and scripts; being replaced |
| [manual-deployment.md](manual-deployment.md) | 19 | **review** | |
| [logging.md](logging.md) | 6 | **stub** | |
| [slack.md](slack.md) | 4 | **stub** | |
| [scons.md](scons.md) | 15 | **review** | SCons for ZIP corpus generation |

## Data Management & Metadata

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [development.md](development.md) | 72 | **review** | Corpus metadata, @PID, @Types |
| [datacite.md](datacite.md) | 120 | **current** | DOI management via cdcs-to-csv; TODOs for consolidation |
| [pid.md](pid.md) | 73 | **review** | Persistent ID generation (Leonid's CMDI command) |
| [cmdi.md](cmdi.md) | 170 | **review** | CMDI metadata generation workflow |
| [metadata.txt](metadata.txt) | 37 | **review** | Corpus metadata headers |
| [oai.md](oai.md) | 107 | **review** | OAI-PMH provider on homebank (jOAI + Tomcat) |
| [update-chat-types.md](update-chat-types.md) | 32 | **outdated** | Now a Rust tool with pre-commit hook |
| [transforming-data.md](transforming-data.md) | 9 | **stub** | |

## Applications & Services

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [talkbank-browser.md](talkbank-browser.md) | 49 | **review** | PHP-based corpus browser; deployed on 14 sites |
| [browser-clan.md](browser-clan.md) | 7 | **stub** | |
| [talkbank-db.md](talkbank-db.md) | 1 | **stub** | |
| [cgi.md](cgi.md) | 1 | **stub** | |
| [checkers.md](checkers.md) | 91 | **outdated** | Validation checkers; partly superseded by `chatter validate` |
| [web-site-checkers.md](web-site-checkers.md) | 77 | **review** | Link/HTTPS/CORS checks |
| [external-contributions.md](external-contributions.md) | 22 | **review** | |

## Security & Operations

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [security.md](security.md) | 43 | **outdated** | Identifies gaps: no formal system, secrets in code |
| [credentials.md](credentials.md) | 32 | **outdated** | Shared accounts, needs audit |
| [shibboleth.md](shibboleth.md) | 155 | **obsolete** | Authentication system retired |
| [testing.md](testing.md) | 19 | **review** | |
| [monitoring.md](monitoring.md) | 6 | **stub** | |
| [mirrors.md](mirrors.md) | 32 | **review** | |

## Development Stack & Tools

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [stack.md](stack.md) | 18 | **outdated** | Technology overview; needs Rust-first update |
| [development-environment.md](development-environment.md) | 17 | **review** | |
| [remote-access.md](remote-access.md) | 20 | **review** | |
| [homebrew.md](homebrew.md) | 91 | **review** | macOS package manager |
| [ssh.md](ssh.md) | 12 | **stub** | |
| [docker.md](docker.md) | 50 | **review** | |
| [kubernetes.md](kubernetes.md) | 54 | **review** | |
| [virtualbox.md](virtualbox.md) | 25 | **review** | |
| [vagrant.md](vagrant.md) | 17 | **review** | Used for Shiny on gandalf |
| [resize.md](resize.md) | 93 | **review** | VirtualBox disk resizing |

## Programming Languages

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [java.md](java.md) | 59 | **outdated** | Needs Java 24+ and Phon context |
| [python.md](python.md) | 54 | **obsolete** | References Python 3.7 (2020); now 3.12/3.14t |
| [php.md](php.md) | 10 | **stub** | TalkBank browser |
| [javascript.md](javascript.md) | — | **stub** | |
| [typescript.md](typescript.md) | 20 | **outdated** | Needs VS Code extension context |
| [rust.md](rust.md) | 9 | **obsolete** | 9 lines; superseded by talkbank-tools/CLAUDE.md |
| [r.md](r.md) | 1 | **stub** | |
| [graphql.md](graphql.md) | 7 | **stub** | |

## Operating Systems

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [macos.md](macos.md) | 28 | **outdated** | References Mojave/Catalina (2019–2020) |
| [ubuntu.md](ubuntu.md) | 251 | **outdated** | References 18.04 LTS |
| [red-hat.md](red-hat.md) | 6 | **stub** | |

## Meta

| Document | Lines | Status | Notes |
|----------|-------|--------|-------|
| [documentation-formats.md](documentation-formats.md) | 24 | **current** | Style guidelines |

---

## Summary Statistics

| Status | Count |
|--------|-------|
| current | 4 |
| outdated | 15 |
| obsolete | 8 |
| stub | 22 |
| review | 35 |
| **Total** | **84** |

## Next Steps

1. **Review** all 35 "review" docs — determine if current or outdated
2. **Update** the 15 "outdated" docs with current information
3. **Archive** the 8 "obsolete" docs (move to `obsolete/` subfolder, keep for reference)
4. **Flesh out or delete** the 22 stubs
5. **Merge** related docs where there's overlap (e.g., ssl/certbot/certificates)
6. **Cross-reference** with talkbank-tools and batchalign3 book docs to avoid duplication
