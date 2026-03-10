# TalkBank Fleet Inventory

> Canonical list of all devices on the TalkBank Tailscale tailnet.
> This is the single source of truth for fleet management.
>
> Last updated: 2026-03-08

## CMU Lab — Batchalign Compute Servers

These machines run batchalign as compute servers. All are macOS, Apple Silicon, user `macw`.

| Hostname | Tailscale IP | Owner | RAM | Notes |
|----------|-------------|-------|-----|-------|
| net | 100.92.113.40 | macw (Brian) | 256 GB | Primary server, NFS media, Redis cache; max 8 workers |
| bilbo | 100.123.21.75 | macw (Brian) | 256 GB | |
| brian | 100.87.231.106 | macw (Brian) | 96 GB | Brian's personal office machine |
| davida | 100.102.15.39 | Davida Fromm | 32 GB | Davida's workstation |
| frodo | 100.78.37.95 | macw (Brian) | 32 GB | |
| andrew | 100.84.163.104 | Andrew (student) | 32 GB | Student workstation |
| lilly | 100.96.102.124 | Lilly (student) | ? | Student workstation; probably turned off by user |
| sue | 100.84.206.29 | Sue (student) | 64 GB | Student workstation |
| vaishnavi | 100.117.243.101 | Vaishnavi (student) | 64 GB | Student workstation |

## Brian's Home Machines

macOS, user `macw`. Not batchalign servers.

| Hostname | Tailscale IP | RAM | Notes |
|----------|-------------|-----|-------|
| study | 100.97.156.40 | 96 GB | Old Ansible inventory incorrectly listed 32 GB |
| monkey | 100.74.127.49 | 32 GB | |

## Brian's Laptop

| Hostname | Tailscale IP | Notes |
|----------|-------------|-------|
| hermes | 100.78.144.37 | macOS, user `macw`, 16 GB |

## Hong Kong

| Hostname | Tailscale IP | Owner | Notes |
|----------|-------------|-------|-------|
| tb-hk | 100.76.9.18 | macw (Brian) | TalkBank HK server, macOS, 48 GB |
| cbs | 100.65.39.70 | ? | macOS; SSH user `cbs`; was reachable earlier today, currently timed out |

## Cloud Servers (Linux)

| Hostname | Tailscale IP | Role | Notes |
|----------|-------------|------|-------|
| talkbank | 100.72.29.3 | talkbank.org web server | |
| git-talkbank | 100.108.195.47 | GitLab server | Being decommissioned |

## Franklin's Personal Machines (not fleet-managed)

| Hostname | Tailscale IP | OS | Notes |
|----------|-------------|-----|-------|
| franklin | 100.65.53.125 | macOS | Home |
| chen | 100.85.98.80 | macOS | Home |
| ming | 100.96.86.56 | macOS | CMU lab, dev machine |
| snorri | 100.68.24.109 | macOS | Home, M2 Mac mini, 8 GB |
| swan | 100.122.40.75 | macOS | Home, 2017 iMac (Intel), 32 GB |
| yoga | 100.102.163.27 | macOS | Home, Intel |
| franklin-ipad-pro | 100.75.180.66 | iOS | Offline 35d |
| franklin16e | 100.126.171.119 | iOS | |

## Planned / Not Yet on Tailnet

| Hostname | Owner | Notes |
|----------|-------|-------|
| fromm | Davida Fromm | Davida's laptop; accidentally deleted from Tailscale; to be re-added |

## Not Managed

| Hostname | Tailscale IP | Owner | Notes |
|----------|-------------|-------|-------|
| extra | 100.67.15.16 | Ross MacWhinney | Brian's son, in NY; still on Tailscale GUI, not CLI |
| shrs-cv9gfycf | 100.114.68.43 | smwilsonau@ | External collaborator; offline 77d |

---

## Fleet Management Scope

**Managed via Ansible** (all of the above except Franklin's personal machines and external collaborators):
- 9 CMU lab compute servers
- 2 Brian's home machines
- 1 Brian's laptop
- 2 Hong Kong machines
- 2 cloud servers
- **Total: 16 managed devices**

**Batchalign compute servers** (subset of managed):
- net, bilbo, brian, davida, frodo, andrew, lilly, sue, vaishnavi
- **Total: 9 servers**
