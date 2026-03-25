# check-media: Proposed Commands for Brian

**Status:** Draft
**Last updated:** 2026-03-24 20:44 EDT

**Context:** Based on your answers to the first questionnaire, we've designed specific commands. We want to make sure these match how you'd actually use them before we build them.

**Important constraint:** Checking and fixing are separate steps. You can *check* from any machine (it's read-only). You can only *fix* files on the machine where your data repo clone lives. Fixes never auto-commit or auto-push — you review and push when ready.

---

## The commands

Here are the 4 commands we're proposing. For each one, we have a question about how you'd use it.

### Command 1: `check-media check`

**What it does:** Scans your `.cha` files and tells you what's wrong. Doesn't change any files.

```
check-media check ~/data/aphasia-data/          # one bank
check-media check ~/data/*-data/                # all banks
check-media check ~/data/*-data/ --refresh      # refresh media list from net first
```

Example output:
```
aphasia:English/GR/newrecording.mp3  — media has no transcript
childes:Eng-NA/MacWhinney/010600a.cha  — @ID corpus is "MacWhinney", should be "Eng-NA"
homebank:Public/VanDam/session3.cha  — media exists but no bullets, needs "unlinked"

3 errors, 0 warnings (checked 4,230 files across 3 banks)
```

### Command 2: `check-media fix create-stubs`

**What it does:** For media files that have no `.cha` transcript, creates a minimal stub `.cha` file with `notrans` status. Reports every file it creates.

```
check-media fix create-stubs ~/data/aphasia-data/
check-media fix create-stubs ~/data/aphasia-data/ --dry-run    # preview first
```

Example output:
```
Created stub transcripts (notrans):
  aphasia-data/English/GR/newrecording.cha
  aphasia-data/English/GR/session2.cha

Created 2 files. Review with `git diff`, then commit and push.
```

### Command 3: `check-media fix add-unlinked`

**What it does:** For `.cha` files where media exists but there are no timing bullets, adds `unlinked` to the `@Media` header.

```
check-media fix add-unlinked ~/data/childes-eng-na-data/
```

### Command 4: `check-media fix fix-corpus`

**What it does:** Rewrites `@ID` corpus fields to match the directory structure.

```
check-media fix fix-corpus ~/data/aphasia-data/
```

---

## Questions

### Q1. When you check, what machine are you typically on?

- **(A)** I'm on net itself — I just moved media files around, and I want to check right away from that machine.
- **(B)** I'm on my own Mac — I have data repos cloned there, and I check from there (net is accessed via SSH for the media list only).
- **(C)** Both — sometimes I'm at net, sometimes on my Mac. It depends on what I was just doing.
- **(D)** I'm on a different machine entirely (e.g., a laptop at home).

---

### Q2. When you check and find problems, what do you do next?

- **(A)** I fix them right away on the same machine where I ran the check. Then I re-check to make sure it's clean.
- **(B)** I note the problems, then go to the machine where the data repo clone is and fix them there.
- **(C)** I fix some immediately and defer others — depends on the type of problem.
- **(D)** I send the list to someone else (Davida, a student) to fix.

---

### Q3. Does this sequence match your workflow? If not, what would you change?

**Scenario: You just uploaded new media to net for aphasia.**

```
Step 1 (check):    check-media check ~/data/aphasia-data/ --refresh
                   → "3 media files have no transcript"

Step 2 (fix):      check-media fix create-stubs ~/data/aphasia-data/
                   → "Created 3 stub .cha files"

Step 3 (verify):   check-media check ~/data/aphasia-data/
                   → "0 errors"

Step 4 (commit):   cd ~/data/aphasia-data && git add . && git push
```

- **(A)** Yes, this matches how I'd work. Four steps is fine.
- **(B)** I'd prefer steps 1-2 combined — check and fix in one command, but still a separate commit step.
- **(C)** I'd skip step 3 (re-check) — I trust the fix worked. Three steps: check, fix, push.
- **(D)** I'd want it even simpler — something like one command that checks, fixes, and tells me to push.

---

### Q4. The `--refresh` flag on `check` makes the tool SSH to net to get the current list of media files. This takes 10-30 seconds. How often do you want this to happen?

- **(A)** Every time I run `check`. I want the freshest data, even if it takes 30 seconds.
- **(B)** Only when I explicitly ask (`--refresh`). Most of the time, the cached list is fine.
- **(C)** Automatically if the cached list is older than a few hours. I shouldn't have to think about it.
- **(D)** I'd rather refresh the manifest separately, so I can control when the slow SSH step happens: `check-media refresh-manifest`, then run `check` many times quickly.

---

### Q5. Do you ever need to fix corpus names and add "unlinked" in the same session? Or are those usually separate situations?

- **(A)** They usually happen together — after a reorganization, I need to fix both.
- **(B)** They're usually separate — corpus name issues happen when I move folders, unlinked issues happen when new media arrives.
- **(C)** I almost never need `fix-corpus` — corpus names are usually right.
- **(D)** I almost never need `add-unlinked` — I usually add it manually when creating `.cha` files.

---

### Q6. Where do your data repos live?

*We need to know the path so the commands work correctly.*

- **(A)** `~/data/*-data/` (same as Franklin's workspace layout)
- **(B)** They're inside `~/staging/repos/*-data/` (the old layout on git-talkbank)
- **(C)** Somewhere else: ________________
- **(D)** I have them in different places on different machines.

---

### Q7. Do you ever run check-media for a subdirectory within a bank (e.g., just `Eng-NA/MacWhinney/`), or always the whole bank/repo?

- **(A)** Always the whole bank or all banks.
- **(B)** Sometimes a subdirectory — when I know exactly what I changed and want a quick targeted check.
- **(C)** Frequently a subdirectory — I work on one corpus at a time and only check that.
- **(D)** I didn't know I could do that. I'd want to if it's faster.
