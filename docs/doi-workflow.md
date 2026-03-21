# DOI Workflow for TalkBank Corpora

**Status:** Current
**Last updated:** 2026-03-20 14:00

---

This guide is for anyone who creates or maintains corpus data repos — Brian, Davida, and any team member who edits `0metadata.cdc` files. No programming knowledge required.

## What is a DOI and why we use them

A DOI (Digital Object Identifier) is a permanent identifier for a dataset. When you cite a TalkBank corpus in a paper — or when someone else cites it — they use the DOI. It looks like `10.21415/T56W31` and resolves to the corpus page on talkbank.org via `https://doi.org/10.21415/T56W31`.

DOIs are hosted at [DataCite](https://datacite.org/), a nonprofit registry. Once minted, a DOI is permanent — it cannot be deleted, only marked inactive.

Each corpus directory gets one DOI, tied to its `0metadata.cdc` file.

---

## The `0metadata.cdc` file

Every corpus directory that has a DOI (or should get one) contains a file named `0metadata.cdc` at its root. For example:

```
childes-eng-na-data/Eng-NA/Brown/Adam/0metadata.cdc
aphasia-data/English/GR/0metadata.cdc
```

The format is simple key-value pairs, one per line. Each line is:

```
Key:	Value
```

**The separator between the key and value must be a real tab character**, not spaces. If your text editor inserts spaces instead of a tab, the tool will warn you.

Do not rename or move the file — the tool finds it by the name `0metadata.cdc`.

---

## Required fields

These three fields must be present for a DOI to be minted or updated:

| Field | Format | Example |
|-------|--------|---------|
| `Title` | Full corpus name | `Title:	Brown Corpus` |
| `Creator` | `Last, First` — one per line | `Creator:	Brown, Roger` |
| `Date` | Year (`2024`) or full date (`2024-03-15`) | `Date:	2004` |

If there are multiple creators, add one `Creator:` line for each:

```
Creator:	MacWhinney, Brian
Creator:	Snow, Catherine
```

---

## Useful optional fields

These fields are optional but strongly encouraged — they make corpora more discoverable in DataCite search, Google Dataset Search, and other academic indexes:

| Field | Example |
|-------|---------|
| `Language` | `Language:	eng` (ISO 639-3 code) |
| `Description` | `Description:	Longitudinal recordings of three children aged 1;6 to 5;0` |
| `Country` | `Country:	United States` |
| `Subject` | `Subject:	child language acquisition` |
| `Publisher` | `Publisher:	TalkBank` (defaults to TalkBank if omitted) |
| `DOI` | `DOI:	doi:10.21415/T56W31` (set automatically — do not edit manually) |

OLAC archival fields (used by linguistic data repositories):

| Field | Example |
|-------|---------|
| `Subject.olac:linguistic-field` | `Subject.olac:linguistic-field:	language_acquisition` |
| `Type.olac:discourse-type` | `Type.olac:discourse-type:	dialogue` |
| `Type.olac:linguistic-type` | `Type.olac:linguistic-type:	primary_text` |
| `Subject.olac:language` | `Subject.olac:language:	eng` |

---

## What happens when you push

When you run `git push` in a data repo, a pre-push hook runs automatically:

1. **Duplicate DOI check** — scans all sibling `*-data` repos for duplicate `DOI:` values. If two repos share the same DOI, the push is blocked with an error.
2. If the check passes, the push proceeds normally.

The hook does **not** validate the full CDC file format, and it does **not** mint new DOIs. Those happen separately.

**DOIs are not automatically minted on push.** Franklin runs `talkbank-doi sync` periodically to process new and updated CDC files. Once a DOI is minted, the tool writes it back into `0metadata.cdc` as:

```
DOI:	doi:10.21415/XXXX
```

That change is then committed and pushed to the repo.

---

## Checking DOI status

To see whether a corpus has a DOI assigned, open the `0metadata.cdc` file and look for a `DOI:` line:

```bash
grep DOI 0metadata.cdc
```

If the line is present with a value like `doi:10.21415/T56W31`, the DOI is assigned. If the line is absent or empty, no DOI has been minted yet.

You can verify that the DOI resolves correctly by visiting `https://doi.org/10.21415/XXXX` in a browser.

---

## How DOIs appear on the website

After `talkbank-doi sync` runs and the CDC file is updated, the corpus HTML pages on talkbank.org will include a clickable link:

```
https://doi.org/10.21415/T56W31
```

If the DOI field is present in the CDC file but the link is not yet on the web page, the HTML needs to be regenerated and deployed. Contact Franklin.

---

## Common mistakes and FAQ

**My push was rejected with a "Duplicate DOIs found" error.**
Two `0metadata.cdc` files across your data repos contain the same `DOI:` value. This usually means a corpus directory was copied and the old DOI wasn't removed from the copy. Find both files, determine which one should keep the DOI, and remove the `DOI:` line from the other.

**The `DOI:` field is missing but the corpus has been published for years.**
The DOI may have been minted but not written back to the CDC file, or the CDC file may not have existed when the DOI was originally created. Send Franklin the path to the CDC file and he will look up the DOI at DataCite and add it.

**I accidentally put spaces instead of a tab character.**
The parser accepts both tabs and spaces (it strips leading whitespace after the colon), but canonical format uses a tab. Use your editor's "insert tab" command or copy the separator from an existing working CDC file.

**Should I manually edit the `DOI:` field?**
No, unless you are absolutely certain of the value and it has been confirmed at DataCite. Incorrect DOI values cause sync errors and can corrupt the DataCite record.

**Can a corpus have more than one DOI?**
No. One corpus directory, one `0metadata.cdc` file, one DOI.
