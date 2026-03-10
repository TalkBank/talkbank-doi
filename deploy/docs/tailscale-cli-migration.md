# Tailscale CLI Migration: GUI → Brew

**Status: COMPLETE for all reachable machines as of 2026-02-25.**
**Last updated: 2026-02-25.**

All CMU lab Macs, Brian's home machines (study, monkey), Davida's laptop
(fromm), and Franklin's home machines (yoga, chen, franklin) have been migrated
from the Tailscale macOS GUI app to the Homebrew CLI formula. This enables
Tailscale SSH (eliminating SSH key management for Mac-to-Mac access).

Remaining: cbs, tb-hk, extra (all unreachable — need physical access).

---

## Table of Contents

1. [Why We Did This](#1-why-we-did-this)
2. [What Was Done](#2-what-was-done)
3. [Machine Status](#3-machine-status)
4. [Known Bug: MagicDNS on macOS CLI](#4-known-bug-magicdns-on-macos-cli)
5. [What's Left](#5-whats-left)
6. [How to Migrate a Machine](#6-how-to-migrate-a-machine)
7. [Troubleshooting](#7-troubleshooting)
8. [Scripts](#8-scripts)
9. [Tailscale API Reference](#9-tailscale-api-reference)
10. [Lessons Learned](#10-lessons-learned)

---

## 1. Why We Did This

Tailscale SSH only works on CLI installs (Linux apt, macOS Homebrew formula).
The macOS GUI app runs in a sandbox and cannot act as an SSH server. Migrating
to the CLI formula enables:

- **Tailscale SSH**: SSH between any two machines without managing SSH keys
- **No key rotation**: Tailscale handles authentication via WireGuard
- **Unified access**: Same mechanism on Linux servers and macOS clients

---

## 2. What Was Done

### 2.1 Homebrew Installed on All Macs

Machines that lacked Homebrew (andrew, lilly, sue, vaishnavi) had it installed
from the admin account. Brew shellenv was added to `~/.zshenv` on these machines
(not `.zprofile`) so it's available in non-login SSH sessions:

```bash
# In ~/.zshenv (ARM Macs)
eval "$(/opt/homebrew/bin/brew shellenv)"

# In ~/.zshenv (Intel Macs — yoga only)
eval "$(/usr/local/bin/brew shellenv)"
```

### 2.2 Tailscale CLI Installed via Brew

On every Mac: `brew install tailscale` (the formula, NOT the cask).

### 2.3 GUI → CLI Migration

Each machine required three phases:

1. **Remote prep** (from ming via SSH): Quit GUI app, `brew install tailscale`
2. **Physical access** (at the machine): System Settings > General > Login Items
   & Extensions > Network Extensions > toggle off Tailscale. Then run
   `sudo bash ~/finish-migrate.sh`
3. **Remote cleanup** (from ming): Delete old Tailscale node, rename new one

The `finish-migrate.sh` script handles: killing GUI/daemon processes,
uninstalling the brew cask, removing `/Applications/Tailscale.app`, relinking
the brew formula, cleaning stale wrapper scripts, starting brew daemon, and
joining the tailnet with `--ssh --accept-routes`.

### 2.4 Reusable Auth Key

Created a Tailscale reusable auth key tagged `tag:talkbank` for lab machines:
```
tskey-auth-k1acv4nvg911CNTRL-PuyA8iWWRS6KsEFcCm9PS6nmz6zyBp7u
```
**Expires: 2026-02-27.** Personal machines (yoga, chen, franklin) use browser
auth instead (`--no-key` flag).

### 2.5 MagicDNS Fix Deployed

Discovered that brew tailscaled 1.74.0+ has a known bug where it fails to
configure macOS system DNS (see Section 4). Deployed `fix-dns.sh` to all
machines and ran it on all migrated machines.

### 2.6 SSH Config with IP Mappings

As a belt-and-suspenders backup, deployed SSH configs with `HostName <Tailscale IP>`
entries to all migrated machines. This ensures SSH works even if MagicDNS fails.

### 2.7 Tailscale Node Cleanup

After each migration, the old GUI node was deleted via API and the new CLI node
(which registers with a `-1` or `-bp` suffix) was renamed to the original hostname.

### 2.8 Hermes SSH Setup

Hermes was previously unreachable (no SSH keys). Set up from scratch:
- Generated Ed25519 key
- SSH config: `id_ed25519_github` for GitHub, `id_ed25519` for GitLab
- Registered key on GitLab (Brian's @macw account, key ID 21)
- Deployed Brian's `id_ed25519_github` for GitHub

### 2.9 Ming Migration

Ming (Franklin's dev machine) was accidentally half-migrated during the
walk-around. Fixed by relinking brew CLI, logging back into Tailscale, and
cleaning up the duplicate node.

### 2.10 Brian Re-registration

Brian's machine was originally authenticated without the `tag:talkbank` tag
(owned by `macw@` instead of `tagged-devices`). This blocked Tailscale SSH
due to ACL rules. Fixed by `tailscale logout` + re-registering with the auth
key.

---

## 3. Machine Status

### CMU Lab Machines

| Machine | CLI Migration | Tailscale SSH | MagicDNS Fix | SSH Config IPs | Notes |
|---------|:---:|:---:|:---:|:---:|-------|
| net | done | enabled | applied | deployed | Production server |
| bilbo | done | enabled | applied | deployed | |
| brian | done | enabled | applied | deployed | Re-registered with auth key for tag:talkbank |
| frodo | done | enabled | applied | deployed | |
| andrew | done | enabled | applied | deployed | |
| davida | done | enabled | applied | deployed | |
| lilly | done | enabled | applied | deployed | |
| sue | done | enabled | applied | deployed | |
| vaishnavi | done | enabled | applied | deployed | |
| hermes | done | enabled | applied | deployed | Brian's office |
| monkey | done | enabled | applied | deployed | Re-registered with auth key for tag:talkbank (2026-02-22). SSH config deployed 2026-02-25. |
| fromm | done | enabled | applied | deployed | Davida's laptop. Migrated 2026-02-25. tag:talkbank, Ed25519 key, GitHub + GitLab registered. IP: 100.74.24.120 |
| ming | done | enabled | not needed | n/a | Franklin's dev machine, MagicDNS works |

### Home Machines

| Machine | CLI Migration | Owner | Notes |
|---------|:---:|-------|-------|
| study | done | Brian | Brian migrated it himself. SSH config with fleet IPs deployed 2026-02-25. |
| yoga | done | Franklin | Intel Mac |
| chen | done | Franklin | |
| franklin | done | Franklin | DERP-only to brian (stale peer routing) — see Troubleshooting |

### HK Machines (no physical access)

| Machine | Status | Notes |
|---------|--------|-------|
| cbs | SSH keys done, GUI Tailscale | User is `cbs` (not macw). Ed25519 key, GitLab, GitHub all set. CLI migration blocked — needs physical access in HK. ACL updated to allow `cbs` user. |
| tb-hk | SSH keys done, GUI Tailscale | Brian's GitHub key deployed. CLI migration blocked — needs physical access in HK. |

### Unreachable / Low Priority

| Machine | Status | Notes |
|---------|--------|-------|
| extra | offline | Last seen 6+ days ago |

---

## 4. Known Bug: MagicDNS on macOS CLI

**Bug:** Brew tailscaled 1.74.0+ on macOS fails to configure system DNS. The
daemon logs show it setting `dns: OScfg: {Nameservers:[100.100.100.100]}` but
macOS never applies it. The `scutil --dns` output shows a `search.tailscale`
resolver with the correct search domain (`tail69ccca.ts.net`) but **no
nameserver**.

**Root cause:** The GUI app uses its Network Extension (which creates a TUN
device) to inject DNS. The CLI daemon runs in userspace networking mode with no
TUN device, so it can't bind a DNS resolver to a network interface.

**GitHub issue:** https://github.com/tailscale/tailscale/issues/13461

**Impact:** Without the fix, Tailscale hostnames (net, frodo, etc.) don't
resolve. This breaks SSH by hostname, VNC, `http://net:8000`, and any other
name-based access. Tailscale *connectivity* still works — only DNS is broken.

**Fix:** Create `/etc/resolver/ts.net` pointing to Tailscale's local DNS:

```bash
sudo mkdir -p /etc/resolver
sudo tee /etc/resolver/ts.net << 'EOF'
nameserver 100.100.100.100
search_order 1
EOF
```

This routes all `*.ts.net` queries to Tailscale's resolver. Since the search
domain `tail69ccca.ts.net` IS configured by tailscaled, bare names like `net`
get expanded to `net.tail69ccca.ts.net` and routed correctly.

The `fix-dns.sh` script on each machine does exactly this.

---

## 5. What's Left

### Completed (2026-02-25)

- [x] **fromm**: Full migration — Tailscale CLI, `tag:talkbank`, SSH enabled, key expiry disabled, Ed25519 key generated, SSH config + authorized_keys deployed, GitHub (`davidafromm`) and GitLab (`fromm`) keys registered. IP: `100.74.24.120`.
- [x] **study**: SSH config with fleet IPs deployed.
- [x] **monkey**: SSH config with fleet IPs deployed.

### Minor Fixups

- [ ] **franklin**: Fix DERP-only routing to brian (`sudo tailscale down && sudo tailscale up --ssh --accept-routes`, then `tailscale ping brian`)
- [ ] **ACL**: Reorder or make Rule 1 more explicit to avoid browser check on personal device SSH

### Low Priority

- [ ] Migrate cbs, tb-hk, extra when they become reachable (need physical access)
- [ ] Consider removing SSH config IP mappings once MagicDNS bug is fixed upstream

---

## 6. How to Migrate a Machine

### Prerequisites
- Homebrew installed (`brew` in PATH)
- `brew install tailscale` done
- `~/finish-migrate.sh` and `~/fix-dns.sh` deployed

### Steps

**Phase 1: Remote prep (from ming)**
```bash
ssh macw@HOSTNAME 'osascript -e "quit app \"Tailscale\"" 2>/dev/null; killall Tailscale 2>/dev/null; echo done'
scp ~/finish-migrate.sh macw@HOSTNAME:~/finish-migrate.sh
scp ~/fix-dns.sh macw@HOSTNAME:~/fix-dns.sh
```

**Phase 2: Physical access (at the machine)**
1. Open System Settings > General > Login Items & Extensions > Network Extensions
2. Toggle OFF "Tailscale" (enter admin password when prompted)
3. Verify: `systemextensionsctl list` should show `[activated disabled]`
4. Kill any remnant extension process: `sudo pkill -f network-extension`
5. Run the migration:
```bash
# Lab machines (tagged):
sudo bash ~/finish-migrate.sh

# Personal machines (browser auth):
sudo bash ~/finish-migrate.sh --no-key
```
6. Fix DNS:
```bash
sudo bash ~/fix-dns.sh
```
7. If `tailscale` gives "command not found":
```bash
brew unlink tailscale; brew link --overwrite tailscale
```

**Phase 3: Remote cleanup (from ming)**
```bash
# Find the old and new nodes
curl -s -H "Authorization: Bearer $TS_API_KEY" \
  "https://api.tailscale.com/api/v2/tailnet/-/devices" | \
  python3 -c "import json,sys; [print(f'{d[\"hostname\"]:20s} id={d[\"id\"]}') for d in json.load(sys.stdin)['devices'] if 'HOSTNAME' in d['hostname'].lower()]"

# Delete old node
curl -s -X DELETE -H "Authorization: Bearer $TS_API_KEY" \
  "https://api.tailscale.com/api/v2/device/OLD_DEVICE_ID"

# Rename new node (drop the -1 or -bp suffix)
curl -s -X POST -H "Authorization: Bearer $TS_API_KEY" \
  -H "Content-Type: application/json" -d '{"name":"HOSTNAME"}' \
  "https://api.tailscale.com/api/v2/device/NEW_DEVICE_ID/name"
```

---

## 7. Troubleshooting

### "command not found: tailscale"
`brew uninstall --cask` removes the formula's symlinks too. Fix:
```bash
brew unlink tailscale; brew link --overwrite tailscale
hash -r
```

### `tailscale up` hangs
The old System Extension is still intercepting traffic. Verify it's disabled:
```bash
systemextensionsctl list
# Must show [activated disabled] for Tailscale
```
If it shows `[activated enabled]`, toggle it off in System Settings.
Then kill the extension process and restart:
```bash
sudo pkill -f network-extension
sudo brew services restart tailscale
sleep 3
sudo tailscale up --ssh --accept-routes
```

### Stale wrapper scripts
Old GUI cask leaves wrapper scripts that point to deleted `/Applications/Tailscale.app`:
```bash
# Check for stale wrappers
for f in /usr/local/bin/tailscale /opt/homebrew/bin/tailscale; do
  file "$f" 2>/dev/null
done
# Remove if they're text/shell scripts (real binary is Mach-O)
```
The `finish-migrate.sh` script handles this automatically.

### "Tailscale SSH server does not run in sandboxed GUI builds"
The old System Extension is still intercepting traffic. Must disable it in
System Settings (see above). `systemextensionsctl uninstall` does NOT
work with SIP enabled.

### "changing settings requires mentioning all non-default flags"
The GUI had `--accept-routes` enabled. Add it to the `tailscale up` command:
```bash
sudo tailscale up --ssh --accept-routes
```

### "tailnet policy does not permit you to SSH to this node"
The machine is not tagged `tag:talkbank`. The ACL only allows SSH to tagged
devices. Fix by re-registering with the auth key:
```bash
sudo tailscale logout
sudo tailscale up --ssh --accept-routes --auth-key=tskey-auth-...
```

### Host key verification failed
After migration, machines have new Tailscale SSH host keys. Clear old entries:
```bash
ssh-keygen -R HOSTNAME
ssh -o StrictHostKeyChecking=accept-new macw@HOSTNAME echo OK
```

### VNC/HTTP/DNS not resolving Tailscale names
MagicDNS bug. Run `sudo bash ~/fix-dns.sh` on the machine. Or use the
Tailscale IP directly (e.g., `vnc://100.92.113.40`).

### Tailscale status shows "Logged out"
The daemon is running but not connected:
```bash
sudo tailscale up --ssh --accept-routes
# For lab machines, add: --auth-key=tskey-auth-...
```

### Version mismatch warning
`client version != tailscaled server version` — the old GUI's daemon (1.94.2)
may still be running instead of brew's (1.94.1). Fix:
```bash
sudo pkill -f tailscaled
sudo brew services restart tailscale
```

### SSH timeout via DERP relay (asymmetric routing)
**Symptom:** Machine A → B times out, but B → A works. Both authorized, same
version, correct ACLs. `tailscale status` shows DERP relay (no direct path) for
the failing direction but the reverse direction works fine (e.g., proved with
`nc` test).

**Root cause:** Asymmetric DERP routing — the initiating machine's Tailscale
daemon has stale peer routing state from the GUI → CLI migration.

**Diagnosis:**
```bash
# From the failing machine:
tailscale ping PEER     # shows "via DERP(nyc)" instead of direct
tailscale status        # no direct connection to peer
# From the peer machine:
tailscale ping FAILING  # also DERP, but connections initiated FROM here work
```

**Fix:** On the machine that can't initiate connections:
```bash
sudo tailscale down && sudo tailscale up --ssh --accept-routes
tailscale ping PEER     # should now establish direct or working DERP path
# If still failing:
sudo brew services restart tailscale
```

**Example:** franklin → brian timed out. Ming → brian worked (direct LAN,
172.24.72.142, 1ms). Brian → franklin via DERP worked. Only franklin-initiated
DERP connections to brian failed. Both v1.94.1, ACL Rule 0 explicitly allows
`FranklinChen@github → tag:talkbank` as macw.

### Browser check on personal device SSH (ACL rule ordering)
**Symptom:** SSH between two personal devices (same owner, e.g.,
`FranklinChen@github`) triggers a browser check instead of connecting directly.

**Root cause:** ACL rule ordering. The intended rule (e.g., Rule 1:
`accept FranklinChen@ → FranklinChen@, users: nonroot`) should match, but a
later rule (e.g., Rule 2: `check member → self`) matches first due to Tailscale's
rule evaluation semantics.

**Impact:** Browser check auth persists ~12 hours per session, so this is an
annoyance rather than a blocker.

**Fix:** Modify ACL to make the accept rule more explicit (specify src/dst by
tag or device) or reorder rules so the accept rule is evaluated before the
check rule.

---

## 8. Scripts

### `~/finish-migrate.sh`
Deployed to all machines. Handles the full GUI → CLI transition:
- Quits Tailscale GUI and kills processes
- **Uninstalls the brew cask** (`brew uninstall --cask tailscale`)
- Removes `/Applications/Tailscale.app`
- **Relinks the brew formula** (cask uninstall breaks symlinks)
- Detects and removes stale wrapper scripts
- Starts brew daemon (`sudo brew services restart tailscale`)
- Joins tailnet with `--ssh --accept-routes`
- Uses auth key for lab machines, `--no-key` for personal machines

**Critical fix applied mid-migration:** The original script only did
`rm -rf /Applications/Tailscale.app` which left the cask's System Extension
and other artifacts behind, causing `tailscale up` to hang on every machine.
The updated script runs `brew uninstall --cask tailscale` first and relinks
the formula after. **Ensure you have the updated version before Monday.**

### `~/fix-dns.sh`
Deployed to all machines. Creates `/etc/resolver/ts.net` with:
```
nameserver 100.100.100.100
search_order 1
```
Fixes MagicDNS for all protocols (SSH, VNC, HTTP, etc.).

---

## 9. Tailscale API Reference

**API key:** `tskey-api-kbwEvQrJxq11CNTRL-RaNy9Kp69naTuKRd629BnarXUSU2sYNb`

**Auth key (lab machines, expires 2026-02-27):**
`tskey-auth-k1acv4nvg911CNTRL-PuyA8iWWRS6KsEFcCm9PS6nmz6zyBp7u`

**Tailnet domain:** `tail69ccca.ts.net`

### Tailscale IPs (current as of 2026-03-08)

| Machine | Tailscale IP |
|---------|-------------|
| ming | 100.96.86.56 |
| net | 100.92.113.40 |
| bilbo | 100.123.21.75 |
| brian | 100.87.231.106 |
| frodo | 100.78.37.95 |
| andrew | 100.84.163.104 |
| davida | 100.102.15.39 |
| lilly | 100.97.70.53 |
| sue | 100.84.206.29 |
| vaishnavi | 100.117.243.101 |
| hermes | 100.78.144.37 |
| monkey | 100.74.127.49 |
| study | 100.97.156.40 |
| cbs | 100.65.39.70 |
| tb-hk | 100.76.9.18 |
| git-talkbank | 100.108.195.47 |
| talkbank | 100.72.29.3 |
| yoga | 100.102.163.27 |
| chen | 100.85.98.80 |
| franklin | 100.65.53.125 |
| snorri | 100.68.24.109 |
| swan | 100.122.40.75 |
| fromm | 100.74.24.120 |

**Note:** IPs change when a machine re-registers (logout + re-auth). Update
the SSH config IP mappings on all machines if this happens.

### Common API calls

```bash
export TS_API_KEY="tskey-api-kbwEvQrJxq11CNTRL-RaNy9Kp69naTuKRd629BnarXUSU2sYNb"

# List all devices
curl -s -H "Authorization: Bearer $TS_API_KEY" \
  "https://api.tailscale.com/api/v2/tailnet/-/devices"

# Delete a device
curl -s -X DELETE -H "Authorization: Bearer $TS_API_KEY" \
  "https://api.tailscale.com/api/v2/device/DEVICE_ID"

# Rename a device
curl -s -X POST -H "Authorization: Bearer $TS_API_KEY" \
  -H "Content-Type: application/json" -d '{"name":"NEW_NAME"}' \
  "https://api.tailscale.com/api/v2/device/DEVICE_ID/name"
```

---

## 10. Lessons Learned

1. **Always `brew uninstall --cask` before removing the GUI app.** Just doing
   `rm -rf /Applications/Tailscale.app` leaves the cask's System Extension and
   other artifacts behind. This caused `tailscale up` to hang on every single
   machine. The biggest mistake of the migration.

2. **The System Extension cannot be removed programmatically.** On macOS with
   SIP enabled (all our machines), `systemextensionsctl uninstall` fails.
   Must use System Settings GUI — requires physical access.

3. **Brew tailscaled does not configure DNS on macOS.** This is a known bug
   since v1.74.0. The `/etc/resolver/ts.net` workaround is essential. Without
   it, only SSH (via config IP mappings) works — VNC, HTTP, and everything
   else that needs name resolution breaks.

4. **`brew uninstall --cask` breaks the formula's symlinks.** After uninstalling
   the cask, must run `brew unlink tailscale; brew link --overwrite tailscale`
   to restore the formula's CLI binaries.

5. **Old GUI leaves stale wrapper scripts.** After uninstalling
   `/Applications/Tailscale.app`, shell wrappers at `/usr/local/bin/tailscale`
   or `/opt/homebrew/bin/tailscale` may survive and shadow the real brew binary.
   The `finish-migrate.sh` script detects and removes these.

6. **Shell hash caching.** After relinking binaries, run `hash -r` or the
   shell finds the old (deleted) path.

7. **New CLI node gets a suffix.** Tailscale registers CLI as a new device
   with `-1` or `-bp` appended. Must delete the old GUI node and rename the
   new one via API.

8. **Host keys change after migration.** Tailscale SSH uses different host
   keys than regular sshd. All connecting machines need their `known_hosts`
   cleared for the migrated hosts.

9. **`sudo` doesn't work over Tailscale SSH without a TTY.** Can't run
   `sudo` commands remotely via `ssh macw@host 'sudo ...'` because Tailscale
   SSH doesn't allocate a pseudo-TTY from non-interactive contexts. Physical
   access or interactive SSH is required for sudo operations.

10. **ACL requires `tag:talkbank` for SSH.** Machines authenticated without the
    auth key get owned by `macw@` instead of `tagged-devices`, which the SSH ACL
    blocks. Must `tailscale logout` and re-register with the auth key.

11. **Test the actual script end-to-end, not just manual steps.** We tested
    manual steps on frodo but never ran the actual `finish-migrate.sh` on a
    test machine before the walk-around. Every machine hit the same cask bug.

12. **VNC is more important than SSH for most users.** Brian and other users
    primarily use VNC (Screen Sharing), not SSH. The MagicDNS fix must be
    prioritized because VNC can't use SSH config IP mappings.

13. **Verify you're on the right machine via Screen Sharing.** When accessing
    multiple home machines via VNC, it's easy to run migration commands on the
    wrong machine (e.g., toggling franklin's extension when you meant yoga).

14. **`brew services list` shows `none` even when tailscaled is running.** This
    is a cosmetic bug with `brew services` when the service was started with
    `sudo`. The daemon is actually running — verify with `pgrep -fl tailscaled`
    and `sudo launchctl list | grep tailscale`. If launchctl shows a PID and
    exit status `0`, everything is fine.

15. **Tailscale daemon doesn't auto-start after reboot unless properly loaded.**
    `sudo brew services start tailscale` must be run at least once to load the
    LaunchDaemon plist. The plist at `/Library/LaunchDaemons/homebrew.mxcl.tailscale.plist`
    has `RunAtLoad=true` and `KeepAlive=true`, so once loaded it should persist.
    After a reboot, verify with `sudo launchctl list | grep tailscale` — if it
    shows no entry, run `sudo brew services restart tailscale` and then
    `sudo tailscale up --ssh --accept-routes`.

16. **Macs going to sleep kills Tailscale connectivity.** Even with `sleep 0`
    configured (`sudo pmset -a sleep 0`), Macs can still sleep if the display
    sleeps and no active assertions prevent it. For production servers like net,
    also set `sudo pmset -a displaysleep 0` or ensure a Screen Sharing session
    holds a `PreventSystemSleep` assertion. After waking, tailscaled may need
    a restart.

17. **Tagging a device via API works without re-registration.** You can tag an
    existing user-owned device with `tag:talkbank` via `POST /device/{id}/tags`
    without needing to `tailscale logout` + re-register with an auth key.
    However, if the device shows as a different node than expected (e.g.,
    `monkey-1` vs `monkey`), re-registration with the auth key is cleaner.

18. **Non-`macw` user accounts need ACL updates.** The default ACL only allows
    SSH as `macw`. Machines with different user accounts (e.g., `cbs` on CBS)
    need that username added to the SSH ACL `users` list.

19. **DNS cache goes stale after re-registration.** When a machine does
    `tailscale logout` + re-register, it gets a new Tailscale IP. MagicDNS
    updates immediately (querying `100.100.100.100` returns the new IP), but
    macOS caches the old answer. Fix: `sudo dscacheutil -flushcache && sudo
    killall -HUP mDNSResponder` on the connecting machine. Without flush, wait
    for TTL to expire (can take minutes). Also update SSH config IP mappings on
    all machines that have them.

20. **Re-registered nodes get a `-1` suffix and must be renamed.** After
    `tailscale logout` + re-register, the new node registers as `hostname-1`.
    Rename via API: `POST /device/{id}/name` with `{"name":"hostname"}`. The
    old node (if not deleted first) will be marked offline and can be deleted.
