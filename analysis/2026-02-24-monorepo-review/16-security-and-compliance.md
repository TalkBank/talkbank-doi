# 16. Security and Compliance

## Findings

- No immediate critical security incident surfaced from static review, but policy coverage is uneven.
- Distributed processing and server endpoints increase attack surface (input payloads, file paths, job orchestration).
- Metadata inconsistencies can cause compliance and trust issues (licenses/repository references).

## Recommendations

1. Add threat models for:
   - server job submission and file handling
   - fleet dispatch trust boundaries
   - extension command execution surfaces
2. Add secure defaults:
   - strict request validation
   - explicit path allowlists
   - resource limits and abuse controls
3. Add dependency vulnerability scanning and patch SLAs.
4. Normalize and verify license metadata in all package manifests.
5. Add secure release process with provenance metadata.

## Tools to leverage

- SAST in CI (language-specific linters + secret scanning)
- Dependency audits (`cargo audit`, `npm audit`, Python audit)
- Optional policy-as-code for release checks

## Security checklist

- [ ] Create threat model docs for server/fleet/extension surfaces
- [ ] Add automated secret scanning and dependency vulnerability gates
- [ ] Add path traversal and payload abuse tests
- [ ] Fix manifest license/repository inconsistencies
- [ ] Add signed release and provenance workflow
