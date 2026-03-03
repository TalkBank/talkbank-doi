# 20. Frontend and Dashboard

## Findings

- `batchalign3/frontend` is modern TypeScript React with generated API types.
- State management is simple and understandable (`zustand`), and WS multi-server support is present.
- The UI is functional but still operationally dense; power-user workflows can be improved.

## Recommendations

1. Add robust data fetching/cache management (`@tanstack/react-query`) to simplify polling/retry/cache invalidation.
2. Add error boundary and offline/reconnect UX states.
3. Add accessibility and keyboard navigation audits.
4. Add frontend test coverage for critical workflows:
   - job list updates
   - filter/search/pagination
   - action buttons and failure states
5. Add contract safety checks for generated API type sync.

## Tools/frameworks to leverage

- `@tanstack/react-query`
- `playwright` for end-to-end dashboard scenarios
- `vitest` + React Testing Library for component behavior

## Frontend checklist

- [ ] Add query/cache layer and remove ad hoc fetch state management
- [ ] Add component tests for job detail and error grouping flows
- [ ] Add e2e smoke tests for multi-server dashboard behavior
- [ ] Add a11y checks and keyboard interaction tests
- [ ] Add CI gate for generated API type freshness
