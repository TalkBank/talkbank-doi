# Data Repo Structure: Migration Guide for TBB

**Status:** Draft
**Last updated:** 2026-03-18

This doc describes how the data repos are being reorganized and where CHAT files will live on talkbank.org after migration. This is for John to plan the TBB filesystem mapping.

---

## Summary

The 16 data repos on git.talkbank.org are being split into 24 repos on GitHub. After migration, all 24 repos will be cloned flat into a single directory on talkbank.org (e.g., `/var/data/`). John's app reads from these clones directly.

---

## Current Structure (git-talkbank)

TBB currently reads from `~/staging/build/{bank}/data-orig/`:

```
~/staging/build/
├── aphasia/data-orig/English/GR/*.cha
├── ca/data-orig/CANDOR/*.cha
├── ca/data-orig/CallFriend/deu/*.cha
├── childes/data-orig/Eng-NA/MacWhinney/*.cha
├── childes/data-orig/French/Lyon/*.cha
├── homebank/data-orig/Public/VanDam-5minute/*.cha
├── homebank/data-orig/Password/Cougar/*.cha
├── phon/data-orig/Eng-NA/*.cha
├── phon/data-orig/French/Lyon/*.cha
└── ... (16 banks)
```

**Key:** `data-orig/` contains the raw repo contents (CHAT files, `0metadata.cdc`, etc.), one flat tree per bank.

---

## New Structure (talkbank.org)

All 24 repos cloned flat into a single directory:

```
/var/data/   (or wherever we decide)
│
│ # ── Unsplit banks (12 repos, identical internal structure to today) ──
├── aphasia-data/English/GR/*.cha
├── asd-data/...
├── biling-data/...
├── class-data/...
├── dementia-data/...
├── fluency-data/...
├── motor-data/...
├── psychosis-data/...
├── rhd-data/...
├── samtale-data/...
├── slabank-data/...
├── tbi-data/...
│
│ # ── CHILDES (was 1 repo, now 4) ──
├── childes-eng-na-data/
│   ├── Eng-NA/MacWhinney/*.cha
│   └── Eng-AAE/...
├── childes-eng-uk-data/
│   ├── Eng-UK/Thomas/*.cha
│   ├── Clinical-Eng/...
│   └── Clinical-Other/...
├── childes-romance-germanic-data/
│   ├── French/Lyon/*.cha
│   ├── German/Leo/*.cha
│   ├── Romance/...
│   ├── Spanish/...
│   ├── DutchAfrikaans/...
│   ├── Scandinavian/...
│   └── Celtic/...
├── childes-other-data/
│   ├── Biling/...
│   ├── Chinese/...
│   ├── EastAsian/...
│   ├── Japanese/...
│   ├── Slavic/...
│   ├── Finno-Ugric/...
│   ├── Other/...
│   ├── Frogs/...
│   ├── MAIN/...
│   ├── GlobalTales/...
│   └── XLing/...
│
│ # ── CA (was 1 repo, now 2) ──
├── ca-candor-data/
│   └── CANDOR/*.cha          # 1450 Zoom conversations, 4.8 GB
├── ca-data/
│   ├── CallFriend/...
│   ├── CallHome/...
│   ├── Jefferson/...
│   └── ... (40+ corpora)
│
│ # ── Phon (was 1 repo, now 2) ──
├── phon-eng-french-data/
│   ├── Eng-NA/...
│   └── French/Lyon/*.cha
├── phon-other-data/
│   ├── Chinese/...
│   ├── Clinical/...
│   ├── Spanish/...
│   └── ... (10+ language groups)
│
│ # ── HomeBank (was 1 repo, now 4) ──
├── homebank-public-data/
│   ├── Public/VanDam-5minute/*.cha
│   └── Secure/...
├── homebank-cougar-data/
│   └── Password/Cougar/...    # 5.4 GB
├── homebank-bergelson-data/
│   └── Password/Bergelson/... # 3.5 GB
└── homebank-password-data/
    └── Password/
        ├── DavisKean/...      # 1.6 GB
        ├── SanJoaquin/...     # 1.2 GB
        ├── FauseyTrio/...     # 947 MB
        ├── Lyon/...           # 362 MB
        └── ... (others)
```

---

## Path Mapping: Old → New

### Unsplit Banks (no change in file paths within repo)

| Bank | Old path | New path |
|------|----------|----------|
| aphasia | `build/aphasia/data-orig/English/GR/file.cha` | `aphasia-data/English/GR/file.cha` |
| asd | `build/asd/data-orig/.../file.cha` | `asd-data/.../file.cha` |
| (same pattern for all 12 unsplit banks) | | |

**Mapping rule:** Strip `build/{bank}/data-orig/` prefix → prepend `{bank}-data/`.

### Split Banks

| Bank | Old path | New repo | New path |
|------|----------|----------|----------|
| childes | `build/childes/data-orig/Eng-NA/MacWhinney/file.cha` | `childes-eng-na-data` | `childes-eng-na-data/Eng-NA/MacWhinney/file.cha` |
| childes | `build/childes/data-orig/French/Lyon/file.cha` | `childes-romance-germanic-data` | `childes-romance-germanic-data/French/Lyon/file.cha` |
| ca | `build/ca/data-orig/CANDOR/file.cha` | `ca-candor-data` | `ca-candor-data/CANDOR/file.cha` |
| ca | `build/ca/data-orig/Jefferson/NB/file.cha` | `ca-data` | `ca-data/Jefferson/NB/file.cha` |
| phon | `build/phon/data-orig/Eng-NA/file.cha` | `phon-eng-french-data` | `phon-eng-french-data/Eng-NA/file.cha` |
| phon | `build/phon/data-orig/Chinese/file.cha` | `phon-other-data` | `phon-other-data/Chinese/file.cha` |
| homebank | `build/homebank/data-orig/Public/file.cha` | `homebank-public-data` | `homebank-public-data/Public/file.cha` |
| homebank | `build/homebank/data-orig/Password/Cougar/file.cha` | `homebank-cougar-data` | `homebank-cougar-data/Password/Cougar/file.cha` |

**Mapping rule for split banks:** The top-level directory inside the repo tells you which split repo it belongs to. See the lookup tables below.

---

## Lookup Tables: Directory → Repo

### CHILDES

| Top-level directory | Repo |
|--------------------|------|
| Eng-NA | childes-eng-na-data |
| Eng-AAE | childes-eng-na-data |
| Eng-UK | childes-eng-uk-data |
| Clinical-Eng | childes-eng-uk-data |
| Clinical-Other | childes-eng-uk-data |
| French | childes-romance-germanic-data |
| Romance | childes-romance-germanic-data |
| Spanish | childes-romance-germanic-data |
| German | childes-romance-germanic-data |
| DutchAfrikaans | childes-romance-germanic-data |
| Scandinavian | childes-romance-germanic-data |
| Celtic | childes-romance-germanic-data |
| Biling | childes-other-data |
| Chinese | childes-other-data |
| EastAsian | childes-other-data |
| Japanese | childes-other-data |
| Slavic | childes-other-data |
| Finno-Ugric | childes-other-data |
| Other | childes-other-data |
| Frogs | childes-other-data |
| MAIN | childes-other-data |
| GlobalTales | childes-other-data |
| XLing | childes-other-data |

### CA

| Top-level directory | Repo |
|--------------------|------|
| CANDOR | ca-candor-data |
| (everything else) | ca-data |

### Phon

| Top-level directory | Repo |
|--------------------|------|
| Eng-NA | phon-eng-french-data |
| French | phon-eng-french-data |
| (everything else) | phon-other-data |

### HomeBank

| Top-level directory | Repo |
|--------------------|------|
| Public | homebank-public-data |
| Secure | homebank-public-data |
| Password/Cougar | homebank-cougar-data |
| Password/Bergelson | homebank-bergelson-data |
| Password/* (everything else) | homebank-password-data |

---

## What Stays the Same

- File paths WITHIN each repo are identical to today (same directory names, same `.cha` file names)
- `0metadata.cdc` files stay in the same relative location within each corpus directory
- Media files are NOT in data repos — they're on `net.talkbank.org` organized by bank name (unchanged)
- Bank-level domain names (childes.talkbank.org, etc.) are unchanged

## What Changes

- No more `build/{bank}/data-orig/` and `build/{bank}/data/` split — just the repo contents directly
- No more pre-built ZIP files in `data/` — TBB generates ZIPs dynamically
- 4 banks now span multiple repos (CHILDES, CA, Phon, HomeBank)
- Repos updated via GitHub Actions `git pull` (push to GitHub triggers automatic pull on talkbank.org)

---

## Questions for John

1. Does TBB need a config mapping from bank name to list of repo directories? (e.g., `"childes" → ["childes-eng-na-data", "childes-eng-uk-data", ...]`)
2. What base path should the repos be cloned to on talkbank.org? (`/var/data/`? Something else?)
3. Does the auth system for password-protected corpora need changes? (HomeBank password repos are now separate, which may simplify access control)
