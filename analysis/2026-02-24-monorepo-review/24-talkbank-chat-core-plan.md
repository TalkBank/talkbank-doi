# 24. TalkBank Chat Core Plan (`talkbank-chat`)

## Strategic direction

Preserve and extend `talkbank-chat` as the canonical correctness backbone while improving downstream compatibility automation and reducing integration friction.

## Current strengths

- Strong generated-spec workflow and parser/model testing rigor.
- High-quality quality gates already present in CI.
- Clear architecture and guardrail documentation.

## Opportunities

1. Improve downstream compatibility enforcement:
   - Automatically verify `talkbank-chatter` and `batchalign3` against core changes.
2. Reduce duplicated crate drift risk via formal sync/version strategy.
3. Keep docs synchronized with evolving corpus and metric numbers.
4. Continue reducing non-essential `unwrap` usage in production paths.

## Recommended initiatives

- Cross-repo contract test suite
- Release compatibility matrix generation
- Doc-freshness automation for key metrics
- Incremental parser performance benchmark trend tracking

## Core plan checklist

- [ ] Add compatibility CI jobs for downstream repos
- [ ] Publish shared crate version/sync policy
- [ ] Add docs freshness checks for corpus/test metrics
- [ ] Maintain parser equivalence and roundtrip hard gates
- [ ] Add release notes section for downstream impact and migration hints
