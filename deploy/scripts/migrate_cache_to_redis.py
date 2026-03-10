#!/usr/bin/env python3
"""Migrate local SQLite cache entries to Redis.

Reads all entries from the local batchalign cache and writes them to Redis.
Duplicate keys are harmless — Redis SET is idempotent and the latest write wins.

Usage:
    # Using environment variable
    REDIS_URL=redis://net:6379/0 uv run python scripts/migrate_cache_to_redis.py

    # Or pass as argument
    uv run python scripts/migrate_cache_to_redis.py redis://net:6379/0
"""

from __future__ import annotations

import json
import os
import socket
import sqlite3
import sys


def main() -> None:
    """Run main."""
    redis_url = os.environ.get("REDIS_URL") or (sys.argv[1] if len(sys.argv) > 1 else "")
    if not redis_url:
        print("Usage: REDIS_URL=redis://host:6379/0 python scripts/migrate_cache_to_redis.py")
        sys.exit(1)

    from batchalign.pipelines.cache_redis import RedisCacheManager

    hostname = socket.gethostname().split(".")[0]
    print(f"Migrating cache from {hostname} to {redis_url}")

    # Connect to Redis
    redis_cache = RedisCacheManager(redis_url)

    # Open local SQLite cache
    from platformdirs import user_cache_dir
    from pathlib import Path

    cache_dir = Path(user_cache_dir("batchalign3", "batchalign3"))
    db_path = cache_dir / "cache.db"

    if not db_path.exists():
        print(f"  No local cache found at {db_path}")
        return

    conn = sqlite3.connect(str(db_path), timeout=30.0)
    conn.execute("PRAGMA journal_mode = WAL")

    # Count entries
    total = conn.execute("SELECT COUNT(*) FROM cache_entries").fetchone()[0]
    print(f"  Found {total} entries in local cache")

    if total == 0:
        print("  Nothing to migrate.")
        conn.close()
        return

    # Migrate in batches
    BATCH_SIZE = 1000
    migrated = 0

    cursor = conn.execute(
        "SELECT key, task, engine_version, batchalign_version, data "
        "FROM cache_entries"
    )

    batch: list[tuple[str, str, str, str, dict[str, object]]] = []
    for key, task, engine_version, batchalign_version, data_blob in cursor:
        data = json.loads(data_blob)
        batch.append((key, task, engine_version, batchalign_version, data))

        if len(batch) >= BATCH_SIZE:
            _write_batch(redis_cache, batch)
            migrated += len(batch)
            print(f"  Migrated {migrated}/{total} entries...", end="\r")
            batch = []

    # Final batch
    if batch:
        _write_batch(redis_cache, batch)
        migrated += len(batch)

    conn.close()
    print(f"  Migrated {migrated} entries from {hostname} to Redis")


def _write_batch(
    redis_cache: object,
    batch: list[tuple[str, str, str, str, dict[str, object]]],
) -> None:
    """Write a batch of entries to Redis, grouped by (task, engine_version)."""
    from batchalign.pipelines.cache_redis import RedisCacheManager
    assert isinstance(redis_cache, RedisCacheManager)

    # Group by (task, engine_version, batchalign_version) for put_batch
    groups: dict[tuple[str, str, str], list[tuple[str, dict[str, object]]]] = {}
    for key, task, engine_version, batchalign_version, data in batch:
        group_key = (task, engine_version, batchalign_version)
        groups.setdefault(group_key, []).append((key, data))

    for (task, engine_version, batchalign_version), entries in groups.items():
        redis_cache.put_batch(entries, task, engine_version, batchalign_version)


if __name__ == "__main__":
    main()
