# check-media: Proposed Commands for Brian

**Status:** Draft
**Last updated:** 2026-03-24 20:44 EDT

**Context:** Based on your answers to the first questionnaire, we've designed specific commands. We want to make sure these match how you'd actually use them before we build them.

**Key rules:**
- `check` is read-only. Defaults to checking all repos.
- `fix` must be run from inside a data repo — it operates on the repo you're standing in, like `git`. Fixes never auto-commit or auto-push — you commit when ready.

---

## The commands

### `check-media check` — what's wrong? (read-only)

Scans `.cha` files and tells you what's wrong. Doesn't change any files. Checks all repos by default.

```
check-media check                              # all repos (default)
check-media check --bank aphasia               # just one bank
check-media check ~/data/aphasia-data/         # explicit path
```

Example output:
```
aphasia:English/GR/newrecording.mp3  — media has no transcript
childes:Eng-NA/MacWhinney/010600a.cha  — @ID corpus is "MacWhinney", should be "Eng-NA"
homebank:Public/VanDam/session3.cha  — media exists but no bullets, needs "unlinked"

3 errors, 0 warnings (checked 4,230 files across 3 banks)
```

### `check-media fix` — fix everything in this repo

Run from inside a data repo (e.g., `cd ~/data/aphasia-data/`). Fixes all fixable problems in the current repo: creates stub `.cha` files, fixes corpus names, adds `unlinked`. Reports everything it did. You then commit and push.

```
cd ~/data/aphasia-data/
check-media fix                    # fix everything fixable
check-media fix --dry-run          # preview what would change
check-media fix --only stubs       # only create stub .cha files
check-media fix --only corpus      # only fix @ID corpus names
check-media fix --only unlinked    # only add "unlinked" to @Media
```

Example output:
```
Created stub transcripts (notrans):
  English/GR/newrecording.cha
  English/GR/session2.cha

Fixed @ID corpus name:
  English/Conte/patient3.cha  — "Conte" → "English"

Added "unlinked" to @Media:
  English/GR/oldfile.cha

Fixed 4 files. Review with `git status`, then commit and push.
```

---

## Important: the media list problem

`check-media` needs to know what media files exist on net (the Mac Studio with the drives). Today, it SSHes to net and runs `find` to get the list. But **net is not reliably reachable** — it's on the CMU LAN, and the VPN request to CMU Help hasn't been answered. Right now, someone has to manually sign into VPN on net for remote access to work.

This means:
- **If you're on net itself**, the tool can scan the media drives directly (fast, no SSH needed).
- **If you're on another machine**, the tool needs a *cached copy* of the media file list. But that cached copy can only be updated when net is reachable.

We have a few options for how to handle this — see Q4 below.

---

## Questions

### Q1. When you check, what machine are you typically on?

- **(A)** I'm on net itself — I just moved media files around, and I want to check right away.
- **(B)** I'm on my own Mac — I have data repos cloned there, and I check from there.
- **(C)** Both — sometimes I'm at net, sometimes on my Mac.
- **(D)** I'm on a different machine entirely (e.g., a laptop at home).

---

### Q2. When you run `check` and find problems, what do you do next?

- **(A)** I `cd` into the repo that has problems, run `fix`, review, commit, push. Then re-check.
- **(B)** I note the problems and come back to fix them later (maybe on a different machine).
- **(C)** I fix some repos right away and defer others — depends on what kind of problems they are.
- **(D)** I send the list to someone else (Davida, a student) to fix.

---

### Q3. Does this sequence match your workflow?

**Scenario: You just uploaded new media to net for aphasia.**

```
Step 1 (on net):     check-media refresh       # update the media file list
Step 2 (anywhere):   check-media check          # see what's wrong
                     → "3 media files have no transcript, 1 corpus name wrong"
Step 3 (at clone):   cd ~/data/aphasia-data/
                     check-media fix
                     → "Created 3 stubs, fixed 1 corpus name"
Step 4 (at clone):   git add . && git commit -m "Add stubs, fix corpus" && git push
```

- **(A)** Yes, this is exactly how I'd work.
- **(B)** I'd skip step 1 — I shouldn't have to remember to refresh.
- **(C)** I'd want `fix` to also show the re-check result — tell me what you fixed AND confirm it's clean.
- **(D)** I'd want something different: ________________

---

### Q4. Since net isn't always reachable from other machines, how should the media file list stay up to date?

**Background:** The tool needs a list of what media files exist on net. Getting this list requires access to net's drives. Options:

- **(A)** **I'll refresh it manually on net.** When I'm at net after moving media, I run `check-media refresh`. That updates the list. Later, when I `check` from my Mac, it uses the list I last generated on net. *(Requires copying the list file to your Mac, or a shared location.)*
- **(B)** **Keep the list on net, and I'll always run `check` on net too.** I don't need to check from other machines — I'll just do everything on net. *(Simplest, but means you can only check from net.)*
- **(C)** **Put the list in a git repo** (or a shared Dropbox/iCloud folder) so it syncs automatically. I refresh it on net, and it shows up on my Mac. *(The list is a few MB of JSON — small enough for any sync mechanism.)*
- **(D)** **Something else:** ________________

---

### Q5. When you `fix`, do you usually want to fix everything at once, or pick specific fix types?

- **(A)** Fix everything — stubs, corpus names, unlinked — all at once. That's the normal case.
- **(B)** I usually just want one type — e.g., only create stubs after uploading media.
- **(C)** It depends on the situation. Having `--only stubs` etc. is useful for when I want control.
- **(D)** I'd always want to preview first (`--dry-run`), then run the real fix.

---

### Q6. Where do your data repos live?

- **(A)** `~/data/*-data/` on my Mac
- **(B)** `~/staging/repos/*-data/` on git-talkbank (the old layout)
- **(C)** Somewhere else: ________________
- **(D)** Different places on different machines.

---

### Q7. Do you ever check or fix a subdirectory within a bank (e.g., just `Eng-NA/MacWhinney/`), or always the whole repo?

- **(A)** Always the whole repo.
- **(B)** Sometimes a subdirectory — when I know exactly what I changed.
- **(C)** Frequently a subdirectory — I work on one corpus at a time.
- **(D)** I didn't know I could do that, but I'd want to if it's faster.
