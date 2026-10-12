# Changelog

Notable changes to this project. The format follows
Keep a Changelog.

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
