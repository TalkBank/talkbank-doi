# Fleet Management Plan — TalkBank Mac Fleet

> **Historical note:** This is a private planning document for an earlier fleet
> rollout model. It is not current public `batchalign3` release guidance.

Assessment of next steps for turning the lab's Mac fleet into a properly managed, self-sufficient collection of batchalign servers with shared media access and automated deployment.

## Current State

### What We Have

**Machines** (all macOS, all Apple Silicon, all on Tailscale):

| Machine | Tailscale IP | User | RAM | Current Role |
|---------|-------------|------|-----|-------------|
| net | 100.92.113.40 | macw | 256 GB | Production server (port 8000) + media volumes |
| bilbo | 100.123.21.75 | macw | 256 GB | Compute server |
| brian | 100.87.231.106 | macw | 96 GB | Brian's office machine |
| davida | 100.102.15.39 | macw | 32 GB | Workstation |
| fromm | 100.74.24.120 | macw | 16 GB | Davida's laptop |
| frodo | 100.78.37.95 | macw | 32 GB | Compute server |
| study | 100.97.156.40 | macw | 96 GB | Brian's home |
| monkey | 100.74.127.49 | macw | 32 GB | Brian's home |
| ming | 100.96.86.56 | **chen** | 64 GB | Dev machine |
| andrew | 100.84.163.104 | macw | 32 GB | Student workstation |
| lilly | 100.97.70.53 | macw | 64 GB | Student workstation |
| sue | 100.84.206.29 | macw | 64 GB | Student workstation |
| vaishnavi | 100.117.243.101 | macw | 64 GB | Student workstation |
| hermes | 100.78.144.37 | macw | 16 GB | Brian's laptop |
| cbs | 100.65.39.70 | cbs | 64 GB | Hong Kong |
| tb-hk | 100.76.9.18 | macw | 48 GB | Hong Kong |
| extra | 100.67.15.16 | macw | ? | Unreachable |

**Important**: Not all machines use the same user account. Most use `macw`, but ming uses `chen` (no `macw` account exists there). Other machines may also have different users. The Ansible inventory must track `ansible_user` per host. Tailscale SSH may help normalize access regardless of local account names.

**All machines will become servers.** The weakest machines have 64 GB RAM — enough for 2 concurrent workers at ~25 GB each. No machine is too small.

**Networking**: All machines on the same Tailscale tailnet (free academic license). Encrypted mesh — any machine can reach any other by hostname. No firewall rules needed.

**Media**: Two external volumes physically attached to Net:
- `/Volumes/CHILDES/CHILDES` — CHILDES corpus media
- `/Volumes/Other/<corpus>` — all other TalkBank corpora (aphasia, dementia, etc.)
- `/Volumes/HomeBank/homebank` — HomeBank media

**Deployment**: The old SSH-heavy shell scripts have been superseded by Ansible plus a thin wrapper. Build once, deploy via inventory-driven playbooks.

**Server**: Only Net runs as a full server. Clients send HTTP requests to Net. The new daemon feature auto-starts a local server on each machine, but without media access it can only do morphotag/translate/utseg (not align/transcribe).

### What's Wrong

1. **Single point of failure**: Net is the only server. If Net is busy, 5 clients sit idle.
2. **SSH key management**: Ad-hoc `~/.ssh/id_rsa_macw` + keychain. No central key management. Keys must be manually copied to each machine. Breaks silently.
3. **No media on clients**: Client machines can't do align/transcribe locally because media files live only on Net's physical volumes.
4. **Manual configuration**: server.yaml, LaunchDaemon plist, PATH setup — all manually managed per machine. No way to know if a machine drifted from the expected state.
5. **No inventory**: We don't systematically know which machines have how much RAM, which macOS version, or which batchalign version is running.
6. **No monitoring**: No way to see at a glance which machines are healthy, which have stale daemons, which are processing jobs.

---

## Goal State

Every Mac in the fleet is a full-fledged batchalign server that:
- Has read-write NFS access to all TalkBank media (mounted from Net)
- Runs a persistent batchalign server via launchd LaunchDaemon
- Can process any command locally (morphotag, align, transcribe, translate, etc.)
- Is managed centrally — one command to deploy, configure, or check status across all machines
- Uses Tailscale for all inter-machine communication (SSH, NFS, HTTP)

---

## Investigation Areas

### 1. Media Sharing — NFS over Tailscale (DECIDED)

**Decision**: NFS read-write over Tailscale. Coexists with existing SMB — users who prefer Finder's "Connect to Server" (`smb://net/...`) keep doing that. NFS auto-mounts give always-on access without any manual step.

**Why NFS over SMB for automation**:
- No credential management (SMB requires stored passwords; NFS uses host-based auth on the Tailscale subnet)
- Auto-mount at boot — media is always visible in Finder without "Connect to Server"
- Slightly lower protocol overhead for directory listing (relevant when scanning thousands of files)
- macOS ships with NFS server and client built in

**Coexistence with SMB**: Net currently serves volumes via macOS File Sharing (SMB). This is what users use when they do Cmd+K → `smb://net/CHILDES` in Finder to browse or contribute media files. NFS exports the same directories over a different protocol. Both can serve the same files simultaneously with no conflict. Users who prefer the SMB workflow keep using it unchanged.

**UX improvement**: NFS auto-mounts are better for nontechnical users — the media appears in Finder at `/Volumes/TalkBank/...` automatically, no "Connect to Server" needed. Users can still drag-and-drop files to contribute media (NFS is read-write).

**What to investigate**:

- [ ] **NFS export configuration on Net**: Test exporting `/Volumes/CHILDES`, `/Volumes/HomeBank`, and `/Volumes/Other` read-write to the Tailscale subnet (`100.64.0.0/10`). Check that macOS doesn't interfere with external volume NFS exports.
- [ ] **Automount on clients**: Use `/etc/auto_nfs` or a LaunchDaemon to auto-mount Net's volumes at boot. Needs to survive reboots and Net being temporarily unreachable.
- [ ] **Fallback when Net is down**: If NFS mount is stale, align/transcribe should fail gracefully with a clear error, not hang. Test NFS timeout behavior.
- [ ] **NFS performance over Tailscale**: Mount a volume on a client, run `batchalign-next align` on a few files. Audio files are 5-50 MB sequential reads — should be fine, but verify.

**Proposed mount layout on clients**:
```
/Volumes/TalkBank/CHILDES    → net:/Volumes/CHILDES/CHILDES
/Volumes/TalkBank/HomeBank   → net:/Volumes/HomeBank/homebank
/Volumes/TalkBank/Other      → net:/Volumes/Other
```

**server.yaml for clients** (same media_mappings, different paths):
```yaml
media_mappings:
  childes-data:  /Volumes/TalkBank/CHILDES
  homebank-data: /Volumes/TalkBank/HomeBank
  aphasia-data:  /Volumes/TalkBank/Other/aphasia
  # ... same set as Net, just different base path
```

Or, simpler — if we mount under the same paths as Net, we can use the **exact same server.yaml** everywhere. Worth investigating whether we can make the mount points identical.

### 2. Configuration Management — Ansible

**Why Ansible**: It's agentless (uses SSH), works great with macOS, handles the exact problems we have (deploying files, running commands, managing state across a fleet). No agent to install on managed machines.

**What to investigate**:

- [ ] **Ansible over Tailscale SSH**: Verify Ansible can reach all machines via their Tailscale hostnames. Create an inventory file and run `ansible all -m ping`.
- [ ] **Ansible on macOS**: Ansible runs on the controller (ming). Managed nodes just need SSH + Python — both already present on all our Macs.
- [ ] **Learning curve**: Ansible playbooks are YAML. Basic playbooks for our use case (copy files, run commands, manage launchd daemons) are straightforward.

**Proposed Ansible structure**:
```
ansible/
  inventory.yml          # All machines, grouped by role
  group_vars/
    all.yml              # Shared config (Tailscale IPs, versions, paths)
    servers.yml           # Server-specific vars
    clients.yml           # Client-specific vars (if any)
  roles/
    batchalign/           # Install batchalign-next + batchalign-core
    nfs-client/           # Mount Net's NFS exports
    nfs-server/           # Configure NFS exports (Net only)
    server/               # server.yaml + LaunchDaemon plist
    ssh-keys/             # Authorized keys management
  playbooks/
    deploy.yml            # Full deploy: build wheels, install, configure, restart
    configure.yml         # Just update config (server.yaml, plist)
    status.yml            # Check health across fleet
    mount-media.yml       # Set up NFS mounts on clients
```

**Example inventory** (`inventory.yml`):
```yaml
all:
  vars:
    ansible_user: macw  # default for most machines
  children:
    # Net: NFS server + media volumes + batchalign server
    media_server:
      hosts:
        net:
          nfs_server: true
    # All other machines: NFS clients + batchalign servers
    compute:
      hosts:
        bilbo:
        brian:
        davida:
        frodo:
        study:
        ming:
          ansible_user: chen  # override — no macw account on ming
    # Machines to onboard later (once Tailscale SSH is set up)
    future:
      hosts:
        andrew:
        cbs:
        extra:
        lilly:
```

All hosts in both `media_server` and `compute` groups are batchalign servers. The distinction is only for NFS: `media_server` exports volumes, `compute` mounts them.

**What Ansible replaces**:
- `deploy_batchalign3.sh` becomes a thin wrapper around `ansible-playbook`
- the older `deploy_clients.sh` and `deploy_server.sh` scripts go away entirely
- Manual SSH + scp for config → `ansible-playbook playbooks/configure.yml`
- Manual health checks → `ansible-playbook playbooks/status.yml`

**Ansible advantages over current scripts**:
- Parallel execution across hosts (default 5 at a time)
- Idempotent — running twice doesn't break anything
- Dry-run mode (`--check`)
- Inventory-driven — add a machine by adding one line
- Handles errors per-host without aborting the whole deploy
- Can manage NFS mounts, launchd plists, server.yaml all in one place

### 3. SSH Key Management

**Current state**: `~/.ssh/id_rsa_macw` on ming, manually added to `authorized_keys` on each machine. Keychain stores the passphrase. Fragile — if a machine is reimaged or a new machine is added, someone has to manually copy keys.

**Options to investigate**:

- [ ] **Tailscale SSH**: Tailscale has built-in SSH support (`tailscale ssh`). If enabled, it eliminates SSH keys entirely — authentication is via Tailscale identity. This is the ideal solution if it works on macOS.
  - Check: `tailscale up --ssh` on all machines
  - Check: Does Ansible work with `tailscale ssh` as the transport?
  - Check: Does it work for the `macw` user (not just the Tailscale-authenticated user)?

- [ ] **Ansible-managed authorized_keys**: If Tailscale SSH doesn't fit, use an Ansible role to push a canonical `authorized_keys` to all machines. One source of truth in the repo.

- [ ] **SSH certificates (advanced)**: Tailscale can issue SSH certificates via its coordination server. More complex but most robust.

**Recommendation**: Try Tailscale SSH first. It's the simplest and most secure — no keys to manage at all.

### 4. Turning All Machines into Servers (DECIDED)

**Decision**: All machines become full servers. The weakest machines have 64 GB RAM — enough for 2 concurrent workers.

Once media is mounted via NFS, making every machine a full server is straightforward:

1. **server.yaml**: Deploy the same config to all machines (media_mappings pointing to NFS mount paths)
2. **LaunchDaemon**: Deploy `org.talkbank.batchalign-server.plist` to `/Library/LaunchDaemons` on all machines
3. **Deploy script**: Install `batchalign-next[serve]` everywhere (not just Net)
4. **warmup**: Each machine warms up its own models on first start

**RAM-based tuning** (per-host in Ansible inventory):

| RAM | `max_concurrent_jobs` |
|-----|-----------------------|
| 64 GB | 2 |
| 128 GB | 4 |
| 256 GB (Net) | 8 |

**Multi-server fan-out**: The CLI already supports `--server http://bilbo:8000,http://frodo:8000,...` for distributing work. With all machines as servers, a user could fan out to the entire fleet.

**Heterogeneous users**: Most machines use `macw`, but ming uses `chen`. The Ansible inventory tracks this per-host. The LaunchDaemon should set `UserName`/`GroupName` per host so service runtime stays non-root and independent of login sessions.

### 5. Fleet Monitoring

**Simple first step**: An Ansible playbook that SSHes to each machine and reports:
- Is the server process running?
- Is `/health` responding?
- What version is installed?
- Are NFS mounts healthy?
- How much RAM/disk is free?

**Future**: A simple dashboard page (could extend the existing HTMX dashboard) that aggregates health from all servers.

---

## Proposed Action Plan

### Phase 1: NFS Media Sharing (1-2 hours)

1. Test NFS export from Net over Tailscale to one client (e.g., ming)
2. Verify batchalign align works over NFS mount
3. Measure performance (should be negligible overhead for audio reads)
4. Document the NFS export/mount configuration

### Phase 2: Ansible Bootstrap (2-3 hours)

1. Install Ansible on ming: `uv tool install ansible` or `brew install ansible`
2. Create inventory file with all machines
3. Verify connectivity: `ansible all -m ping`
4. Write a minimal deploy playbook that replaces the old shell deployment logic
5. Test on one machine, then roll out to all

### Phase 3: Tailscale SSH (1 hour)

1. Test `tailscale up --ssh` on one machine
2. Verify Ansible can use it as transport
3. If it works, enable on all machines and remove manual key management

### Phase 4: Full Fleet Servers (1-2 hours)

1. Write Ansible roles for NFS client mount + server.yaml + LaunchDaemon
2. Deploy to all clients
3. Verify each machine can serve align/transcribe requests
4. Replace deployment scripts with Ansible-first workflows

### Phase 5: Multi-Server Workflow (optional)

1. Test `--server bilbo:8000,brian:8000,...` fan-out from ming
2. Consider a simple "fleet dispatch" mode that auto-discovers healthy servers on the tailnet
3. Load-balance based on server health endpoint (workers_available)

---

## Open Questions

1. ~~**NFS vs. SMB**~~ — **DECIDED**: NFS read-write, coexisting with SMB.
2. ~~**Which machines get server role?**~~ — **DECIDED**: All of them. Weakest has 64 GB (enough for 2 workers).
3. ~~**Media write access**~~ — **DECIDED**: Read-write. Users contribute media via NFS/SMB.
4. ~~**LaunchDaemon vs. LaunchAgent**~~ — **DECIDED**: LaunchDaemon in `system` domain, with per-host `UserName` and `GroupName`.
5. ~~**macOS auto-login dependency**~~ — **DECIDED**: no longer required once LaunchDaemon is used.
6. ~~**Legacy cluster experiment infrastructure**~~ — **DECIDED**: removed.
7. **`davida` machine**: Not in the current Tailscale inventory. What's its Tailscale IP? Is it on the tailnet?
8. ~~**Machines not currently deployed**~~ — **DECIDED**: andrew, cbs, extra, lilly will be onboarded once Tailscale SSH is set up and we can inspect their specs.

---

## Legacy Cluster Cleanup (Completed)

An earlier distributed-cluster experiment was removed in favor of HTTP
multi-server fan-out. Cleanup actions:

- Removed cluster-specific setup docs that no longer apply.
- Updated server setup and troubleshooting guidance to match current runtime.
- Removed tooling references to abandoned cluster orchestration paths.

### Rust-Native Distributed Computing — Future Assessment

As we migrate more of batchalign to Rust (`batchalign_core`), it's worth investigating whether Rust-native distributed computing frameworks could eventually replace the Python HTTP server approach. This is a long-term research topic, not an immediate action item.

**What to investigate when the time comes**:

- [ ] **Tokio-based work distribution**: Rust's async runtime (Tokio) handles HTTP natively. Could a Rust server distribute work across a fleet without Python's GIL limitations?
- [ ] **gRPC / Tonic**: Rust gRPC framework. Could replace the FastAPI HTTP layer with lower-latency RPC if batchalign's server moves to Rust.
- [ ] **Nydus / Constellation / other Rust cluster frameworks**: Evaluate maturity and operational fit for future Rust-native distribution.
- [ ] **Hybrid approach**: Keep Python for ML model orchestration (Stanza, Whisper, PyAnnote) but use Rust for the dispatch/coordination layer. This matches our current architecture where Rust handles parsing/serialization and Python handles ML inference.

**Current assessment**: The HTTP fan-out architecture is sufficient for our fleet of ~10 Macs. Rust-native distribution would only matter at much larger scale or if we want to eliminate Python from the server entirely. Not a priority until the Rust migration is further along.

---

## Quick Wins (No Investigation Needed)

These can be done immediately:

1. **Audit machine inventory**: SSH to each machine, record RAM, macOS version, disk space, Tailscale status. Store in `ansible/inventory.yml`.
2. **Install Ansible on ming**: `brew install ansible` — zero risk, no changes to fleet.
3. **Test Tailscale SSH**: `tailscale up --ssh` on ming + one client — reversible, no impact.
4. **Version bumping**: Set up a version bump step in the deploy script (or use git describe) so daemon version-mismatch detection actually works. Currently the version file is static at `0.8.1-post.12`.
