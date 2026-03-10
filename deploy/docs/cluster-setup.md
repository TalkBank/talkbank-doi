# Cluster Setup (Fleet + Redis Cache)

> **Historical note:** This is a BA-next-era private ops note. It is not current
> public `batchalign3` release guidance.

Practical setup guide for enabling multi-server batchalign dispatch with shared
Redis cache.

For full architecture details, see [`fleet-management-plan.md`](fleet-management-plan.md).

## 1. Enable Fleet Discovery on Client Macs

Create `~/.batchalign-next/fleet.yaml` on every client machine that submits
jobs:

```yaml
servers:
  - name: net
    url: http://net:8000
  - name: bilbo
    url: http://bilbo:8000
  - name: brian
    url: http://brian:8000
```

Verify:

```bash
batchalign-next fleet status
```

## 2. Enable Distributed Redis Cache on Server Macs

On every server machine, add `redis_url` to `~/.batchalign-next/server.yaml`:

```yaml
redis_url: redis://net:6379/0
```

Restart server after config change:

```bash
batchalign-next serve stop
batchalign-next serve start
```

## 3. Verify Redis Cache Is Actually Active

Check each server:

```bash
curl -s http://net:8000/health | python3 -m json.tool
```

Expected fields:

- `cache_backend: "hybrid"`
- `redis_cache_enabled: true`
- `redis_cache_connected: true`

If `redis_cache_enabled` is `true` but `redis_cache_connected` is `false`,
the server is in local SQLite fallback mode (Redis configured but unreachable).

## 4. Seed Existing Local Cache into Redis (Optional)

Run on each server machine:

```bash
REDIS_URL=redis://net:6379/0 uv run python scripts/migrate_cache_to_redis.py
```

## 5. Disable Redis Cache

Remove `redis_url` from `server.yaml`, then restart server. Cache will run in
local SQLite mode only.
