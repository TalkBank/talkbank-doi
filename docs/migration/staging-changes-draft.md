# Staging Script Changes: Draft for Migration Window

**Status:** Draft
**Last updated:** 2026-03-18

All changes to apply to `~/talkbank/staging/` during the migration window (Steps 1.1–1.3). Review carefully, then push to staging and pull on git-talkbank.

---

## Change 1: config.py — Add BANK_TO_DATA_REPOS (DONE)

Already applied locally. Forward mapping from bank name to list of data repo names.

---

## Change 2: config.py — Add split repos to GIT_REPOS

Add these entries to the `GIT_REPOS` dict. During transition, keep the old unsplit entries too (they'll be removed once splits are verified).

```python
    # ── Split repos (childes) ──
    "childes-eng-na-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/childes-eng-na-data.git",
        root_dir="",
        git_clean=True,
    ),
    "childes-eng-uk-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/childes-eng-uk-data.git",
        root_dir="",
        git_clean=True,
    ),
    "childes-romance-germanic-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/childes-romance-germanic-data.git",
        root_dir="",
        git_clean=True,
    ),
    "childes-other-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/childes-other-data.git",
        root_dir="",
        git_clean=True,
    ),
    # ── Split repos (ca) ──
    "ca-candor-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/ca-candor-data.git",
        root_dir="",
        git_clean=True,
    ),
    # ca-data already exists (keeps the non-CANDOR corpora)
    # ── Split repos (phon) ──
    "phon-eng-french-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/phon-eng-french-data.git",
        root_dir="",
        git_clean=True,
    ),
    "phon-other-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/phon-other-data.git",
        root_dir="",
        git_clean=True,
    ),
    # ── Split repos (homebank) ──
    "homebank-public-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/homebank-public-data.git",
        root_dir="",
        git_clean=True,
    ),
    "homebank-cougar-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/homebank-cougar-data.git",
        root_dir="",
        git_clean=True,
    ),
    "homebank-bergelson-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/homebank-bergelson-data.git",
        root_dir="",
        git_clean=True,
    ),
    "homebank-password-data": RepoInfo(
        url=f"{TALKBANK_GITBUCKET_BASE}/homebank-password-data.git",
        root_dir="",
        git_clean=True,
    ),
```

---

## Change 3: config.py — Update HOSTS[].repos for split banks

Replace the repos lists for the 4 split banks:

```python
    "ca": HostInfo(
        ...
        repos=["ca-candor-data", "ca-data"],
    ),
    "childes": HostInfo(
        ...
        repos=["childes-eng-na-data", "childes-eng-uk-data",
               "childes-romance-germanic-data", "childes-other-data"],
    ),
    "homebank": HostInfo(
        ...
        repos=["homebank-public-data", "homebank-cougar-data",
               "homebank-bergelson-data", "homebank-password-data"],
    ),
    "phon": HostInfo(
        ...
        repos=["phon-eng-french-data", "phon-other-data"],
    ),
```

---

## Change 4: config.py — Remove dead *-site entries from GIT_REPOS

Remove all of these from `GIT_REPOS`:
- `psychosis-site`, `aphasia-site`, `asd-site`, `biling-site`, `ca-site`
- `childes-site`, `class-site`, `dementia-site`, `fluency-site`
- `homebank-site`, `motor-site`, `phon-site`, `rhd-site`
- `samtale-site`, `slabank-site`, `tbi-site`
- `talkbank-site`

Also remove from `HOSTS[].repos` lists.

Also remove:
- `cdcs-to-csv` entry (Bitbucket, stale)
- `ADMINISTRATORS_GITBUCKET_BASE` (dead)
- `SITE_SUFFIX` constant

---

## Change 5: config.py — Remove dead functions

Remove `site_to_host_site()` and `host_to_site_name()` if they are truly unused (verify by grep first).

---

## Change 6: tasks.py — Rewrite for multi-repo builds

Replace `data_repo_name()` and `data_repo_dir()` with:

```python
def data_repo_names(bank: str) -> List[str]:
    """All data repo names for a bank."""
    return config.BANK_TO_DATA_REPOS[bank]


def data_repo_dirs(bank: str) -> List[str]:
    """All data repo paths for a bank."""
    return [repo_path(name) for name in data_repo_names(bank)]
```

Rewrite `build_data(name)`:

```python
def build_data(name: str) -> bool:
    """Build data for a bank. Handles multiple data repos per bank (splits)."""
    logging.info("%s: building data...", name)

    data_orig = "data-orig"

    local_build_dir: str = build_dir(name)
    if not os.path.exists(local_build_dir):
        logging.debug("Making directory %s...", local_build_dir)
        os.mkdir(local_build_dir)

    data_orig_clone = os.path.join(local_build_dir, data_orig)

    # Rsync EACH data repo into the shared data-orig/ directory.
    # Split repos have disjoint top-level dirs, so they merge cleanly.
    for local_data_dir in data_repo_dirs(name):
        if not os.path.exists(local_data_dir):
            logging.warning("%s: data repo dir %s not found, skipping",
                            name, local_data_dir)
            continue
        logging.debug("Syncing %s to %s...", local_data_dir, data_orig_clone)
        rsync_args: List[str] = (
            config.DATA_ORIG_RSYNC_ARGS
            + list(site_excludes(config.GIT_EXCLUDES))
            + [
                f"--link-dest={data_orig_clone}",
                f"{local_data_dir}/",
                f"{data_orig_clone}/",
            ]
        )
        rsync_did_work(rsync_args)

    # Check for duplicate DOIs across all data repos.
    dois.check_duplicates()

    # Update @Types in EACH data repo individually.
    for local_data_dir in data_repo_dirs(name):
        if not os.path.exists(local_data_dir):
            continue
        logging.info("%s: updating @Types in %s...", name, local_data_dir)
        num_chat_types_changed = update_chat_types.update_chat_types(
            local_data_dir, False
        )
        logging.info("%s: %s files had @Types changed.", name, num_chat_types_changed)
        if num_chat_types_changed > 0:
            do_git_commit_push(
                local_data_dir, "Automatically update @Types."
            )

    # Pull latest generate-from-chat.
    sconstruct_repo_name = "generate-from-chat"
    if pull_got_new(sconstruct_repo_name):
        logging.debug("generate-from-chat: updated")

    scons_dir = repo_path(sconstruct_repo_name)

    # Web repo path (still 1:1 per bank).
    site_dir = site_repo_dir(True, name)

    # Run generate_chat_data.py on the merged data-orig/.
    scons_args = [
        "python3",
        f"{scons_dir}/generate_chat_data.py",
        f"--base={local_build_dir}",
        f"--access={site_dir}/access",
    ]

    logging.debug(scons_args)
    logging.info("%s: running zipper...", name)
    scons_status = subprocess.run(scons_args)
    scons_succeeded: bool = scons_status.returncode == 0

    if not scons_succeeded:
        logging.error(
            "%s: zipper failed: cannot deploy site until fixed!", name
        )

    host_name = config.site_to_host_site(name)
    config.HOSTS[host_name]

    return scons_succeeded
```

Remove `site_repo_name()` and its call (dead code referencing `*-site` repos).

Remove old `data_repo_name()` and `data_repo_dir()` (replaced by `data_repo_names()` and `data_repo_dirs()`).

---

## Change 7: tasks.py — Update site_repo_dir()

Currently references `/mnt/homebank_web/banks/{site_name}-bank/site`. Verify this path is correct on git-talkbank:

```bash
ssh macw@git-talkbank 'ls /mnt/homebank_web/banks/'
```

If the mount doesn't exist, this function may already be broken. Check.

---

## Verification Plan

After applying all changes, on git-talkbank:

```bash
# 1. Pull staging changes
cd ~/staging && git pull

# 2. Syntax check
python3 -c "import config; import tasks"

# 3. Verify config
python3 -c "
import config
# Check forward mapping
assert len(config.BANK_TO_DATA_REPOS['childes']) == 4
assert len(config.BANK_TO_DATA_REPOS['aphasia']) == 1
assert len(config.BANK_TO_DATA_REPOS) == 16  # 16 banks

# Check all split repos in GIT_REPOS
for repos in config.BANK_TO_DATA_REPOS.values():
    for repo in repos:
        assert repo in config.GIT_REPOS, f'{repo} not in GIT_REPOS'

print('All checks passed')
"

# 4. Test deploy of unsplit repo (should work identically)
./scripts/deploy-1.py --repo tbi-data --force

# 5. Test deploy of split repo (after repos are created on GitLab)
./scripts/deploy-1.py --repo childes-eng-na-data --force
```
