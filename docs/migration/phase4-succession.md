# Phase 4: Succession & Handover Planning

**Status:** Draft
**Last updated:** 2026-03-18

**Timeline:** 3-5 years (aligned with Brian's retirement)

**Context:** Brian MacWhinney (~80) will retire, and the entire core team — Leonid Spektor, Davida Fromm, Franklin Chen, and likely John Kowalski — will leave simultaneously. TalkBank must be in a state where an external successor (a professor in the TalkBank user community) can take over with minimal institutional knowledge transfer.

---

## The Problem

TalkBank today depends on:
- **Brian** — PI, sole authority on CLAN manual, corpus curation standards, contributor relationships, DOI management
- **Leonid** — sole author of CLAN C/C++ source code (OSX-CLAN, Windows-CLAN)
- **Chen** — sole operator of infrastructure (servers, deploy pipeline, Rust tooling, all the migration work in this doc)
- **Davida** — corpus data management, batchalign operations
- **John** — TBB app, TalkBankDB, auth system, SLA servers

When everyone leaves at once, a successor inherits:
- 24+ data repos with 100K+ CHAT files
- 17 web repos
- Rust tooling (talkbank-tools, batchalign3)
- Server infrastructure (talkbank.org, net, media server)
- Relationships with hundreds of contributors worldwide
- DOI obligations at DataCite
- CLAN legacy (C/C++ codebase nobody else can maintain)

---

## Guiding Principle

**Every system must be operable by someone who has never met us.**

This means:
- No tribal knowledge required
- No SSH access to specific machines required for routine operations
- All processes documented and automated
- All credentials in recoverable, documented locations
- All code in public repos with contribution guides

---

## What Must Be True Before Handover

### 1. Infrastructure is Self-Sustaining

| Requirement | Status | Notes |
|------------|--------|-------|
| All repos on GitHub (public where possible) | Phase 2 in progress | Data repos private (sensitive materials) |
| No self-hosted servers required for git | After Phase 2 | GitHub handles all git hosting |
| Deploy is automated (GitHub Actions) | Phase 3 | No human-operated deploy scripts |
| Pre-commit hooks handle data quality | Phase 3 | No manual validation steps. Includes `check-media` (Rust, own repo) for media ↔ CHAT validation. |
| DOI management is documented and scriptable | Phase 3 | New tool replaces 3 divergent copies |
| Media server has a maintenance plan | TODO | net is CMU hardware — what happens when CMU reclaims? |
| talkbank.org VM has a maintenance plan | TODO | CMU Campus Cloud — who pays after Brian? |
| SSL certs auto-renew | TODO | Currently Let's Encrypt? Or manual? |
| DNS is documented | TODO | Who controls talkbank.org DNS? |

### 2. Code is Maintainable Without Us

| Requirement | Status | Notes |
|------------|--------|-------|
| talkbank-tools: public, documented, CI passing | In progress | Book, spec, tests |
| batchalign3: public, documented, CI passing | In progress | Book, migration docs |
| CLAN C/C++: succession plan for legacy code | TODO | Leonid is sole maintainer. Code is complex, old (VS 2013, MFC). Options: freeze, community fork, or rewrite in Rust (already partially done in talkbank-tools) |
| TBB/TalkBankDB: documented, transferable | TODO | John's code — needs documentation and handover guide |
| Pre-commit hooks: installable via standard tooling | Phase 3 | pip/uv installable, or simple bash scripts |

### 3. Institutional Knowledge is Documented

| Knowledge | Where to Document | Status |
|-----------|------------------|--------|
| How to add a new corpus | Contributor guide | TODO |
| How to add a new bank | Ops runbook | TODO |
| How to mint DOIs | DOI tool docs | Phase 3 |
| How to manage media files | Media server docs | Partial (docs/media-server.md) |
| How to update CLAN binaries | CLAN maintenance guide | TODO |
| How to respond to contributor requests | Brian's workflow | Not documented |
| How to handle password-protected corpora | Access control guide | TODO |
| CHAT format specification | talkbank-tools/spec | In progress |
| Server access and credentials | Ops runbook | Partial (deploy/docs/) |
| DataCite account ownership | Transfer plan | TODO |
| DNS registrar access | Transfer plan | TODO |
| CMU Campus Cloud account | Transfer plan | TODO |
| GitHub org ownership | Transfer plan | TODO |
| Who pays for what | Financial handover | TODO |

### 4. External Successor Can Operate Day 1

The successor (an external professor) likely has:
- Graduate students who can code
- No knowledge of our infrastructure
- No access to CMU systems
- Familiarity with CHAT files and CLAN (they're a user)

They need:
- GitHub org admin access
- Server access (or a way to avoid needing servers)
- DOI management credentials (DataCite)
- DNS control
- A clear "getting started as maintainer" guide

---

## Strategic Questions

### Q1: Can we eliminate servers entirely?

If everything is on GitHub with Actions, and John's app is either:
- Hosted on a managed platform (Vercel, Fly.io, etc.) instead of a VM
- Or replaced by static site generation

...then there's no server to maintain. The successor just needs GitHub.

**Media is the problem.** 20 TB on net, served via nginx. Options:
- **Cloud storage** (S3, Backblaze B2) with CDN — ongoing cost, but no server maintenance
- **University hosting** — successor's university provides storage
- **Internet Archive** — free, permanent, but less control
- **Keep net running** — requires CMU cooperation post-Brian

### Q2: Who owns the GitHub org after Brian?

GitHub org ownership must be transferred. Currently Brian (and Chen?) are org owners. Plan:
- Add successor as org owner before Brian retires
- Document the transfer process
- Ensure 2FA recovery codes are handed over

### Q3: Who owns the domain?

`talkbank.org` DNS — who is the registrar? Who pays? Must be transferred to successor or an institution that outlives Brian's CMU appointment.

### Q4: What happens to CLAN?

Options:
- **Freeze:** Ship final binaries, document "CLAN is no longer maintained, use talkbank-tools"
- **Community fork:** Open source it (it already is, on GitHub), hope someone maintains it
- **Rust replacement:** talkbank-tools already replaces most CLAN functionality. Complete the remaining commands (see talkbank-clan/book/src/divergences/command-parity-audit.md)

### Q5: What about batchalign?

batchalign3 depends on ML models (Whisper, Stanza, etc.) that evolve. Without active maintenance:
- **Pin model versions** and document them
- **Containerize** (Docker) so it can be run anywhere without dependency hell
- **Publish to PyPI/crates.io** so users can install without building from source

### Q6: What about ongoing contributor relationships?

Brian personally manages relationships with hundreds of corpus contributors worldwide. They submit data, request access, report issues. This social infrastructure doesn't transfer via documentation. The successor needs:
- A list of active contributors and their corpora
- Templates for common interactions (new corpus onboarding, access requests)
- The mailing list / communication channels

---

## Concrete Deliverables (Start Now)

These are things we can start working on immediately, independent of the migration:

### 1. Ops Runbook

A single document: "How to operate TalkBank infrastructure." Covers:
- Server inventory (what runs where, how to access)
- How to deploy (after Phase 3: just push to GitHub)
- How to add a new corpus / bank
- How to manage DOIs
- How to manage media
- How to handle access requests
- Troubleshooting common issues

### 2. Financial / Account Inventory

| Account | Owner | Cost | Transfer Plan |
|---------|-------|------|---------------|
| GitHub org (TalkBank) | Brian? Chen? | Free (private repos) | Add successor as owner |
| DataCite | Brian (SML.TALKBANK) | ? | Transfer to successor's institution |
| talkbank.org domain | ? | ? | Transfer registrar to successor |
| CMU Campus Cloud VMs | Brian's CMU account | ? | Successor's university provides equivalent |
| Let's Encrypt certs | Automated | Free | Tied to server — moves with VM |
| Apple Developer (code signing) | ? | $99/year | Transfer or successor creates own |
| Media server hardware | CMU/Brian | ? | Successor's university or cloud storage |

### 3. "Getting Started as TalkBank Maintainer" Guide

Target audience: the external professor who takes over. Written in plain language. Covers:
- What TalkBank is and how it works
- What you inherited (repos, servers, data)
- How to do common tasks
- Who to contact for what
- What costs money and how to pay for it

### 4. Containerization

Docker images for:
- batchalign3 (ML models included)
- talkbank-tools CLI
- The web stack (nginx + data repos + John's app)

Reduces "works on our machines" risk to zero.

### 5. Public Documentation

- talkbank-tools book (in progress)
- batchalign3 book (in progress)
- CHAT specification (in talkbank-tools/spec)
- Contributor guide (TODO)

---

## Timeline

| When | What |
|------|------|
| Now | Start ops runbook, financial inventory, containerization investigation |
| After Phase 3 | Complete ops runbook, "getting started" guide |
| 1-2 years before retirement | Identify successor, begin relationship transfer |
| 6 months before retirement | Transfer all accounts, train successor |
| Retirement | Handover complete, team available for questions (limited) |

---

## Technical North Star: One-Command TalkBank Instance

**Goal:** `docker compose up` (or equivalent) stands up a complete, working TalkBank — web sites, data browsing, media serving, batchalign — from scratch, on any machine, by anyone.

### What This Requires

**1. Infrastructure-as-code for the web stack**

Everything on talkbank.org today is hand-configured. It needs to become a repo:

```
talkbank-infra/
├── docker-compose.yml          # Orchestrates everything
├── nginx/
│   ├── nginx.conf              # Generated from webdev/ templates
│   └── certs/                  # Let's Encrypt auto-renewal
├── tbb/                        # John's Node app (or successor)
├── runner/                     # GitHub Actions self-hosted runner config
└── README.md                   # "How to stand up TalkBank"
```

A successor clones this repo, sets a few environment variables (domain, media path), and `docker compose up` gives them a working TalkBank instance.

**2. Published packages**

| Package | Registry | Status |
|---------|----------|--------|
| talkbank-tools CLI (`chatter`) | crates.io + GitHub Releases (binaries) | TODO |
| batchalign3 CLI | PyPI (`batchalign3-cli`) + crates.io | In progress |
| batchalign3 Python (ML server) | PyPI (`batchalign3`) | In progress |
| Pre-commit hooks | pip-installable or standalone scripts | Phase 3 |

Users and the successor should be able to `pip install batchalign3` or `cargo install talkbank-cli` without cloning repos or building from source.

**3. Media strategy**

20 TB on a physical Mac Studio at CMU. Options ranked by reproducibility:

| Option | Cost | Reproducibility | Maintenance |
|--------|------|----------------|-------------|
| Cloud object storage (S3/B2) + CDN | ~$100-200/month | Excellent — anyone can replicate | Zero (managed) |
| University-hosted NAS | Depends on institution | Good — standard IT request | Low |
| Internet Archive | Free | Excellent — permanent | Zero (but slow, less control) |
| Physical Mac at successor's institution | One-time hardware | Poor — hardware-dependent | Medium |

Cloud storage is the most Phase 4-compatible. Can migrate incrementally: start mirroring from net to S3/B2 now, switch DNS later.

**4. Data pipeline reproducibility**

After Phase 3, the data pipeline is:
```
User pushes → pre-push hooks (local) → GitHub → Actions pulls on server → TBB reads clones
```

To make this reproducible for a successor:
- Pre-push hooks are installed via setup script (already planned)
- GitHub Actions workflows are in each repo (already planned)
- The self-hosted runner is part of the Docker Compose infra (see #1)
- No manual steps anywhere

**5. Batchalign containerization**

batchalign3 needs ML models (Whisper, Stanza, etc.) which are multi-GB downloads. A Docker image with models baked in means:
- `docker run talkbank/batchalign3 align input.cha` just works
- No CUDA/MPS/driver issues on the host
- Successor's grad students can run it without understanding the Python/Rust stack

### Phase 4 Design Principles

These should inform every decision made in Phases 1-3:

1. **No snowflakes.** Every server configuration must be reproducible from code. If a VM dies, we can rebuild it from a repo, not from someone's memory.

2. **No gatekeepers.** Every routine operation (adding a corpus, minting a DOI, deploying a change) must be possible without SSH access to a specific machine.

3. **No single points of failure.** No person, machine, or account whose loss makes TalkBank inoperable. Every credential has a documented recovery path.

4. **Prefer managed services over self-hosted.** GitHub over self-hosted GitLab. Cloud storage over local drives. Let's Encrypt over manual certs. Fewer things to maintain = fewer things that break when nobody's maintaining them.

5. **Prefer standard tooling over custom scripts.** Docker over bespoke deploy scripts. Pre-commit framework over custom hooks. Well-known CI patterns over clever pipelines.

### Decisions in Phases 1-3 That Advance Phase 4

| Decision | How it helps Phase 4 |
|----------|---------------------|
| Move to GitHub | Eliminates self-hosted git server. Successor just needs GitHub org access. |
| Pre-commit hooks replace deploy | No server-side deploy script. Pipeline works on any machine with hooks installed. |
| GitHub Actions for pulls | Standard CI, no custom orchestration. Self-hosted runner is commodity infrastructure. |
| Standardized workspace layout | Setup script works for anyone, anywhere. |
| Fresh git init (no history) | Smaller repos, faster clones, no archaeology needed. |
| John's dynamic ZIPs | Eliminates generated artifacts that need a build step. |
| Single DOI tool | One tool to maintain, not three. |

### Decisions to Avoid (Would Make Phase 4 Harder)

| Temptation | Why it hurts Phase 4 |
|------------|---------------------|
| Custom CI server (Jenkins, etc.) | One more thing to maintain; GitHub Actions is sufficient. |
| Hardcoding paths to CMU machines | Successor won't be at CMU. Use config/env vars. |
| Storing credentials in repos | Not transferable, not secure. Use secret managers or env vars. |
| Building tools that require our specific server setup | Docker/containers ensure portability. |
| Keeping any process that requires "ask Chen" | Document it or automate it. |

---

## Open Questions for Brian

1. Has he identified potential successors? (Which professors, which universities?)
2. Who owns the talkbank.org domain? What registrar?
3. What is the DataCite account situation? (Who can transfer it?)
4. What is the CMU Campus Cloud billing situation? (Will CMU continue hosting after retirement?)
5. Does he want CLAN to be frozen or community-maintained?
6. What contributor relationships are most critical to transfer?
