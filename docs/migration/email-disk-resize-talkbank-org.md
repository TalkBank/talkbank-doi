# Email: talkbank.org disk resize for git.talkbank.org decommission

**Status:** Historical (resize completed 2026-03-23, 120 GB → 520 GB)
**Last updated:** 2026-03-23 14:35 EDT

---

**To:** Brian, John
**From:** Franklin
**Subject:** Need to resize talkbank.org disk before we can move John's app and kill git.talkbank.org

Brian, John —

We're ready to move John's data-browsing/ZIP app from git.talkbank.org to talkbank.org
so we can shut down the git.talkbank.org VM (GitLab is already gone). But we hit a
disk space problem.

**The numbers:**
- The 24 data repos total **72 GB**
- talkbank.org currently has **29 GB free** (117 GB disk, 84 GB used)
- git.talkbank.org has a **490 GB** disk — that's where the data fits today

**What we need:**
Brian — please request a disk resize for talkbank.org from CMU Campus Cloud. We need
at least **250 GB total** (ideally 300-500 GB to match what git.talkbank.org has and
leave room for growth). We've done this before for git.talkbank.org — the resize
instructions are in `docs/legacy/resize.md`. CMU provisions the extra space on their
side, then I run a few commands on the VM to expand the partition. No downtime.

**What happens after the resize:**

1. I clone the 24 data repos onto talkbank.org
2. I set up mergerfs (a standard Linux union filesystem) to present the split repos as
   the flat per-bank directory structure John's app expects — zero code changes needed
   on John's side
3. John deploys his Node app on talkbank.org (port 4000, proxied by nginx)
4. I update all the bank website HTML to change `git.talkbank.org` links to
   `talkbank.org` links
5. We set up a redirect on git.talkbank.org for any straggler bookmarks
6. After a few weeks of redirect with no issues, we delete the git.talkbank.org VM

**John** — once the disk is resized and I have the repos cloned, I'll set up the
mergerfs mounts so you see the same directory layout you have now
(`/{bank}/` with all corpora merged). Your app just needs a new data root path.
We can coordinate on the port and systemd service setup whenever you're ready.

The full cutover plan is in `docs/migration/git-talkbank-cutover-plan.md`.

Franklin
