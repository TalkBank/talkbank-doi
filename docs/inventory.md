# TalkBank Project Inventory

**Status:** Current
**Last updated:** 2026-03-18

Complete map of all TalkBank project assets, their locations, and migration status.
Created 2026-03-10 to support the consolidation into `talkbank-dev`.

## Source Locations

| Platform | URL | Purpose | Repo Count |
|----------|-----|---------|------------|
| **GitHub** | github.com/TalkBank | Code, web repos, some data banks | 54 |
| **GitLab** (self-hosted) | git.talkbank.org/talkbank | Corpus data repos | 16 |
| **Dropbox** | ~/Dropbox/documentation/ | Legacy infrastructure docs | 84 files |
| **Local only** | ~/ (various) | Experiments, prototypes, archived work | ~50+ dirs |

**Migration goal:** Decommission GitLab (git.talkbank.org), consolidate into GitHub + talkbank-dev.

---

## 1. Active Code Repositories

### Core Development (in talkbank-dev workspace)

| Repo | GitHub | Local Path | Visibility | Status |
|------|--------|------------|------------|--------|
| talkbank-tools | TalkBank/talkbank-tools | talkbank-tools/ | **Public** | Active |
| batchalign3 | TalkBank/batchalign3 | batchalign3/ | **Public (pending)** | Active |

### Infrastructure & Deployment

| Repo | GitHub | Local Path | Visibility | Status |
|------|--------|------------|------------|--------|
| staging | *GitLab: talkbank/staging* | ~/staging/ | Private | Active — deploy orchestration |
| webdev | TalkBank/webdev | ~/webdev/ | Private | Active — web config generation |
| gra-cgi | TalkBank/gra-cgi | ~/gra-cgi/ | Private | Active — MOR/GRA diagram CGI |
| sync-media | TalkBank/sync-media | ~/sync-media/ | Private | Active — rclone-based media sync (gandalf↔media server) |
| generate-from-chat | TalkBank/generate-from-chat | ~/staging/repos/generate-from-chat/ | Private | **Critical** — builds corpus ZIPs, injects DOIs into HTML |
| cdcs-to-csv | TalkBank/cdcs-to-csv | ~/staging/repos/cdcs-to-csv/ | Private | **Critical** — DOI lifecycle (mint, update, export via DataCite API) |
| save-word-html-pdf | TalkBank/save-word-html-pdf | ? | Private | Active |
| update-chat-types | TalkBank/update-chat-types | ? | Private | Active |
| combine-chat | TalkBank/combine-chat | ? | Private | Active |
| homebrew-tap | TalkBank/homebrew-tap | ? | Private | Active — chatter distribution |
| talkbank-manifests | TalkBank/talkbank-manifests | ? | Private | git-repo manifests |

### Legacy CLAN (C/C++)

| Repo | GitHub | Local Path | Visibility | Status |
|------|--------|------------|------------|--------|
| OSX-CLAN | TalkBank/OSX-CLAN | ~/OSX-CLAN/ | **Public** | Active (Leonid) |
| Unix-CLAN | TalkBank/Unix-CLAN | ? | **Public** | Active (Leonid) |
| Windows-CLAN | TalkBank/Windows-CLAN | ? | **Public** | Active (Leonid) |
| clan-info | TalkBank/clan-info | ~/clan-info/ | **Public** | Active — supporting materials |
| mor | TalkBank/mor | ? | **Public** | MOR grammar files |

**Note:** `cdcs-to-csv` and `generate-from-chat` are critical to the deploy pipeline but currently buried as clones inside `~/staging/repos/`. Post-consolidation, they should be top-level siblings in talkbank-dev.

### Pre-Commit / Build Tools

| Repo | GitHub | Local Path | Status |
|------|--------|------------|--------|
| update-chat-types | TalkBank/update-chat-types | ~/update-chat-types/ | **Active** — Rust; updates @Types headers from 0types.txt; has pre-commit hook + bootstrap.sh |
| save-word-html-pdf | TalkBank/save-word-html-pdf | ~/save-word-html-pdf/ | **Active** — macOS Word→HTML/PDF export; should be wired as pre-commit for doc repos |
| talkbank-xml-schema | TalkBank/talkbank-xml-schema | ~/talkbank-xml-schema/ | **Legacy** — TalkBank XML Schema (v3.2.1); Phon may still reference; public repo |

### CHAT Tools (active, under personal GitHub)

| Repo | GitHub | Local Path | Status |
|------|--------|------------|--------|
| java-chatter-stable | FranklinChen/java-chatter | ~/java-chatter-stable/ | **Active** — canonical Java chatter (ANTLR 3), builds Mac app; Java 25 |
| talkbank-ipa-fragments | FranklinChen/talkbank-ipa-fragments | ~/talkbank-ipa-fragments/ | Active — IPA fragment extraction (Java/Maven) |

### Collaborator Repos (read-only reference)

| Repo | GitHub | Local Path | Status |
|------|--------|------------|--------|
| phon | phon-ca/phon | ~/phon/ | Reference — Java phonological analysis |
| phontalk | phon-ca/phontalk | ~/phontalk/ | Reference — Phon↔CHAT converter |

### APIs & Libraries

| Repo | GitHub | Visibility | Status |
|------|--------|------------|--------|
| TBDBr | TalkBank/TBDBr | **Public** | R API to TalkBankDB |
| TBDBpy | TalkBank/TBDBpy | **Public** | Python API to TalkBankDB |
| balite | TalkBank/balite | **Public** | Lightweight online batchalign client |
| talkbank-xml-schema | TalkBank/talkbank-xml-schema | **Public** | XML Schema for CHAT |

### ML/Research (public, mostly archived)

| Repo | GitHub | Visibility | Status |
|------|--------|------------|--------|
| batchalign2 | TalkBank/batchalign2 | **Public** | Legacy (superseded by batchalign3) |
| chat-whisper | TalkBank/chat-whisper | **Public** | Whisper fine-tuning on CHAT |
| whisper-timestamped | TalkBank/whisper-timestamped | **Public** | Fork — word-level timestamps |
| utterance-tokenizer | TalkBank/utterance-tokenizer | **Public** | CHAT utterance tokenizer |
| testchat | TalkBank/testchat | **Public** | CHAT test files |

### Utility Scripts (private, mostly one-off)

| Repo | GitHub | Visibility | Last Pushed | Purpose |
|------|--------|------------|-------------|---------|
| talkbank-browser-check | TalkBank/talkbank-browser-check | Private | 2025-06 | **Active** — link/404 checking with login support; ~/talkbank-browser-check/ |
| check-chat-bookmarks | TalkBank/check-chat-bookmarks | Private | 2024-07 | Bookmark validation |
| squash-compound-words | TalkBank/squash-compound-words | Private | 2022-07 | CHAT compound words |
| rename-by-age | TalkBank/rename-by-age | Private | 2022-02 | File renaming |
| git-rename-case | TalkBank/git-rename-case | Private | 2022-02 | Case-sensitive rename |
| reorganize-lena-data | TalkBank/reorganize-lena-data | Private | 2016-12 | LENA data |
| random-scripts | TalkBank/random-scripts | Private | 2016-03 | Misc scripts |
| update_chat_types | TalkBank/update_chat_types | Private | 2020-04 | Older version of update-chat-types |
| check_chat_html | TalkBank/check_chat_html | Private | 2019-12 | Older version of check-chat-bookmarks |
| test | TalkBank/test | Private | 2025-05 | Test repo |
| psyling-web | TalkBank/psyling-web | Private | 2026-03 | Psyling website |

### Archived

| Repo | GitHub | Status |
|------|--------|--------|
| batchalign | TalkBank/batchalign | **Archived** — superseded by batchalign2 → batchalign3 |
| shiny-vagrant | TalkBank/shiny-vagrant | Inactive since 2020 |

---

## 2. Web Repositories (GitHub)

The TalkBank website is a set of per-bank repos, each containing static site content.

| Repo | GitHub | Visibility | Description |
|------|--------|------------|-------------|
| talkbank-web | TalkBank/talkbank-web | Private | Main talkbank.org site |
| childes-bank | TalkBank/childes-bank | Private | childes.talkbank.org |
| aphasia-bank | TalkBank/aphasia-bank | Private | aphasia.talkbank.org |
| dementia-bank | TalkBank/dementia-bank | Private | dementia.talkbank.org |
| asd-bank | TalkBank/asd-bank | Private | asd.talkbank.org |
| biling-bank | TalkBank/biling-bank | Private | biling.talkbank.org |
| ca-bank | TalkBank/ca-bank | Private | ca.talkbank.org |
| class-bank | TalkBank/class-bank | Private | class.talkbank.org |
| fluency-bank | TalkBank/fluency-bank | Private | fluency.talkbank.org |
| homebank-bank | TalkBank/homebank-bank | Private | homebank.talkbank.org |
| motor-bank | TalkBank/motor-bank | Private | motor.talkbank.org |
| phon-bank | TalkBank/phon-bank | Private | phonbank.talkbank.org |
| psychosis-bank | TalkBank/psychosis-bank | Private | psychosis.talkbank.org |
| rhd-bank | TalkBank/rhd-bank | Private | rhd.talkbank.org |
| samtale-bank | TalkBank/samtale-bank | Private | samtale.talkbank.org |
| slabank-bank | TalkBank/slabank-bank | Private | slabank.talkbank.org |
| tbi-bank | TalkBank/tbi-bank | Private | tbi.talkbank.org |

Local checkout: `~/web/` (mani-managed workspace with all bank repos as sub-repos).

---

## 3. Corpus Data Repositories (GitLab → GitHub migration in progress)

Originally 16 repos on **git.talkbank.org** (self-hosted GitLab). Being split into 24 repos and migrated to GitHub. 24 empty private repos created on GitHub (2026-03-18). See `docs/migration/implementation-plan.md` for the full plan.

### Current state: GitLab (16 repos, being decommissioned)

| Repo | Status |
|------|--------|
| childes-data | Splitting → 4 repos |
| phon-data | Splitting → 2 repos |
| ca-data | Splitting → 2 repos |
| homebank-data | Splitting → 4 repos |
| aphasia-data, asd-data, biling-data, class-data, dementia-data, fluency-data, motor-data, psychosis-data, rhd-data, samtale-data, slabank-data, tbi-data | Migrating as-is |

### Target state: GitHub (24 repos)

| Repo | GitHub | Bank | Contents | Size |
|------|--------|------|----------|------|
| aphasia-data | TalkBank/aphasia-data | aphasia | All | 94 MB |
| asd-data | TalkBank/asd-data | asd | All | 56 MB |
| biling-data | TalkBank/biling-data | biling | All | 122 MB |
| ca-candor-data | TalkBank/ca-candor-data | ca | CANDOR only | 4.8 GB |
| ca-data | TalkBank/ca-data | ca | Everything except CANDOR | 300 MB |
| childes-eng-na-data | TalkBank/childes-eng-na-data | childes | Eng-NA, Eng-AAE | 766 MB |
| childes-eng-uk-data | TalkBank/childes-eng-uk-data | childes | Eng-UK, Clinical-Eng, Clinical-Other | 1.2 GB |
| childes-romance-germanic-data | TalkBank/childes-romance-germanic-data | childes | French, Romance, Spanish, German, DutchAfrikaans, Scandinavian, Celtic | 862 MB |
| childes-other-data | TalkBank/childes-other-data | childes | Biling, Chinese, EastAsian, Japanese, Slavic, Other, etc. | 1.5 GB |
| class-data | TalkBank/class-data | class | All | 83 MB |
| dementia-data | TalkBank/dementia-data | dementia | All | 80 MB |
| fluency-data | TalkBank/fluency-data | fluency | All | 313 MB |
| homebank-public-data | TalkBank/homebank-public-data | homebank | Public + Secure | 719 MB |
| homebank-cougar-data | TalkBank/homebank-cougar-data | homebank | Password/Cougar | 5.4 GB |
| homebank-bergelson-data | TalkBank/homebank-bergelson-data | homebank | Password/Bergelson | 3.5 GB |
| homebank-password-data | TalkBank/homebank-password-data | homebank | Password/ remainder | 4.3 GB |
| motor-data | TalkBank/motor-data | motor | All | 4 KB |
| phon-eng-french-data | TalkBank/phon-eng-french-data | phon | Eng-NA, French | 2.7 GB |
| phon-other-data | TalkBank/phon-other-data | phon | All other languages | 2.6 GB |
| psychosis-data | TalkBank/psychosis-data | psychosis | All | 87 MB |
| rhd-data | TalkBank/rhd-data | rhd | All | 19 MB |
| samtale-data | TalkBank/samtale-data | samtale | All | 2.4 MB |
| slabank-data | TalkBank/slabank-data | slabank | All | 221 MB |
| tbi-data | TalkBank/tbi-data | tbi | All | 38 MB |

---

## 4. Legacy Documentation (~/Dropbox/documentation/)

84 files (~5,800 lines) covering infrastructure, servers, deployment, and development stack.
Git repo, last updated Feb 2025. **Must be transferred to talkbank-dev and updated.**

### By Topic

**Infrastructure & Servers** (15 docs):
servers, server-setup, cloud, non-cloud-servers, gandalf, media-server, homebank, ssl (obsolete),
certbot, certificates, apache-httpd (496 lines), tomcat, new-machine-setup, shibboleth (obsolete)

**Git & Version Control** (8 docs):
git, git-server, github, gitlab, gitlab-ci, gitlab-docker, gitlab-no-docker-ce, bitbucket (obsolete)

**CI/CD & Deployment** (8 docs):
continuous-delivery, continuous-integration, automated-build, continuous-deployment,
automated-deployment, manual-deployment, gitlab-ci, logging

**Data Management & Metadata** (8 docs):
development, datacite (DOI management), pid, cmdi, metadata, oai, update-chat-types, transforming-data

**Development Stack** (13 docs):
stack, java, python (outdated — says 3.7), php, javascript, typescript, rust (9 lines!),
r, graphql, docker, kubernetes, scons, homebrew, vagrant

**Operations** (13 docs):
macos, ubuntu, red-hat, ssh, credentials, security, virtualbox, resize, remote-access,
development-environment, slack, monitoring, mirrors

**Testing & QA** (5 docs):
testing, checkers, web-site-checkers, external-contributions, send2clan

**Applications** (4 docs):
talkbank-browser, browser-clan, talkbank-db, cgi

**CHAT Format** (5 docs):
clan, clan-info, chatter (203 lines — ANTLR 3 architecture), talkbank-api, xml-schema

### Obsolete/Superseded

| Document | Why |
|----------|-----|
| ssl.md | Replaced by certbot.md |
| shibboleth.md | Authentication system retired |
| bitbucket.md | Migrated away |
| python.md | References Python 3.7 (2020) |
| rust.md | 9 lines, superseded by talkbank-tools docs |
| chatter.md | Describes Java/ANTLR 3 chatter, superseded by Rust chatter |

---

## 5. Experimental/Archived Directories in ~/

These are local-only directories, mostly prototypes and experiments. Candidates for cleanup/archival.

### Parser Experiments (all superseded by talkbank-tools)
```
~/chat-antlr4-parser/    ~/chat-grmtools/       ~/chat-logos/
~/chat-nom/              ~/chat-parser/          ~/chat-regex/
~/chat-lexer/            ~/rust-chat-parser/     ~/talkbank-chat-antlr/
~/talkbank-chat-chumsky/
```

### Chatter Variants (all superseded by Rust chatter)
```
~/chatter/               ~/chatter-gui/         ~/chatter-utils/
~/java-chatter/          ~/java-chatter-antlr3-cleanup/
~/java-chatter-llm/      ~/java-chatter-main/   ~/java-chatter-stable/
~/new-chatter-demo/
```

### Batchalign Variants (all superseded by batchalign3)
```
~/batchalign-benchmarking/    ~/batchalign-core/
~/batchalign-next/            ~/batchalign-next-from-net/
~/batchalign2-bench-baseline/ ~/batchalign2-master/
```

### send2clan Variants (consolidated into talkbank-tools)
```
~/send2clan/             ~/send2clan-java/      ~/send2clan-macos/
~/send2clan-old/         ~/send2clan-rust/       ~/old-send2clan/
```

### Other Local Experiments
```
~/clan-app/              ~/codemirror-lang-talkbank/
~/tree-sitter-grammar-utils/  ~/talkbank-corpus-analysis/
~/chat-validator-gui/
~/talkbank-utils/             ~/talkbank-utils-cleanup/
~/micase-to-chat/             ~/generate-from-chat/
~/talkbank-web-config/        ~/talkbank-browser-check/
~/talkbank-manifests/
```

### Archived Snapshots
```
~/old-talkbank/          ~/saved-talkbank/
~/dali.talkbank.org/     ~/homebank.talkbank.org-web-setup/
~/testphontalk/          ~/testchat/            ~/minimal-test-chat/
```

---

## 6. Environment Variables

| Variable | Default | Purpose |
|----------|---------|---------|
| `TALKBANK_DATA_ROOT` | `~/data` | Corpus data root |
| `CHAT_HTML_PATH` | unset | Local chat.html for anchor checks |
| `CHAT_HTML_URL` | TalkBank URL | Remote chat.html source |
| `GITHUB_ACTOR` | — | GitHub Packages auth (for Phon/PhonTalk builds) |
| `GITHUB_TOKEN` | — | GitHub Packages auth |

---

## 7. Servers & Infrastructure

| Host | Purpose | Status |
|------|---------|--------|
| git.talkbank.org | Self-hosted GitLab (data repos, deploy) | **Decommissioning** |
| homebank.talkbank.org | Ubuntu server: Apache, Tomcat, OAI, GitHub Actions runner | Active |
| gandalf.talkbank.org | Brian's Mac: Apache, Shiny (Vagrant), media source | Active |
| media.talkbank.org | Media file server (synced from gandalf) | Active |
| sla.talkbank.org/TBB/ | TalkBank Browser node app (reads repo clones) | Active |
| talkbank.org + 15 subdomains | Per-bank websites (served from `-bank` repos) | Active |

---

## 8. Proposed talkbank-dev Layout (Post-Consolidation)

```
~/talkbank-dev/                     # This repo (private, tracked in git)
├── CLAUDE.md                       # Private workspace guidance
├── Makefile                        # Cross-repo commands
├── analysis/                       # Audits and reports
├── archive/                        # Archived docs and code
├── deploy/                         # Batchalign deploy (Ansible, scripts)
├── docs/                           # Internal docs
│   ├── build-notes/                # Phon/PhonTalk build instructions
│   ├── inventory.md                # THIS FILE
│   └── legacy/                     # ← transferred from ~/Dropbox/documentation/
│       ├── infrastructure/         # servers, apache, ssl, certbot, ...
│       ├── deployment/             # CI/CD, deploy, gitlab-ci, ...
│       ├── data-management/        # datacite, pid, cmdi, oai, ...
│       └── development/            # stack, languages, tools, ...
├── known-issues/                   # Validation baselines
├── ops/                            # Operational scripts
├── scripts/                        # Maintenance and analysis scripts
│
├── talkbank-tools/                 # Public (gitignored, cloned)
├── batchalign3/                    # Public (gitignored, cloned)
├── staging/                        # Deploy orchestration (gitignored, cloned)
├── webdev/                         # Web config/deploy (gitignored, cloned)
├── web/                            # Bank websites (gitignored, mani-managed)
├── OSX-CLAN/                       # Legacy CLAN (gitignored, cloned)
├── clan-info/                      # CLAN materials (gitignored, cloned)
├── gra-cgi/                        # GRA diagram CGI (gitignored, cloned)
├── phon/                           # Collaborator (gitignored, cloned)
├── phontalk/                       # Collaborator (gitignored, cloned)
│
└── data/                           # Corpus data (gitignored, cloned from GitLab→GitHub)
    ├── childes-data/
    ├── phon-data/
    ├── ca-data/
    └── ... (16 repos)
```

---

## 9. Migration Roadmap

### Phase 1: Workspace Consolidation (talkbank-dev)
- [ ] Absorb talkbank-private (**done**)
- [ ] Transfer ~/Dropbox/documentation/ → docs/legacy/
- [ ] Update Makefile with full `make clone` targets
- [ ] Update .gitignore for all sibling repos
- [ ] Rename ~/talkbank/ → ~/talkbank-dev/
- [ ] Create GitHub repo TalkBank/talkbank-dev (private)

### Phase 2: GitLab → GitHub Data Migration
- [ ] Boss sign-off on repo split proposals (childes, phon, ca, homebank)
- [ ] GitHub Campus Program application (50 GB LFS, 50K Actions min/month)
- [ ] Pilot with one small repo (tbi-data)
- [ ] Migrate remaining 15 data repos
- [ ] Update data repo remotes in workspace

### Phase 3: Deploy Modernization
- [ ] Replace `deploy` command with GitHub Actions
- [ ] Consolidate DOI management (3 copies of cdcfile.py → 1)
- [ ] Implement pre-commit hooks for data repos:
  - [ ] `update-chat-types` — already Rust with bootstrap.sh; wire into all data repos
  - [ ] `save-word-html-pdf` — Word→HTML/PDF export for doc changes
  - [ ] File size limits, media extension blocking
  - [ ] DOI duplicate checking
  - [ ] `chatter validate` on changed .cha files

### Phase 4: Decommission
- [ ] Move TBB node app to homebank
- [ ] Decommission git.talkbank.org
- [ ] Archive ~/Dropbox/documentation/ (replaced by docs/legacy/)
- [ ] Clean up experimental directories in ~/

---
Last Updated: 2026-03-10
