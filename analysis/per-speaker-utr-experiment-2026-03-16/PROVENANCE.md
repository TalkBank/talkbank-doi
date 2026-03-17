# Experiment Provenance

**Status:** Current
**Last updated:** 2026-03-17

Checksums and source locations for all files used in the overlap alignment
experiments. Enables full reproduction on any machine with access to `net`.

## Tool Versions

| Component | Commit | Date |
|-----------|--------|------|
| batchalign3 | `f02702e4` | 2026-03-17 |
| talkbank-tools | `fe5bb89` | 2026-03-17 |
| talkbank-dev (this workspace) | `a0b94dc` | 2026-03-17 |
| utr-experiment | (in talkbank-dev) | 2026-03-17 |

## CHAT Source Files

All CHAT files are taken from corpus data repos at the commits shown below.
To reproduce, check out the exact commit and copy the file.

### aphasia-data (commit `e22599f8e`)

| File | Corpus path | MD5 |
|------|-------------|-----|
| 2265_T4.cha | `aphasia-data/English/Protocol/APROCSA/2265_T4.cha` | `0e65ce05198c6e476412a80ea5ee488a` |
| 2420_T3.cha | `aphasia-data/English/Protocol/APROCSA/2420_T3.cha` | `c26943be0e9f7eca96e9607c2cfc3cc7` |
| 2432_T2.cha | `aphasia-data/English/Protocol/APROCSA/2432_T2.cha` | `15bdbf5571feae5de2059901a6e034a2` |
| 2463_T2.cha | `aphasia-data/English/Protocol/APROCSA/2463_T2.cha` | `37c0ffa16f5f0daf42bca78fd3b0fc85` |

### childes-data (commit `5c748519e8`)

| File | Corpus path | MD5 |
|------|-------------|-----|
| 060211a2.cha | `childes-data/Eng-NA/MacWhinney/060211a2.cha` | `5c363448f24bbdb7f70468eb23e9f3e3` |
| 060211b1.cha | `childes-data/Eng-NA/MacWhinney/060211b1.cha` | `9076dd4484a4a437cb9b661c8758d7c6` |
| 060406b2.cha | `childes-data/Eng-NA/MacWhinney/060406b2.cha` | `74edb51024e8951f72e98fd940246b8b` |
| 020518b.cha | `childes-data/Eng-NA/Snow/020518b.cha` | `97d1b8c596f2fa799f4c39a74fd1fdec` |

## Audio Files

Audio files are stored on `macw@net` and fetched via `scp`. They are not
checked into git (too large). Verify integrity with the checksums below.

### APROCSA (fetched to `data/audio/`)

| File | Net path | Size | MD5 |
|------|----------|------|-----|
| 2265_T4.mp4 | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2265_T4.mp4` | 267 MB | `dd7eeb7da688bb32af3bc3fec0ed428a` |
| 2420_T3.mp4 | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2420_T3.mp4` | 691 MB | `a1981d3dbd9301b01809c82adc023785` |
| 2432_T2.mp4 | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2432_T2.mp4` | 764 MB | `1edb6c31a65b7cb57d6c28ac9ab9c7a5` |
| 2463_T2.mp4 | `/Volumes/Other/aphasia/English/Protocol/APROCSA/2463_T2.mp4` | 710 MB | `42c0223d4fb729558bd91e94dc258521` |

### CHILDES (fetched to `data/audio-childes/`)

| File | Net path | Size | MD5 |
|------|----------|------|-----|
| 060211a2.mp3 | `/Volumes/CHILDES/CHILDES/Eng-NA/MacWhinney/060211a2.mp3` | 25 MB | `02ff19d56b2402fa0d423315041a0a1e` |
| 060211b1.mp3 | `/Volumes/CHILDES/CHILDES/Eng-NA/MacWhinney/060211b1.mp3` | 26 MB | `52f3835b33d38cffd30320ce9e66422b` |
| 060406b2.mp3 | `/Volumes/CHILDES/CHILDES/Eng-NA/MacWhinney/060406b2.mp3` | 25 MB | `f67c54acf0ef3867c9762608d6e190cc` |
| 020518b.mp3 | `/Volumes/CHILDES/CHILDES/Eng-NA/Snow/020518b.mp3` | 74 MB | `cafc759af9e8ff698fee4ccc9f31ba3b` |

### CORAAL (fetched to `data/audio-coraal/`)

| File | Net path | Size |
|------|----------|------|
| m_02_1.mp3 | `/Volumes/Other/ca/CORAAL/PRV/se0-ag2/m_02_1.mp3` | 41 MB |
| f_02_1.mp3 | `/Volumes/Other/ca/CORAAL/PRV/se0-ag3/f_02_1.mp3` | 41 MB |
| f_02.mp3 | `/Volumes/Other/ca/CORAAL/DTA/se3-ag4/f_02.mp3` | 42 MB |

### Bilingual (fetched to `data/audio-biling/`)

| File | Net path | Size | Lang |
|------|----------|------|------|
| 28.mp3 | `/Volumes/Other/biling/MLE-MPF/28.mp3` | 41 MB | fra, eng |
| fusser12.mp3 | `/Volumes/Other/biling/Bangor/Siarad/fusser12.mp3` | 27 MB | cym, eng |
| 30.mp3 | `/Volumes/Other/biling/Bangor/Patagonia/30.mp3` | 22 MB | cym, eng, spa |

### International CHILDES (fetched to `data/audio-childes-intl/`)

| File | Net path | Size | Lang |
|------|----------|------|------|
| 030005.mp3 | `/Volumes/CHILDES/CHILDES/Slavic/Serbian/SCECL/AndjelaNj/030005.mp3` | 110 MB | srp |
| 050814.mp3 | `/Volumes/CHILDES/CHILDES/German/Rigol/Sebastian/050814.mp3` | 13 MB | deu |

### TBI (fetched to `data/audio-tbi/`)

| File | Net path | Size |
|------|----------|------|
| n22.mp3 | `/Volumes/Other/tbi/English/Coelho/N/n22.mp3` | 24 MB |
| tb23.mp3 | `/Volumes/Other/tbi/English/Coelho/TB/tb23.mp3` | 27 MB |

### Large files (deferred — run on net or fetch later)

| File | Net path | Size | Lang |
|------|----------|------|------|
| 010715.mp4 | `/Volumes/Other/phon/French/Paris/Antoine/010715.mp4` | 984 MB | fra |
| afrik.mp4 | `/Volumes/Other/aphasia/English/NonProtocol/Penn/afrik.mp4` | 1059 MB | eng |
| 11-14-17.mp4 | `/Volumes/Other/dementia/English/Lanzi/Treatment/11-14-17.mp4` | 511 MB | eng |

## Derived Files

These are produced by the experiment scripts and can be regenerated from
the source files above.

| Directory | Contents | How to regenerate |
|-----------|----------|-------------------|
| `data/input/` | APROCSA CHAT with timing stripped | `utr-experiment strip` from corpus source |
| `data/converted/` | APROCSA CHAT converted `&*` → `+<` | `utr-experiment convert` from `data/input/` |
| `data/converted-no-linker/` | Same but without `+<` linker | `utr-experiment convert --no-linker` |
| `data/groundtruth/` | CHILDES original CHAT (timing intact) | Copy from corpus data repo |
| `data/stripped-input/` | CHILDES CHAT with timing stripped | `utr-experiment strip` from groundtruth |
| `results/overlap-experiment/` | APROCSA alignment results (4 conditions) | `scripts/run-overlap-experiment.sh` |
| `results/groundtruth-experiment/` | CHILDES alignment results (2 conditions) | `scripts/run-groundtruth-experiment.sh` |

## Reproduction Steps

```bash
# 1. Clone workspace and repos
git clone ... talkbank-dev
cd talkbank-dev
make clone-minimal   # gets talkbank-tools + batchalign3

# 2. Check out exact commits
(cd talkbank-tools && git checkout fe5bb89)
(cd batchalign3 && git checkout f02702e4)

# 3. Build tools
(cd batchalign3 && cargo build --release -p batchalign-cli)
(cd analysis/per-speaker-utr-experiment-2026-03-16 && cargo build --release)

# 4. Fetch audio (requires net access)
cd analysis/per-speaker-utr-experiment-2026-03-16
bash scripts/fetch-audio.sh
# + manual scp for CHILDES audio (see Audio Files section)

# 5. Verify checksums
md5 data/audio/*.mp4 data/audio-childes/*.mp3
# Compare against checksums in this file

# 6. Run experiments
bash scripts/run-overlap-experiment.sh
bash scripts/run-groundtruth-experiment.sh
```
