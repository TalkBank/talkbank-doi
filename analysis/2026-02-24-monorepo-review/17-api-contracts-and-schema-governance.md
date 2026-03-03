# 17. API Contracts and Schema Governance

## Findings

- API contracts exist in multiple forms (Rust types, OpenAPI-generated TS types, CLI flags, parser specs).
- Contract governance is strong in `talkbank-chat`, less formalized in other repos.
- Opportunities exist to reduce contract drift across Python server responses and frontend consumers.

## Recommendations

1. Version API contracts explicitly (semantic versioning for service contracts).
2. Generate and validate contracts in CI:
   - server OpenAPI schema
   - frontend generated types
   - compatibility tests against snapshots
3. Introduce backward compatibility checks for public/remote interfaces.
4. Create one change policy for contract-breaking modifications.
5. Add deprecation windows and migration docs for users.

## Tools/frameworks to leverage

- OpenAPI diffing tool in CI
- Contract tests for key endpoints
- JSON schema validation for persisted artifacts

## API governance checklist

- [ ] Define API versioning policy and compatibility guarantees
- [ ] Add schema-diff CI checks on every API-changing PR
- [ ] Add generated-type freshness checks in frontend CI
- [ ] Add deprecation/migration template for breaking changes
- [ ] Document public interface ownership by subsystem
