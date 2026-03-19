# GitLab-to-GitHub Migration: Implementation Plan

**Status:** Phases 1+2 COMPLETE (2026-03-19). Phase 3 in progress.
**Last updated:** 2026-03-19

Ordered sequence of steps. Phases 1 and 2 were executed in a single window on 2026-03-19, going straight to GitHub (skipping the planned GitLab intermediate step).

Supporting docs:
- [gitlab-to-github-research.md](gitlab-to-github-research.md) — facts, decisions, open questions
- [dependency-map.md](dependency-map.md) — what breaks, what works, dead code, conventions
- [phase3-plan.md](phase3-plan.md) — decommission git-talkbank, pre-commit hooks, GitHub Actions
- [phase4-succession.md](phase4-succession.md) — succession planning

---

## Phases 1+2: Split Repos and Migrate to GitHub (COMPLETE)

**Completed:** 2026-03-19 in a single migration window.

**What was done:**
1. Split 4 large repos into 12 new repos on git-talkbank (verified: all file counts match)
2. Created fresh `git init` for all 12 unsplit repos (no history bloat)
3. Pushed all 24 repos to GitHub (`ca-candor-data` needed batched push due to 2 GB limit)
4. Cleaned config.py: removed 666 lines of dead code (Apache, SSL, *-site, mor, screencasts, GitBucket)
5. Rewrote tasks.py: `build_data()` now iterates `BANK_TO_DATA_REPOS` for multi-repo merges
6. Fixed rsync `--delete` bug (was deleting previous repos' files during merge)
7. Set up brian, davida, study machines with 24 fresh GitHub clones
8. Created `sync-staging.sh` (rsync replaces GitLab push/pull for staging)
9. GitLab purged from git-talkbank

**Original plan below preserved for reference** (steps were adapted during execution).

---

## Phase 1 (original plan): Split Repos on GitLab

**Goal:** Split 4 large repos into 12 new repos. Update all scripts so the deploy pipeline works with 24 repos instead of 16.

**Migration window:** Brian and Davida paused pushes for ~1 hour.

---

### Step 1.1: Audit and clean dead code in config.py

**Prerequisites:** None. Can be done immediately, independently of everything else.

**Do:**
- Remove all `*-site` entries from `GIT_REPOS` (16 entries: `aphasia-site`, `asd-site`, etc.)
- Remove all `*-site` entries from `HOSTS[].repos` lists
- Remove `SITE_SUFFIX` constant
- Remove `site_repo_name()` from tasks.py and its call at line 376
- Remove `ADMINISTRATORS_GITBUCKET_BASE`
- Update "GitBucket" comments to "GitLab"
- Remove stale `cdcs-to-csv` Bitbucket entry from `GIT_REPOS`
- Verify: `CapVid`, `check-chat-bookmarks`, `screencasts`, `mor` — are these still deployed? Keep or remove.

**Verify:** `deploy` still works for an unsplit repo (e.g., `tbi-data`) after the cleanup. Run on git-talkbank:
```bash
cd ~/staging && git pull
# Then trigger a test deploy of tbi-data
./scripts/deploy-1.py --repo tbi-data --force
```

**Rollback:** `git revert` the staging commit.

---

### Step 1.2: Add forward mapping and update config.py for splits

**Prerequisites:** Step 1.1 committed and verified.

**Do:**

Add `BANK_TO_DATA_REPOS` dict to config.py:
```python
BANK_TO_DATA_REPOS: Dict[str, List[str]] = {
    "aphasia": ["aphasia-data"],
    "asd": ["asd-data"],
    "biling": ["biling-data"],
    "ca": ["ca-candor-data", "ca-data"],
    "childes": ["childes-eng-na-data", "childes-eng-uk-data",
                "childes-romance-germanic-data", "childes-other-data"],
    "class": ["class-data"],
    "dementia": ["dementia-data"],
    "fluency": ["fluency-data"],
    "homebank": ["homebank-public-data", "homebank-cougar-data",
                 "homebank-bergelson-data", "homebank-password-data"],
    "motor": ["motor-data"],
    "phon": ["phon-eng-french-data", "phon-other-data"],
    "psychosis": ["psychosis-data"],
    "rhd": ["rhd-data"],
    "samtale": ["samtale-data"],
    "slabank": ["slabank-data"],
    "tbi": ["tbi-data"],
}
```

Add split repo entries to `GIT_REPOS` (with GitLab URLs for now). Keep old unsplit entries temporarily — they'll be removed after the split repos are created on GitLab.

Update `HOSTS[].repos` lists for the 4 split banks.

**Verify:** Python syntax check. Import config in a REPL, verify `BANK_TO_DATA_REPOS["childes"]` returns 4 repo names.

**Rollback:** `git revert`.

---

### Step 1.3: Update tasks.py for multi-repo builds

**Prerequisites:** Step 1.2 committed.

**Do:**

Replace `data_repo_name()` and `data_repo_dir()` with:

```python
def data_repo_names(bank: str) -> List[str]:
    """All data repo names for a bank."""
    return config.BANK_TO_DATA_REPOS[bank]

def data_repo_dirs(bank: str) -> List[str]:
    """All data repo paths for a bank."""
    return [repo_path(name) for name in data_repo_names(bank)]
```

Rewrite `build_data(name)` to iterate over all repos for the bank:

```python
def build_data(name: str) -> bool:
    local_build_dir = build_dir(name)
    if not os.path.exists(local_build_dir):
        os.mkdir(local_build_dir)

    data_orig_clone = os.path.join(local_build_dir, "data-orig")

    # Rsync EACH data repo into the shared data-orig/ directory
    for local_data_dir in data_repo_dirs(name):
        if not os.path.exists(local_data_dir):
            logging.warning("%s: repo dir %s does not exist, skipping", name, local_data_dir)
            continue
        rsync_args = (
            config.DATA_ORIG_RSYNC_ARGS
            + list(site_excludes(config.GIT_EXCLUDES))
            + [
                f"--link-dest={data_orig_clone}",
                f"{local_data_dir}/",
                f"{data_orig_clone}/",
            ]
        )
        rsync_did_work(rsync_args)

    # DOI duplicate check (scans all *-data repos automatically)
    dois.check_duplicates()

    # Update @Types in EACH data repo
    for local_data_dir in data_repo_dirs(name):
        if not os.path.exists(local_data_dir):
            continue
        logging.info("%s: updating @Types in %s...", name, local_data_dir)
        num_changed = update_chat_types.update_chat_types(local_data_dir, False)
        logging.info("%s: %s files had @Types changed.", name, num_changed)
        if num_changed > 0:
            do_git_commit_push(local_data_dir, "Automatically update @Types.")

    # Pull generate-from-chat
    sconstruct_repo_name = "generate-from-chat"
    if pull_got_new(sconstruct_repo_name):
        logging.debug("generate-from-chat: updated")

    scons_dir = repo_path(sconstruct_repo_name)
    site_dir = site_repo_dir(True, name)

    # Run generate_chat_data.py on the merged data-orig/
    scons_args = [
        "python3",
        f"{scons_dir}/generate_chat_data.py",
        f"--base={local_build_dir}",
        f"--access={site_dir}/access",
    ]
    logging.info("%s: running zipper...", name)
    scons_status = subprocess.run(scons_args)
    scons_succeeded = scons_status.returncode == 0

    if not scons_succeeded:
        logging.error("%s: zipper failed!", name)

    # Look up web host info
    host_name = config.site_to_host_site(name)
    config.HOSTS[host_name]

    return scons_succeeded
```

Also update `chatmedia.check_bank()` to iterate repos (or skip for now — low priority).

**Verify:** Deploy still works for unsplit repos. Run `deploy-1.py --repo tbi-data --force` and check output. The unsplit banks have 1-element lists in `BANK_TO_DATA_REPOS` so the loop runs once — same behavior as before.

**Rollback:** `git revert`.

---

### Step 1.4: Create split repos on GitLab

**Prerequisites:** Steps 1.1–1.3 committed and verified on git-talkbank. Brian and Davida paused.

**Do:**

For each of the 4 banks being split, on git-talkbank:

**childes-data → 4 repos:**
```bash
cd ~/staging/repos

# Create fresh repos from the split directories
for split in childes-eng-na-data childes-eng-uk-data childes-romance-germanic-data childes-other-data; do
    mkdir "$split"
    cd "$split"
    git init
    cd ..
done

# Copy language dirs into the right split repos
# childes-eng-na-data gets: Eng-NA, Eng-AAE
cp -a childes-data/Eng-NA childes-eng-na-data/
cp -a childes-data/Eng-AAE childes-eng-na-data/

# childes-eng-uk-data gets: Eng-UK, Clinical-Eng, Clinical-Other
cp -a childes-data/Eng-UK childes-eng-uk-data/
cp -a childes-data/Clinical-Eng childes-eng-uk-data/
cp -a childes-data/Clinical-Other childes-eng-uk-data/

# childes-romance-germanic-data gets: French, Romance, Spanish, German, DutchAfrikaans, Scandinavian, Celtic
cp -a childes-data/French childes-romance-germanic-data/
cp -a childes-data/Romance childes-romance-germanic-data/
cp -a childes-data/Spanish childes-romance-germanic-data/
cp -a childes-data/German childes-romance-germanic-data/
cp -a childes-data/DutchAfrikaans childes-romance-germanic-data/
cp -a childes-data/Scandinavian childes-romance-germanic-data/
cp -a childes-data/Celtic childes-romance-germanic-data/

# childes-other-data gets: everything else
# (Biling, Chinese, EastAsian, Japanese, Slavic, Finno-Ugric, Other, Frogs, MAIN, GlobalTales, XLing)
for dir in Biling Chinese EastAsian Japanese Slavic Finno-Ugric Other Frogs MAIN GlobalTales XLing; do
    cp -a "childes-data/$dir" childes-other-data/ 2>/dev/null
done

# Copy .gitignore into each split repo
for split in childes-eng-na-data childes-eng-uk-data childes-romance-germanic-data childes-other-data; do
    cp childes-data/.gitignore "$split/"
done
```

Repeat similarly for ca-data, phon-data, homebank-data.

For each new repo:
```bash
cd $split_repo
git add -A
git commit -m "Initial import from ${parent}-data split"
```

Create the repos on GitLab (via API or web UI) and push.

**Verify:**
```bash
# For each split bank, verify the merge matches the original
diff <(find childes-data -not -path '*/.git/*' -type f | sed 's|^childes-data/||' | sort) \
     <(for r in childes-eng-na-data childes-eng-uk-data childes-romance-germanic-data childes-other-data; do
         find "$r" -not -path '*/.git/*' -type f | sed "s|^${r}/||"
       done | sort)
# Should produce no output (identical file sets)
```

---

### Step 1.5: Wire split repos into deploy pipeline

**Prerequisites:** Step 1.4 repos created and pushed to GitLab.

**Do:**

On git-talkbank, update config.py:
- Remove old unsplit entries from `GIT_REPOS`: `childes-data`, `ca-data`, `phon-data`, `homebank-data`
- Verify split entries have correct GitLab URLs

Pull the staging changes on git-talkbank:
```bash
cd ~/staging && git pull
```

Clone the new split repos:
```bash
cd ~/staging/repos
for repo in childes-eng-na-data childes-eng-uk-data childes-romance-germanic-data childes-other-data \
            ca-candor-data phon-eng-french-data phon-other-data \
            homebank-public-data homebank-cougar-data homebank-bergelson-data homebank-password-data; do
    git clone ssh://git@git-talkbank:29418/TalkBank/${repo}.git
done
```

**Verify:** Test deploy for each split bank:
```bash
./scripts/deploy-1.py --repo childes-eng-na-data --force
./scripts/deploy-1.py --repo ca-candor-data --force
./scripts/deploy-1.py --repo phon-eng-french-data --force
./scripts/deploy-1.py --repo homebank-public-data --force
```

Check that `build/{bank}/data-orig/` has the correct merged contents:
```bash
ls ~/staging/build/childes/data-orig/
# Should show: Eng-NA, Eng-UK, French, German, Biling, Chinese, etc. (all language groups)
```

Check that `generate_chat_data.py` produced ZIPs:
```bash
ls ~/staging/build/childes/data/
```

---

### Step 1.6: Verify end-to-end and resume normal operations

**Prerequisites:** Step 1.5 verified.

**Do:**
- Verify John's TBB app still works (reads from `build/{bank}/data-orig/`)
- Verify web pages still load (existing HTML unchanged)
- Verify all 12 unsplit repos still deploy correctly
- Tell Brian and Davida they can resume

**Verify:**
```bash
# Deploy all 24 repos
for repo in $(python3 -c "import config; print(' '.join(r for r in config.GIT_REPOS if r.endswith('-data')))"); do
    echo "=== $repo ==="
    ./scripts/deploy-1.py --repo "$repo" --force
done
```

**Rollback:** If something is broken:
1. Restore old unsplit entries in `GIT_REPOS`
2. `git pull` on staging
3. Old deploy pipeline works again (old repos still on GitLab, read-only but functional)

---

### Step 1.7: Update meta-repo (~/talkbank/)

**Prerequisites:** Step 1.6 verified — deploy pipeline working with splits.

**Do:**
- Update `Makefile`: `DATA_REPOS` → 24 repos, `GITLAB_HOST` → correct URL for splits
- Update `scripts/adopt-repos.sh` for 24 repos
- Update `CLAUDE.md` workspace layout section for 24 repos
- Update `docs/inventory.md` data repo entries
- Split `known-issues/phon-data-validation-baseline.txt` into 2 files (`phon-eng-french-data-...` and `phon-other-data-...`) matching the new repo boundaries
- Update `known-issues/README.md` for split baselines
- Verify `deploy/scripts/fix_underline_markers.py` — its 11 hardcoded `ca-data/Jefferson/NB*` paths should stay in `ca-data` (not `ca-candor-data` since Jefferson is not CANDOR). Confirm and update if needed.
- Update local `data/` clones: clone new split repos, remove old unsplit ones
- Update `cdcs-to-csv/metadatas.py` with split repo entries (for when DOI minting resumes)

**Verify:** `make clone-data` works in a fresh workspace. `make status` shows all 24 data repos.

---

### Step 1.8: Update batchalign3 server.yaml and docs

**Prerequisites:** Step 1.6 verified.

**Do:**

Update `~/.batchalign3/server.yaml` on `net` (the batchalign server). Media is organized by bank (`/Users/macw/media/childes/`), but the CLI extracts the `*-data` repo name from the input path as the mapping key. So split repos need multiple keys pointing to the same media dir:

```yaml
media_mappings:
  # Split repos — all map to same bank-level media dir
  childes-eng-na-data: /Users/macw/media/childes
  childes-eng-uk-data: /Users/macw/media/childes
  childes-romance-germanic-data: /Users/macw/media/childes
  childes-other-data: /Users/macw/media/childes
  ca-candor-data: /Users/macw/media/ca
  ca-data: /Users/macw/media/ca
  phon-eng-french-data: /Users/macw/media/phon
  phon-other-data: /Users/macw/media/phon
  homebank-public-data: /Users/macw/media/homebank
  homebank-cougar-data: /Users/macw/media/homebank
  homebank-bergelson-data: /Users/macw/media/homebank
  homebank-password-data: /Users/macw/media/homebank
  # Unsplit repos — unchanged
  aphasia-data: /Users/macw/media/aphasia
  # ... etc
```

Also update:
- `book/src/developer/server-yaml-template.yaml` — update template with split repo examples
- Test fixtures in `crates/batchalign-cli/tests/command_matrix.rs:383` and `crates/batchalign-app/tests/json_compat.rs:29,150` — update test strings

**Verify:** Run `batchalign3 align` on a test file from a split repo path, confirm media resolution works.

---

### Step 1.9: Update talkbank-tools documentation

**Prerequisites:** Step 1.6 verified.

**Do:** Documentation-only updates (no code changes needed):
- `spec/errors/E502_auto.md`, `E600_auto.md` — update example source paths where they reference split repos
- `book/src/architecture/alignment.md:513` — update `data/ca-data/` example
- `src/bin/clear-cache-prefix.rs:16` — update example path
- `spec/tools/src/bin/perturb_corpus.rs:20` — update docstring example

The `../data/` convention is unchanged — the data directory still contains `*-data` repos. Commands like `chatter validate ../data/ --force` automatically discover all repos.

**Verify:** `make test` in talkbank-tools still passes (tests use embedded reference corpus, not external data).

---

### Step 1.10: Set up Brian and Davida's workspaces

**Prerequisites:** Steps 1.7–1.9 committed.

**Do:**
- Write a setup script that clones all 24 data repos into `~/0data/`
- Run it on Brian's and Davida's machines
- Brief email: "childes is now 4 repos, ca is 2, phon is 2, homebank is 4. Clone script sets you up. Push works the same — just make sure you're in the right repo for your language group."

**Verify:** Brian does a test push to one of the split repos.

---

## Phase 2: Switch Remotes to GitHub

**Goal:** Move all 24 data repos from GitLab to GitHub. The deploy pipeline continues to work, just reading from GitHub instead of GitLab.

**Migration window:** ~30 minutes. Brian and Davida pause pushes.

---

### Step 2.1: Create 24 repos on GitHub

**Prerequisites:** Phase 1 complete. All splits working on GitLab.

**Do:**
```bash
for repo in $(cat repo-list.txt); do
    gh repo create TalkBank/$repo --private --confirm
done
```

---

### Step 2.2: Push working trees to GitHub

**Prerequisites:** Step 2.1. Brian and Davida paused.

**Do:** For each of the 24 data repos on git-talkbank:
```bash
cd ~/staging/repos/$repo
git remote add github git@github.com:TalkBank/$repo.git
git push github main
```

**Verify:** Check on GitHub that each repo has the correct contents.

---

### Step 2.3: Switch remotes

**Do:**

On git-talkbank, update `config.py`:
- Change `GIT_SSH_SERVER` and `TALKBANK_GITBUCKET_BASE` to point to GitHub
- Or replace each `GIT_REPOS` entry URL individually

Switch each repo's origin:
```bash
cd ~/staging/repos/$repo
git remote set-url origin git@github.com:TalkBank/$repo.git
git pull  # verify
```

Update Makefile `GITLAB_HOST` → GitHub.

**Verify:**
```bash
# Deploy a test repo
./scripts/deploy-1.py --repo tbi-data --force

# Deploy a split repo
./scripts/deploy-1.py --repo childes-eng-na-data --force
```

---

### Step 2.4: Switch Brian and Davida's remotes

**Do:** Update the setup script to clone from GitHub. Run it on their machines (or have them run it).

```bash
cd ~/0data/$repo
git remote set-url origin git@github.com:TalkBank/$repo.git
```

**Verify:** Brian does a test push.

---

### Step 2.5: Mark old GitLab repos read-only

**Do:** On GitLab (port 8929, if still accessible) or via API, mark all repos as archived/read-only.

**Verify:** Attempted push to GitLab fails.

---

### Step 2.6: Tell Brian and Davida to resume

Normal operations resume. Everything works the same, just on GitHub.

---

## Phase 3: Decommission git-talkbank (future)

**Goal:** Move everything off git-talkbank VM and shut it down. This is a separate project.

**Prerequisites:** Phase 2 complete. John's app migration coordinated separately.

### Step 3.1: Set up data repo clones on talkbank.org

Clone all 24 data repos to talkbank.org (alongside existing web repos):
```bash
ssh macw@talkbank
mkdir -p /var/data
cd /var/data
for repo in $(cat repo-list.txt); do
    git clone git@github.com:TalkBank/$repo.git
done
```

### Step 3.2: Add GitHub Actions workflows to data repos

Same pattern as web repos. For each data repo, add `.github/workflows/deploy.yml`:
```yaml
name: Deploy {repo}
on:
  push:
    branches: [main]
jobs:
  deploy:
    runs-on: [self-hosted, talkbank]
    steps:
      - name: Deploy
        run: git -C /var/data/{repo} pull
```

### Step 3.3: Deploy John's Node app on talkbank.org

Coordinate with John. His app reads from `/var/data/` instead of `~/staging/build/`.

### Step 3.4: Rewrite HTML download URLs

Once John's app is live on talkbank.org, bulk find-and-replace across all `*-bank` repos:
- `git.talkbank.org` → `talkbank.org` (exact URL scheme TBD with John)

### Step 3.5: Rebuild DOI system

- Single `cdcfile.py`
- Modern DataCite REST API
- Credentials in secret store
- GitHub Actions for duplicate checking
- URL mapping updated for `talkbank.org` paths

### Step 3.6: Set up pre-commit hooks

- `@Types` update as pre-commit hook (replaces deploy-time update)
- Large file prevention
- DOI duplicate checking
- CHAT validation

### Step 3.7: Shut down git-talkbank

- Verify nothing still points to git-talkbank
- Back up any remaining data
- `sudo apt purge gitlab-ee` (or equivalent)
- Request VM decommission from CMU

---

## Risk Summary

| Risk | Likelihood | Impact | Mitigation |
|------|-----------|--------|------------|
| Split repo merge produces wrong file set | Low | High | Diff verification in Step 1.4 |
| Deploy breaks after config changes | Medium | Medium | Test with `--force` flag; rollback is `git revert` |
| Brian pushes during migration window | Low | Low | Direct communication; short window (~2 hours) |
| DOI minting needed during migration | Low | Medium | Tell Brian no new corpora during window |
| GitHub push fails (file too large) | Low | High | Pre-verified: no files > 100 MB after .gitignore |
| John's app breaks on build/ structure change | None (Phase 1-2) | — | John's app unchanged until Phase 3 |

---

## Checklist

### Phase 1 Prerequisites
- [ ] Verify all 16 data repos locally up to date (`make pull`)
- [ ] Verify split directory assignments (which dirs go in which repo)
- [ ] Verify .gitignore in all repos covers *.cex, *.mp4, *.wav, *.mp3
- [ ] Notify Brian and Davida of upcoming migration window
- [ ] Back up `cdcs-to-csv/output.csv` (DOI inventory)

### Phase 2 Prerequisites
- [ ] Phase 1 fully verified, running for at least a few days
- [ ] SSH keys for GitHub set up on git-talkbank (verify: `ssh -T git@github.com`)
- [ ] GitHub repos created under TalkBank org (private)
- [ ] Notify Brian and Davida of ~30-minute pause

### Phase 3 Prerequisites
- [ ] Phase 2 stable for at least 2 weeks
- [ ] John's app migration planned and scheduled
- [ ] DOI system replacement designed
- [ ] Pre-commit hooks tested
