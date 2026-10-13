# Changelog

Notable changes to this project. The format follows
Keep a Changelog.

## Unreleased

### Changed

- Split the workspace into two crates (roadmap D13).
  `crystal-cif-fmt` (new, 0.1.0) owns the parser, the IR value
  types, the term materialization traits, and the canonical JSON.
  `crystal-cif-io` (0.3.0) keeps `data_dict` and `builder` and
  depends on `crystal-cif-fmt` by path.
- Breaking: `parse_cif`, `rich_error`, `CifDialect`, and all IR
  types (`Tag`, `Value`, `DataBlock`, `DataItems`, `SaveFrame`,
  `Matrix`, the numeric and string value types) moved from
  `crystal-cif-io` to `crystal-cif-fmt`. The term traits
  (`CifTerm`, `SingleValueTerm`, `LoopValueTerm`) now live in
  `crystal-cif-fmt`; `crystal_cif_io::data_dict` re-exports them.
- The test corpora moved with their crate. `tests/real_data` and
  `tests/fixtures` live with the parser crate. The dictionary
  inventories and the pinned `.dic` files live with the io crate.
  The CI fetch step and `codegen/extract_depends.py` use the new
  paths. `codegen/depends.json` regenerated over the new paths.

### Added

- A COMCIFS `cif_api` test-data corpus in
  `tests/real_data/cif_api/`: the 21 edge-case CIFs of the
  upstream reference implementation, pinned to commit `7882c12`.
  The CIFs are fetched at test time, not committed: the
  reproducible `tests/real_data/cif_api/fetch.py` (with a
  `manifest.json` pin) downloads them and verifies the sha256
  and git blob SHA of each file. The CI workflow
  (`.github/workflows/ci.yml`) runs the fetch before `cargo
  test`. Fourteen files pin the parse output. Seven more start
  as `known_gap` pins that record the 2.0 grammar gaps they
  expose (tables, comments inside lists, triple-quoted strings,
  nested save frames, non-ASCII values, VT/FF line terminators).
  The D9 entry below parses all 21. No `known_gap` pin remains.
  Provenance, license, and per-file notes live in
  `tests/real_data/cif_api/SOURCE.md`.
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
- CIF 2.0 syntax completion (roadmap decision D9,
  `docs/roadmap-dict-items.md` section 14). The parser accepts
  the 2.0 syntax that the `cif_api` corpus pins:
  - 2.0 table values: a `Value::Table` variant and the
    canonical `table` value kind.
  - 2.0 triple-quoted strings: the `'''` and `"""` forms,
    multiline.
  - 2.0 nested save frames: a save frame may hold save frames.
  - 2.0 list refinements: a comment between list values and a
    `;` text field as a list member.
  - 2.0 Unicode: non-ASCII in block codes, frame codes, item
    names, values, and comments. The 1.1 dialect keeps the
    ASCII set.
  - VT/FF line terminators in every dialect (decision D9a, a
    recorded 1.1 deviation).
  All 21 `cif_api` files parse. The seven `known_gap` pins
  flipped to full IR pins. Fixtures 21-26 plus five negative
  fixtures pin the new rules.

### Changed

- The `tests/real_data/cif_api/*.cif` files are no longer
  committed; they are fetched by `tests/real_data/cif_api/fetch.py`
  (CI runs it in `.github/workflows/ci.yml`). The expected JSON
  pins in `tests/real_data/expected/` stay committed. The
  `real_data_verify` harness prints a note and skips a corpus
  directory that holds no `.cif` files.

- The five IUCr real-data expected files (`example_single`,
  `example_multiple`, `example_shelxl2013`, `example_olex2`,
  `example_extra_table`) are upgraded from recorded parser
  rejections to full canonical-IR pins. The parser now parses
  those files, so the old records were stale; the generator
  rewrote them. As with all real-data pins, they are a
  self-consistency pin, not an independent ground truth.

- `CifDialect::detect` accepts both spellings of the CIF 2.0
  version code: the IUCr `##CIF_2.0` and the escaped
  `#\#CIF_2.0` used by the COMCIFS `cif_api` reference
  implementation. The thirteen `cif_api` files that carry the
  escaped spelling now parse under the 2.0 value rules. Their
  expected files moved to schema version 2. Six of those files
  still fail on 2.0-only syntax at this step (tables, a comment
  inside a list, triple-quoted strings, nested save frames,
  non-ASCII values). The D9 entry below parses all of them. It
  removes every `known_gap` pin.


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
