# Validation and UX Audit

**Date:** 2026-02-18
**Scope:** All 171 talkbank-model error codes, their propagation through batchalign-next, and how they surface in CLI / server dashboard / server API.

---

## Error Code Inventory

### By Layer

| Layer | Count | Examples |
|-------|------:|---------|
| Parser | 48 | E101 (missing @UTF8), E142 (invalid bullet) |
| Validation | 123 | E301 (orphan tier), E342 (missing element) |

### By Severity

| Severity | Count | Meaning |
|----------|------:|---------|
| Error | 134 | Invalid CHAT — must fix |
| Warning | 37 | Suspicious but parseable |

---

## Propagation Chain

```
Rust parser/validator
  → VecErrorSink (collects all errors)
  → JSON serialization (error code, message, span)
  → PyO3 boundary: validate_structured() returns JSON
  → Python: CHATValidationException.errors field
  → CLI: failure summary block (error codes, line numbers, file counts)
  → Server API: per-file error detail in job status
  → Dashboard: error display in file table
```

### Strict vs Lenient

| Mode | Used By | Behavior |
|------|---------|----------|
| Strict (`parse_strict_pure`) | morphotag | Rejects on ANY error |
| Lenient (`parse_lenient_pure`) | align, translate | Error recovery, marks tainted tiers |

Morphotag uses strict because it reads its own output — if the input can't parse cleanly, something is wrong. Align uses lenient because it must handle legacy corpus files with known issues (e.g., malformed %wor tiers from old CLAN versions).

---

## CLI UX Assessment

### Failure Summary Block

After processing completes, all 4 dispatch paths (local, server, daemon, pipelined) print a structured summary:

```
╭─ Processing Summary ──────────────────────────────────────╮
│ Files processed: 126                                       │
│ Succeeded: 120                                             │
│ Failed: 6                                                  │
│                                                            │
│ Failures:                                                  │
│   E142 (invalid bullet): 3 files                           │
│     010523.cha:47  020100.cha:12  030200.cha:89            │
│   E301 (orphan tier): 2 files                              │
│     bad_file.cha:5  another.cha:22                         │
│   RuntimeError: 1 file                                     │
│     crash.cha (worker died)                                │
╰────────────────────────────────────────────────────────────╯
```

**Grade: A** — Clear, actionable, shows exactly what failed and where.

### Progress During Processing

| Dispatch Path | Granularity | Visual |
|---------------|-------------|--------|
| Local (ProcessPool) | File done/fail | Rich spinner per file |
| Local (ThreadPool) | File done/fail | Rich spinner per file |
| Server | File completion count | Single counter line |
| Dashboard | File status text | HTML table |

**Grade: B** — No per-utterance progress within a file (morphotag shows "Processing..." for 30s then "DONE"). See `developer/progress-reporting.md` for improvement plan.

### Error Detail Access

```bash
# See last run's errors:
batchalign-next logs --last

# Raw JSONL for scripting:
batchalign-next logs --raw

# Export for bug reports:
batchalign-next logs --export
```

**Grade: A** — Full structured access to error details.

---

## Server API UX Assessment

### Job Status Response

```json
{
  "job_id": "abc123",
  "status": "completed",
  "total_files": 126,
  "completed_files": 120,
  "error_files": 6,
  "file_statuses": {
    "010523.cha": {
      "status": "error",
      "error": "E142: Invalid timing bullet at line 47",
      "error_code": "E142"
    }
  }
}
```

**Grade: A** — Structured error codes and messages per file.

### Dashboard

The HTMX dashboard at `/dashboard/` shows a table of files with status. Error files show the error message inline. Clicking a file shows full error detail.

**Grade: B** — Functional but could show progress bars per file (see progress-reporting assessment).

---

## Error Code Coverage

### Well-Tested (parser layer)

All 48 parser-layer error codes have corresponding spec files in `spec/errors/`. Each spec file generates a Rust test via `make test-gen`. Coverage is comprehensive.

### Partially Tested (validation layer)

Of 123 validation-layer codes:
- **67** have spec files and generated tests
- **56** have `Status: not_implemented` (generates `#[ignore]` tests)

The not-implemented codes are tracked and will be enabled as validation rules are added.

### Error Codes in Practice

From a 99,000-file corpus validation run:

| Error Code | Count | Description |
|------------|------:|-------------|
| E342 | 902 | Missing required element (trailing bullet in %wor) |
| E301 | 247 | Orphan dependent tier (no parent utterance) |
| E142 | 89 | Invalid timing bullet format |
| E101 | 12 | Missing @UTF8 header |
| Other | 341 | Various validation issues |

The top error (E342) is entirely from legacy data — see `reference/wor-terminator-bullet-behavior.md`.

---

## Recommendations

### Already Done

1. **Structured error propagation** — Error codes flow from Rust through Python to CLI/server
2. **Failure summary block** — Clear per-file error reporting in all dispatch paths
3. **Run logging** — Structured JSONL with error codes and timing

### Should Do

1. **Dashboard progress bars** — Trivial, ~10 lines of template code
2. **Server-mode CLI progress detail** — Show per-file utterance count during polling
3. **Sub-batch Stanza progress** — Enable per-utterance progress for morphotag

### Lower Priority

4. **Rich per-file progress bar** — Blocked on sub-batch progress for morphotag
5. **ETA estimation** — Too noisy to be reliable
