# Changelog

Notable changes to this project. The format follows
Keep a Changelog.

## Unreleased

### Added

- A COMCIFS `cif_api` test-data corpus in
  `tests/real_data/cif_api/`: the 21 edge-case CIFs of the
  upstream reference implementation, pinned to commit `7882c12`
  (provenance, license, and per-file notes in
  `tests/real_data/cif_api/SOURCE.md`). Fourteen files pin the
  parse output; seven are `known_gap` pins that record the
  2.0 grammar gaps they expose (nested values, tables,
  triple-quoted strings, nested save frames, non-ASCII values,
  VT/FF line terminators).
- A Crystallography Open Database (COD) test corpus in
  `tests/real_data/cod/` (six entries, fetched via the COD REST
  API). The real-data harness and the `gt_generate` generator now
  scan both `tests/real_data/iucr/` and `tests/real_data/cod/`.
- `docs/cod-api.md`: the COD REST API summary and notes on the CIF
  file shape the parser meets.
- `NOTICE.md`: third-party data attribution. The COD credit, the
  public-domain and IUCr header notices, the six IUCr-notice
  article citations, and the COD citation page
  (https://wiki.crystallography.net/cod/citing/) with its eight
  recommended papers.
- The multi-axis test-suite design (`docs/test-axes.md`):
  - `tests/real_data/cod_sg/`: one COD entry per space group
    1..230 (230 files, ~1 MB), fetched by the reproducible
    `fetch.py` with a provenance manifest. Pinned by
    `space_group_coverage` in `tests/space_groups.rs`.
  - `tests/feature_coverage.rs`: a lexical/structural feature
    guard over all corpora (13 features, each with at least one
    evidence file).
  - `tests/dict_coverage.rs` plus
    `examples/dict_coverage_gen.rs`: a dictionary-item coverage
    gate with a committed baseline (320 of 732 items at first
    recording).
  - `tests/fixtures/20_utf8_nonascii.cif`: a UTF-8 fixture with
    accented names, a Greek letter, and a hash inside a quoted
    string.

## 0.2.0 (2026-10-09)

The dict-items roadmap is complete. All eight work units are
merged: T0, T2, T3, T4, T5, D8, and T6.

### Added

- The IUCr CIF_CORE value layer. All 734 standard dictionary
  items are typed items.
- The core shell additions (T6): 262 of the 453-item set. The
  191 `_su` items are skipped per decision D2.
- CIF 2.0 support (D8): dialect detection, `Value::List`, the
  2.0 unquoted-value restriction, `Matrix::from_list`.
- The `Matrix` value type with dictionary shape checks (D3).
- The dictionary inventory tests for the standard and shell
  inventories.
- The groundtruth harness with schema versions 1 and 2. Thirty
  fixtures plus eight negative fixtures.
- The coverage report: `cargo run --example dict_report`.
- The codegen step: `examples/dict_codegen.rs` with
  `codegen/annotations.json`.

### Changed

- Breaking: the legacy `data_types` API is removed. The crate is
  pre-1.0 and nothing external used it (decision D6).
- Breaking: `MatrixError` is now an enum.
- The CIF 1.1 parser now accepts unquoted values that begin with
  `[` or `]`. This deviates from 1.1 spec paragraph 19 and is
  recorded in the roadmap section 13.3.
- A leading UTF-8 BOM is now elided for every dialect, not only
  for CIF 2.0 files.

### Unchanged

- The CIF 1.1 fixture expected JSON files are byte-identical to
  the 0.1.x state.

## 0.1.x

No changelog was kept for the 0.1 series. The last version
before this one is 0.1.2.
