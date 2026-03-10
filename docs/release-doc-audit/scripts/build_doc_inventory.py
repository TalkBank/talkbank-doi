#!/usr/bin/env python3
from __future__ import annotations

from pathlib import Path


ROOT = Path("/Users/chen/talkbank")
PRIVATE_ROOT = ROOT / "talkbank-private" / "docs" / "release-doc-audit"


def rel(path: Path) -> str:
    return str(path.relative_to(ROOT))


def write_tsv(path: Path, rows: list[tuple[str, ...]]) -> None:
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text("\n".join("\t".join(col for col in row) for row in rows) + "\n")


def collect_talkbank_tools() -> list[tuple[str, ...]]:
    repo = ROOT / "talkbank-tools"
    rows: list[tuple[str, ...]] = [
        ("artifact", "audience", "doc_type", "risk", "status", "notes"),
    ]

    priority = {
        "README.md",
        "CONTRIBUTING.md",
        ".github/workflows/ci.yml",
        ".github/workflows/release.yml",
        "crates/talkbank-cli/README.md",
        "book/src/user-guide/installation.md",
        "book/src/user-guide/cli-reference.md",
        "book/src/user-guide/migrating-from-clan.md",
        "book/src/integrating/library-usage.md",
        "book/src/integrating/json-output.md",
        "book/src/integrating/json-schema.md",
        "book/src/integrating/diagnostic-contract.md",
    }

    for path in sorted(repo.rglob("*.md")):
        repo_rel = rel(path)
        local_rel = str(path.relative_to(repo))
        if "book/build/" in repo_rel or "/target/" in repo_rel:
            continue
        audience = "developer"
        doc_type = "reference"
        risk = "medium"
        if local_rel.startswith("book/src/user-guide/") or local_rel == "README.md":
            audience = "user"
            risk = "critical" if local_rel in priority else "high"
        elif local_rel.startswith("book/src/integrating/"):
            audience = "integrator"
            risk = "critical"
        elif local_rel.startswith("book/src/contributing/") or local_rel == "CONTRIBUTING.md":
            audience = "developer"
            risk = "high"
        elif local_rel.startswith("crates/") and local_rel.endswith("README.md"):
            audience = "developer"
            doc_type = "crate_readme"
            risk = "high" if local_rel in priority else "medium"
        elif local_rel.startswith("book/src/architecture/"):
            audience = "developer"
            doc_type = "architecture"
        elif local_rel.startswith("book/src/chat-format/"):
            audience = "user"
            doc_type = "format_reference"
            risk = "high"
        elif local_rel.startswith("book/src/"):
            doc_type = "book"

        status = "unreviewed"
        notes = ""
        if local_rel in priority:
            notes = "priority-1"
        rows.append((repo_rel, audience, doc_type, risk, status, notes))

    for wf in sorted((repo / ".github" / "workflows").glob("*.yml")):
        rows.append((rel(wf), "developer", "workflow", "critical", "unreviewed", "priority-1"))

    return rows


def collect_batchalign3() -> list[tuple[str, ...]]:
    repo = ROOT / "batchalign3"
    rows: list[tuple[str, ...]] = [
        ("artifact", "audience", "doc_type", "risk", "status", "notes"),
    ]

    priority = {
        "README.md",
        ".github/workflows/test.yml",
        ".github/workflows/release.yml",
        ".github/workflows/release-cli.yml",
        "pyproject.toml",
        "cli-pyproject.toml",
        "book/src/user-guide/quick-start.md",
        "book/src/user-guide/cli-reference.md",
        "book/src/user-guide/server-mode.md",
        "book/src/user-guide/troubleshooting.md",
        "book/src/migration/index.md",
        "book/src/migration/user-migration.md",
        "book/src/migration/developer-migration.md",
        "book/src/migration/algorithms-and-language.md",
    }

    md_roots = [repo / "book" / "src", repo]
    seen: set[str] = set()
    for root in md_roots:
        for path in sorted(root.rglob("*.md")):
            repo_rel = rel(path)
            local_rel = str(path.relative_to(repo))
            if local_rel.startswith("book/build/") or "/target/" in repo_rel:
                continue
            if repo_rel in seen:
                continue
            seen.add(repo_rel)
            audience = "developer"
            doc_type = "reference"
            risk = "medium"
            if local_rel == "README.md" or local_rel.startswith("book/src/user-guide/"):
                audience = "user"
                risk = "critical" if local_rel in priority else "high"
            elif local_rel.startswith("book/src/migration/"):
                audience = "user+developer"
                doc_type = "migration"
                risk = "critical"
            elif local_rel.startswith("book/src/developer/"):
                audience = "developer"
                doc_type = "developer"
                risk = "high"
            elif local_rel.startswith("book/src/reference/"):
                audience = "developer"
                doc_type = "reference"
                risk = "high"
            elif local_rel.startswith("book/src/architecture/"):
                audience = "developer"
                doc_type = "architecture"
            elif local_rel.startswith(".github/workflows/"):
                audience = "developer"
                doc_type = "workflow"
                risk = "critical"
            elif local_rel.endswith("README.md"):
                audience = "developer"
                doc_type = "readme"

            status = "unreviewed"
            notes = ""
            if local_rel in priority:
                notes = "priority-1"
            rows.append((repo_rel, audience, doc_type, risk, status, notes))

    for extra in [
        repo / ".github" / "workflows",
        repo,
    ]:
        if extra.is_dir():
            for wf in sorted(extra.glob("*.toml")):
                local_rel = str(wf.relative_to(repo))
                if local_rel in {"pyproject.toml", "cli-pyproject.toml"}:
                    rows.append((rel(wf), "developer", "metadata", "critical", "unreviewed", "priority-1"))
        if extra == repo / ".github" / "workflows":
            for wf in sorted(extra.glob("*.yml")):
                rows.append((rel(wf), "developer", "workflow", "critical", "unreviewed", "priority-1"))

    return rows


def main() -> None:
    write_tsv(PRIVATE_ROOT / "inventories" / "talkbank-tools.tsv", collect_talkbank_tools())
    write_tsv(PRIVATE_ROOT / "inventories" / "batchalign3.tsv", collect_batchalign3())


if __name__ == "__main__":
    main()
