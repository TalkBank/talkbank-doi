# check-media Workflow Questionnaire for Brian

**Status:** Answered
**Last updated:** 2026-03-24 14:30 EDT

**Context:** We're rebuilding the media validation tool (`check_media`) as part of the git.talkbank.org decommission. Before we finalize the design, we want to make sure it fits how you actually work — not how we assume you work. Please pick the option closest to what you'd prefer for each question. If none fit, write in your own.

---

### Q1. When you add new media files to net, how do you want to find out which ones still need CHAT transcripts?

*This is the "media without CHAT" check — the one you run most often today.*

- **(A)** I get an **email** (or Slack message) automatically within a few minutes of copying files, listing exactly which new media files have no `.cha` yet. I don't run any command.
- **(B)** I run a command when I'm ready to check — but it finishes in **under 10 seconds**, not minutes like today.
- **(C)** There's a **web page** (on talkbank.org) I can refresh anytime that shows the current status of every bank — which media files are missing transcripts, which are mismatched, etc.
- **(D)** The current approach is fine — I SSH to git-talkbank and run `check_media`. I just want it to work with the new GitHub repos.

---

### Q2. When you reorganize corpus folders on net (move, rename, delete media directories), how do you want to verify nothing broke?

- **(A)** Automatically — within minutes of moving files, I get a notification listing any newly broken CHAT↔media links. No command needed.
- **(B)** I run a command targeting just the bank I reorganized (e.g., `check-media check --bank childes`), and it completes quickly because it already knows what's on net.
- **(C)** I don't need immediate feedback — I'll find out at push time when the pre-push hook catches problems.
- **(D)** I'd like a **dry-run/preview mode** I can run *before* I move files, that shows me what would break if I make a planned change.

---

### Q3. How do you want to handle the "new media, no transcript" situation?

*Today, `check_media --newchat` can auto-create stub `.cha` files with `notrans` status for media that has no transcript.*

- **(A)** Automatically create the stub `.cha` files whenever new media appears — I'll fill in the transcripts later. Don't ask me.
- **(B)** Show me the list of media without transcripts, and let me choose which ones to create stubs for (a confirmation step).
- **(C)** Just show me the list. I'll create the `.cha` files myself when I'm ready.
- **(D)** I almost never need stub creation — I usually already have `.cha` files before adding media.

---

### Q4. When a CHAT file has `@Media` pointing to a file that doesn't exist, when should you be told?

- **(A)** **At push time** — the pre-push hook rejects the push and tells me exactly which files have broken `@Media` references. This is enough.
- **(B)** **While I'm editing** — my text editor (or a desktop notification) warns me as soon as I save a `.cha` file with a broken `@Media` reference.
- **(C)** **In a daily summary** — I get one email/message per day listing all broken references across all banks. I deal with them in batch.
- **(D)** **On demand** — I run a check command when I want to know, on the banks I care about at that moment.

---

### Q5. How often does the media on net actually change?

*This helps us decide how aggressive to be about keeping the media inventory up to date.*

- **(A)** Multiple times a day — I'm constantly uploading new recordings, reorganizing folders, or deleting old files.
- **(B)** A few times a week — usually in batches (a contributor sends files, I upload them all at once).
- **(C)** A few times a month — media is fairly stable, most of my daily work is editing CHAT files.
- **(D)** It varies wildly — some weeks nothing changes, other weeks I'm reorganizing entire banks.

---

### Q6. When you run `check_media` today, what do you do with the output?

- **(A)** I scan for new problems, fix them immediately, and re-run to confirm. I care about **zero errors** before I move on.
- **(B)** I look at the summary to get a sense of scale, then fix the most important issues. Some warnings I intentionally ignore.
- **(C)** I save the output to a file and work through it over several days.
- **(D)** I mainly run it to confirm everything is clean after a batch of changes — I expect zero or near-zero issues.

---

### Q7. Do you run `check_media` for all banks at once, or one bank at a time?

- **(A)** Almost always **all banks** (`--bank ALL`) — I want the full picture.
- **(B)** Usually **one bank at a time** — I know which bank I've been working on and just check that one.
- **(C)** It depends — all banks after a big reorganization, one bank after focused work.
- **(D)** I run it for a specific **subdirectory within a bank** (e.g., just `Eng-NA/MacWhinney/`), not the whole bank.

---

### Q8. The `--fixcorpus` and `--addunlinked` flags modify CHAT files and auto-commit+push. How do you want fixes to work going forward?

- **(A)** **Fully automatic** — detect the problem, fix the file, commit, push. I trust the tool to do the right thing.
- **(B)** **Show me first, then fix** — show what would change (like a diff), and I confirm before it writes. No auto-commit.
- **(C)** **Fix files but don't commit** — write the changes, but let me review and commit manually.
- **(D)** **Never auto-fix** — just report problems. I'll fix everything by hand.

---

### Q9. Where are you when you run media checks?

*This affects whether we can use macOS desktop notifications, web dashboards, email, etc.*

- **(A)** I'm always sitting at net (the Mac Studio) — physically at the keyboard or Screen Sharing.
- **(B)** I'm on my own Mac, SSHed into net or git-talkbank. I work remotely from net.
- **(C)** I switch between machines throughout the day — sometimes at net, sometimes on my laptop, sometimes at home.
- **(D)** I mostly work on my own machine and only go to net when I need to move media files.

---

### Q10. If we could set up a system where you never had to think about media↔CHAT consistency — it just notified you when something needed attention — would you trust it?

- **(A)** Yes — if it's reliable, I'd love to stop running checks manually. Just tell me when something is wrong.
- **(B)** Mostly — I'd trust automated notifications for routine issues, but I'd still want to run a manual full check before major milestones (releases, new corpora going live).
- **(C)** I'd want both — automatic notifications plus the ability to run checks on demand. The notifications catch things I might miss, the manual checks give me confidence.
- **(D)** I'd rather keep running checks myself. I like being in control of when and what gets checked.

---

### Anything else?

Is there something about how you use `check_media` today that frustrates you, or something you wish it could do that it can't? (Free-form — write as much or as little as you want.)

---

## Brian's Answers (2026-03-24)

| Q | Answer | Summary |
|---|--------|---------|
| 1 | **D** | Keep running a command. Just make it work with new repos. |
| 2 | **B** | Run a command per-bank, fast because manifest is already cached. |
| 3 | **A** (with caveat) | Auto-create stubs, but tell me what was created. |
| 4 | **A** | Pre-push hook is sufficient for broken `@Media`. |
| 5 | **B** | Media changes a few times a week in batches. |
| 6 | **A** | Zero errors: fix immediately, re-run to confirm clean. |
| 7 | **C** | All banks after big reorg, one bank after focused work. |
| 8 | **A** | Fully automatic fixes. Trust the tool. |
| 9 | **C** | Switches between machines throughout the day. |
| 10 | **C** | Both: automatic notifications + manual on-demand checks. |

---

## Analysis

### What Brian wants

Brian is a **command-line power user who wants speed and full automation**. He does NOT want:
- File watchers, dashboards, or email notifications (Q1=D)
- Manual fix workflows or confirmation dialogs (Q8=A)
- To wait minutes for SSH `find` scans (Q2=B)

He DOES want:
- A fast command he runs on demand (Q1=D, Q2=B)
- Automatic fixes with reporting (Q3=A, Q8=A)
- Zero-error workflow: run → auto-fix → re-run → clean (Q6=A)
- Pre-push hook as the safety gate for CHAT-side checks (Q4=A)
- Flexible scope: all banks or one bank (Q7=C)
- Both automatic notifications and manual checks available (Q10=C)

### Design implications

1. **Manifest must be fast to refresh.** Media changes weekly (Q5=B). A `--refresh-if-stale` flag that auto-refreshes when the manifest is older than N hours means Brian rarely waits. When manifest is fresh, checks are instant (local only).

2. **Auto-fix is the default, not opt-in.** `check-media check` should fix corpus names, add `unlinked`, and create stubs automatically — then report what it did. Today's `--fixcorpus`, `--addunlinked`, `--newchat` flags become the default behavior. Add `--check-only` for read-only mode (CI, pre-push).

3. **Pre-push hook uses `--check-only`.** No fixes at push time — just reject if errors remain. Brian's workflow: run `check-media` (with auto-fix), verify clean, then push.

4. **Stub creation reports what it created.** Q3=A with caveat: "tell me what is being created." After auto-creating stubs, print a summary like:
   ```
   Created 3 stub transcripts (notrans):
     aphasia/English/GR/newfile1.cha
     aphasia/English/GR/newfile2.cha
     aphasia/English/GR/newfile3.cha
   ```

5. **Q10=C means notifications are a future nice-to-have**, not a blocker. Brian wants manual checks to work well first. Automatic notifications can layer on top later (perhaps a cron on talkbank.org that runs check-media and emails Brian if new errors appear).

### Revised command design

```bash
# Brian's daily workflow (auto-fix + report):
check-media run ~/data/aphasia-data/          # one bank
check-media run ~/data/*-data/                # all banks
check-media run ~/data/*-data/ --refresh      # force manifest refresh first

# Pre-push hook (read-only, gate):
check-media check . --fail-on-error --quiet

# CI on talkbank.org (read-only, JSON):
check-media check /home/macw/data/*-data/ --format json

# Manifest management:
check-media refresh-manifest
check-media show-manifest --bank childes
```

The key change: `run` is the new primary command (check + auto-fix + report). `check` becomes the read-only mode for hooks and CI.
