# net Server Reference

**Status:** Current
**Last updated:** 2026-03-21

> **This doc covers net (the Mac Studio).** For talkbank.org (the public web server),
> see `docs/talkbank-org-server.md`. These are different machines.

## Hardware

- **Machine:** Mac Studio (Mac15,14)
- **Chip:** Apple M3 Ultra (28 cores)
- **RAM:** 256 GB unified memory
- **Storage:** Internal SSD + 3 external APFS volumes (~22 TB total, see [Media Volumes](#media-volumes) below)
- **OS:** macOS
- **Access:** `ssh macw@net` (CMU LAN or CMU General VPN only)

net is a **private Mac** in someone's office on the CMU campus LAN. It is NOT publicly
accessible — you need either campus LAN access or CMU General VPN.

## Role

- Local storage of media files (audio/video) on internal and external drives
- Favored place to run batchalign ML jobs (M3 Ultra Neural Engine + 256 GB RAM)
- macOS File Sharing (SMB) for Brian/Davida to drag files via Finder
- Source of media synced to talkbank-02 via `sync-media`

## Services

### batchalign-next (Python NLP pipeline)

Runs on port 8000. See `deploy/scripts/deploy_server.sh`.

### batchalign3 (Rust NLP pipeline)

Runs on port 8001 (coexistence). See `deploy/scripts/deploy_batchalign3.sh --server`.

### sync-media (media sync to talkbank-02)

Syncs `~/media/<bank>` to the media server (talkbank-02) via rclone/SFTP.

```bash
sync-media childes          # Sync a single bank
sync-media --all            # Sync all 18 banks
sync-media childes --dry-run  # Preview without transferring
```

Config: `~/.sync-media/config` (TOML). Binary: `~/.local/bin/sync-media` (Rust).
Deploy: `bash deploy/scripts/deploy_sync_media.sh` from the talkbank workspace.

Before each bank sync, the tool walks the source directory and fixes file permissions
to 644 and directory permissions to 755 (configurable). This replaces the old
`fix-permissions-on-gandalf.sh` + `sync-all.sh` workflow.

### macOS File Sharing (SMB)

Brian and Davida access net's drives via Finder over the campus LAN. This is how they
drag media files to/from their local machines for batchalign processing, and how Brian
stages new media received via WeTransfer.

## Media Volumes

TalkBank media (~12 TB of audio/video across 18 banks) does not fit on a single drive.
Three external APFS volumes are connected to net:

| Volume | Mount | Size | Used | Banks |
|--------|-------|------|------|-------|
| CHILDES | `/Volumes/CHILDES` | 7.3 TB | 3.6 TB | childes |
| HomeBank | `/Volumes/HomeBank` | 7.3 TB | 4.0 TB | homebank |
| Other | `/Volumes/Other` | 7.3 TB | 4.7 TB | all other 16 banks |

Brian's convention is that all banks appear as subdirectories of `~/media/` so tools
see a flat namespace. Since the actual data lives on different volumes, `~/media/<bank>`
entries are **symlinks** to the corresponding volume:

```
~/media/
  childes  -> /Volumes/CHILDES/CHILDES
  homebank -> /Volumes/HomeBank/homebank
  aphasia  -> /Volumes/Other/aphasia
  asd      -> /Volumes/Other/asd
  ...16 more -> /Volumes/Other/...
```

**Any tool that walks `~/media/<bank>` must follow symlinks.** The `sync-media` Rust
binary uses `walkdir` with `follow_links(true)` for this reason — without it, the root
symlink is reported as a file rather than a directory, and permission checks break.
The old `fix-permissions-on-gandalf.sh` used `find -L` (the `-L` flag follows symlinks).

If a volume is not mounted (e.g., after a reboot where the drive wasn't connected),
the symlink will be dangling and `sync-media` will skip that bank with "source not found."

## Incident History

- **2026-03-11:** Kernel panic from Python 3.14t memory exhaustion. See `docs/postmortems/2026-03-11-net-kernel-panic.md`.
