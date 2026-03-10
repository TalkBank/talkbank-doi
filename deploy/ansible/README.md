# TalkBank Ansible Fleet Management

Ansible playbooks for managing the TalkBank fleet of 16 devices across CMU lab,
Brian's home, Hong Kong, and cloud servers. All machines communicate over
Tailscale.

## Prerequisites

- Ansible installed on the controller machine (ming): `brew install ansible`
- SSH access to fleet machines (via Tailscale hostnames or SSH keys)
- Run all commands from this directory (`talkbank-private/batchalign/ansible/`)

`ansible.cfg` in this directory auto-selects `inventory.yml` and configures
Tailscale-friendly SSH settings (no host key checking, pipelining, 10s timeout).

## Quick Reference

```bash
# Who's reachable right now?
ansible all -m ping

# Full Tailscale health report (version, SSH, DNS, uptime, RAM)
ansible-playbook playbooks/tailscale-health.yml

# Health report for just CMU lab machines
ansible-playbook playbooks/tailscale-health.yml --limit compute_servers

# Restart Tailscale on a specific machine
ansible-playbook playbooks/tailscale-fix.yml --limit brian

# Batchalign server health (HTTP /health endpoint)
ansible-playbook playbooks/status.yml

# Deploy batchalign to a specific machine
ansible-playbook playbooks/deploy.yml --limit bilbo \
  -e batchalign3_wheel=/path/to/wheel.whl \
  -e batchalign_core_wheel=/path/to/core.whl
```

## Inventory Groups

| Group | Hosts | Purpose |
|-------|-------|---------|
| `compute_servers` | net, bilbo, brian, davida, frodo, andrew, lilly, sue, vaishnavi | CMU lab batchalign servers |
| `home` | study, monkey | Brian's home machines |
| `laptop` | hermes | Brian's laptop |
| `hongkong` | tb-hk, cbs | Hong Kong machines |
| `cloud` | talkbank, git-talkbank | Linux servers |
| `all_macs` | all of the above except cloud | macOS-specific tasks |

See `inventory.yml` for per-host RAM values and batchalign worker counts.

## Playbooks

### `playbooks/ping.yml` — Connectivity Check

Quick check of which machines are reachable. Unreachable hosts are reported
gracefully (no abort).

### `playbooks/tailscale-health.yml` — Tailscale Health Report

Reports per host:
- Tailscale version and self-status line
- Whether Tailscale SSH is enabled (`RunSSH`)
- Whether the `ts.net` DNS resolver is in place
- System uptime and total RAM

### `playbooks/tailscale-fix.yml` — Restart Tailscale

Always use `--limit` to target specific machines:

```bash
ansible-playbook playbooks/tailscale-fix.yml --limit brian
```

Actions:
1. Restarts `tailscaled` (via `brew services` on macOS, `systemctl` on Linux)
2. Waits for Tailscale to come back (retries 10x at 3s intervals)
3. Checks if SSH is enabled, warns if not
4. Checks for `ts.net` DNS resolver, deploys it if missing (macOS only)

### `playbooks/status.yml` — Batchalign Server Health

Queries the `/health` HTTP endpoint on each compute server. Reports worker
count, active jobs, and version.

### `playbooks/deploy.yml` — Deploy Batchalign

Full deployment: install wheels, configure, restart. Runs one host at a time
(`serial: 1`). Requires pre-built wheels passed as extra vars.

### `playbooks/configure.yml` — Update Configuration

Push server.yaml and fleet.yaml without reinstalling batchalign.

### `playbooks/mount-media.yml` — NFS Media Mounts

Set up NFS mounts for TalkBank media volumes on client machines.

### `playbooks/migrate-cache.yml` — Migrate Validation Cache

Migrate the talkbank validation cache between machines.

## Known Unreachable Machines

As of 2026-02-22:
- **lilly** — SSH timeout, probably powered off by user. Last seen on Tailscale 2d ago.
- **cbs** — SSH timeout. Has `ansible_user: cbs` (not `macw`).

Both are handled gracefully by `ignore_unreachable: true` in the Tailscale playbooks.

## Related Docs

- `../docs/fleet-inventory.md` — Canonical machine inventory with Tailscale IPs
- `../docs/fleet-management-plan.md` — Architecture and future plans
- `../docs/ssh-key-migration.md` — SSH key state per machine
