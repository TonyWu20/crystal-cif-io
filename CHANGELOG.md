# Changelog

Notable changes to this project. The format follows
Keep a Changelog.

## 0.2.0 (2026-10-16)

### Added

- Added typed document structs for the T3, T4, and T5 tiers
  (roadmap D12j): the thirteen T3 structure and chemistry
  categories, the sixteen T4 metadata and publication categories,
  and the sixteen T5 raw-diffraction categories. Each category
  keeps its own directory. The row groups type to `Vec` row
  structs. `CoreCif` carries one group per category.
- Added the closed-set parses of the T3, T4, and T5 waves. The
  wave-1 enums gain the dictionary codes. The five T4 closed
  code sets type as enums.
- Added the read-path tolerances the real-data corpora need
  (roadmap D12k, D12l, D12m). The text-field items
  (`chemical_name_systematic`, `geom_special_details`,
  `refine_special_details`) read through `read_char_text`. The
  `<N` assertion of `refine_ls_shift/su_max` records in the
  dedicated less-than field. The `AbsorptCorrectionType` set
  gains the real-data code `semi-empirical` (recorded in D12m).

### Changed

- The union-merged wave merges repaired on `dev-0.2` (the
  dropped delimiters of `lib.rs`, `convert.rs`, `core_cif.rs`,
  and the import block). The post-merge `cargo test` gate
  caught each.

- Added typed document structs for the twelve categories of the T2
  tier (roadmap D12j). The categories: `exptl`, `exptl_crystal`,
  `exptl_crystal_face`, `refln`, `reflns`, `reflns_class`,
  `reflns_scale`, `reflns_shell`, `refine`, `refine_ls_class`,
  `symmetry`, and `symmetry_equiv`.
- The 187 registry items type to `Option` fields on the twelve
  structs. The four loop families type to the nested row structs
  `ExptlAbsorptCorrectionT`, `ReflnsDResolution`, `ReflnsLimit`,
  and `RefineDiffDensity`.
- `CoreCif` carries one group per category. `to_block` emits the
  groups after the wave 1 groups and before the `extra` residue.
- Closed-set items reuse the twelve dictionary enums of the
  category `codes` modules. The crate root re-exports the sixteen
  structs. No new `validate` checks. Untyped tier tags and save
  frames stay in `CoreCif::extra`, order kept.

### Changed

- The `space_group` items `bravais_type`, `centring_type`, and
  `it_coordinate_system_code` type to the closed-set enums
  `BravaisType`, `CentringType`, and `ItCoordinateSystemCode`
  (roadmap D12, the review P2-4 note).
- `CoreCif::validate` now checks the `atom_site` coordinate
  mode and the aniso columns per row (roadmap D12, the review
  P2-1 note).
- Breaking (roadmap D12h): the five `bon` builders now build the
  typed document structs instead of their private ones. The builder
  entry points (`Cell::builder`, `SpaceGroup::builder`,
  `Symop::builder`, `AtomSite::builder`, `ChemicalFormula::builder`)
  return `data_dict::core_cif::cell::Cell`,
  `space_group::SpaceGroup`, `Vec<space_group_symop::Symop>`,
  `Vec<atom_site::AtomSite>`, and
  `chemical_formula::ChemicalFormula`. The builder-private structs
  and their `to_data_items` paths are removed. The `CifCategory`
  trait `to_members` is the materialization path now. `Symop::builder` now takes the resolved `crystallographic_group::SpaceGroup` in place of the removed builder-local `SpaceGroup`.
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

- The typed document layer of tier T4 (roadmap D12, wave 4). The
  sixteen metadata and publication categories gain one `Option`-
  field struct each under `data_dict::core_cif`. Two are
  single-value structs (`audit`, `audit_contact_author`), four are
  loop-row structs (`audit_link`, `citation_author`, `publ_body`,
  `publ_manuscript_incl`), and ten carry both a single struct and
  a row struct.
- `CoreCif` holds the sixteen category groups. `from_block`
  threads the residue through the category `from_members` calls.
  `to_block` emits the T4 field blocks after the wave 1 blocks,
  then the `extra` residue. The new lines carry the `// T4 wave:`
  marker for the parallel tier merges. The crate root re-exports
  the sixteen structs.
- The tier closed-set items type to enums. `CoordinateLinkage`,
  `RequestedCategory`, `Element`, `Format`, and `ExtraDefn` gain
  the `parse` method. Two new enums carry the shell-dictionary
  sets: `audit::Schema` (`_audit_schema`) and
  `journal_index::Type` (`_journal_index_type`). The ranged item
  `journal_paper_pages` reuses the `PaperPages` newtype through a
  recorded-value read.
- Char items of the tier read text-field values into the shared
  `CharString` type. The T4 corpus stores char items as semicolon
  text fields (`_publ_body.contents`, the contact-author items).
  The read helper is `convert::read_char_text`.
- `tests/typed_round_trip_w4.rs`: the tier round-trip over the
  `iucr/example_publcif.cif`, `cod/1000000.cif`, and
  `iucr/example_squeeze.cif` corpus files, the residue test, and
  the `journal_index::Type` closed-set spot check.

- A typed document layer for five core categories (roadmap D12,
  wave 1). The `CifCategory` trait in `data_dict::core_cif` reads
  and writes `DataBlockMember` sets. One `Option`-field struct per
  category: `cell::Cell`, `space_group::SpaceGroup`,
  `space_group_symop::Symop`, `atom_site::AtomSite`, and
  `chemical_formula::ChemicalFormula`. The `CoreCif` view in
  `crystal_cif_io::core_cif` holds the five category groups and the
  `extra` residue; `from_block`, `to_block`, and `validate` move
  between the typed view and a data block. The `CifError` type
  records value-class, shape, and missing-item mismatches.
  Closed-set items type to enums; clamped items keep their newtypes
  (`CellAngle`, `ITNumber`, `SymMultiplicity`, `FormulaWeight`).
  Save frames and foreign tags stay in `extra`, order kept (roadmap
  D12e).
- A typed document layer for the 13 structure and chemistry
  categories of tier T3 (roadmap D12, wave w3):
  `atom_sites`, `atom_type`, `chemical`, `chemical_conn_atom`,
  `chemical_conn_bond`, `geom`, `geom_angle`, `geom_bond`,
  `geom_contact`, `geom_hbond`, `geom_torsion`, `valence_param`,
  and `valence_ref`. Each category gains a struct in
  `data_dict::core_cif` under its directory: 11 single-value
  structs and 12 loop-row structs, all with `Option` fields and
  tolerant reads (roadmap D12b, D12a). Mixed categories hold the
  single struct plus a `Vec` of row structs; the
  `chemical_conn_atom` and `valence_ref` loop categories hold the
  `Vec` directly. `CoreCif` gains 23 category fields and the
  `lib.rs` re-export block follows.
- The T3 closed sets type to the wave enums: `SolutionMethod`
  (12 codes), `BondType` (8 codes), and `PublFlag` (3 codes)
  gain `parse` on their closed values. `AbsoluteConfiguration`
  extends by five codes (`ad`, `dyn`, `rmad`, `syn`, `unk`);
  `EnantioexcessBulkTechnique` and `EnantioexcessCrystalTechnique`
  extend by `other`.
- The range newtypes of the wave-1 entries gain read-path
  constructors (`from_numeric`) for the T3 ranged items:
  `AnalyticalMass`, `NumberInCell`, `OxidationNumber`, `Radius`,
  `Enantioexcess`, `Temperature`, `ConnAtomNumber`,
  `ConnAtomCharge`, `ConnAtomOccupancy`, `ConnAtomDisplacement`,
  `BondAtomNumber`, `BondDistance`, `BondMultiplicity`,
  `ContactDistance`, and `HbondDistance`.
- `convert::read_numeric_list` types the T6 list items of
  `atom_type`, `valence_param`, `chemical_conn_bond`, and the
  `geom_angle` and `geom_torsion` special lists to
  `Option<Vec<Numeric>>`.
- The round-trip tests `tests/typed_round_trip_w3.rs`: the
  fixture and the iucr and cod corpus blocks read, rewrite, and
  re-read with the typed fields and the `extra` residue stable.
  The residue test pins the save-frame order.
- Typed category structs for the 16 raw-diffraction categories of
  tier T5 (roadmap D12, the T5 wave): `cell_measurement_refln`,
  `diffrn`, `diffrn_attenuator`, `diffrn_detector`,
  `diffrn_measurement`, `diffrn_orient_matrix`,
  `diffrn_orient_refln`, `diffrn_radiation`,
  `diffrn_radiation_wavelength`, `diffrn_refln`, `diffrn_reflns`,
  `diffrn_reflns_class`, `diffrn_scale_group`, `diffrn_source`,
  `diffrn_standard_refln`, and `diffrn_standards`. The
  single-value structs and the loop-row structs follow the wave 1
  shape. The `CoreCif` view gains the 16 category groups. The
  ranged items keep the entry newtypes. The closed-set items keep
  the typed enums. The six closed-set enums gain the parse method
  of the adp_type pattern.
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
