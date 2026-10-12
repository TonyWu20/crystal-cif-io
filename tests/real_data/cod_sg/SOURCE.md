# Source of the space-group corpus

The files in this directory hold one COD entry per
three-dimensional space group (groups 1..230, 230 files, ~1 MB).

- fetched: 2026-10-12, via `python3 tests/real_data/cod_sg/fetch.py`
- API doc: `https://wiki.crystallography.net/RESTful_API/`
- search used: `space_group_number=<N>` with `format=lst` and
  `format=json` on the search endpoint
  `https://www.crystallography.net/cod/result`
- entry URL pattern: `https://www.crystallography.net/cod/<id>.cif`

Selection rule: the first candidate from the `lst` answer whose
CIF is at most 150 KB. If all of the first 8 candidates are
oversized, the smallest of them is kept. `manifest.json` records
the entry id, group number, H-M symbol, Hall symbol, formula,
flags, byte size, URL, and the top 5 candidate ids per group.

Rerun notes:

- `python3 tests/real_data/cod_sg/fetch.py` resumes from the
  manifest and fills only missing groups.
- `--force` refetches every group.
- `--only 1-50` limits the run to a range; a partial run adopts
  already-downloaded files on the next full run.

Terms and attribution: 226 files carry the public-domain notice.
4 files (COD entries 1000001, 2013563, 2232341, 1513328) carry
the IUCr Journals notice. That notice requires attribution to the
source article. The citations and the repo-level credit live in
`NOTICE.md`. The API notes live in `docs/cod-api.md`. The files
are used here as parser regression inputs.

The design and the test that pins this corpus live in
`docs/test-axes.md` (`space_group_coverage` in
`tests/space_groups.rs`).
