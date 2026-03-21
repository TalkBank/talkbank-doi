# Ansible Operator Runbook

This is the first-time operator guide for the internal `batchalign3` deployment
flow. It is written for people who are new to Ansible and want the shortest
reliable path from "I have the repo" to "I can deploy and verify a machine."

## What Ansible Does Here

Ansible is the orchestration layer for the internal `batchalign3` fleet. It is
responsible for:

- naming the machines in one inventory
- grouping machines into server/client/fleet targets
- pushing config and wheels
- restarting the server on the right hosts
- checking health after deploys

The old SSH-heavy deploy scripts are retired. The remaining shell wrapper is a
thin convenience layer over `ansible-playbook`.

## Prerequisites

Install Ansible on the controller machine, which is the machine you run deploys
from:

```bash
brew install ansible
```

or:

```bash
uv tool install ansible
```

You also need:

- SSH access to the managed machines
- access to the `talkbank` repo checkout
- a working Tailscale connection if you are using the Tailscale hostnames

## Where To Start

Work from the Ansible directory:

```bash
cd /Users/chen/talkbank/deploy/ansible
```

`ansible.cfg` in this directory automatically selects `inventory.yml`, so most
commands can be run from here without extra flags.

## Inventory Layout

The inventory lives in [inventory.yml](/Users/chen/talkbank/deploy/ansible/inventory.yml).

Important groups:

- `batchalign3_server`
  Current internal server target. Right now this is `net`.
- `batchalign3_clients`
  Current client-install targets.
- `batchalign3_fleet`
  Union of the server and client groups.

Other useful groups:

- `compute_servers`
- `home`
- `laptop`
- `hongkong`
- `all_macs`

The inventory also tracks per-host details such as `ansible_user`,
`ram_gb`, and `max_concurrent_jobs`.

## The Three Commands You Will Use Most

Check reachability:

```bash
ansible all -m ping
```

Inspect the resolved inventory:

```bash
ansible-inventory --graph
ansible-inventory --host ming
```

Check internal server health:

```bash
ansible-playbook playbooks/status.yml
```

## Deploy Paths

The main deploy entrypoint is the thin wrapper:

```bash
bash /Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh
```

What it does:

- builds the dashboard
- builds a `batchalign3` wheel
- calls `ansible-playbook playbooks/deploy.yml`
- targets `batchalign3_fleet` by default

Common variants:

```bash
# Preview only
bash /Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh --dry-run

# Deploy only the server host(s)
bash /Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh --server

# Deploy only the client-install hosts
bash /Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh --clients

# Deploy to specific hosts
bash /Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh bilbo brian

# Reuse an existing wheel
bash /Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh --no-build --clients
```

If you want to use raw Ansible directly, the deploy playbook expects a wheel
path:

```bash
ansible-playbook playbooks/deploy.yml \
  --limit batchalign3_server \
  -e batchalign3_wheel=/absolute/path/to/batchalign3.whl
```

## What The Playbooks Do

`playbooks/deploy.yml`

- server hosts: install the wheel, write config, restart the daemon, wait for
  `/health`
- client hosts: install the wheel only

`playbooks/configure.yml`

- update `server.yaml` and `fleet.yaml` without reinstalling the wheel

`playbooks/status.yml`

- query `/health` on the server group and report worker/job/version status

`playbooks/tailscale-health.yml`

- check Tailscale version, SSH status, DNS, uptime, and memory

`playbooks/tailscale-fix.yml`

- restart Tailscale on a host and repair DNS if needed

`playbooks/mount-media.yml`

- configure the NFS media mounts for client machines

## Sanity Checks

Before and after a deploy, use these checks:

```bash
ansible-playbook playbooks/deploy.yml --syntax-check \
  -e batchalign3_wheel=/tmp/batchalign3.whl

ansible-playbook playbooks/deploy.yml --list-hosts \
  --limit batchalign3_fleet \
  -e batchalign3_wheel=/tmp/batchalign3.whl

ansible-playbook playbooks/status.yml
```

If you are testing a single host, narrow the limit:

```bash
ansible-playbook playbooks/deploy.yml \
  --limit bilbo \
  -e batchalign3_wheel=/absolute/path/to/batchalign3.whl
```

## What To Remember

- `batchalign3_server` is the internal server group.
- `batchalign3_clients` are install-only targets.
- `batchalign3_fleet` is the normal wrapper default.
- Use the wrapper for day-to-day deploys.
- Use raw `ansible-playbook` when you need syntax checks, host limits, or
  explicit extra vars.

## Related Docs

- [Fleet Management Plan](/Users/chen/talkbank/deploy/docs/fleet-management-plan.md)
- [Fleet Inventory](/Users/chen/talkbank/deploy/docs/fleet-inventory.md)
- [Deploy Script](/Users/chen/talkbank/deploy/scripts/deploy_batchalign3.sh)
