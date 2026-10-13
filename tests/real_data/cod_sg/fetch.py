#!/usr/bin/env python3
"""Fetch one COD entry per three-dimensional space group (1..230).

Writes `tests/real_data/cod_sg/sgNNN_<id>.cif` files plus
`manifest.json`. Rerunning skips groups already in the manifest
unless `--force` is given. Plain HTTP only; no dependencies.

Usage:
  python3 tests/real_data/cod_sg/fetch.py            # resume, fill gaps
  python3 tests/real_data/cod_sg/fetch.py --force     # refetch all
  python3 tests/real_data/cod_sg/fetch.py --only 1,2,3

The API is documented in docs/cod-api.md. The search endpoint is
https://www.crystallography.net/cod/result with the
`space_group_number` parameter.
"""

import argparse
import concurrent.futures
import json
import sys
import time
import urllib.request
from pathlib import Path

BASE = "https://www.crystallography.net/cod"
HERE = Path(__file__).resolve().parent
MANIFEST = HERE / "manifest.json"
SIZE_CAP = 150_000          # bytes; keep the corpus lean
MAX_CANDIDATES = 5          # candidates to look at per group
POLL = 8                    # candidate CIFs to download before giving up on size
WORKERS = 12
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


def lst_ids(sg: int) -> list[int]:
    url = f"{BASE}/result?space_group_number={sg}&format=lst"
    lines = [l.strip() for l in get(url).decode().splitlines() if l.strip()]
    return [int(l) for l in lines]


def metadata(cod_id: int) -> dict:
    url = f"{BASE}/result?id={cod_id}&format=json"
    raw = json.loads(get(url).decode())
    if not raw:
        return {}
    row = raw[0]
    return {
        "sg_symbol": row.get("sg"),
        "hall": row.get("sgHall"),
        "formula": row.get("formula"),
        "flags": row.get("flags"),
    }


def pick_candidate(sg: int, ids: list[int]) -> tuple[int, int] | None:
    """Pick the first candidate whose CIF is at most SIZE_CAP bytes."""
    for cod_id in ids[:POLL]:
        data = get(f"{BASE}/{cod_id}.cif")
        if len(data) <= SIZE_CAP:
            return cod_id, len(data)
    if not ids:
        return None
    # All candidates are oversized; keep the smallest of them.
    smallest = min(
        ((cid, len(get(f"{BASE}/{cid}.cif"))) for cid in ids[:POLL]),
        key=lambda t: t[1],
    )
    print(f"  note: sg {sg}: all candidates exceed the size cap; "
          f"using the smallest ({smallest[0]})", file=sys.stderr)
    return smallest


def process_group(sg: int, force: bool, existing: dict) -> dict:
    entry = existing.get(str(sg))
    if entry and not force:
        if entry.get("adopted"):
            # The CIF file is already on disk (from an earlier
            # partial run). Fill in the metadata and candidate list.
            entry["candidates"] = lst_ids(sg)[:MAX_CANDIDATES]
            entry["bytes"] = (HERE / entry["file"]).stat().st_size
            entry["url"] = f"{BASE}/{entry['id']}.cif"
            entry.update(metadata(entry["id"]))
            entry.pop("adopted", None)
            print(f"  sg {sg:3d} -> adopted {entry['file']} ({entry['bytes']} B)")
        return entry
    ids = lst_ids(sg)
    picked = pick_candidate(sg, ids) if ids else None
    if picked is None:
        return {"sg": sg, "gap": True, "candidates": ids}
    cod_id, n_bytes = picked
    meta = metadata(cod_id)
    fname = f"sg{sg:03d}_{cod_id}.cif"
    (HERE / fname).write_bytes(get(f"{BASE}/{cod_id}.cif"))
    entry = {
        "sg": sg,
        "id": cod_id,
        "file": fname,
        "bytes": n_bytes,
        "candidates": ids[:MAX_CANDIDATES],
        "url": f"{BASE}/{cod_id}.cif",
        **meta,
    }
    print(f"  sg {sg:3d} -> {fname} ({n_bytes} B) {meta.get('sg_symbol')!r}")
    return entry


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument("--force", action="store_true", help="refetch all groups")
    ap.add_argument("--only", help="comma separated sg numbers to process")
    args = ap.parse_args()

    only = None
    if args.only:
        only = set()
        for part in args.only.split(","):
            part = part.strip()
            if not part:
                continue
            lo, _, hi = part.partition("-")
            only.update(range(int(lo), int(hi or int(lo) + 1)))

    old, old_gaps = {}, []
    if MANIFEST.exists():
        manifest = json.loads(MANIFEST.read_text())
        old = {str(g["sg"]): g for g in manifest["groups"]}
        old_gaps = manifest.get("gaps", [])

    # Adopt CIF files written by an earlier partial (--only) run.
    import re as _re
    for path in sorted(HERE.glob("sg*.cif")):
        m = _re.fullmatch(r"sg(\d{3})_(\d+)\.cif", path.name)
        if not m:
            continue
        sg = int(m.group(1))
        key = str(sg)
        if key not in old:
            old[key] = {"sg": sg, "id": int(m.group(2)),
                       "file": path.name, "adopted": True}

    groups = sorted({int(g) for g in old} | (only or set(range(1, 231))))
    if args.force:
        old = {}
        old_gaps = []

    done, gaps = {}, []
    with concurrent.futures.ThreadPoolExecutor(max_workers=WORKERS) as pool:
        futs = {pool.submit(process_group, sg, args.force, old): sg
                for sg in groups}
        for fut in concurrent.futures.as_completed(futs):
            sg = futs[fut]
            try:
                entry = fut.result()
            except Exception as e:
                print(f"  sg {sg:3d}: FETCH FAILED: {e}", file=sys.stderr)
                continue
            if entry.get("gap"):
                gaps.append(sg)
            else:
                done[str(sg)] = entry

    all_groups = sorted(
        (v for v in {**old, **done}.values() if not v.get("gap")),
        key=lambda e: e["sg"],
    )
    manifest = {
        "source": f"{BASE}/",
        "api_doc": "https://wiki.crystallography.net/RESTful_API/",
        "fetched": time.strftime("%Y-%m-%d"),
        "note": ("one entry per three-dimensional space group 1..230; "
                 "see tests/real_data/cod_sg/SOURCE.md"),
        "groups": all_groups,
        "gaps": sorted(set(gaps) | set(old_gaps)),
    }
    if not args.only:
        MANIFEST.write_text(json.dumps(manifest, indent=1) + "\n")
    print(f"groups processed this run: {len(done)}, "
          f"gaps: {sorted(gaps) or 'none'}")


if __name__ == "__main__":
    main()
