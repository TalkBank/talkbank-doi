# Data Repo Structure: Migration Guide for John's App

**Status:** Current
**Last updated:** 2026-03-21

This doc describes how the data repos are being reorganized and how John's app will
access them on talkbank.org after migration from git.talkbank.org.

---

## Summary

The 16 data repos on git.talkbank.org have been split into 24 repos on GitHub (4 banks
were split: CHILDES, CA, Phon, HomeBank). All 24 repos will be cloned into
`/home/macw/data/` on talkbank.org. A **mergerfs** union filesystem presents the split
repos as a flat per-bank view at `/var/data/view/{bank}/`, preserving the same
directory layout John's app sees today.

**John's app changes: update the data root path. No code changes.**

His app reads from `/var/data/view/{bank}/` (read-only) instead of
`~/staging/build/{bank}/data-orig/`. The directory structure within each bank is
identical to today.

---

## How It Works

### Repos on disk

All 24 repos cloned flat into `/home/macw/data/`:

```
/home/macw/data/
├── aphasia-data/English/GR/*.cha
├── asd-data/...
├── childes-eng-na-data/Eng-NA/MacWhinney/*.cha
├── childes-eng-uk-data/Eng-UK/Thomas/*.cha
├── childes-romance-germanic-data/French/Lyon/*.cha
├── childes-other-data/Biling/*.cha
├── ca-candor-data/CANDOR/*.cha
├── ca-data/Jefferson/*.cha
├── phon-eng-french-data/Eng-NA/*.cha
├── phon-other-data/Chinese/*.cha
├── homebank-public-data/Public/VanDam-5minute/*.cha
├── homebank-cougar-data/Password/Cougar/*.cha
├── homebank-bergelson-data/Password/Bergelson/*.cha
├── homebank-password-data/Password/DavisKean/*.cha
└── ... (24 repos total)
```

### mergerfs virtual view

mergerfs mounts present the split repos as one flat directory per bank:

```
/var/data/view/
├── aphasia/      ← mergerfs mount of aphasia-data (1 repo)
├── childes/      ← mergerfs mount of 4 childes repos merged
├── ca/           ← mergerfs mount of 2 ca repos merged
├── phon/         ← mergerfs mount of 2 phon repos merged
├── homebank/     ← mergerfs mount of 4 homebank repos merged
├── asd/          ← mergerfs mount of asd-data (1 repo)
└── ... (18 banks)
```

What John's app sees at `/var/data/view/childes/`:
```
Eng-NA/MacWhinney/foo.cha      <-- from childes-eng-na-data
French/Lyon/bar.cha             <-- from childes-romance-germanic-data
Chinese/Zhou/baz.cha            <-- from childes-other-data
```

This is the same flat layout as the current `~/staging/build/childes/data-orig/`.

### How updates work

1. Push to a data repo on GitHub triggers a GitHub Actions self-hosted runner on
   talkbank.org
2. Runner does `git pull` in `/home/macw/data/{repo}/`
3. mergerfs reads through to the source directories on every access — changes are
   immediately visible, no rebuild step needed

### Filtering

`.git` and `.gitignore` are visible in the merged view (the old rsync approach excluded
them). John's app should filter these from directory listings.

---

## What John's App Needs

| Setting | Value |
|---------|-------|
| Data root | `/var/data/view/` |
| Bank path pattern | `/var/data/view/{bank}/` |
| Port | 4000 |
| Access | Read-only |
| Auth backend | sla2.talkbank.org (unchanged) |

The app serves:
- `/{bank}/data/{path}.zip` — on-the-fly ZIP downloads
- `/{bank}/data-orig/{path}` — raw CHAT file browsing/download

nginx on talkbank.org proxies these routes to the app on port 4000.

---

## Bank-to-Repo Mapping (for reference)

John's app doesn't need this mapping (mergerfs handles it), but it's documented here
for operational reference.

| Bank | Repos |
|------|-------|
| aphasia | aphasia-data |
| asd | asd-data |
| biling | biling-data |
| ca | ca-candor-data, ca-data |
| childes | childes-eng-na-data, childes-eng-uk-data, childes-romance-germanic-data, childes-other-data |
| class | class-data |
| dementia | dementia-data |
| fluency | fluency-data |
| homebank | homebank-public-data, homebank-cougar-data, homebank-bergelson-data, homebank-password-data |
| motor | motor-data |
| open | open-data |
| phon | phon-eng-french-data, phon-other-data |
| psyling | psyling-data |
| psychosis | psychosis-data |
| rhd | rhd-data |
| samtale | samtale-data |
| slabank | slabank-data |
| tbi | tbi-data |

---

## What Stays the Same

- File paths within each bank are identical to today
- `0metadata.cdc` files stay in the same relative locations
- Media files are NOT in data repos — they're on net, synced to talkbank-02 separately
- Auth for password-protected corpora works the same way (sla2.talkbank.org)

## What Changes

- Data root path: `/var/data/view/{bank}/` instead of `~/staging/build/{bank}/data-orig/`
- No more pre-built ZIP files — app generates ZIPs on the fly (already implemented)
- `.git` directories visible in listings (filter them)
- Repos updated via GitHub Actions `git pull` instead of the old staging deploy script

---

## Prerequisites

Before John can deploy:

1. **Disk resize** — talkbank.org needs at least 250 GB total disk (currently 117 GB,
   repos are 72 GB). Brian needs to request this from CMU Campus Cloud.
2. **Repos cloned** — Franklin clones all 24 repos to `/home/macw/data/`
3. **mergerfs mounted** — Franklin sets up the union mounts at `/var/data/view/`
4. **nginx proxy** — Franklin adds the `/{bank}/data/` and `/{bank}/data-orig/` routes

Full cutover plan: `docs/migration/git-talkbank-cutover-plan.md`
