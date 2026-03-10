#!/usr/bin/env python3
"""Demo: process local CHAT files via a remote Batchalign server.

The server (e.g. Net) has access to media files on its local drives.
The client (e.g. Frodo) has .cha files and sends only the tiny CHAT text
(~2KB each) over HTTP. The server resolves media via its configured
media_mappings and processes everything locally. No media files cross
the network.

Prerequisites:
    The server must already be running with media_mappings configured:
        # ~/.batchalign/server.yaml on the server machine
        media_mappings:
          childes-data: /Volumes/CHILDES/CHILDES
          aphasia-data: /Volumes/Other/aphasia
        # Then: batchalign serve start

Usage:
    # Align CHAT files against media on the server
    python tools/demo_cluster.py \\
        --server http://net:8000 \\
        --data-dir ~/0data/childes-data/Eng-NA/MacWhinney/0young-ASR/ \\
        --command align

    # Morphotag (no media needed)
    python tools/demo_cluster.py \\
        --server http://net:8000 \\
        --data-dir ~/0data/childes-data/Eng-NA/MacWhinney/0young-ASR/ \\
        --command morphotag

    # Local demo with bench_data (server must be running locally)
    python tools/demo_cluster.py \\
        --server http://localhost:8000 \\
        --data-dir bench_data/align_small
"""

from __future__ import annotations

import argparse
import os
import sys
import tempfile
import time
from pathlib import Path
from typing import Any


# ---------------------------------------------------------------------------
# Server health
# ---------------------------------------------------------------------------


def _check_server(server_url: str, console: Any) -> dict[str, Any]:
    """Check server health and return the health response dict."""
    import requests

    try:
        resp = requests.get(f"{server_url}/health", timeout=5)
        resp.raise_for_status()
        return resp.json()
    except Exception as exc:
        console.print(f"[bold red]Cannot reach server at {server_url}:[/bold red] {exc}")
        console.print("\nIs the server running? Start it with:")
        console.print("  batchalign serve start")
        sys.exit(1)


# ---------------------------------------------------------------------------
# Media mapping detection
# ---------------------------------------------------------------------------


def _detect_media_mapping(data_dir: str,
                          mapping_keys: list[str]) -> tuple[str, str]:
    """Detect media mapping from input path.

    Looks for a mapping key (e.g. 'childes-data') as a path component of
    *data_dir*. Returns (mapping_key, subdir) where subdir is the relative
    path after the key. Returns ("", "") if no mapping is detected.
    """
    if not mapping_keys:
        return "", ""

    abs_path = os.path.abspath(data_dir)
    parts = Path(abs_path).parts

    for key in mapping_keys:
        if key in parts:
            idx = parts.index(key)
            subdir = str(Path(*parts[idx + 1:])) if idx + 1 < len(parts) else ""
            return key, subdir

    return "", ""


# ---------------------------------------------------------------------------
# File discovery and submission
# ---------------------------------------------------------------------------


def _discover_cha_files(data_dir: str) -> list[tuple[str, str]]:
    """Walk data_dir for .cha files. Returns list of (filename, content)."""
    results: list[tuple[str, str]] = []
    for dirpath, _dirs, files in os.walk(data_dir):
        for f in sorted(files):
            if f.endswith(".cha"):
                path = os.path.join(dirpath, f)
                with open(path, encoding="utf-8") as fh:
                    results.append((f, fh.read()))
    return results


def _submit_and_poll(server_url: str, command: str,
                     files: list[tuple[str, str]],
                     media_mapping: str, media_subdir: str,
                     console: Any) -> tuple[str, list[dict[str, Any]]]:
    """Submit a job and poll until completion with a Rich Live table.

    Returns (job_id, list of result file dicts).
    """
    import requests
    from rich.live import Live
    from rich.table import Table

    # Build payload
    file_payloads = [
        {"filename": name, "content": content}
        for name, content in files
    ]
    payload: dict[str, Any] = {
        "command": command,
        "files": file_payloads,
        "media_mapping": media_mapping,
        "media_subdir": media_subdir,
    }

    # Submit
    resp = requests.post(f"{server_url}/jobs", json=payload, timeout=30)
    resp.raise_for_status()
    job_info = resp.json()
    job_id: str = job_info["job_id"]
    total_files: int = job_info["total_files"]

    console.print(f"Job [bold]{job_id}[/bold] submitted "
                  f"({total_files} file{'s' if total_files != 1 else ''})\n")

    # Status styling
    _STYLE: dict[str, str] = {
        "queued": "dim",
        "processing": "bold blue",
        "done": "green",
        "error": "bold red",
    }

    start_time = time.monotonic()

    def _build_table(info: dict[str, Any]) -> Table:
        """Build table."""
        elapsed = time.monotonic() - start_time
        table = Table(
            title=f"Job {job_id}  ({elapsed:.0f}s)",
            show_edge=False,
        )
        table.add_column("File", style="cyan")
        table.add_column("Status", justify="center")
        for entry in info.get("file_statuses", []):
            style = _STYLE.get(entry["status"], "")
            table.add_row(
                entry["filename"],
                f"[{style}]{entry['status']}[/{style}]",
            )
        completed = info.get("completed_files", 0)
        total = info.get("total_files", total_files)
        table.caption = f"{completed}/{total} files completed"
        return table

    # Poll loop with Rich Live display
    live_started = False
    live: Live | None = None
    status = "running"

    try:
        while True:
            time.sleep(1.0)
            try:
                resp = requests.get(f"{server_url}/jobs/{job_id}", timeout=10)
                resp.raise_for_status()
            except Exception:
                continue

            info = resp.json()
            file_statuses = info.get("file_statuses", [])

            if file_statuses and not live_started:
                live = Live(
                    _build_table(info), console=console, refresh_per_second=2,
                )
                live.start()
                live_started = True
            elif live_started and live is not None:
                live.update(_build_table(info))

            status = info["status"]
            if status in ("completed", "failed", "cancelled"):
                if live_started and live is not None:
                    live.update(_build_table(info))
                    live.stop()
                else:
                    c = info.get("completed_files", 0)
                    console.print(f"  {c}/{total_files} files completed")
                break
    finally:
        if live_started and live is not None:
            try:
                live.stop()
            except Exception:
                pass

    if status == "failed":
        error = info.get("error", "unknown")  # type: ignore[possibly-undefined]
        console.print(f"\n[bold red]Job failed:[/bold red] {error}")

    if status == "cancelled":
        console.print("[yellow]Job was cancelled.[/yellow]")
        return job_id, []

    # Fetch results
    resp = requests.get(f"{server_url}/jobs/{job_id}/results", timeout=30)
    resp.raise_for_status()
    result_files: list[dict[str, Any]] = resp.json().get("files", [])
    return job_id, result_files


def _write_results(results: list[dict[str, Any]],
                   out_dir: str) -> list[str]:
    """Write result files to out_dir. Returns list of output paths."""
    os.makedirs(out_dir, exist_ok=True)
    paths: list[str] = []
    for result in results:
        if result.get("error"):
            continue
        out_path = os.path.join(out_dir, result["filename"])
        with open(out_path, "w", encoding="utf-8") as f:
            f.write(result["content"])
        paths.append(out_path)
    return paths


# ---------------------------------------------------------------------------
# Summary display
# ---------------------------------------------------------------------------


def _show_summary(input_files: list[tuple[str, str]],
                  results: list[dict[str, Any]],
                  elapsed: float, out_dir: str,
                  command: str,
                  console: Any) -> None:
    """Print a before/after summary table."""
    from rich.table import Table

    # Build a lookup from filename -> result content
    result_map: dict[str, str] = {}
    for r in results:
        if not r.get("error"):
            result_map[r["filename"]] = r["content"]

    table = Table(title="Results Summary", show_edge=False)
    table.add_column("File", style="cyan")
    table.add_column("Input lines", justify="right")
    table.add_column("Output lines", justify="right")
    table.add_column("Annotations added", style="green")

    for fname, content in input_files:
        in_lines = len(content.splitlines())
        out_content = result_map.get(fname, "")
        out_lines = len(out_content.splitlines()) if out_content else 0

        annotations: list[str] = []
        if out_content:
            if "%mor:" in out_content and "%mor:" not in content:
                annotations.append("%mor")
            if "%gra:" in out_content and "%gra:" not in content:
                annotations.append("%gra")
            # Check for timing annotations (align)
            if "\x15" in out_content and "\x15" not in content:
                annotations.append("timing")

        ann_str = ", ".join(annotations) if annotations else "-"
        table.add_row(fname, str(in_lines), str(out_lines), ann_str)

    console.print()
    console.print(table)
    console.print()
    console.print(f"[bold]Command:[/bold]    {command}")
    console.print(f"[bold]Wall time:[/bold]  {elapsed:.1f}s")
    console.print(f"[bold]Files:[/bold]      {len(input_files)}")
    console.print(f"[bold]Output:[/bold]     {out_dir}")

    if command == "align":
        console.print()
        console.print("[dim]Note: Only ~2KB of CHAT text per file crossed the network.[/dim]")
        console.print("[dim]Media files stayed on the server — no audio was transferred.[/dim]")


# ---------------------------------------------------------------------------
# Main
# ---------------------------------------------------------------------------


def main() -> None:
    """Run main."""
    parser = argparse.ArgumentParser(
        description=__doc__,
        formatter_class=argparse.RawDescriptionHelpFormatter,
    )
    parser.add_argument(
        "--server", default="http://localhost:8000",
        help="Server URL (default: http://localhost:8000)",
    )
    parser.add_argument(
        "--command", default="align",
        help="Batchalign command to run (default: align)",
    )
    parser.add_argument(
        "--data-dir", required=True,
        help="Input directory with .cha files",
    )
    parser.add_argument(
        "--out-dir", default=None,
        help="Output directory (default: temp dir)",
    )
    parser.add_argument(
        "--media-mapping", default=None,
        help="Override auto-detected media mapping key (e.g. 'childes-data')",
    )
    parser.add_argument(
        "--media-subdir", default=None,
        help="Override auto-detected media subdir",
    )
    args = parser.parse_args()

    from rich.console import Console
    console = Console()

    server_url = args.server.rstrip("/")
    out_dir = args.out_dir or tempfile.mkdtemp(prefix="ba_demo_out_")

    console.print()
    console.print("[bold]Batchalign Server Demo[/bold]")
    console.print("=" * 50)

    # Check server health
    console.print(f"  Server:   {server_url}")
    health = _check_server(server_url, console)
    console.print(f"  Status:   [green]{health['status']}[/green]")
    console.print(f"  Version:  {health.get('version', '?')}")

    mapping_keys = health.get("media_mapping_keys", [])
    if mapping_keys:
        console.print(f"  Mappings: {', '.join(mapping_keys)}")

    # Discover input files
    console.print(f"\n  Data:     {args.data_dir}")
    input_files = _discover_cha_files(args.data_dir)
    if not input_files:
        console.print(f"[bold red]No .cha files found in {args.data_dir}[/bold red]")
        sys.exit(1)
    console.print(f"  Files:    {len(input_files)}")
    console.print(f"  Command:  {args.command}")
    console.print(f"  Output:   {out_dir}")

    # Detect or use explicit media mapping
    if args.media_mapping is not None:
        media_mapping = args.media_mapping
        media_subdir = args.media_subdir or ""
    else:
        media_mapping, media_subdir = _detect_media_mapping(
            args.data_dir, mapping_keys)

    if media_mapping:
        console.print(f"\n  [bold]Media mapping:[/bold] {media_mapping}")
        console.print(f"  [bold]Media subdir:[/bold]  {media_subdir}")
        console.print("  [dim]Server will resolve media from its local drives[/dim]")
    elif args.command in ("align", "transcribe", "transcribe_s"):
        console.print("\n  [yellow]Warning: No media mapping detected.[/yellow]")
        console.print("  [yellow]Server will search media_roots by filename (may be slow).[/yellow]")

    console.print()

    try:
        # Submit and poll
        wall_start = time.monotonic()
        job_id, results = _submit_and_poll(
            server_url, args.command, input_files,
            media_mapping, media_subdir, console,
        )
        elapsed = time.monotonic() - wall_start

        # Write results
        if results:
            output_paths = _write_results(results, out_dir)
            console.print(f"\n  Wrote [bold]{len(output_paths)}[/bold] result files")

            # Show summary
            _show_summary(input_files, results, elapsed, out_dir,
                          args.command, console)
        else:
            console.print("\n[yellow]No results to write.[/yellow]")

    except KeyboardInterrupt:
        console.print("\n[yellow]Interrupted.[/yellow]")
    except Exception as exc:
        console.print(f"\n[bold red]Error:[/bold red] {exc}")
        import traceback
        traceback.print_exc()

    console.print()


if __name__ == "__main__":
    main()
