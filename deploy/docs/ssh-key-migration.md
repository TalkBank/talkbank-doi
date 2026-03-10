# SSH Key & Access Migration

**Status: IN PROGRESS — partially complete as of 2026-02-20.**

This document records TalkBank's SSH key migration from the legacy shared-key
model to per-machine Ed25519 keys + Tailscale networking. It covers what was
done, what is temporary, and what remains.

---

## Table of Contents

1. [Background](#1-background)
2. [What Was Done (2026-02-20)](#2-what-was-done-2026-02-20)
3. [Current State](#3-current-state)
4. [Temporary Hacks to Undo Later](#4-temporary-hacks-to-undo-later)
5. [Not Yet Done](#5-not-yet-done)
6. [Mistakes Made & Lessons](#6-mistakes-made--lessons)
7. [Reference: Tailscale Configuration](#7-reference-tailscale-configuration)
8. [Reference: Machine Inventory](#8-reference-machine-inventory)
9. [Reference: GitLab Users & Keys](#9-reference-gitlab-users--keys)
10. [Reference: GitHub Keys](#10-reference-github-keys)
11. [Appendix: Installing Tailscale on Ubuntu Servers](#appendix-a-installing-tailscale-on-ubuntu-servers)
12. [Appendix: macOS Tailscale GUI Limitation](#appendix-b-macos-tailscale-gui-limitation)

---

## 1. Background

TalkBank's SSH infrastructure accumulated 20+ years of access debt:
- A **1024-bit RSA key from 2003** (`chen@swan.psy.cmu.edu`) on Franklin's GitHub
  and in `authorized_keys` on every machine
- A **shared 2048-bit RSA key** (`macw@BRIAN.TALKBANK.ORG`) copied to every lab Mac
  as `~/.ssh/id_rsa`, with the passphrase loaded by `keychain` in `.zshenv`
- `ForwardAgent yes` on cloud servers
- Plaintext GitHub tokens in `~/.gitconfig`
- `HostKeyAlgorithms +ssh-rsa` / `PubkeyAcceptedKeyTypes +ssh-rsa` everywhere
- No key rotation in 7+ years

The goal: replace all of this with per-machine Ed25519 keys, per-person Git
service accounts, and Tailscale SSH where possible.

---

## 2. What Was Done (2026-02-20)

### 2.1 Tailscale on Cloud Servers

Installed Tailscale (via apt, Ubuntu 25.10 questing) on both cloud servers:

| Server | Tailscale hostname | Tailscale IP | Tailscale SSH |
|--------|-------------------|-------------|:---:|
| git.talkbank.org | git-talkbank | 100.108.195.47 | **working** |
| talkbank.org | talkbank | 100.72.29.3 | **working** |

These are Linux CLI installs, so Tailscale SSH works. See [Appendix A](#appendix-a-installing-tailscale-on-ubuntu-servers) for install steps.

### 2.2 Tailscale ACL Policy

Configured SSH ACLs via the Tailscale API. All TalkBank lab machines and
infrastructure servers are tagged `tag:talkbank`. Franklin's personal machines
(ming, franklin, chen, yoga) are **untagged** — they can SSH out to tagged
machines but nobody can SSH into them.

See [Section 7](#7-reference-tailscale-configuration) for the full ACL policy.

### 2.3 Cloud Server SSH Config Hardening

**git.talkbank.org** (`macw@git-talkbank`):
- Changed `ForwardAgent yes` → `ForwardAgent no`
- Added routing: `Host homebank.talkbank.org` → `Hostname talkbank` (Tailscale)

**talkbank.org** (`macw@talkbank`):
- Changed `ForwardAgent yes` → `ForwardAgent no`
- Removed `StrictHostKeyChecking no`
- Kept `github_deploy` key for GitHub Actions runner

### 2.4 Franklin's New Personal Key

Generated Ed25519 key on ming:
```
ssh-ed25519 AAAAC3NzaC1lZDI1NTE5AAAAIDJEanwXGZ3EtLYB3fnuCrKZOf0sr8uKX52f2PjEvIYJ franklin@talkbank.org
```

- Deployed to all 4 personal machines (ming, franklin, chen, yoga)
- Deployed to `authorized_keys` on all reachable lab Macs
- Registered on Franklin's GitHub account (id 143496212)
- Registered on Franklin's GitLab account (id 7)

**ming SSH config** (`~/.ssh/config`):
```
Host github.com
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host gitlab.talkbank.org
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host *
    AddKeysToAgent yes
```

### 2.5 Removed from Franklin's GitHub

- `chen@swan.psy.cmu.edu` (1024-bit RSA, key ID 175862) — **deleted**
- `GitHub CLI` (RSA, key ID 132644627) — **deleted**

### 2.6 Removed from Franklin's GitLab

- `chen@swan.psy.cmu.edu` (old RSA) — **deleted**
- `macw@BRIAN.TALKBANK.ORG` (shared key) — **deleted**
  (**This broke GitLab for everyone** — see [Section 6](#6-mistakes-made--lessons))

### 2.7 Plaintext Token Cleanup

- Removed `github.token` and `github.oauth-token` from `~/.gitconfig` on ming
- Removed plaintext GitHub tokens from `~/.gitconfig` on bilbo and frodo

### 2.8 Keychain Removal

Removed `eval \`keychain --quiet --eval ...\`` from `~/.zshenv` on all
reachable machines. This was loading the old RSA keys into the SSH agent.

**Machines cleaned:** ming, franklin, chen, yoga, net, bilbo, brian, davida,
frodo, study, andrew, lilly, sue, vaishnavi, tb-hk.

### 2.9 Per-Machine Ed25519 Keys Generated

Generated `~/.ssh/id_ed25519` (no passphrase, comment `macw@talkbank.org`) on
every reachable lab Mac:

| Machine | Key fingerprint | Generated |
|---------|----------------|:---------:|
| brian | SHA256:WJp8borTceZ50D9QNkn74Kmu8tCLXQu40qA0HozwCA0 | earlier session |
| net | SHA256:+1Httk1neWOzG3bW7dU1Mpzp7ab7szr4v+yfSROWQ08 | 2026-02-20 |
| bilbo | SHA256:XEcIxTqtzOaWvQKLr5iTY9c0Lz2ygDjg/Bx2CeG3fSU | 2026-02-20 |
| davida | SHA256:l9FwiBeV2YedaKL1ISMdZcu7i2QEeN1kailVezcHT54 | 2026-02-20 |
| frodo | SHA256:KQiJBANMpW06ndbzMjQEB0BrA024LpbNQ6u5K1ANG9A | 2026-02-20 |
| study | SHA256:bscsZMGjc/k6xviKlq8MXUaEV+vLthMw1U/1L/0X5F8 | 2026-02-20 |
| andrew | SHA256:Cl86KICtD/g0wfFW9tcLcWHJVcFT4X4yqfjvx8lf4P0 | 2026-02-20 |
| lilly | SHA256:Iivph6GAN0MOqFp87SiqQSG7zqviyvJwUtANidyxWKU | 2026-02-20 |
| sue | SHA256:oYM9m8ryr+aBshxG2sRetc8ot83NhfYGhT44N+QIfS4 | 2026-02-20 |
| vaishnavi | SHA256:WcDBTiCFyw4vUpXCxu7yL8ipvq3kRghmh8u/Xnp+8lg | 2026-02-20 |
| tb-hk | SHA256:Nvtnxdn/tRh0r430RQK9CeSw+qWk4pNFjSqcOp8J+aw | 2026-02-20 |

**Not generated:** cbs, monkey (SSH auth failures), extra, hermes (offline/timeout).

### 2.10 SSH Config Updated on All Lab Macs

Every reachable lab Mac now has this `~/.ssh/config`:

```
Host github.com
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host gitlab.talkbank.org
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host *
    AddKeysToAgent yes
```

(The RSA fallback was removed on 2026-02-20 — see [Section 4.1](#41-github-rsa-fallback--removed-2026-02-20).)

The old `HostKeyAlgorithms +ssh-rsa` and `PubkeyAcceptedKeyTypes +ssh-rsa`
lines were removed from all machines.

### 2.11 SSH Agent Bridge

Added to `~/.zshenv` on all lab Macs so the macOS SSH agent is accessible in
remote SSH sessions (replacing what `keychain` used to do):

```bash
# Bridge macOS SSH agent to SSH sessions
if [ -z "$SSH_AUTH_SOCK" ]; then
  export SSH_AUTH_SOCK=$(ls /private/tmp/com.apple.launchd.*/Listeners 2>/dev/null | head -1)
fi
```

### 2.12 GitLab Keys Registered

Used the GitLab admin API (token scope: `api`) to register each machine's
Ed25519 key on the correct user's GitLab account:

| Machine | GitLab user | GitLab key ID |
|---------|-----------|:---:|
| brian | @macw (Brian MacWhinney) | 8 |
| study | @macw | 9 |
| tb-hk | @macw | 10 |
| net | @chen (Franklin Chen) | 11 |
| bilbo | @chen | 12 |
| frodo | @chen | 13 |
| davida | @fromm (Davida Fromm) | 14 |
| andrew | @ayankes (Andrew Yankes) | 15 |
| lilly | @lillirighter (Lilli Righter) | 16 |
| sue | @sh4s (Sue Holm) | 17 |
| vaishnavi | @velanchen (Vaishnavi Elanchezhian) | 18 |

**Verified working:** `ssh -T git@gitlab.talkbank.org` returns the correct
`Welcome to GitLab, @username!` from every machine. Real `git fetch` tested
from net (aphasia-data) and davida (aphasia-data).

### 2.13 GitLab Accounts Created

Three new GitLab accounts were created for users who didn't have one:

| Username | Name | Email | Notes |
|----------|------|-------|-------|
| lillirighter | Lilli Righter | lillirighter@talkbank.org | Placeholder email — update later |
| sh4s | Sue Holm | sh4s@andrew.cmu.edu | |
| velanchen | Vaishnavi Elanchezhian | velanchen@andrew.cmu.edu | |

**These users do not currently use Git.** The accounts exist in case they need
GitLab access in the future. They have NOT been added to any GitLab projects.

### 2.14 GitHub Keys — Franklin's Machines

Franklin's frodo machine key is registered on his GitHub account:

| Machine | GitHub key title |
|---------|-----------------|
| frodo | frodo-machine (macw@talkbank.org) |

**Verified working:** `ssh -T git@github.com` returns `Hi FranklinChen!` from
frodo.

### 2.15 GitHub Keys — Brian's Machines

Brian updated his own GitHub account: removed the old RSA key (`macw@BRIAN`,
key ID 23196921) and added his new Ed25519 key from `brian`.

For machines Brian primarily uses (net, bilbo, study, tb-hk), we deployed
Brian's Ed25519 key pair as `~/.ssh/id_ed25519_github` and configured SSH to
use it for GitHub. These machines keep their own `id_ed25519` for GitLab.

| Machine | GitHub key | GitLab key | GitHub auth |
|---------|-----------|-----------|-------------|
| brian | `id_ed25519` (Brian's own) | `id_ed25519` | @macw |
| net | `id_ed25519_github` (copy of Brian's) | `id_ed25519` (machine-specific) | @macw |
| bilbo | `id_ed25519_github` (copy of Brian's) | `id_ed25519` (machine-specific) | @macw |
| study | `id_ed25519_github` (copy of Brian's) | `id_ed25519` (machine-specific) | @macw |
| tb-hk | not yet deployed (intermittently unreachable) | `id_ed25519` (machine-specific) | broken |

**Verified working:** `ssh -T git@github.com` returns `Hi macw!` from brian,
net, bilbo, and study.

### 2.16 Davida's GitHub Access

Davida's machine (`davida`) authenticates to GitHub via her **new Ed25519 key**,
which is registered on her own GitHub account (`davidafromm`, key ID 143499025).

**Verified working:** `ssh -T git@github.com` returns `Hi davidafromm!` from
davida.

### 2.17 git-talkbank Deploy Pipeline Fix

The staging deploy pipeline on git-talkbank broke at ~11:39 on 2026-02-20.
The deploy script does `git remote update` on local clones of GitLab repos
(e.g., `~/staging/repos/childes-data`), which connects to GitLab via SSH.

**Root cause:** The `GitLab` Ed25519 key on git-talkbank (from 2022, used by
GitLab Runner and the deploy pipeline) was previously registered on Franklin's
GitLab account. When we removed keys from Franklin's GitLab earlier in the day,
this key lost its registration. All deploys after 11:39 silently failed — the
log shows `git remote update` with no follow-up.

**Fix:** Re-registered git-talkbank's existing Ed25519 key on Franklin's GitLab
account (key ID 19, title "git-talkbank deploy (GitLab Runner)").

**Verified:** `git remote update` on `~/staging/repos/childes-data` succeeds.

**Second breakage:** The deploy pipeline also pulls from GitHub — the
`generate-from-chat` repo is at `git@github.com:TalkBank/generate-from-chat`.
With `ForwardAgent no` and no GitHub key configured, this also failed. Fixed by
deploying Brian's Ed25519 key as `~/.ssh/id_ed25519_github` on git-talkbank and
adding a `Host github.com` entry to the SSH config.

**git-talkbank SSH config** (final):
```
Host homebank.talkbank.org
    Hostname talkbank

Host github.com
    IdentityFile ~/.ssh/id_ed25519_github
    IdentitiesOnly yes

Host *
    ForwardAgent no
```

Deploys that failed between ~11:39 and ~16:00 on 2026-02-20 need to be re-run.

### 2.18 GitHub RSA Fallback Removed

Removed the dead `IdentityFile ~/.ssh/id_rsa` fallback for GitHub from SSH
configs on all reachable lab Macs. Brian had already removed his old RSA key
from GitHub and added his Ed25519, so the RSA line was just wasted round-trips.

**Removed from:** net, bilbo, brian, davida, frodo, study, andrew, lilly, sue,
vaishnavi. (tb-hk unreachable — still has the fallback line, harmless.)

### 2.19 Homebrew Installed on Remaining Machines

Four lab Macs (andrew, lilly, sue, vaishnavi) lacked Homebrew, needed for the
Tailscale CLI migration. Franklin installed brew on each by logging into the
admin account locally (the `macw` user doesn't have admin/sudo rights).

Added `eval "$(/opt/homebrew/bin/brew shellenv)"` to `~/.zshenv` on all four
machines so brew is in PATH for non-login SSH sessions.

### 2.20 Keychain Cleanup on Franklin's Home Machines

Found stale `eval \`keychain --quiet --eval id_rsa id_rsa_macw\`` in
`~/.zshenv` on `chen` and `franklin` (home machines missed in the earlier
keychain cleanup). Removed from both.

Added `eval "$(/usr/local/bin/brew shellenv)"` to `~/.zshenv` on `yoga`
(Intel Mac, brew was installed but not in PATH for SSH sessions).

### 2.21 Tailscale CLI Migration — Attempted on yoga (BLOCKED)

Attempted the first GUI-to-CLI Tailscale migration on yoga as a test case.

**Steps completed:**
1. Quit the Tailscale GUI app
2. `brew install tailscale` (CLI formula)
3. Removed conflicting cask symlinks, linked the CLI formula
4. `sudo rm -rf /Applications/Tailscale.app`
5. `sudo brew services start tailscale` — daemon started

**Blocked at:** `sudo tailscale up --ssh --accept-routes` fails with:
```
The Tailscale SSH server does not run in sandboxed Tailscale GUI builds.
```

**Root cause:** The GUI installs a **macOS System Extension**
(`io.tailscale.ipn.macsys.network-extension`, team ID `W5364U7YZB`) that
persists even after the GUI app is deleted. The brew daemon starts, but the
`tailscale` CLI still connects to the System Extension's tailscaled (v1.94.2)
instead of the brew daemon (v1.94.1). The System Extension refuses to enable
SSH because it thinks it's a sandboxed GUI build.

**Uninstalling the System Extension:**

`systemextensionsctl uninstall` does NOT work with SIP enabled (macOS 26):
```
At this time, this tool cannot be used if System Integrity Protection is enabled.
```

The **only way** to remove the System Extension is through the macOS GUI:
**System Settings > General > Login Items & Extensions > Network Extensions >
toggle off Tailscale > enter admin password.** This cannot be done over SSH.

**Current state of yoga:** Halfway migrated. The System Extension is still
running (Tailscale networking works), but the GUI app is deleted and the brew
CLI is installed. To finish, Franklin needs to physically go to yoga and:
1. System Settings > General > Login Items & Extensions > Network Extensions >
   toggle off Tailscale
2. Run `bash /tmp/finish-migrate.sh` (already on the machine)

### 2.22 Tailscale CLI Migration — Successful on frodo

**First fully successful migration.** Frodo is now running Tailscale SSH via
the brew CLI daemon. The procedure that worked:

**Phase 1 — Remote prep (from ming, no physical access needed):**

```bash
# 1. Quit the GUI app
ssh macw@frodo "osascript -e 'quit app \"Tailscale\"'"

# 2. Install CLI formula
ssh macw@frodo "brew install tailscale"

# 3. Remove old cask symlinks and link CLI
ssh macw@frodo "rm -f /opt/homebrew/bin/tailscale /opt/homebrew/bin/tailscaled && brew link tailscale"

# 4. Copy finish script to the machine
scp /tmp/finish-migrate.sh macw@frodo:/tmp/finish-migrate.sh
```

**Phase 2 — Physical access (at frodo's screen):**

1. Kill old processes and remove GUI app (in Terminal):
   ```bash
   sudo pkill -f tailscaled
   sudo rm -rf /Applications/Tailscale.app
   ```

2. Remove System Extension via GUI:
   **System Settings > General > Login Items & Extensions > Network Extensions**
   → toggle off **Tailscale** → enter admin password

3. Run finish script (in Terminal):
   ```bash
   bash /tmp/finish-migrate.sh
   ```
   This does: `sudo brew services start tailscale`, waits, then
   `sudo tailscale up --ssh --accept-routes --auth-key=<tagged key>`.
   Prompts for sudo password once.

**Phase 3 — Remote cleanup (from ming):**

```bash
# Delete the old GUI node from Tailscale (it appears as offline)
curl -s -X DELETE -u "$TAILSCALE_API_KEY:" \
  "https://api.tailscale.com/api/v2/device/OLD_DEVICE_ID"

# Rename the new node (it gets a -bp suffix to avoid collision)
curl -s -X POST -u "$TAILSCALE_API_KEY:" \
  -H "Content-Type: application/json" \
  -d '{"name":"frodo"}' \
  "https://api.tailscale.com/api/v2/device/NEW_DEVICE_ID/name"
```

**Verified working:** `ssh macw@frodo` from ming authenticates via Tailscale
SSH — no SSH keys involved. The node is tagged `tag:talkbank` and has the
Tailscale IP `100.78.37.95`.

**Key discovery:** The machine registers as a NEW Tailscale node (with a `-bp`
hostname suffix) because the CLI daemon has different state from the GUI app.
The old node must be deleted and the new one renamed via the Tailscale API.

### 2.23 Tailscale Reusable Auth Key Created

Created a pre-approved, reusable Tailscale auth key tagged `tag:talkbank` for
the lab machine migration (expires 2026-02-27):

```
tskey-auth-k1acv4nvg911CNTRL-PuyA8iWWRS6KsEFcCm9PS6nmz6zyBp7u
```

This key auto-tags devices as `tag:talkbank` and skips browser approval. Use
it for **lab machines only** — Franklin's personal machines (yoga, chen,
franklin) should join **untagged** (use browser-based auth instead).

### 2.23 Franklin's SSH Config — Default User for Lab Machines

_(Originally 2.18.)_ Added `User macw` default to ming's SSH config for all lab/infra hostnames,
so `ssh git-talkbank` works without specifying `-l macw`:

```
Host git-talkbank talkbank net bilbo brian davida frodo study andrew lilly sue vaishnavi tb-hk cbs monkey extra hermes
    User macw
```

Without this, Tailscale SSH rejects the connection because the ACL only allows
SSH as `macw`, not `chen`.

---

## 3. Current State

### 3.1 What Works

| Service | From machine | Status |
|---------|-------------|--------|
| **GitLab SSH** | brian, study, tb-hk, net, bilbo, frodo, davida, fromm, andrew, lilly, sue, vaishnavi | **Working** — new Ed25519 keys |
| **GitLab SSH** | git-talkbank (deploy pipeline) | **Working** — re-registered Ed25519 key (Section 2.17) |
| **GitHub SSH** | brian, net, bilbo, study | **Working** — Brian's Ed25519 key (@macw) |
| **GitHub SSH** | frodo | **Working** — machine Ed25519 on Franklin's account |
| **GitHub SSH** | davida, fromm | **Working** — Ed25519 on her own account (@davidafromm) |
| **GitHub SSH** | ming | **Working** — Franklin's personal Ed25519 |
| **Tailscale SSH** | git-talkbank, talkbank (Linux) | **Working** — CLI Tailscale install |
| **Tailscale SSH** | frodo (macOS, first Mac migrated) | **Working** — brew CLI, no SSH keys needed |
| **Machine-to-machine SSH** | all reachable Macs | **Working** — old keys still in authorized_keys |

### 3.2 What's Broken or Degraded

| Issue | Machines affected | Why |
|-------|------------------|-----|
| **GitHub SSH fails** | andrew, sue, vaishnavi | Ed25519 not on any GitHub account |
| **Unreachable** | extra | Connection timeout — offline or network issue |
| **Tailscale SSH not working on Macs** | all Macs | GUI System Extension must be removed physically (see [Section 5.8](#58-high-priority--finish-tailscale-gui--cli-migration)) |
| **yoga halfway migrated** | yoga | GUI deleted, CLI installed, but System Extension still active — finish tonight |

### 3.3 SSH Config on Each Machine Type

**Franklin's personal machines** (ming, franklin, chen, yoga):
```
Host github.com
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host gitlab.talkbank.org
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host *
    AddKeysToAgent yes
```

**Brian's machines** (net, bilbo, study — and tb-hk when reachable):
```
Host github.com
    IdentityFile ~/.ssh/id_ed25519_github       ← copy of Brian's key
    IdentitiesOnly yes

Host gitlab.talkbank.org
    IdentityFile ~/.ssh/id_ed25519              ← machine-specific key
    IdentitiesOnly yes

Host *
    AddKeysToAgent yes
```

**Other lab Macs** (brian, davida, frodo, andrew, lilly, sue, vaishnavi):
```
Host github.com
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host gitlab.talkbank.org
    IdentityFile ~/.ssh/id_ed25519
    IdentitiesOnly yes

Host *
    AddKeysToAgent yes
```

**Cloud servers** (git-talkbank, talkbank):
- `ForwardAgent no` (hardened)
- git-talkbank uses its 2022 Ed25519 key (`GitLab`) for deploy pipeline
- talkbank.org has `github_deploy` key for GitHub Actions runner

---

## 4. Temporary Hacks to Undo Later

### 4.1 ~~GitHub RSA Fallback~~ — REMOVED (2026-02-20)

**What was:** Lab Mac SSH configs listed `IdentityFile ~/.ssh/id_rsa` as a
fallback for `github.com`. This was dead weight — the old RSA key was no longer
registered on any GitHub account (Brian removed it when he added his Ed25519).

**Removed from:** net, bilbo, brian, davida, frodo, study, andrew, lilly, sue,
vaishnavi. tb-hk was unreachable — still has the fallback line (harmless).

### 4.2 Brian's Key Copied to Multiple Machines

**What:** Brian's Ed25519 private key is copied to net, bilbo, study, and
git-talkbank as `~/.ssh/id_ed25519_github`. This is the same key that lives on
`brian` as `~/.ssh/id_ed25519`.

**Why:** Brian primarily uses these machines for GitHub pushes. Rather than
registering each machine's unique key on Brian's GitHub account (he'd need to
do it himself for each one), we deployed his existing key.

**Risk:** If any of these machines is compromised, Brian's GitHub key is exposed.
This is the same risk model as the old shared macw key, but scoped to one person
and one service (GitHub only).

**To undo:** Generate unique keys per machine, register each on Brian's GitHub,
and delete `id_ed25519_github` from net/bilbo/study. For git-talkbank
specifically, replace with a dedicated deploy key (see [Section 4.5](#45-git-talkbank-uses-brians-key-for-github)). Low priority.

### 4.3 Old RSA Keys Still on All Machines

**What:** Every lab Mac still has `~/.ssh/id_rsa` (the shared
`macw@BRIAN.TALKBANK.ORG` key) and the old keys are still in `authorized_keys`.

**Why:** We didn't remove them because (a) the RSA key is still needed as a
GitHub fallback on some machines, and (b) removing `authorized_keys` entries
before everything is verified would risk lockouts.

**To undo:** After confirming all access paths work with Ed25519 keys only:

1. Remove old entries from `authorized_keys` on every machine:
   ```bash
   for host in net bilbo brian davida frodo study andrew lilly sue vaishnavi tb-hk; do
     ssh macw@$host "grep -v 'chen@swan\|macw@BRIAN' ~/.ssh/authorized_keys > ~/.ssh/authorized_keys.new && mv ~/.ssh/authorized_keys.new ~/.ssh/authorized_keys"
   done
   ```
2. Archive (don't delete) old key files on ming:
   ```bash
   mkdir -p ~/.ssh/archive
   mv ~/.ssh/id_rsa ~/.ssh/id_rsa.pub ~/.ssh/archive/
   mv ~/.ssh/id_rsa_macw ~/.ssh/id_rsa_macw.pub ~/.ssh/archive/
   mv ~/.ssh/github_rsa ~/.ssh/github_rsa.pub ~/.ssh/archive/
   mv ~/.ssh/libra_id_rsa ~/.ssh/libra_id_rsa.pub ~/.ssh/archive/
   ```

### 4.4 SSH Agent Bridge in .zshenv

**What:** Every lab Mac's `~/.zshenv` has a snippet that discovers the macOS
SSH agent socket for remote SSH sessions.

**Why:** We removed the `keychain` eval that used to do this. Without it, SSH
sessions from ming → lab Mac have no agent, so passphrase-protected keys
(the old RSA) don't work.

**To undo:** Once the old RSA keys are fully retired and all keys are
passphrase-free Ed25519, the agent bridge is unnecessary. Remove it:

```bash
for host in net bilbo brian davida frodo study andrew lilly sue vaishnavi tb-hk; do
  ssh macw@$host "sed -i.bak '/Bridge macOS SSH agent/,/^fi$/d' ~/.ssh/config"
done
```

Actually, `AddKeysToAgent yes` in the SSH config means the Ed25519 keys will
be auto-added to the agent on first use. The bridge may still be useful for
convenience, so this is low priority to remove.

### 4.5 git-talkbank Uses Brian's Key for GitHub

**What:** Brian's Ed25519 private key is deployed on git-talkbank as
`~/.ssh/id_ed25519_github`, used by the deploy pipeline to pull from
`git@github.com:TalkBank/generate-from-chat`.

**Why:** The deploy pipeline previously relied on `ForwardAgent yes` to borrow
the operator's GitHub credentials. We disabled agent forwarding (security
hardening), so the server needed its own GitHub key. Brian's key was the
fastest fix.

**Risk:** A personal key on a shared server. If git-talkbank is compromised,
Brian's GitHub account is exposed.

**To undo:** Create a dedicated **deploy key** on the TalkBank GitHub org
(read-only access to `generate-from-chat` and any other repos the pipeline
needs). Then replace `id_ed25519_github` on git-talkbank with the deploy key
and delete Brian's key:

```bash
ssh macw@git-talkbank "rm ~/.ssh/id_ed25519_github && rm ~/.ssh/id_ed25519_github.pub"
# Then update ~/.ssh/config to point at the new deploy key
```

### 4.6 git-talkbank's GitLab Key on Franklin's Account

**What:** The 2022 "GitLab" Ed25519 key on git-talkbank (used by the deploy
pipeline and GitLab Runner) is registered on Franklin's GitLab account (key
ID 19).

**Why:** This key was originally registered on Franklin's GitLab account before
the migration. We accidentally de-registered it when cleaning Franklin's keys,
then re-registered it as a hotfix.

**To undo:** This undoes itself — **GitLab is being decommissioned next week.**
Once all repos are on GitHub, the deploy pipeline will only pull from GitHub,
and the GitLab key becomes irrelevant. No action needed.

---

## 5. Not Yet Done

### 5.1 HIGH PRIORITY — Everyone Adds Ed25519 Key to Their Own GitHub

Every person who pushes to GitHub needs to add their machine's Ed25519 public
key to their own GitHub account. This is a **manual step** — each person must
log into github.com themselves.

**Instructions to send everyone:**

1. Open Terminal on your machine
2. Run: `cat ~/.ssh/id_ed25519.pub | pbcopy`
3. Go to https://github.com/settings/ssh/new
4. Title: your machine name (e.g., "brian", "davida")
5. Paste (Cmd-V) and click "Add SSH key"
6. Verify: `ssh -T git@github.com`

**Who needs to do this:**

| Person | GitHub account | Machine(s) | Status |
|--------|---------------|------------|--------|
| Andrew Yankes | ayankes (?) | andrew | **Needs to add** |
| John Kowalski | jkau1 | ? | **Needs key generated** on his machine, then added |
| Leonid Spektor | spektor-cmu | ? | **Needs key generated** on his machine, then added |

**Already done:** Franklin (ming, frodo), Brian (brian, net, bilbo, study),
Davida (davida). Lilli, Sue, Vaishnavi do not use GitHub.

### 5.2 HIGH PRIORITY — Unreachable Machines

| Machine | Problem | Fix |
|---------|---------|-----|
| cbs | "Too many authentication failures" | Investigate SSH config, may need physical access |
| monkey | "Too many authentication failures" | Same |
| extra | Connection timeout | Check if online, may be powered off |
| hermes | Connection timeout | Same |

These machines have no Ed25519 key, no updated SSH config, and no GitLab key
registration. They need the full treatment once reachable.

### 5.3 MEDIUM PRIORITY — John and Leonid's Machines

John Kowalski (`jkau1` on GitHub, `jkau` on GitLab) and Leonid Spektor
(`spektor-cmu` on GitHub, `spektor` on GitLab) are in the TalkBank org but
we don't know which machines they use. They need:

1. Identify their machine(s)
2. Generate Ed25519 keys
3. Update SSH config
4. Register keys on GitLab and GitHub

### 5.4 MOOT — GitLab Being Decommissioned

**GitLab (git.talkbank.org) is being decommissioned next week.** All repos will
move to GitHub. This makes all GitLab-specific work (new accounts, project
access, key registration) throwaway. The GitLab setup will keep working for the
remaining week with no further changes needed.

The three new GitLab accounts (lillirighter, sh4s, velanchen) were unnecessary
but harmless.

### 5.5 LOW PRIORITY — Remove Old Keys from authorized_keys

See [Section 4.3](#43-old-rsa-keys-still-on-all-machines). Only do this after
all access paths are confirmed working with Ed25519 keys and the GitHub RSA
fallback is no longer needed.

### 5.6 LOW PRIORITY — Cloud Server authorized_keys Cleanup

The `authorized_keys` on git.talkbank.org and talkbank.org still have the
old `chen@swan` and `macw@BRIAN` keys, plus Franklin's new Ed25519 was added.
Clean up the old entries once Tailscale SSH is the primary access method.

### 5.7 LOW PRIORITY — Archive Old Keys on Ming

Franklin's old keys (`id_rsa`, `id_rsa_macw`, `github_rsa`, `libra_id_rsa`)
are still in `~/.ssh/`. Archive them to `~/.ssh/archive/` once the migration
is fully complete.

### 5.8 HIGH PRIORITY — Finish Tailscale GUI → CLI Migration

**Status: BLOCKED on physical access.** The migration requires clicking through
a macOS System Extension authorization dialog on each machine's physical screen.

**What's ready (remote prep complete):**
- All 10 CMU lab Macs have Homebrew in macw's PATH
- Reusable auth key created (tag:talkbank, expires 2026-02-27)
- Migration procedure proven on yoga (all steps work except system extension removal)

**What must be done on each machine's physical console:**

```bash
# Step 1: Quit Tailscale GUI (can also be done remotely)
osascript -e 'quit app "Tailscale"'

# Step 2: Install CLI and remove GUI cask conflicts (can be done remotely)
brew install tailscale
brew unlink tailscale 2>/dev/null; brew link --overwrite tailscale

# Step 3: Kill old GUI processes and remove the app
sudo pkill -f tailscaled
sudo rm -rf /Applications/Tailscale.app

# Step 4: Remove the System Extension (REQUIRES PHYSICAL GUI ACCESS)
# This pops up a macOS authorization dialog — click to approve
sudo systemextensionsctl uninstall W5364U7YZB io.tailscale.ipn.macsys.network-extension

# Step 5: Start brew daemon and join tailnet
sudo brew services start tailscale
sleep 3

# For lab machines (tagged):
sudo tailscale up --ssh --accept-routes \
  --auth-key=tskey-auth-k1acv4nvg911CNTRL-PuyA8iWWRS6KsEFcCm9PS6nmz6zyBp7u

# For personal machines (untagged — opens browser auth URL):
sudo tailscale up --ssh --accept-routes

# Step 6: Verify
tailscale status
```

**Machine status:**

| Machine | Brew ready | GUI removed | SysExt removed | CLI joined | Notes |
|---------|:---:|:---:|:---:|:---:|-------|
| frodo | yes | yes | yes | **DONE** | Tailscale SSH working (100.78.37.95) |
| yoga | yes | yes | **no** | **no** | Finish tonight at home |
| ming | yes | no | no | no | Can do now (at CMU) |
| chen | yes | no | no | no | Do at home |
| franklin | yes | no | no | no | Do at home |
| net | yes | no | no | no | Physical access at CMU |
| bilbo | yes | no | no | no | Physical access at CMU |
| brian | yes | no | no | no | Physical access at CMU |
| davida | yes | no | no | no | Physical access at CMU |
| study | yes | no | no | no | Physical access at CMU |
| andrew | yes | no | no | no | Physical access at CMU |
| lilly | yes | no | no | no | Physical access at CMU |
| sue | yes | no | no | no | Physical access at CMU |
| vaishnavi | yes | no | no | no | Physical access at CMU |
| tb-hk | no | no | no | no | Hong Kong — next trip |

**After all machines are migrated:** Tailscale SSH handles all Mac-to-Mac
authentication via ACLs — no more SSH keys needed for inter-machine access.
The old RSA key can then be fully retired (remove from `authorized_keys`,
archive key files).

### 5.9 FUTURE — Purpose-Scoped Deploy Keys

The original plan called for dedicated deploy keys (`deploy_batchalign`,
`deploy_staging`, `deploy_homebank`) instead of using personal keys for
automation. This was deferred — deploy scripts currently use Franklin's
personal Ed25519 key or the old RSA key via authorized_keys.

### 5.10 FUTURE — Replace rsync/apache Keys on git.talkbank.org

The server-to-server keys (`rsync1-key`, `rsync2-key`, `apache-key`) on
git.talkbank.org are still the old 2048-bit RSA keys. Replace with a single
Ed25519 deploy key when convenient.

---

## 6. Mistakes Made & Lessons

### 6.1 Removed Shared Key from GitLab Before Deploying New Keys

**What happened:** Removed the `macw@BRIAN.TALKBANK.ORG` key from Franklin's
GitLab profile. This was the key that ALL lab Macs used for GitLab SSH (everyone
was authenticating as Franklin via the shared key). Instant breakage for everyone.

**Fix:** Generated Ed25519 keys on every machine and registered them on the
correct GitLab user accounts via the admin API.

**Lesson:** The migration plan said "add new keys first, remove old ones only
after verification" (Phase 7: Day 7+). We violated this on Day 1.

### 6.2 Replaced SSH Configs Before Registering New Keys on GitHub

**What happened:** Overwrote `~/.ssh/config` on all lab Macs to use `id_ed25519`
for GitHub, removing the `HostKeyAlgorithms +ssh-rsa` lines that enabled the
old RSA keys. But the new Ed25519 keys weren't registered on anyone's GitHub
account. GitHub broke for all machines except those with the old key in the agent.

**Fix:** Added `IdentityFile ~/.ssh/id_rsa` as a fallback for GitHub. Then
registered Franklin's machine keys on his GitHub account. Brian's RSA key was
already on his own GitHub account and works via the macOS agent.

**Lesson:** SSH config changes should have been additive (add the new key as
preferred, keep the old key as fallback) from the start.

### 6.3 Removed Keychain Without Replacing Agent Functionality

**What happened:** Removed the `keychain` eval from `.zshenv` on all machines.
This was loading the old RSA keys (which have passphrases) into the SSH agent.
Without it, SSH sessions from ming → lab Macs had no agent, so passphrase-
protected keys were unusable.

**Fix:** Added a macOS SSH agent socket bridge to `.zshenv` so the system agent
is accessible in remote SSH sessions.

**Lesson:** Before removing infrastructure (even hacky infrastructure), ensure
the replacement is in place. `keychain` was ugly but it served a real purpose.

### 6.4 Broke the Staging Deploy Pipeline on git-talkbank

**What happened:** The staging deploy system on git-talkbank does `git remote
update` on local clones of GitLab repos. This uses SSH to connect to
`git@gitlab.talkbank.org`. The key it uses — an Ed25519 key from 2022 labeled
"GitLab" — was registered on Franklin's GitLab account. When we removed keys
from Franklin's GitLab, this key lost its registration. All deploys silently
failed from ~11:39 to ~15:30.

**Fix:** Re-registered the key on Franklin's GitLab account (key ID 19).
Deploys that ran during the outage need to be manually re-triggered.

**Lesson:** Before removing ANY key from a service, check if it's used by
automated systems (CI runners, deploy scripts, cron jobs), not just interactive
users. The deploy pipeline was an invisible consumer we didn't audit.

### 6.5 General Lesson

**Always additive-first.** The correct order is:
1. Generate new keys
2. Register new keys on all services (GitLab, GitHub)
3. Update SSH configs to **prefer** new keys (but keep old as fallback)
4. Verify everything works with new keys
5. THEN remove old keys

We did steps 1-3 partially and out of order, breaking things along the way.

---

## 7. Reference: Tailscale Configuration

### Tailscale API Key

```
tskey-api-kbwEvQrJxq11CNTRL-RaNy9Kp69naTuKRd629BnarXUSU2sYNb
```

Use with: `curl -u "$TAILSCALE_API_KEY:" https://api.tailscale.com/api/v2/...`

### ACL Policy (live)

Managed via API or https://login.tailscale.com/admin/acls:

```jsonc
{
    "tagOwners": {
        "tag:talkbank": ["autogroup:admin"]
    },
    "grants": [
        {"src": ["*"], "dst": ["*"], "ip": ["*"]}
    ],
    "ssh": [
        // TalkBank devices + users → tagged lab/infra machines as macw
        {
            "action": "accept",
            "src":    ["tag:talkbank", "FranklinChen@github", "macw@github", "davidafromm@github"],
            "dst":    ["tag:talkbank"],
            "users":  ["macw"]
        },
        // Franklin's personal machines → Franklin's personal machines
        {
            "action": "accept",
            "src":    ["FranklinChen@github"],
            "dst":    ["FranklinChen@github"],
            "users":  ["autogroup:nonroot"]
        },
        // Everyone else → own devices with browser check
        {
            "action": "check",
            "src":    ["autogroup:member"],
            "dst":    ["autogroup:self"],
            "users":  ["autogroup:nonroot", "root"]
        }
    ],
    "nodeAttrs": [
        {
            "target": ["autogroup:member"],
            "attr":   ["funnel"]
        }
    ]
}
```

**Key concepts:**
- `tag:talkbank` — applied to all lab machines and infrastructure servers
- Franklin's personal machines (ming, franklin, chen, yoga) are **untagged**
- Tagged devices lose user identity in ACLs — `tag:talkbank` must be in `src`
- `smwilsonau@github` intentionally excluded (external collaborator)
- Identity strings use `@github` suffix (e.g., `FranklinChen@github`, NOT `FranklinChen@`)

### Tagging Devices

```bash
export TAILSCALE_API_KEY="tskey-api-..."

# List all devices
curl -s -u "$TAILSCALE_API_KEY:" \
  https://api.tailscale.com/api/v2/tailnet/-/devices \
  | python3 -c "
import sys, json
for d in json.load(sys.stdin)['devices']:
    name = d['name'].split('.')[0]
    print(f'{d[\"id\"]:20s}  {name:25s}  {d.get(\"tags\", [])}')
"

# Tag a device
curl -s -X POST -u "$TAILSCALE_API_KEY:" \
  -H "Content-Type: application/json" \
  -d '{"tags":["tag:talkbank"]}' \
  "https://api.tailscale.com/api/v2/device/DEVICE_ID/tags"
```

### Updating ACL Policy

```bash
curl -s -X POST -u "$TAILSCALE_API_KEY:" \
  -H "Content-Type: application/hujson" \
  --data-binary @policy.json \
  https://api.tailscale.com/api/v2/tailnet/-/acl
```

---

## 8. Reference: Machine Inventory

### Lab Macs (on Tailscale, tagged `tag:talkbank`)

| Machine | Tailscale IP | Owner account | Ed25519 key | SSH config | GitLab | GitHub |
|---------|-------------|--------------|:-----------:|:----------:|:------:|:------:|
| net | 100.113.61.37 | macw@ | yes | yes | @chen | @macw (Brian's key) |
| bilbo | 100.90.204.45 | macw@ | yes | yes | @chen | @macw (Brian's key) |
| brian | 100.66.91.120 | macw@ | yes | yes | @macw | @macw |
| davida | 100.100.157.10 | davidafromm@ | yes | yes | @fromm | @davidafromm |
| fromm | 100.74.24.120 | tag:talkbank | yes | yes | @fromm (key 24) | @davidafromm |
| frodo | 100.79.155.73 | macw@ | yes | yes | @chen | @FranklinChen |
| study | 100.95.76.54 | macw@ | yes | yes | @macw | @macw (Brian's key) |
| andrew | 100.121.108.84 | macw@ | yes | yes | @ayankes | **needs setup** |
| lilly | 100.96.102.124 | macw@ | yes | yes | @lillirighter | N/A (no GitHub use) |
| sue | 100.65.176.30 | macw@ | yes | yes | @sh4s | N/A (no GitHub use) |
| vaishnavi | 100.84.238.83 | macw@ | yes | yes | @velanchen | N/A (no GitHub use) |
| tb-hk | 100.76.9.18 | macw@ | yes | yes | @macw | @macw (Brian's key) |
| cbs | 100.65.39.70 | tagged-devices | yes | yes | @macw (key 23) | @macw (Brian's key) |
| monkey | 100.74.127.49 | tagged-devices | yes | yes | @macw (key 22) | @macw (Brian's key) |
| extra | 100.67.15.16 | macw@ | **no** | **no** | **no** | **no** |
| hermes | 100.78.144.37 | macw@ | yes | yes | @macw (key 21) | @macw (Brian's key) |

### Franklin's Personal Machines (on Tailscale, **untagged**)

| Machine | Tailscale IP | Ed25519 key | SSH config |
|---------|-------------|:-----------:|:----------:|
| ming | 100.102.133.90 | yes | yes |
| franklin | 100.90.91.95 | yes | yes |
| chen | 100.96.148.40 | yes | yes |
| snorri | 100.68.24.109 | yes | yes |
| swan | 100.122.40.75 | yes | yes |
| yoga | 100.68.70.110 | yes | yes |

### Cloud Servers (on Tailscale, tagged `tag:talkbank`)

| Machine | Tailscale name | Tailscale IP | OS | Tailscale SSH |
|---------|---------------|-------------|-----|:---:|
| git.talkbank.org | git-talkbank | 100.108.195.47 | Ubuntu 25.10 | **working** |
| talkbank.org | talkbank | 100.72.29.3 | Ubuntu 25.10 | **working** |
| media.talkbank.org | (not on Tailscale) | N/A | RHEL 8.10 | N/A |

### Git User → Machine Mapping

| Machine | git user.name | git user.email |
|---------|--------------|----------------|
| brian | Brian MacWhinney (Brian at CMU) | macw@cmu.edu |
| davida | Davida Fromm (davida.talkbank.org) | fromm@andrew.cmu.edu |
| fromm | Davida Fromm (fromm laptop) | fromm@andrew.cmu.edu |
| frodo | Franklin Chen (Frodo) | franklinchen@franklinchen.com |
| study | Brian MacWhinney (Brian on whisper) | macw@cmu.edu |
| andrew | Andrew Yankes | ayankes@andrew.cmu.edu |
| lilly | Lilli Righter | 70528758+lillirighter@users.noreply.github.com |
| sue | Sue Holm | sh4s@andrew.cmu.edu |
| vaishnavi | Vaishnavi Elanchezhian | velanchen@andrew.cmu.edu |
| tb-hk | Brian MacWhinney (Brian at tb-hk) | macw@cmu.edu |
| net | (unset) | (unset) |
| bilbo | (unset) | (unset) |

---

## 9. Reference: GitLab Users & Keys

**GitLab URL:** https://git.talkbank.org:8929 (SSH on port 22)

**Admin API token:** `L1cU-joyQkraVDgvjGdF` (scope: `api`, expires ~2026-02-27)

### All Users

| ID | Username | Name | Email | Admin | SSH keys |
|:--:|----------|------|-------|:-----:|:--------:|
| 1 | chen | Franklin Chen | franklinchen@franklinchen.com | yes | id 7 (personal), 11-13 (machines) |
| 3 | macw | Brian MacWhinney | macw@andrew.cmu.edu | yes | id 8-10 (machines) |
| 4 | jkau | John Kowalski | jkau@andrew.cmu.edu | yes | none |
| 5 | spektor | Leonid Spektor | spektor@andrew.cmu.edu | yes | none |
| 6 | ayankes | Andrew Yankes | ayankes@andrew.cmu.edu | yes | id 15 |
| 7 | fromm | Davida Fromm | fromm@andrew.cmu.edu | yes | id 14 (davida desktop), 24 (fromm laptop) |
| 10 | sh4s | Sue Holm | sh4s@andrew.cmu.edu | no | id 17 |
| 11 | velanchen | Vaishnavi Elanchezhian | velanchen@andrew.cmu.edu | no | id 18 |
| 12 | lillirighter | Lilli Righter | lillirighter@talkbank.org | no | id 16 |

### API Examples

```bash
GL_TOKEN="L1cU-joyQkraVDgvjGdF"
GL_URL="https://git.talkbank.org:8929/api/v4"

# List all users
curl -s --header "PRIVATE-TOKEN: $GL_TOKEN" "$GL_URL/users?per_page=50"

# List user's SSH keys
curl -s --header "PRIVATE-TOKEN: $GL_TOKEN" "$GL_URL/users/USER_ID/keys"

# Add SSH key to a user
curl -s --header "PRIVATE-TOKEN: $GL_TOKEN" \
  --data-urlencode "title=MACHINE-NAME (macw@talkbank.org)" \
  --data-urlencode "key=$(cat KEY.pub)" \
  "$GL_URL/users/USER_ID/keys"
```

---

## 10. Reference: GitHub Keys

### FranklinChen Account

| Key ID | Title | Type |
|--------|-------|------|
| 123525957 | TalkBank runner deploy | Ed25519 (on talkbank.org for Actions runner) |
| 143496212 | franklin@talkbank.org (Ed25519) | Ed25519 (Franklin's personal) |
| 143499474 | frodo-machine (macw@talkbank.org) | Ed25519 |

net-server and bilbo-machine keys were removed (those machines now use Brian's key).

### macw Account (Brian)

Brian removed his old RSA key and added his Ed25519 key. This key is also
deployed as `id_ed25519_github` on net, bilbo, and study.

tb-hk still needs Brian's key deployed (intermittently unreachable).

### davidafromm Account (Davida)

| Key ID | Title | Type |
|--------|-------|------|
| 143499025 | (Ed25519) | Ed25519 (davida desktop key) |
| (new) | fromm-laptop (macw@talkbank.org) | Ed25519 (fromm laptop key, added 2026-02-25) |

### TalkBank GitHub Org Members

FranklinChen, macw, davidafromm, jkau1, spektor-cmu, lillirighter, Jemoka, smwilsonau

---

## Appendix A: Installing Tailscale on Ubuntu Servers

Both git.talkbank.org and talkbank.org run Ubuntu 25.10 (questing). The `macw`
user has sudo access (password required).

```bash
ssh macw@SERVER

# Add Tailscale apt repo
curl -fsSL https://pkgs.tailscale.com/stable/ubuntu/questing.noarmor.gpg \
  | sudo tee /usr/share/keyrings/tailscale-archive-keyring.gpg >/dev/null
curl -fsSL https://pkgs.tailscale.com/stable/ubuntu/questing.tailscale-keyring.list \
  | sudo tee /etc/apt/sources.list.d/tailscale.list

# Install
sudo apt update
sudo apt install tailscale

# Join tailnet with SSH enabled
sudo tailscale up --ssh --hostname=HOSTNAME
# → Opens an auth URL — open in browser to approve
```

After both are approved:
```bash
tailscale status | grep -E 'git-talkbank|talkbank'
ssh macw@git-talkbank "hostname"
ssh macw@talkbank "hostname"
```

If Ubuntu is upgraded, update `questing` to the new codename.
See https://pkgs.tailscale.com/stable/.

---

## Appendix B: macOS Tailscale GUI → CLI Migration

### Why This Is Needed

**Tailscale SSH does NOT work on the macOS GUI app** (App Store or download).
The GUI build is sandboxed and cannot run an SSH server. This affects ALL Macs
in the lab.

Tailscale SSH only works on:
- Linux (CLI install via apt/yum) — git-talkbank and talkbank work
- macOS CLI install via `brew install tailscale` (not the GUI cask)

### Current State (as of 2026-02-20)

| Machine type | Tailscale install | Tailscale SSH | SSH auth method |
|-------------|-------------------|:---:|-----------------|
| Lab Macs | GUI app (migration pending) | no | OpenSSH + Ed25519/RSA keys |
| Franklin's Macs | GUI app (yoga halfway) | no | OpenSSH + Ed25519 key |
| Linux servers | CLI (apt) | **yes** | Tailscale SSH ACLs |

### The System Extension Problem

The Tailscale GUI installs a **macOS System Extension**
(`io.tailscale.ipn.macsys.network-extension`) that persists even after:
- Quitting the GUI app
- Deleting `/Applications/Tailscale.app`
- Starting the brew `tailscaled` daemon

The `tailscale` CLI connects to the System Extension's `tailscaled` (not the
brew one), and refuses to enable SSH because it detects a "sandboxed GUI build."

**Removing the System Extension requires physical GUI access:**
```bash
sudo systemextensionsctl uninstall W5364U7YZB io.tailscale.ipn.macsys.network-extension
```
This pops up a macOS authorization dialog that cannot be clicked over SSH.
There is no workaround — this is a macOS security requirement.

### Migration Procedure

See [Section 5.8](#58-high-priority--finish-tailscale-gui--cli-migration) for
the full per-machine procedure and status table.

### Key Gotchas Discovered During yoga Test

1. **`brew install tailscale` won't link** if the GUI cask is installed — need
   `brew link --overwrite tailscale` after removing the cask symlinks
2. **`tailscale up` requires all non-default flags** — if the GUI had
   `--accept-routes` enabled, you must pass `--accept-routes` to `tailscale up`
   or use `--reset`
3. **Version mismatch** between brew CLI (1.94.1) and GUI System Extension
   (1.94.2) is normal — the GUI auto-updates independently of brew
4. **`sudo` required** for `brew services start tailscale` and `tailscale up`
   — the `macw` user on lab Macs has sudo with password (not passwordless)
5. **Personal machines must NOT use the tagged auth key** — use browser auth
   instead, or they'll be tagged `tag:talkbank` and lose their personal identity
