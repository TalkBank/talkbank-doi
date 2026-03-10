# Stanza `<PAD>` Bug in Dependency Parsing

**Status:** Documented, awaiting response from Stanza developer (John Bauer).

## Summary

Stanza's dependency parser produces `<PAD>` as a `deprel` value instead of a real Universal Dependencies relation. This is a training artifact — `<PAD>` is a padding token from the neural network output layer leaking into predictions. It results in unparseable `%gra` lines like:

```
%gra:	1|0|ROOT 2|3|DET 3|1|<PAD> 4|1|PUNCT
```

Our tree-sitter parser rejects `<PAD>` as an invalid GRA relation, producing E316.

## Root Cause

The bug is triggered when **Stanza's depparse model receives input without sentence-final punctuation**. Both batchalign2 and batchalign3 strip the CHAT terminator (`.`, `?`, `!`, `+...`, etc.) before sending words to Stanza, so Stanza never sees a punctuation token.

```python
# Both batchalign2 and batchalign3 send words WITHOUT the terminator:
#   batchalign2: nlp("eta gehiago beltzarekin")          — raw text, no period
#   batchalign3: nlp([["eta", "gehiago", "beltzarekin"]]) — pretokenized, no period

# With punctuation → correct:
nlp("eta gehiago beltzarekin .")  # → deprel="nmod" ✓

# Without punctuation → bug:
nlp("eta gehiago beltzarekin")    # → deprel="<PAD>" ✗
```

This is **not a batchalign3 regression** — batchalign2 had the same behavior and produced the same `<PAD>` output in existing CHILDES files.

## Affected Models

Confirmed in Stanza 1.11.0, Basque model `bdt_nocharlm`. John Bauer confirms "several of the models have `<PAD>` in the output layer."

## Reproduction

```python
import stanza

# Pretokenized (batchalign3 path) — triggers <PAD>
nlp = stanza.Pipeline('eu', processors='tokenize,pos,lemma,depparse',
                       verbose=False, tokenize_pretokenized=True)
doc = nlp([['eta', 'gehiago', 'beltzarekin']])
print(doc.sentences[0].words[2].deprel)  # → "<PAD>"

# Raw text without terminator (batchalign2 path) — also triggers <PAD>
nlp2 = stanza.Pipeline('eu', processors='tokenize,pos,lemma,depparse',
                        verbose=False)
doc2 = nlp2('eta gehiago beltzarekin')
print(doc2.sentences[0].words[2].deprel)  # → "<PAD>"

# Raw text WITH terminator — correct
doc3 = nlp2('eta gehiago beltzarekin .')
print(doc3.sentences[0].words[2].deprel)  # → "nmod"
```

## Known Affected Files

Found by reprocessing the Basque/Soto/Maider corpus (2026-02-24):

| File | Utterance | Word getting `<PAD>` |
|------|-----------|---------------------|
| `Maider/021023.cha` line 1261 | `eta gehiago beltzarekin .` | `beltzarekin` (NOUN, Case=Com) |
| `Maider/030420.cha` | `baina patineteareki .` | `patineteareki` (NOUN, Case=Com) |

Pattern: both are Basque nouns with comitative case (`-Com`) as the last content word before the terminator.

## Proposed Fix

Append the terminator's underlying punctuation character to the word list before sending to Stanza. Map CHAT terminators to simple punctuation:

| CHAT terminator | Send to Stanza |
|----------------|---------------|
| `.` | `.` |
| `?` | `?` |
| `!` | `!` |
| `+...` `+..?` `+/.` `+//.` `+"/.` `+".` etc. | last punctuation char (`.` or `?`) |

Implementation: in `_stanza_batch_callback.py`, before building the text to send to Stanza, append the mapped punctuation to the word list (for pretokenized) or text string (for raw text mode). Then strip the Stanza-generated punctuation token from the response before passing to the Rust mapping layer (which adds its own terminator PUNCT relation).

**Not yet implemented** — waiting for feedback on whether to fix on our side, wait for Stanza fix, or both.

## Communication with Stanza Developer

John Bauer's response:

> It's 100% a bug. Several of the models have `<PAD>` in the output layer which makes no sense. If you would fill out a complete explanation of how to get this (such as what text was used and which annotation is showing `<PAD>`) I can fix it.

**Info to send John:**
- Language: `eu` (Basque), model `bdt_nocharlm`, Stanza 1.11.0
- `tokenize_pretokenized=True` with input `[["eta", "gehiago", "beltzarekin"]]` (no punctuation token)
- The `depparse` annotator returns `deprel="<PAD>"` for token 3 (`beltzarekin`)
- Adding `.` as a 4th token eliminates the bug
- The depparse model's output layer has `<PAD>` labels that leak through when input lacks sentence-final punctuation
