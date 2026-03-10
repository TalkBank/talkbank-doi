#!/usr/bin/env python3
"""Capture release-audit CLI evidence for talkbank-tools and batchalign3.

This script snapshots the current public CLI/help surfaces into a dated
directory under `docs/release-doc-audit/evidence/`.  It is intentionally
conservative: only public entry points and generated contracts that are relevant
to release docs are captured.
"""

from __future__ import annotations

import datetime as _dt
import json
import subprocess
from pathlib import Path


ROOT = Path(__file__).resolve().parents[4]
AUDIT_ROOT = ROOT / "talkbank-private" / "docs" / "release-doc-audit"
EVIDENCE_ROOT = AUDIT_ROOT / "evidence"


def run_capture(
    *,
    repo: str,
    cwd: Path,
    name: str,
    cmd: list[str],
    output_ext: str = ".txt",
) -> None:
    proc = subprocess.run(
        cmd,
        cwd=cwd,
        capture_output=True,
        text=True,
        check=False,
    )

    target_dir = EVIDENCE_ROOT / _dt.date.today().isoformat() / repo
    target_dir.mkdir(parents=True, exist_ok=True)

    stem = target_dir / name
    payload = {
        "cmd": cmd,
        "cwd": str(cwd),
        "returncode": proc.returncode,
        "captured_at": _dt.datetime.now().astimezone().isoformat(),
        "stdout_file": f"{name}{output_ext}",
        "stderr_file": f"{name}.stderr.txt",
    }
    (stem.with_suffix(".meta.json")).write_text(
        json.dumps(payload, indent=2) + "\n",
        encoding="utf-8",
    )
    (stem.with_suffix(output_ext)).write_text(proc.stdout, encoding="utf-8")
    (stem.with_name(f"{name}.stderr.txt")).write_text(proc.stderr, encoding="utf-8")

    if proc.returncode != 0:
        raise SystemExit(
            f"capture failed for {repo}/{name}: exit {proc.returncode}\n"
            f"command: {' '.join(cmd)}"
        )


def main() -> None:
    talkbank_tools = ROOT / "talkbank-tools"
    batchalign3 = ROOT / "batchalign3"

    talkbank_commands = [
        ("chatter-help", ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "--help"]),
        (
            "chatter-validate-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "validate", "--help"],
        ),
        (
            "chatter-normalize-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "normalize", "--help"],
        ),
        (
            "chatter-to-json-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "to-json", "--help"],
        ),
        (
            "chatter-from-json-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "from-json", "--help"],
        ),
        (
            "chatter-schema-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "schema", "--help"],
        ),
        (
            "chatter-cache-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "cache", "--help"],
        ),
        (
            "chatter-cache-stats-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "cache", "stats", "--help"],
        ),
        (
            "chatter-cache-clear-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "cache", "clear", "--help"],
        ),
        (
            "chatter-clan-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "clan", "--help"],
        ),
        (
            "chatter-clan-freq-help",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "clan", "freq", "--help"],
        ),
        (
            "chatter-schema-url",
            ["cargo", "run", "-q", "-p", "talkbank-cli", "--", "schema", "--url"],
        ),
    ]

    batchalign_commands = [
        ("batchalign3-help", ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "--help"]),
        (
            "batchalign3-align-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "align", "--help"],
        ),
        (
            "batchalign3-transcribe-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "transcribe", "--help"],
        ),
        (
            "batchalign3-morphotag-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "morphotag", "--help"],
        ),
        (
            "batchalign3-setup-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "setup", "--help"],
        ),
        (
            "batchalign3-logs-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "logs", "--help"],
        ),
        (
            "batchalign3-serve-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "serve", "--help"],
        ),
        (
            "batchalign3-serve-start-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "serve", "start", "--help"],
        ),
        (
            "batchalign3-serve-status-help",
            [
                "cargo",
                "run",
                "-q",
                "-p",
                "batchalign-cli",
                "--",
                "serve",
                "status",
                "--help",
            ],
        ),
        (
            "batchalign3-jobs-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "jobs", "--help"],
        ),
        (
            "batchalign3-cache-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "cache", "--help"],
        ),
        (
            "batchalign3-cache-stats-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "cache", "stats", "--help"],
        ),
        (
            "batchalign3-cache-clear-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "cache", "clear", "--help"],
        ),
        (
            "batchalign3-openapi-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "openapi", "--help"],
        ),
        (
            "batchalign3-bench-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "bench", "--help"],
        ),
        (
            "batchalign3-models-help",
            ["cargo", "run", "-q", "-p", "batchalign-cli", "--", "models", "--help"],
        ),
    ]

    for name, cmd in talkbank_commands:
        run_capture(repo="talkbank-tools", cwd=talkbank_tools, name=name, cmd=cmd)

    for name, cmd in batchalign_commands:
        run_capture(repo="batchalign3", cwd=batchalign3, name=name, cmd=cmd)

    openapi_target = (
        EVIDENCE_ROOT
        / _dt.date.today().isoformat()
        / "batchalign3"
        / "batchalign3-openapi.json"
    )
    openapi_target.parent.mkdir(parents=True, exist_ok=True)
    run_capture(
        repo="batchalign3",
        cwd=batchalign3,
        name="batchalign3-openapi-generate",
        cmd=[
            "cargo",
            "run",
            "-q",
            "-p",
            "batchalign-cli",
            "--",
            "openapi",
            "--output",
            str(openapi_target),
        ],
    )


if __name__ == "__main__":
    main()
