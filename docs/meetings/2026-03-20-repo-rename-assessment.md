# Assessment: Proposed Data Repo Renaming and Re-splitting

**Status:** Draft
**Last updated:** 2026-03-20 10:00

Brian,

Here's my assessment of your proposed changes to the data repo names and structure. I'm looking at this from both a technical cost perspective and a usability perspective, keeping in mind that someone who has never met us will eventually inherit all of this.

---

## Context

We completed the GitLab-to-GitHub migration on 2026-03-19. This involved splitting 4 large repos into 12 new repos (24 total), creating all 24 GitHub repos, setting up your machine, Davida's machine, and the study machines with fresh clones, and rewriting the entire deploy pipeline (config.py, tasks.py, workspace.toml, clone scripts). Every proposal below would require redoing a portion of that work.

---

## Proposal-by-Proposal Assessment

### 1. childes-eng-na-data -> childes-na-data (make Eng-AAE a subfolder of Eng-NA, then Eng-NA is no longer a subfolder)

**Moving Eng-AAE into Eng-NA as a subfolder:** This is fine. It's a directory restructure inside the existing repo. Easy, non-breaking, no rename needed.

**Dropping "eng" from the repo name:** I'd recommend against this. The "eng" prefix disambiguates language from region. "childes-na" could be read as "North American" or "not applicable" or just look incomplete. The current name "childes-eng-na-data" is immediately clear: CHILDES, English, North America.

### 2. childes-eng-uk-data -> childes-uk-data (remove Clinical-Eng and Clinical-Other)

Same concern about dropping "eng" from the name.

Extracting Clinical-Eng and Clinical-Other into their own repo is possible, but these are small (roughly 50 MB combined). A standalone `childes-clinical-data` repo for that amount of data adds operational overhead (another repo to clone, another entry in the deploy config, another thing for the successor to understand) without much benefit. They were grouped with UK because the sizes made sense together.

### 3. childes-other-data -> split into childes-af-data, childes-go-data, childes-mz-data

This is the most disruptive proposal. It turns 1 repo into 3, bringing the CHILDES repo count from 4 to 7 (or 8 with clinical).

My main concern is the naming. "af," "go," and "mz" are alphabetical range codes — they tell you nothing about what's inside unless you've memorized the folder list. A few years from now, or for anyone new, "childes-go-data" is opaque. What does "go" stand for? Compare that with the current name "childes-other-data" or even hypothetical names like "childes-romance-data" and "childes-germanic-data," which are self-describing.

If the goal is to break up childes-other-data because it's too large (1.5 GB), I'd suggest splits along linguistic lines with descriptive names rather than alphabetical ranges. For example:

- `childes-romance-data` (French, Romance, Spanish)
- `childes-germanic-data` (German, DutchAfrikaans, Scandinavian, Celtic)
- `childes-other-data` (Biling, Chinese, EastAsian, Japanese, Slavic, Finno-Ugric, Other, Frogs, MAIN, GlobalTales, XLing)

But I'd only do this if the current size is actually causing problems (slow clones, painful pushes).

### 4. phon-eng-french-data -> phon-engfra-data

This saves 3 characters. "engfra" isn't a standard language code (ISO 639 uses "eng" and "fra" separately). I don't think the savings justify the rename cost and the loss of readability.

### 5. Drop "-data" from all repo names

I'd recommend against this. The "-data" suffix is how we distinguish data repos from code repos in the same workspace. Without it:

- `aphasia` — is this a code tool? A website? A data corpus?
- `phon-engfra` — is this the Phon Java application or phonology data?
- `biling` — is this a tool for bilingualism research or corpus data?

Tab completion handles the extra 5 characters. The disambiguation value is worth more than the keystrokes saved, especially for someone new who is trying to understand what each repo is.

---

## Technical Cost of These Changes

If we proceed with all of the above, here is what would need to happen:

1. **GitHub:** Rename or delete and recreate roughly 8 repositories
2. **Re-split the CHILDES data:** The current 4-way split becomes a 5-to-8-way split with different folder groupings
3. **Re-clone on all machines:** Your machine, Davida's, study machines, and net all need fresh clones of every changed repo
4. **Rewrite the deploy pipeline (again):** config.py, workspace.toml, clone-all-data.sh, tasks.py, BANK_TO_DATA_REPOS mapping
5. **Update this workspace:** CLAUDE.md, Makefile, and 14+ documentation files reference the current names
6. **DOI workflows:** Any DOI metadata already referencing these repo names

This is roughly the same amount of work as the migration we just completed.

---

## Recommendation

1. **Eng-AAE as subfolder of Eng-NA:** Yes, let's do this. It's simple and non-breaking.
2. **Everything else:** Defer until Phase 3 (GitHub Actions, pre-commit hooks) is complete and the system has settled. If specific repos are causing operational pain — too large, wrong corpora grouped together, confusing for a particular workflow — let's address those specific problems then.
3. **If we do re-split later:** Use descriptive names (romance, germanic, clinical), not alphabetical ranges (af, go, mz).
4. **Keep "-data":** The disambiguation is worth the keystrokes.

The key question: what problem are these changes solving? If there's a concrete operational issue (a repo that's too large to push, corpora that are frequently updated but stuck in a huge repo), that's worth fixing now. If it's about aesthetics or shorter names, I'd suggest we let the current system stabilize first.

Franklin
