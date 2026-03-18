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
| Pre-commit hooks handle data quality | Phase 3 | No manual validation steps |
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

## Open Questions for Brian

1. Has he identified potential successors? (Which professors, which universities?)
2. Who owns the talkbank.org domain? What registrar?
3. What is the DataCite account situation? (Who can transfer it?)
4. What is the CMU Campus Cloud billing situation? (Will CMU continue hosting after retirement?)
5. Does he want CLAN to be frozen or community-maintained?
6. What contributor relationships are most critical to transfer?
