"""Dispatch router — selects server or local dispatch.

Local (ProcessPoolExecutor) is the default for single-machine use.
Server is used when --server is set to send files over HTTP.
"""

from __future__ import annotations

import os
from pathlib import Path
from typing import TYPE_CHECKING

if TYPE_CHECKING:
    import click
    from rich.console import Console

# Re-export for backward compatibility with imports from this module
from batchalign.cli.file_io import _discover_files, Cmd2Task  # noqa: F401


def _resolve_inputs(
    paths: tuple[str, ...], output: str | None,
    file_list: str | None, in_place: bool,
) -> tuple[list[str], str | None]:
    """Resolve CLI arguments into ``(input_paths, output_dir)``.

    *input_paths*: list of files and/or directories to process.
    *output_dir*: explicit output dir, or ``None`` for in-place.
    """
    # --file-list mode
    if file_list:
        with open(file_list) as f:
            items = [line.strip() for line in f if line.strip() and not line.startswith("#")]
        for p in items:
            if not os.path.exists(p):
                raise SystemExit(f"Error: file from --file-list does not exist: {p}")
        if not items:
            raise SystemExit("Error: --file-list is empty.")
        return items, output

    if len(paths) == 0:
        raise SystemExit("Error: no input paths provided.")

    # --in-place or -o: all paths are inputs
    if in_place or output is not None:
        for p in paths:
            if not os.path.exists(p):
                raise SystemExit(f"Error: input path does not exist: {p}")
        return list(paths), output  # output is None when --in-place

    # Backward compat: exactly 2 paths, and the second is a directory
    # or the first is a directory and the second doesn't exist.
    if len(paths) == 2:
        if os.path.isdir(paths[1]) or (os.path.isdir(paths[0]) and not os.path.exists(paths[1])):
            return [paths[0]], paths[1]

    # Single path or multiple files → in-place
    for p in paths:
        if not os.path.exists(p):
            raise SystemExit(f"Error: input path does not exist: {p}")

    return list(paths), None


def _dispatch(command: str, lang: str, num_speakers: int,
              extensions: list[str], ctx: click.Context,
              inputs: list[str], out_dir: str | None,
              console: Console, **kwargs: object) -> None:
    """Route to server or local dispatch."""
    # Inject global override_cache into kwargs so all dispatch paths see it
    if ctx.obj.get("override_cache") and "override_cache" not in kwargs:
        kwargs["override_cache"] = True
    server = ctx.obj.get("server")

    # --bank requires --server
    bank = kwargs.get("bank")
    if bank and not server:
        console.print("[bold red]Error:[/bold red] --bank requires --server.")
        raise SystemExit(1)

    if server:
        # Transcribe uses local audio — remote server can't access it
        if command in ("transcribe", "transcribe_s"):
            console.print(
                "[yellow]Transcribe uses local audio files — "
                "ignoring --server, processing locally.[/yellow]"
            )
            # Fall through to local dispatch below
        else:
            from batchalign.cli.dispatch_server import _dispatch_server
            _dispatch_server(command, lang, num_speakers,
                             extensions, ctx, inputs, out_dir,
                             console, **kwargs)
            return
    else:
        # No explicit --server: check fleet.yaml for multi-server config
        try:
            from batchalign.serve.fleet import resolve_server_urls
            fleet_urls = resolve_server_urls(None)
        except (ImportError, ModuleNotFoundError):
            fleet_urls = []
        if fleet_urls and command not in ("transcribe", "transcribe_s"):
            # Fleet config found — use multi-server dispatch
            ctx.obj["server"] = ",".join(fleet_urls)
            from batchalign.cli.dispatch_server import _dispatch_server
            _dispatch_server(command, lang, num_speakers,
                             extensions, ctx, inputs, out_dir,
                             console, **kwargs)
            return

        # No fleet config — try local daemon (shared model process)
        try:
            from batchalign.runtime import is_command_available
            cmd_available = is_command_available(command)
        except ImportError:
            cmd_available = True

        if cmd_available:
            # Host Python can run this — use main daemon
            try:
                from batchalign.cli.daemon import ensure_daemon
                daemon_url = ensure_daemon(console)
            except ImportError:
                daemon_url = None
            if daemon_url is not None:
                from batchalign.cli.dispatch_server import _dispatch_paths_mode
                _dispatch_paths_mode(daemon_url, command, lang, num_speakers,
                                     extensions, inputs, out_dir,
                                     console, **kwargs)
                return
        else:
            # Host Python lacks deps — try sidecar daemon
            try:
                from batchalign.cli.daemon import ensure_sidecar_daemon
                sidecar_url = ensure_sidecar_daemon(console)
            except ImportError:
                sidecar_url = None
            if sidecar_url is not None:
                from batchalign.cli.dispatch_server import _dispatch_paths_mode
                _dispatch_paths_mode(sidecar_url, command, lang,
                                     num_speakers, extensions, inputs,
                                     out_dir, console, **kwargs)
                return
            # Sidecar unavailable — fall through to direct local
            # (will fail with a clear ImportError)

        # Daemon unavailable — fall through to direct local dispatch
        console.print(
            "[yellow]Local daemon unavailable, processing directly...[/yellow]")

    # Local mode — create output dir if specified but doesn't exist
    if out_dir is not None:
        os.makedirs(out_dir, exist_ok=True)

    from batchalign.cli.dispatch_local import _dispatch_local
    _dispatch_local(command, lang, num_speakers,
                    extensions, ctx, inputs, out_dir,
                    console, **kwargs)
