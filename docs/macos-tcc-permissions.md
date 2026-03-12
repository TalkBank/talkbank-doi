# macOS TCC Permissions for Fleet Machines

**Status:** Reference
**Last updated:** 2026-03-12

## Problem

macOS TCC (Transparency, Consent, and Control) requires GUI-click approval for
folder access, Full Disk Access, and other sensitive permissions. When SSH'd into
a machine via Tailscale, scripts that touch protected folders (`~/Desktop`,
`~/Documents`, `~/Downloads`, etc.) trigger a dialog on the physical screen.
Nobody is there to click it. The script silently fails or hangs.

This has caused deploy scripts and maintenance tasks to fail without any useful
error message — the process simply gets denied access and either errors out or
produces incomplete results.

## Background: How TCC Works

Every macOS app that accesses protected resources must be explicitly approved by
the user. Permissions are stored in SQLite databases:

- System-wide: `/Library/Application Support/com.apple.TCC/TCC.db` (SIP-protected)
- Per-user: `~/Library/Application Support/com.apple.TCC/TCC.db`

There is **no way** to grant these permissions fully unattended. Apple designed
it so that at least one interactive approval is always required. The question is
how to minimize the number of times you need to do it.

## Key Insight: TCC Inheritance

TCC permissions are inherited by child processes from their parent. The chain
that matters depends on how you're connected:

**Local console (Terminal.app):**
Terminal.app → shell → all child processes (scripts, rsync, python, etc.)
→ Grant **Terminal.app** FDA.

**Tailscale SSH:**
tailscaled → login shell → all child processes
→ Grant **tailscaled** (`/opt/homebrew/opt/tailscale/bin/tailscaled`) FDA.

**tmux:** tmux processes inherit from whatever started them. If you SSH in via
Tailscale and start tmux, tmux and all its panes inherit from tailscaled. If
you detach and reattach from a different session, the tmux server process still
has the permissions from its original parent — this is fine as long as
tailscaled had FDA when tmux was first started. **However**, if tmux was started
from a console Terminal session and you later reattach via Tailscale SSH, the
tmux server has Terminal's permissions (which may differ from tailscaled's).

**Bottom line: grant FDA to both Terminal.app AND tailscaled.** This covers
local console use, Tailscale SSH, and tmux in either scenario.

This is a security consideration: any code running in these contexts gets access
to all user files. For our use case (university research machines behind
Tailscale, not public-facing), this is acceptable.

## Current Fleet

| Host | Role | Access |
|------|------|--------|
| `net` | Server (production) | `ssh macw@net` |
| `bilbo`, `brian`, `davida`, `frodo`, `andrew`, `lilly`, `sue`, `vaishnavi` | Clients | `ssh macw@<host>` |

All machines run macOS and connect via Tailscale SSH.

## Immediate Fix: Screen-Sharing Approval

For each machine (~5 minutes per machine, ~45 minutes total):

1. **Connect via screen sharing:**
   ```bash
   # If screen sharing is enabled:
   open vnc://macw@bilbo
   # Or use Apple Remote Desktop
   ```

2. **Grant Full Disk Access to Terminal AND tailscaled:**
   - System Settings → Privacy & Security → Full Disk Access
   - Click the lock to authenticate if needed
   - Toggle **Terminal** on (or click `+` and add `/Applications/Utilities/Terminal.app`)
   - Click `+`, press Cmd+Shift+G, enter `/opt/homebrew/opt/tailscale/bin/tailscaled`
     (this is the Homebrew-managed binary that runs as a LaunchDaemon)

3. **Grant FDA to other dev tools that may trigger dialogs** (see full list below):
   - System Settings → Privacy & Security → Full Disk Access
   - Add any of the following that are installed on the machine:
     - `/Applications/iTerm.app` (if used instead of Terminal)
     - `/Applications/Xcode.app` (needed for `xcodebuild`, `xcrun`)
     - `/usr/bin/ssh` (for outbound SSH from the machine)

5. **Verify from Tailscale SSH:**
   ```bash
   ssh macw@bilbo       # via Tailscale
   ls ~/Desktop/        # Should succeed without triggering a dialog
   ls ~/Documents/      # Should succeed
   ls ~/Downloads/      # Should succeed
   ```

### Checklist

Track which machines have been configured:

- [ ] net
- [ ] bilbo
- [ ] brian
- [ ] davida
- [ ] frodo
- [ ] andrew
- [ ] lilly
- [ ] sue
- [ ] vaishnavi

## After macOS Upgrades

Major macOS upgrades (e.g., Ventura → Sonoma → Sequoia) can **reset TCC
permissions**. After upgrading any fleet machine:

1. SSH in and test: `ls ~/Desktop/`
2. If it fails or triggers a dialog, repeat the screen-sharing approval
3. Also re-check that tailscaled has FDA (the binary path may change if
   Homebrew relocates the Cellar)

Add this to the macOS upgrade checklist.

## Long-Term: MDM-Based Permission Management

For a more maintainable approach, especially as the fleet grows or changes
hands, use an MDM server to push PPPC (Privacy Preferences Policy Control)
profiles. This is the only Apple-supported mechanism for pre-approving TCC
permissions without user interaction (after initial MDM enrollment).

### Why MDM

- Push permission changes to all machines at once
- No per-machine screen sharing visits after initial enrollment
- Survives macOS upgrades (MDM profiles are re-applied)
- Audit trail of what permissions are granted where
- Also useful for: OS update policies, security baselines, fleet inventory

### MDM Options

| Solution | Cost | Notes |
|----------|------|-------|
| [Fleet](https://fleetdm.com/) | Free (open-source, MIT) | Best option. Includes osquery for fleet visibility. Could run on `net`. |
| [NanoMDM](https://github.com/micromdm/nanomdm) | Free (open-source) | Minimalist. More library than turnkey product. |
| [SimpleMDM](https://simplemdm.com/) | ~$4/device/month | Easiest commercial option. No server to run. |
| [Mosyle](https://mosyle.com/) | Free tier for ≤30 devices | Full-featured, education-friendly. |

### MDM Enrollment (One-Time Per Machine)

Each Mac must be enrolled in the MDM exactly once. This requires one interactive
approval (System Settings → General → Device Management → approve the MDM
profile). Can be done via screen sharing. After that, all future profile pushes
are silent.

### PPPC Profile for Terminal + Tailscale

Once MDM is set up, push a profile like this. The profile grants Full Disk
Access and folder access to Terminal and tailscaled:

```xml
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN"
  "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>PayloadContent</key>
    <array>
        <dict>
            <key>PayloadType</key>
            <string>com.apple.TCC.configuration-profile-policy</string>
            <key>PayloadIdentifier</key>
            <string>com.talkbank.pppc.tcc</string>
            <key>PayloadUUID</key>
            <string><!-- generate with uuidgen --></string>
            <key>PayloadVersion</key>
            <integer>1</integer>
            <key>Services</key>
            <dict>
                <key>SystemPolicyAllFiles</key>
                <array>
                    <dict>
                        <key>Identifier</key>
                        <string>com.apple.Terminal</string>
                        <key>IdentifierType</key>
                        <string>bundleID</string>
                        <key>CodeRequirement</key>
                        <string>identifier "com.apple.Terminal" and anchor apple</string>
                        <key>Allowed</key>
                        <true/>
                    </dict>
                    <dict>
                        <key>Identifier</key>
                        <string>/opt/homebrew/opt/tailscale/bin/tailscaled</string>
                        <key>IdentifierType</key>
                        <string>path</string>
                        <key>CodeRequirement</key>
                        <string></string>
                        <key>Allowed</key>
                        <true/>
                        <key>Comment</key>
                        <string>Homebrew tailscaled — ad-hoc signed Go binary, identified by path</string>
                    </dict>
                </array>
                <key>SystemPolicyDesktopFolder</key>
                <array>
                    <dict>
                        <key>Identifier</key>
                        <string>com.apple.Terminal</string>
                        <key>IdentifierType</key>
                        <string>bundleID</string>
                        <key>CodeRequirement</key>
                        <string>identifier "com.apple.Terminal" and anchor apple</string>
                        <key>Allowed</key>
                        <true/>
                    </dict>
                </array>
                <key>SystemPolicyDocumentsFolder</key>
                <array>
                    <dict>
                        <key>Identifier</key>
                        <string>com.apple.Terminal</string>
                        <key>IdentifierType</key>
                        <string>bundleID</string>
                        <key>CodeRequirement</key>
                        <string>identifier "com.apple.Terminal" and anchor apple</string>
                        <key>Allowed</key>
                        <true/>
                    </dict>
                </array>
                <key>SystemPolicyDownloadsFolder</key>
                <array>
                    <dict>
                        <key>Identifier</key>
                        <string>com.apple.Terminal</string>
                        <key>IdentifierType</key>
                        <string>bundleID</string>
                        <key>CodeRequirement</key>
                        <string>identifier "com.apple.Terminal" and anchor apple</string>
                        <key>Allowed</key>
                        <true/>
                    </dict>
                </array>
            </dict>
        </dict>
    </array>
    <key>PayloadDescription</key>
    <string>TalkBank fleet TCC permissions for remote management</string>
    <key>PayloadDisplayName</key>
    <string>TalkBank TCC Policy</string>
    <key>PayloadIdentifier</key>
    <string>com.talkbank.pppc</string>
    <key>PayloadOrganization</key>
    <string>TalkBank</string>
    <key>PayloadType</key>
    <string>Configuration</string>
    <key>PayloadUUID</key>
    <string><!-- generate with uuidgen --></string>
    <key>PayloadVersion</key>
    <integer>1</integer>
</dict>
</plist>
```

**Important:** PPPC profiles **must originate from a User Approved MDM server**.
Installing them manually via `profiles install` or double-clicking **does not
work** — macOS Sequoia rejects the TCC payload silently. This is by Apple's
design.

### Getting CodeRequirements

To get the exact code signing requirement for any binary on your machines:

```bash
codesign -dr - /opt/homebrew/opt/tailscale/bin/tailscaled 2>&1 | awk -F ' => ' '/designated/{print $2}'
```

Note: Homebrew-built Go binaries are ad-hoc signed (no Apple Developer
certificate), so the CodeRequirement is typically just an identifier check. For
path-based PPPC entries, the CodeRequirement can be left empty.

### Profile Generation Tools

- [PPPC-Utility](https://github.com/jamf/PPPC-Utility) — Jamf's open-source macOS app. Drag an app in, it reads code signing info and generates the .mobileconfig.
- [iMazing Profile Editor](https://imazing.com/profile-editor) — Free macOS app for editing .mobileconfig files.

## Other Apps That May Need TCC Attention

Beyond Terminal and tailscaled, these are apps commonly used in development and
remote management that can trigger TCC dialogs. Grant them FDA if they're
installed on fleet machines and used in remote/automated contexts.

### Shells and Terminal Emulators

| App | Bundle ID | Notes |
|-----|-----------|-------|
| Terminal.app | `com.apple.Terminal` | Must have FDA for local console use |
| iTerm2 | `com.googlecode.iterm2` | If anyone uses it instead of Terminal |
| tmux | N/A (inherits from parent) | No separate TCC entry needed — inherits from Terminal or tailscaled |
| screen | N/A (inherits from parent) | Same as tmux |

### Remote Access and SSH

| App | Path / Bundle ID | Notes |
|-----|------------------|-------|
| tailscaled (Homebrew) | `/opt/homebrew/opt/tailscale/bin/tailscaled` | **Critical.** SSH sessions via Tailscale inherit from this. |
| sshd (Apple) | `/usr/sbin/sshd` | If using regular SSH alongside Tailscale. Grant FDA so SSH sessions can access protected folders. |
| Apple Remote Desktop (ARD) | `com.apple.RemoteDesktop` | May need Accessibility + FDA on the target machine. |

### Build and Development Tools

| App | Notes |
|-----|-------|
| Xcode / `xcodebuild` | Needs FDA for code signing operations that access keychains. Also triggers Accessibility dialogs for UI testing. |
| `xcrun` | Inherits from parent shell, but Xcode.app itself may need FDA. |
| VS Code | `com.microsoft.VSCode` — needs FDA if used to edit files in protected folders. Not typically used on fleet machines. |

### System Utilities Used in Scripts

| Tool | TCC concern |
|------|-------------|
| `rsync` | Inherits from parent. If parent has FDA, rsync works fine. |
| `cp`, `mv`, `rm` | Same — inherits. |
| `python3`, `cargo`, `uv` | Same — inherits. No separate TCC entry needed. |
| `hdiutil` | May trigger a dialog when creating DMGs from protected folders. Inherits from parent. |
| `codesign` | May need Keychain access (separate from FDA). Usually works if invoked from an FDA-approved parent. |
| `cron` / `launchd` agents | **Watch out.** LaunchAgents and LaunchDaemons do NOT inherit from Terminal. The launched process itself needs FDA. See section below. |

### LaunchAgents and LaunchDaemons (Important)

Processes started by `launchd` (via `.plist` files in `~/Library/LaunchAgents/`
or `/Library/LaunchDaemons/`) run outside of any terminal session. They do NOT
inherit FDA from Terminal or tailscaled. If a launchd-managed service needs to
access protected folders:

- The **specific binary** listed in the plist's `ProgramArguments` must have its
  own FDA grant.
- For our batchalign3 server (if run as a LaunchDaemon), the `batchalign3`
  binary itself would need FDA.
- For Python workers spawned by the server, they inherit from the server process.

To grant FDA to a specific binary:
1. System Settings → Privacy & Security → Full Disk Access
2. Click `+`, press Cmd+Shift+G to open the path dialog
3. Enter the full path (e.g., `/Users/macw/.local/bin/batchalign3`)

### Automation and Apple Events

Some scripts use `osascript` or `open` to interact with other apps. This
triggers the **Automation** (Apple Events) TCC category, which is separate from
FDA. If a deploy script uses `osascript` to, say, quit an app before
redeploying, the calling app needs Automation permission for the target app.

This is less common for our use case but worth knowing about.

## What Does NOT Work

| Approach | Why it fails |
|----------|-------------|
| `profiles install` with .mobileconfig | PPPC payloads are rejected unless from a User Approved MDM |
| `tccutil` (built-in) | Can only **reset** (remove) permissions, not grant them |
| Direct TCC.db editing | Requires SIP disabled (Recovery Mode, physical access). Fragile — schema changes between macOS versions. |
| Any fully unattended approach | Apple requires at least one interactive approval, always |

## TCC Service Names Reference

| Permission | Profile key | TCC database key |
|------------|------------|------------------|
| Full Disk Access | `SystemPolicyAllFiles` | `kTCCServiceSystemPolicyAllFiles` |
| Desktop folder | `SystemPolicyDesktopFolder` | `kTCCServiceSystemPolicyDesktopFolder` |
| Documents folder | `SystemPolicyDocumentsFolder` | `kTCCServiceSystemPolicyDocumentsFolder` |
| Downloads folder | `SystemPolicyDownloadsFolder` | `kTCCServiceSystemPolicyDownloadsFolder` |
| Accessibility | `Accessibility` | `kTCCServiceAccessibility` |
| Automation/Apple Events | `AppleEvents` | `kTCCServiceAppleEvents` |

## Debugging TCC Issues

When a script fails mysteriously on a remote machine:

```bash
# Check what TCC permissions Terminal has (per-user database)
sqlite3 ~/Library/Application\ Support/com.apple.TCC/TCC.db \
  "SELECT service, client, auth_value FROM access WHERE client = 'com.apple.Terminal';"

# auth_value: 0=denied, 2=allowed

# Check system Console.app logs for TCC denials (last 5 minutes)
log show --predicate 'subsystem == "com.apple.TCC"' --last 5m --style compact

# Check if a specific path is TCC-protected
# (protected folders: ~/Desktop, ~/Documents, ~/Downloads, ~/Library/Mail, etc.)
```

## Homebrew Tailscale Upgrades

Our fleet uses the **Homebrew open-source `tailscaled`** (CLI-only, no GUI, no
App Store). This is the only variant that supports Tailscale SSH server. It runs
as a LaunchDaemon managed by `brew services`.

### The Upgrade Problem

When you run `brew upgrade tailscale`:

1. Homebrew downloads the new version to `/opt/homebrew/Cellar/tailscale/X.Y.Z/`
2. The symlink `/opt/homebrew/opt/tailscale` is updated to point to the new version
3. **The running `tailscaled` process is NOT restarted.** It's still the old binary
   loaded in memory (the old Cellar directory remains on disk).
4. The LaunchDaemon plist points at the symlink path
   (`/opt/homebrew/opt/tailscale/bin/tailscaled`), so launchd *would* start the
   new version on next boot or manual restart — but until then, you're running
   the old binary.

This can cause:
- **Version mismatch between `tailscale` CLI and `tailscaled`** — the CLI binary
  resolves to the new version immediately (it's a new process each invocation),
  but the daemon is still old. Some commands may fail or behave unexpectedly.
- **Stale TCC permissions** — if macOS grants FDA by code signature and the
  signature changes between versions, the old TCC grant might not apply to the
  new binary. In practice, Homebrew-built Go binaries are ad-hoc signed, so this
  is less likely than with app bundles, but it's a risk.
- **SSH sessions drop during restart** — if you restart tailscaled while
  connected via Tailscale SSH, your session dies and you need to reconnect.

### Correct Upgrade Procedure

```bash
# 1. Upgrade the formula
brew upgrade tailscale

# 2. Restart the daemon (requires sudo because it's a LaunchDaemon)
sudo brew services restart tailscale

# 3. Verify versions match
tailscale version          # CLI version
tailscale status           # Shows daemon version in the connection info

# 4. Verify SSH still works (from another machine)
ssh macw@<host>            # Test from a different terminal before closing this one
```

**Important:** If you're connected via Tailscale SSH when you restart, your
session will drop. Always have a backup access method (screen sharing, Apple
Remote Desktop, or physical access) when upgrading tailscale on remote machines.

### Upgrade Script for Fleet

For upgrading all fleet machines from a local terminal:

```bash
#!/usr/bin/env bash
# Upgrade tailscale on a fleet machine via Tailscale SSH.
# Run this FROM your local machine, not on the target.
# Your SSH session will survive because you reconnect after the restart.
set -euo pipefail

host="${1:?Usage: $0 <hostname>}"

echo "=== Upgrading tailscale on $host ==="

ssh "macw@$host" 'brew upgrade tailscale 2>&1 || echo "Already up to date"'
echo "Restarting tailscaled (SSH will briefly drop)..."
ssh "macw@$host" 'sudo brew services restart tailscale' || true
sleep 3
echo "Reconnecting to verify..."
ssh "macw@$host" 'tailscale version && tailscale status | head -5'
echo "=== Done: $host ==="
```

### LaunchDaemon Details

The Homebrew service plist lives at:
```
/Library/LaunchDaemons/homebrew.mxcl.tailscale.plist
```

It runs `/opt/homebrew/opt/tailscale/bin/tailscaled` (a symlink that Homebrew
updates during `brew upgrade`). The daemon runs as root with `KeepAlive` and
`RunAtLoad`, so it starts at boot and auto-restarts if it crashes.

Logs go to `/opt/homebrew/var/log/tailscaled.log`.

## Related Docs

- [Apple: PPPC payload settings](https://support.apple.com/guide/deployment/privacy-preferences-policy-control-payload-dep38df53c2a/web)
- `docs/code-signing-and-distribution.md` — Apple signing for our own binaries
