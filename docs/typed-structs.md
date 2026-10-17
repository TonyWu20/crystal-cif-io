# Typed Structs For Entries (D12)

This document designs the typed document layer for the crate.
The IR stays `Tag` + `Value`. The layer adds typed structs grouped
by dictionary category. It mirrors the grouping in
`castep_cell_io` (the sister crate). Required and optional items
map to `T` and `Option<T>`. The `bon` builders in `src/builder`
become the write-side factories of the layer.

## 1. The problem

The public tour (`README.md`, the API tour) shows the current read
path:

```rust
let v = block.find_single_value_by_tag("cell_length_a")?;
let a = v.value().as_numeric().expect("a number");
```

Three costs:

| cost | cause |
|---|---|
| `as_*` plus `unwrap` / `expect` on every value | `Value` is an enum, not a type |
| the user must know the value class of the item | no typed field names the class |
| edit and write need the term enum or a builder | no plain data struct to mutate |

The `castep_cell_io` crate solves the same class of problem. It
groups its structs by parameter category. The user accesses one
`Option` field per keyword. This document ports that shape to CIF.

## 2. The `castep_cell_io` model

| `castep_cell_io` element | shape | CIF counterpart |
|---|---|---|
| keyword leaf (`Comment(pub String)` + `FromKeyValue` / `ToCell`) | one type per keyword, typed value | the value types already exist: `Numeric`, `CharString`, `Matrix`, the closed-set enums in `data_dict` |
| group struct (`GeneralParams { task: Option<Task>, ... }`) | named `Option` fields, `Default`, `bon` `Builder` | one struct per dictionary category in a new `src/doc` module |
| document struct (`ParamDocument`) | one group field, `from_cell_file` / `to_cell_file` | `CoreCif`, one typed view of one data block |
| file parse (`parse::<ParamDocument>`) | whole-file struct parse | `CoreCif::from_block` |
| inter-group `validate()` | mutual-exclusion checks | `CoreCif::validate()` for the required sets |

Two structural differences decide the rest of the design:

1. A CIF data block carries tags of any dictionary. A mmCIF or a
   custom tag is legal next to a core item. The view must keep a
   residue, or the round trip loses data. CASTEP files have no
   such case.
2. A data block holds loops and save frames. A loop category maps
   to a row struct plus a `Vec`. A save frame maps to the residue
   in wave 1.

## 3. Crate layout and module placement

The user decision (2026-10-17) ports the castep layout. The repo
splits into two crates, mirroring `castep_cell_fmt` +
`castep_cell_io`:

| crate | role | mirror |
|---|---|---|
| `crystal-cif-fmt` | parser and IR level: the value types, blocks, loops, frames, the chumsky parser, canonical JSON | `castep_cell_fmt` |
| `crystal-cif-io` | data level: `data_dict` (terms, closed sets, entry and category structs), `builder`, the block view | `castep_cell_io` |

The repo root becomes the workspace. `crystal-cif-io` depends on
`crystal-cif-fmt` by path. Section 10 carries the split detail
(D13).

The category structs live in the existing category tree, not a
new module. The tree already carries per-entry files.
`cell/angle.rs` holds `CellAngle`.
`diffrn_radiation_wavelength/diffrn_radiation_wavelength.rs`
holds the clamped value type. The new structs follow the same
rule:

| path (in the `io` crate) | type | category |
|---|---|---|
| `data_dict/core_cif/cell/cell.rs` | `Cell` | `cell` (single-value) |
| `data_dict/core_cif/space_group/space_group.rs` | `SpaceGroup` | `space_group` (single-value) |
| `data_dict/core_cif/space_group_symop/space_group_symop.rs` | `Symop` | `space_group_symop` (loop row) |
| `data_dict/core_cif/atom_site/atom_site.rs` | `AtomSite` | `atom_site` (loop row) |
| `data_dict/core_cif/chemical_formula/chemical_formula.rs` | `ChemicalFormula` | `chemical_formula` (single-value) |
| `data_dict/core_cif/mod.rs` | the `CifCategory` trait, the residue rules | the layer |
| crate root (`core_cif.rs`) | `CoreCif`, `CifError` | the block view |

Per-entry value types keep their entry files. A closed set keeps
its enum (`MethodPrecession`). A range-checked item keeps its
clamped newtype (`CellAngle`). No new per-entry newtype is added
for a plain `Numeric` or `CharString` item. The category struct
names the value on the field, and that is where the type safety
sits. This is the castep difference: CASTEP keywords earn a
newtype each because their shapes differ. CIF entries share a
value class, so the field name carries the meaning.

The newtype convention is confirmed by the user (2026-10-17).
An entry newtype wraps the spec-level value type and carries the
domain guard. `CellAngle` wraps `Float` and clamps to `0:180`.
Code that holds a `CellAngle` cannot carry an out-of-range
float. A raw `Float`, `f32`, or `f64` is refused at the call
site. Closed-set items earn the typed enum. Ranged items earn
the clamped newtype. Plain items keep the shared value type,
because a guardless wrapper adds a name but no guard. This is
the same rule `castep_cell_io` applies to its keywords.

Name note: `builder::cell::Cell` and the new category struct
share a name. Section 6 resolves it: the category struct becomes
the builder output, and the builder-local struct goes away. The
crate root re-exports the category structs, the castep pattern
(`castep_cell_io::ParamDocument`).

## 4. Struct shape

Field roles follow the spec table in `docs/dict-completeness.md`
section 5.

| role | struct field | example |
|---|---|---|
| required | `Option<T>` (parse is tolerant, `validate()` reports the gap) | `AtomSite::label` |
| optional | `Option<T>` | `ChemicalFormula::moiety` |
| derived | `Option<T>`, the builder computes and cross-checks it | `Cell::volume` |
| su twin | `Option<Numeric>`, paired with the base field | `AtomSite::fract_x_su` |
| closed set | the typed enum from `data_dict` | `AtomSite::adp_type` |
| loop column | a row struct, held as `Vec<Row>` in `CoreCif` | `AtomSite` |

Wave 1 field sketch:

```rust
pub struct Cell {
    pub length_a: Option<Numeric>,
    pub length_b: Option<Numeric>,
    pub length_c: Option<Numeric>,
    pub angle_alpha: Option<CellAngle>,
    pub angle_beta: Option<CellAngle>,
    pub angle_gamma: Option<CellAngle>,
    pub volume: Option<Numeric>,
    pub formula_units_z: Option<Numeric>,
    pub measurement_temperature: Option<Numeric>,
    // ... the remaining 45 items of the category
}

pub struct AtomSite {
    pub label: Option<CharString>,
    pub type_symbol: Option<CharString>,
    pub fract_x: Option<Numeric>,
    pub fract_y: Option<Numeric>,
    pub fract_z: Option<Numeric>,
    pub occupancy: Option<Numeric>,
    pub adp_type: Option<AdpType>,
    // ... the remaining columns of the main loop
}

pub struct CoreCif {
    pub cell: Cell,
    pub space_group: SpaceGroup,
    pub space_group_symop: Vec<Symop>,
    pub atom_site: Vec<AtomSite>,
    pub chemical_formula: ChemicalFormula,
    /// Items no typed category consumed, plus save frames.
    pub extra: Vec<DataBlockMember>,
}
```

All fields are public. Every group is present by default
(`Default`), with every field `None`. The user edits one field
and writes the block back. No group is `Option`: an absent
category is a group of `None`s. This matches the `castep_cell_io`
access depth (`doc.general.task`).

## 5. Conversion

One trait per category. `from_members` consumes only the tags of
its own category. It returns the residue.

```rust
pub trait CifCategory: Default + Sized {
    /// Emit the set members. `None` fields emit nothing.
    fn to_members(&self) -> Vec<DataBlockMember>;

    /// Fill the fields from the members.
    /// Returns the unconsumed members (foreign tags, frames).
    fn from_members(
        &mut self,
        members: &[DataBlockMember],
    ) -> Result<Vec<DataBlockMember>, CifError>;
}
```

Block level:

```rust
impl CoreCif {
    /// Read one data block into the typed view.
    /// Every mismatched value class of a typed item fails the
    /// call. Foreign tags land in `extra`.
    pub fn from_block(block: &DataBlock) -> Result<CoreCif, CifError>;

    /// Write the view back. Typed items first, then `extra`.
    pub fn to_block(&self, heading: &str) -> DataBlock;

    /// Check the wave 1 required sets.
    /// Reports every missing required item, not just the first.
    pub fn validate(&self) -> Result<(), CifError>;
}
```

Error type:

```rust
pub enum CifError {
    /// A typed item carries a value class the dictionary denies.
    ValueClass {
        tag: String,
        expected: &'static str,
        found: &'static str,
    },
    /// A matrix item carries the wrong shape.
    Shape {
        tag: String,
        expected: (usize, usize),
        found: (usize, usize),
    },
    /// A required item is absent (from `validate()`).
    Missing {
        category: &'static str,
        item: &'static str,
    },
}
```

`from_block` collects every `ValueClass` and `Shape` failure. It
never stops at the first one. A file with ten broken items
reports ten.

### Round-trip guarantee

`from_block` then `to_block` then `parse_cif` preserves every
item. Typed items may reorder. `extra` items keep their order and
bytes. The guarantee is IR round-trip, not byte round-trip. The
groundtruth fixtures pin it.

### Save frames

Wave 1 puts every save frame in `extra`. Loop rows that carry
frames (for example `atom_sites_cartn_transform`) stay whole in
`extra`. Modeling frames is a later decision (D12e).

### Trait placement

The `CifCategory` trait and `CifError` live in the `io` crate, in
`data_dict/core_cif`. They name only `fmt` crate IR types, so no
dependency cycle forms. A strict castep mirror puts the trait in
the fmt crate. That would force `CifError` into the parser crate
and carry dictionary semantics with it.

## 6. Builders

The category struct is the builder output. The current
`builder::cell::Cell` (private fields, accessor methods,
`recorded_items`) is replaced by the category struct.

| builder | today's output | new output |
|---|---|---|
| `CellBuilder`, `CellVectorsBuilder` | `builder::cell::Cell` | `core_cif::cell::Cell` |
| `SpaceGroupBuilder` | `builder::space_group::SpaceGroup` | `core_cif::space_group::SpaceGroup` |
| `SymopBuilder` | `SymopSet` | `Vec<Symop>` |
| `ChemicalFormulaBuilder` | `builder::chemical_formula::ChemicalFormula` | `core_cif::chemical_formula::ChemicalFormula` |
| `FormulaUnits` (structure level) | `FormulaUnits` | unchanged, consumes the category structs |

The builder entry points keep their home in the `builder`
module of the `io` crate.
`Cell::builder()` style conveniences can follow. The pre-1.0
breaking change is recorded in `CHANGELOG.md`, matching the
`MatrixError` precedent.

`validate()` uses the required sets of the five spec tables in
`docs/dict-completeness.md` section 5. Expansion categories need
the same marks. They come from `codegen/annotations.json` (D4a),
one entry per required item.

## 7. Scope

| wave | categories | items | note |
|---|---|---|---|
| D12 wave 1 | `cell`, `space_group`, `space_group_symop`, `atom_site`, `chemical_formula` | ~164 | the D11 five, the reviewed spec tables |
| D12 wave 2+ | the remaining 57 standard categories (T2 to T5) | ~568 | a codegen target over `depends.json` plus annotations |
| shell categories | 29 categories the shell adds | out of scope per D12i. Reopen only if the shell scope is wanted |

The five categories of wave 1 are the rollout slice, not the
bound of the layer. The scope is the IUCr category view of the
CIF 1.1 standard dictionary: 732 items in 62 categories (D12i).
Categories fall into three shapes: a single-value struct (11),
a loop row struct held in a `Vec` (40), and a mix of both (11).
Wave 1 proves the shape on the reviewed set. The codegen target
carries the remaining 57. The rollout order is open (D12j).

Wave 1 leaves the `atom_site_aniso` and
`atom_sites_cartn_transform` loops in `extra`. They join wave 2.

## 8. Tests and docs

1. One round-trip test per wave 1 category, over the groundtruth
   fixtures (`iucr`, `cod`, `real_data`).
2. A required-set test per category: `validate()` on a block with
   one required item removed reports exactly that item.
3. A residue test: a block with a mmCIF tag and a save frame
   keeps both in `extra`, unchanged.
4. A builder test: each builder output is a doc struct. Build,
   write, read, and the fields match.
5. The README tour gains a typed example. The `as_*` tour moves
   to the IR appendix.

## 9. Decisions

The user confirmed the layer form on 2026-10-17 (this session):
the castep-style per-category structs, the `Option<T>` mapping,
and the builder handoff. `D12b-D12h` confirm option A.
`D12a` resolves as B: the structs live in the category tree,
and the crate split (D13, section 10) follows. The open points
below need a call.

| id | question | options | recommendation |
|---|---|---|---|
| D12a | where the structs live | A: a new `src/doc` module. B: inside `data_dict/core_cif/<cat>` | **B (user, 2026-10-17)**. The entry files keep the per-entry value types. The category struct takes the `<cat>/<cat>.rs` name. The crate split of D13 carries the layer |
| D12b | required vs optional fields | A: all fields `Option<T>`, `validate()` reports missing required items. B: required fields are plain `T`, `from_block` fails on a missing required item | A. Matches `castep_cell_io` and the D10-D11 builder policy. Tolerant reads, strict checks |
| D12c | group presence | A: groups always present, `Default`, fields `Option`. B: group fields are `Option<Group>` | A. One level of `Option`, the castep access depth |
| D12d | value-class mismatch of a typed item | A: `from_block` fails, listing every mismatch. B: the item falls into `extra` with a note | A. A broken core item is a conformance error. Foreign tags never error. Wave-2 addendum (D12k): the items that carry text fields use the `read_char_text` reader (the T4 convention). `read_char` stays strict. The two T3 items the corpus shows as text fields, `chemical_name_systematic` and `geom_special_details`, route through `read_char_text`. |
| D12e | save frames in wave 1 | A: residue only. B: model the frame rows now | A. Frames are rare. Wave 2 models them |
| D12f | wave 1 scope | A: the five D11 categories. B: start with `cell` only | A. The five spec tables are reviewed. One trait proves the design |
| D12g | the block view name | A: `CoreCif`. B: `CifDataBlock`. C: `DataBlockDoc` | A. It names the dictionary the structs follow |
| D12h | builder outputs | A: the builders return the doc structs. The builder-local structs go. B: the doc structs wrap the builder outputs | A. One struct per category, no shadow copy |
| D13 | the crate split | A: a repo-root workspace, two subdirectory crates. B: two sibling repos | A, the castep mirror. Open detail D13a-D13d in section 10 |
| D12i | the final scope of the struct layer | A: all 91 shell categories (1139 items). B: the 62 standard categories (732 items). C: a domain subset the user names | **B (user, 2026-10-17)**. The IUCr category view of the CIF 1.1 standard dictionary is the scope authority. The official list pins in `crystal_cif_io/tests/dict_categories_official.json`. The `dict_categories_official` test gates drift. Shell-only categories stay out of the struct layer |
| D12j | the rollout order after wave 1 | A: the roadmap tiers T2 to T6. B: the user application domain first. C: the real-data coverage data first | **A (user, 2026-10-17)**. The 57 remaining standard categories proceed in roadmap tier order, tiers 2 through 5. Tier 6 is out of scope per D12i |

### The closed-value gate

The gate `crystal_cif_io/tests/closed_values.rs` proves that each
typed closed-set enum agrees with the recorded closed values of
`tests/dict_inventory.json` plus the marked extensions of
`codegen/closed_value_extensions.json`. A row exists for every closed
item of the pin. The gate found five enums short of their dictionary
codes. The enums now parse every dictionary code. The five are: the
hydrogen treatment codes, `greyish`, the include and observed status
codes, the wavelength determination codes, and `TE`.

## 10. The crate split (D13)

The user decision (2026-10-17) splits the repo into two crates,
mirroring `castep_cell_fmt` + `castep_cell_io`. The parser level
and the data level get their own crates. The repo root becomes
the workspace, exactly as `castep-cell-io` does it (a
`[workspace]`-only root `Cargo.toml`, member crates in
subdirectories).

### 10.1 Layout

```
crystal-cif-io/            (the repo root, [workspace])
├── Cargo.toml            members: crystal_cif_fmt, crystal_cif_io
├── crystal_cif_fmt/      package crystal-cif-fmt, starts at 0.1.0
│   └── src/
│       ├── lib.rs
│       ├── grammar/      current src/grammar (value types, blocks, loops, frames)
│       ├── parser/       current src/parser (chumsky + ariadne, parse_cif, CifDialect)
│       ├── canonical.rs  current src/canonical.rs (canonical JSON, spec-level checks)
│       └── serde/        current src/serde
└── crystal_cif_io/       package crystal-cif-io
    ├── src/
    │   ├── lib.rs        re-exports the category structs (the castep pattern)
    │   ├── core_cif.rs   CoreCif, CifError (the castep param_document.rs seat)
    │   ├── data_dict/    current src/data_dict, plus the category structs
    │   └── builder/      current src/builder
    ├── tests/            dict_inventory*, depends_gen, dict_coverage, space_groups, real_data
    └── examples/         dict_codegen, dict_report, dict_coverage_gen, the *_gen tools
```

The `ground_truth` test and the `gt_generate` example pin the
parser IR. They move to the `fmt` crate with what they test.

### 10.2 Ownership

| today's module | crate | note |
|---|---|---|
| `grammar/*` | fmt | the value layer moves whole: `Tag`, `Value`, `Numeric`, `CharString`, `TextField`, `Matrix`, `Comments` |
| `parser/*` | fmt | `parse_cif`, `rich_error`, `CifDialect` |
| `canonical` | fmt | canonical JSON plus the spec-level checks |
| `serde` | fmt | the IR serde support |
| `data_dict/*` | io | terms, closed sets, the tag registry, plus the new category structs |
| `builder/*` | io | the bon builders; the output becomes the category struct |
| `CoreCif`, `CifError`, `CifCategory` | io | `CoreCif` at the crate root, the trait in `data_dict/core_cif/mod.rs` |

The root `core_cif` module and `data_dict::core_cif` share a
leaf name. The paths differ, so no use site is ambiguous.

### 10.3 Dependencies

| crate | deps |
|---|---|
| `crystal-cif-fmt` | chumsky, ariadne, serde_json (the canonical JSON) |
| `crystal-cif-io` | crystal-cif-fmt (path), bon, elements_rs, crystallographic-group, serde_json (the codegen tools) |

### 10.4 Wave shape

The split is a pure move. No logic change, no re-export shim
(pre-1.0 clean break, the `MatrixError` precedent). The crate
name `crystal-cif-io` stays; its public parse API now sits in
the `fmt` crate. `CHANGELOG.md` records both moves. Acceptance:
`cargo test` green in both crates. The D12 wave 1 then builds on
the stable layout.

Decisions (all confirmed by the user, 2026-10-17, option A on each):

| id | question | outcome |
|---|---|---|
| D13a | the split shape | A: a repo-root workspace, two subdirectory crates. The castep mirror. One repo to review |
| D13b | the fmt crate name | A: `crystal-cif-fmt`. The crate carries the value layer, the parser, and the canonical JSON |
| D13c | timing | A: the split wave first (a pure move), then the D12 wave 1. The wave 1 files land once, not moved twice |
| D13d | the read-side trait home | A: the `io` crate (`data_dict/core_cif`), `CifError` with it. The trait names `CifError` |
