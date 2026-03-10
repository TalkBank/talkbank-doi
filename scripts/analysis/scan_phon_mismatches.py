#!/usr/bin/env python3
"""Scan Phon XML source files for orthography↔IPA word count mismatches.

Compares <w> elements in <orthography> with <pw> elements in <ipaTarget>
and <ipaActual>. Reports files and records where counts disagree.

Usage:
    python3 scripts/analysis/scan_phon_mismatches.py ~/data/phon-data
    python3 scripts/analysis/scan_phon_mismatches.py ~/data/phon-data --verbose
"""

import xml.etree.ElementTree as ET
import glob
import os
import argparse


def analyze_file(xml_file: str) -> list[dict]:
    """Return list of mismatched records in a Phon XML file."""
    try:
        tree = ET.parse(xml_file)
    except ET.ParseError:
        return []

    root = tree.getroot()
    ns = ""
    if root.tag.startswith("{"):
        ns = root.tag.split("}")[0] + "}"

    mismatches = []
    for record in root.iter(f"{ns}r"):
        ortho = record.find(f"{ns}orthography")
        if ortho is None:
            continue
        u_elem = ortho.find(f"{ns}u")
        if u_elem is None:
            continue

        ortho_words = len(u_elem.findall(f"{ns}w"))
        ortho_pauses = len(u_elem.findall(f"{ns}pause"))

        for tier_name, tier_tag in [
            ("target", f"{ns}ipaTarget"),
            ("actual", f"{ns}ipaActual"),
        ]:
            tier = record.find(tier_tag)
            if tier is None:
                continue
            pho = tier.find(f"{ns}pho")
            if pho is None:
                continue

            pw_count = len(pho.findall(f"{ns}pw"))
            ipa_pauses = sum(
                1 for c in pho if c.tag.replace(ns, "") in ("pause", "silence")
            )

            if pw_count == 0:
                continue

            if ortho_words != pw_count:
                rid = record.get("id", "?")
                ortho_text = " ".join(
                    w.text or "" for w in u_elem.findall(f"{ns}w")
                )
                mismatches.append(
                    {
                        "record": rid,
                        "tier": tier_name,
                        "ortho_words": ortho_words,
                        "ortho_pauses": ortho_pauses,
                        "ipa_words": pw_count,
                        "ipa_pauses": ipa_pauses,
                        "ortho_text": ortho_text,
                    }
                )

    return mismatches


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("data_dir", help="Root directory containing 0phon/ folders")
    parser.add_argument(
        "--verbose", "-v", action="store_true", help="Show per-record details"
    )
    args = parser.parse_args()

    xml_files = sorted(
        glob.glob(f"{args.data_dir}/**/0phon/**/*.xml", recursive=True)
    )

    total_files = len(xml_files)
    total_mismatches = 0
    files_with_mismatches = 0

    for xml_file in xml_files:
        mismatches = analyze_file(xml_file)
        if mismatches:
            files_with_mismatches += 1
            total_mismatches += len(mismatches)
            rel = os.path.relpath(xml_file, args.data_dir)
            print(f"  {rel}: {len(mismatches)} mismatches")
            if args.verbose:
                for m in mismatches:
                    print(
                        f"    {m['record']} ({m['tier']}): "
                        f"ortho={m['ortho_words']} ipa={m['ipa_words']}  "
                        f"[{m['ortho_text'][:80]}]"
                    )

    print(f"\nTotal XML files scanned: {total_files}")
    print(f"Files with mismatches: {files_with_mismatches}")
    print(f"Total mismatched records: {total_mismatches}")


if __name__ == "__main__":
    main()
