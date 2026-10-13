#!/usr/bin/env python3
"""Fetch the COMCIFS cif_api test-data CIFs into this directory.

`manifest.json` pins the upstream commit and records, per file, the
byte length, the sha256, and the git blob SHA from the GitHub
contents API. Rerunning keeps files that already match the manifest
and refetches the rest; `--force` refetches all. Plain HTTP only;
no dependencies.

Usage:
  python3 tests/real_data/cif_api/fetch.py            # resume
  python3 tests/real_data/cif_api/fetch.py --force    # refetch all

The files are gitignored; the expected JSON pins in
`tests/real_data/expected/` are committed. CI runs this script
before `cargo test` (`.github/workflows/ci.yml`).
"""

import argparse
import hashlib
import json
import time
import urllib.request
from pathlib import Path

HERE = Path(__file__).resolve().parent
MANIFEST = HERE / "manifest.json"
TIMEOUT = 60


def get(url: str, retries: int = 3) -> bytes:
    last = None
    for i in range(retries):
        try:
            with urllib.request.urlopen(url, timeout=TIMEOUT) as r:
                return r.read()
        except Exception as e:
            last = e
            time.sleep(1.5 * (i + 1))
    raise RuntimeError(f"GET {url} failed: {last}")


def git_blob_sha(data: bytes) -> str:
    header = b"blob " + str(len(data)).encode() + b"\x00"
    return hashlib.sha1(header + data).hexdigest()


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--force", action="store_true", help="refetch all files")
    args = parser.parse_args()

    manifest = json.loads(MANIFEST.read_text())
    base = (
        f"https://raw.githubusercontent.com/"
        f"{manifest['repo']}/{manifest['commit']}/{manifest['path']}/"
    )

    for entry in manifest["files"]:
        dest = HERE / entry["file"]
        if not args.force and dest.exists():
            old = dest.read_bytes()
            if hashlib.sha256(old).hexdigest() == entry["sha256"]:
                print(f"  keep {entry['file']} (matches manifest)")
                continue
            print(f"  stale {entry['file']} (refetching)")
        data = get(base + entry["file"])
        sha256 = hashlib.sha256(data).hexdigest()
        if sha256 != entry["sha256"]:
            raise RuntimeError(f"{entry['file']}: sha256 mismatch")
        blob = git_blob_sha(data)
        if blob != entry["blob_sha"]:
            raise RuntimeError(f"{entry['file']}: git blob SHA mismatch")
        dest.write_bytes(data)
        print(f"  wrote {entry['file']} ({len(data)} bytes)")


if __name__ == "__main__":
    main()
