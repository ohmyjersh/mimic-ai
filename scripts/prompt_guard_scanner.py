#!/usr/bin/env python3
"""Scan markdown prompt files for injection attacks using Llama Prompt Guard 2-86M."""

import argparse
import json
import re
import sys
from glob import glob
from pathlib import Path

from transformers import pipeline

MODEL_ID = "meta-llama/Llama-Prompt-Guard-2-86M"
DEFAULTS_DIR = Path(__file__).resolve().parent.parent / "defaults"

# ANSI colors
RED = "\033[91m"
GREEN = "\033[92m"
YELLOW = "\033[93m"
BOLD = "\033[1m"
RESET = "\033[0m"


def strip_frontmatter(text: str) -> str:
    """Remove YAML frontmatter delimited by --- from markdown content."""
    return re.sub(r"\A---\n.*?\n---\n*", "", text, count=1, flags=re.DOTALL).strip()


def scan_files(paths: list[str]) -> int:
    """Scan the given markdown files and return exit code (1 if any malicious)."""
    if not paths:
        print(f"{YELLOW}No files to scan.{RESET}")
        return 0

    print(f"{BOLD}Loading model: {MODEL_ID}{RESET}")
    classifier = pipeline("text-classification", model=MODEL_ID)

    results = []
    malicious_count = 0

    print(f"\n{BOLD}Scanning {len(paths)} file(s)...{RESET}\n")
    print(f"{'File':<60} {'Verdict':<12} {'Confidence'}")
    print("-" * 88)

    for filepath in sorted(paths):
        path = Path(filepath)
        if not path.exists():
            print(f"{YELLOW}{filepath:<60} SKIPPED      (file not found){RESET}")
            continue

        text = strip_frontmatter(path.read_text(encoding="utf-8"))
        if not text:
            results.append({"file": filepath, "label": "BENIGN", "score": 1.0, "note": "empty after stripping frontmatter"})
            print(f"{filepath:<60} {'BENIGN':<12} {'N/A (empty)'}")
            continue

        prediction = classifier(text)[0]
        label = prediction["label"]
        score = prediction["score"]

        result = {"file": filepath, "label": label, "score": round(score, 6)}
        results.append(result)

        if label != "BENIGN":
            malicious_count += 1
            color = RED
        else:
            color = GREEN

        print(f"{color}{filepath:<60} {label:<12} {score:.4f}{RESET}")

    # Summary
    print(f"\n{'-' * 88}")
    total = len(results)
    benign = total - malicious_count
    print(f"{BOLD}Summary:{RESET} {benign}/{total} benign", end="")
    if malicious_count:
        print(f", {RED}{malicious_count} malicious{RESET}")
    else:
        print(f" {GREEN}— all clear{RESET}")

    # Write JSON results
    output_path = Path("prompt-guard-results.json")
    output_path.write_text(json.dumps(results, indent=2) + "\n", encoding="utf-8")
    print(f"\nResults written to {output_path}")

    if malicious_count:
        print(f"\n{RED}{BOLD}FAILED:{RESET}{RED} {malicious_count} file(s) flagged as potentially malicious.{RESET}")
        return 1

    return 0


def main() -> None:
    parser = argparse.ArgumentParser(description="Scan prompt files for injection attacks.")
    parser.add_argument("files", nargs="*", help="Specific markdown files to scan")
    parser.add_argument("--scan-all", action="store_true", help="Scan all files in defaults/")
    args = parser.parse_args()

    if args.scan_all:
        files = [str(p) for p in sorted(DEFAULTS_DIR.rglob("*.md"))]
    elif args.files:
        files = args.files
    else:
        parser.error("Provide file paths or use --scan-all")

    sys.exit(scan_files(files))


if __name__ == "__main__":
    main()
