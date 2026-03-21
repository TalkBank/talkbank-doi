# TalkBank Media Workflow

**Status:** Current
**Last updated:** 2026-03-20

## Overview

TalkBank hosts ~12 TB of audio/video media across 18 corpus banks. Media flows
through a two-machine pipeline: **net** (local staging with direct-attached drives)
and **talkbank-02** (public media server at `media.talkbank.org`). The goal is for
talkbank-02 to mirror net — any additions, deletions, and renames on net should be
reflected on talkbank-02.

## The machines

| Machine | Role | Storage | Access |
|---------|------|---------|--------|
| **net** (Mac Studio, M3 Ultra) | Staging + processing | 3 external APFS volumes (~22 TB), direct-attached | LAN, macOS File Sharing, SSH |
| **talkbank-02** (CMU Campus Cloud Plus VM) | Public media server (`media.talkbank.org`) | VM disk | SSH/SFTP (requires VPN or firewall rule), HTTP/HTTPS public |

talkbank-02 serves media to the world via HTTP/HTTPS. John's **TalkBank Browser** web
app connects to talkbank-02 to stream audio/video to external researchers interactively.
This is the primary reason talkbank-02 exists — it's the public-facing media CDN.
CMU's Campus Cloud Plus plan provides storage and bandwidth at significantly lower cost
than commercial cloud hosting (AWS/GCS/Azure) or trying to serve media directly off net
(which is on a private campus subnet, not suitable for public-facing traffic).

Media on net is organized as `~/media/<bank>/` where each bank is a symlink to the
appropriate volume (see `docs/net-talkbank-server.md` for the volume layout).

## How media gets to net

### 1. External contributors send files via WeTransfer

Contributors (researchers, labs) record audio/video and send files to Brian via
[WeTransfer](https://wetransfer.com/). Brian receives a download link, downloads the
files, and places them in the correct `~/media/<bank>/<corpus>/` path on net.

Brian handles all ingest — he decides which bank and corpus path each file belongs in.
There is no automated ingest pipeline.

> **Should WeTransfer be reassessed?** It works but has drawbacks:
>
> - **No automation.** Every transfer is a manual download-and-sort by Brian.
> - **Ephemeral links.** WeTransfer links expire after 7 days (free) or 30 days (Pro).
>   If Brian doesn't download in time, the files are gone and the contributor must resend.
> - **No deduplication.** If a contributor resends, Brian has to notice and avoid duplicates.
> - **No metadata.** The transfer has no structured information about which bank/corpus
>   the files belong to — Brian carries that context in his head.
> - **Not reproducible.** There's no log of what was received when, from whom, for which corpus.
>
> **Why Brian chose WeTransfer:** He tried several alternatives before settling on it.
>> Google Drive had problems with very large video files (likely the 15 GB free storage
> cap and/or unreliable uploads for multi-GB files). He also tried some form of Dropbox
> but rejected it (reason unknown). Other services failed with interrupted/incomplete
> transfers. The hard requirements for any replacement are:
>
> 1. **Handles huge files** — multi-GB video recordings, potentially 50-100+ GB batches
> 2. **Resumable uploads** — contributors are on university WiFi; connections drop
>    mid-transfer and the upload must survive that (this eliminated several alternatives)
> 3. **No account required** for contributors — link-based, zero-friction
> 4. **Persistent** — doesn't expire before Brian gets to it
> 5. **Browsable by Brian** — he needs to see what arrived in a GUI, not CLI
> 6. **Affordable** at TalkBank's scale
>
> **Open questions (ask Brian):**
> - Are you on WeTransfer free or Pro? (Free = 2 GB/transfer, 7-day expiry. Pro = 200 GB, 30 days.)
> - What's the typical size of a contributor upload? Single files or batches?
> - What specifically failed with Google Drive — storage limit, upload reliability, or something else?
> - What other services did you try? What failed about them?
> - Does CMU provide institutional Box, OneDrive, or a research data service?
>
> **Alternatives worth evaluating for succession** (needs real testing with large files,
> not just reading feature pages):
>
> - **Cloud storage bucket** (S3, GCS, or Backblaze B2) with a simple upload form that
>   collects bank/corpus metadata. Contributors upload directly; files land in a staging
>   area that Brian (or anyone) can review and approve into the right path. Would need
>   a web frontend that supports chunked/resumable uploads (e.g., tus.io protocol).
> - **Globus** — designed for research data transfer, common in academic settings,
>   handles large files well with automatic resume. But adds complexity and requires
>   Globus accounts (though many universities already have institutional access).
> - **CMU institutional services** — Box, OneDrive, or whatever CMU provides. May
>   already handle the storage/bandwidth cost. Needs investigation.
> - **SFTP drop box** — a shared directory on net or talkbank-02 where contributors
>   upload via SFTP. Simple but requires account management and is not zero-friction.
>
> The key requirement: whatever replaces WeTransfer must handle interrupted multi-GB
> uploads gracefully, work for non-technical contributors who just want to send files,
> and must not depend on Brian being the only person who can receive them.

### 2. Brian stages files on net

Brian organizes files into the correct directory structure on net:

```
~/media/<bank>/<corpus>/<file>.{mp3,mp4,wav,...}
```

He uses Finder on his local Mac connected to net via macOS File Sharing (SMB/AFP over
the local network). There is no validation step — correctness depends on Brian knowing
the corpus structure.

## How team members access media

### Local network: Finder + File Sharing

Nontechnical team members (Brian, Davida, others) use **macOS Finder** to access net's
drives via the local network. net runs macOS File Sharing (SMB), so the drives appear
as network volumes in Finder.

**Typical workflows:**

- **To process locally:** Drag files from net to their local machine via Finder, run
  `batchalign3 transcribe` or `batchalign3 align` locally, then drag results back to
  net via Finder.

- **To stage new media:** Drag files from their local machine (received via WeTransfer
  or other means) to the correct path on net via Finder.

This works because all team members are on the CMU campus LAN. A successor at a
different institution would need an equivalent local network setup or an alternative
file access method.

### SSH/SCP

Technical team members (Franklin, Chen) use SSH/SCP directly:

```bash
scp macw@net:~/media/childes/Eng-NA/MacWhinney/010411.mp4 .
```

## Why two machines?

The obvious question: why not just put everything on talkbank-02 and skip net?

### Reasons net exists

1. **Local drive speed.** net has direct-attached USB-C/Thunderbolt drives. Processing
   media locally (batchalign transcribe/align) reads files from local disk, which is
   substantially faster than reading over SFTP from talkbank-02, especially for batch
   operations that touch many files. The M3 Ultra's Neural Engine and 256 GB RAM make
   net the fastest machine for ML workloads.

2. **File Sharing for nontechnical users.** Brian and Davida use Finder to drag files
   around. This requires macOS File Sharing on the same LAN. talkbank-02 is a Linux VM
   behind a firewall requiring VPN + 2FA — it's not accessible via Finder, and asking
   nontechnical users to use VPN + SFTP is not realistic.

3. **No VPN pain.** Accessing talkbank-02 currently requires Cisco AnyConnect VPN with
   CMU credentials + Duo 2FA. A firewall rule is pending to eliminate VPN for
   server-to-server sync, but even then, casual file browsing by team members would
   still require VPN.

### Speed comparison

For a single file, the bottleneck is the ML model (Whisper, Stanza), not I/O. Local vs
network makes little difference for one-at-a-time transcription.

For batch processing (dozens of files), local NVMe/USB-C reads at ~1-3 GB/s vs SFTP
over campus LAN at ~100-500 MB/s. The difference matters when loading many audio files
into memory or when the pipeline reads files repeatedly (e.g., forced alignment passes).

**Net is the right place to process media.** The sync to talkbank-02 is a distribution
step, not a processing step.

## Net to talkbank-02: sync-media

The `sync-media` tool pushes changes from net to talkbank-02:

```bash
# Sync all banks (Brian's typical usage)
sync-media --all

# Sync one bank after staging new files
sync-media childes
```

This uses `rclone sync` over SFTP, which handles additions, deletions, and renames.
See `sync-media/README.md` for full documentation.

**Who runs it:** Brian runs it periodically after staging new media. Sometimes Davida
runs it. It is not automated (no cron/launchd) because it currently requires manual
VPN connection. Once the firewall rule is approved, it can be automated with a daily
launchd job.

## Known gaps and succession risks

| Gap | Risk | Mitigation |
|-----|------|------------|
| WeTransfer ingest is manual and undocumented | Brian is the only person who knows which files go where | Document corpus directory conventions; evaluate cloud ingest bucket |
| File Sharing requires campus LAN | Successor at different institution can't use Finder workflow | Provide alternative (web upload, SFTP with clear docs) |
| No ingest log | No record of what was received, when, from whom | Cloud ingest bucket would provide audit trail automatically |
| sync-media requires VPN (for now) | Can't automate until firewall rule is approved | Firewall rule in progress (see `docs/media-server-firewall-request.md`) |
| Brian carries corpus structure in his head | No one else knows the full directory convention | Document the convention for all 18 banks |

## Future: automated ingest (Phase 3 in sync-media roadmap)

Replace WeTransfer with a cloud ingest bucket + upload form:

1. Contributor fills in bank/corpus metadata and uploads files via a web form
2. Files land in a staging bucket with metadata
3. `sync-media ingest pull` downloads staged files to the correct path on net
4. `sync-media --all` syncs to talkbank-02

This eliminates Brian as a bottleneck, provides an audit trail, and works for a
successor who has never met the contributors.
