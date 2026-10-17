# Dictionary codegen step (roadmap decision D4)

The codegen step generates the typed category modules under
`src/data_dict/core_cif/<category>/` for the bulk item tiers (T4–T6,
roadmap `docs/roadmap-dict-items.md` section 6). It landed in T4 and
is reused by T5 (the `diffrn` categories) as-is.

The step has two inputs and a small set of outputs:

| input | role |
|---|---|
| `tests/dict_inventory.json` | the recorded item list (schema 2). It drives which items exist per category, their value class (`char`/`numb`), the loop flag, the `_enumeration` metadata, and the per-item `_definition`/`_units`/`_units_detail` doc text |
| `codegen/annotations.json` | the per-item annotations. They drive the output: the Rust value type, closed code sets, range checks, name overrides, and notes |

The annotations are JSON, not TOML: the crate carries `serde_json`
already, and the mission constraint is no new external dependencies.
A TOML parser is not available in the dependency set.

## Invocation

```text
cargo run --example dict_codegen
```

The generator runs from the crate root (it resolves
`CARGO_MANIFEST_DIR`). It validates the annotations against the
inventory first, then writes. On a validation error it prints every
problem and exits non-zero without writing anything.

## The annotation format

`codegen/annotations.json` holds one table per annotated category.
The key is the item name with the category prefix removed, exactly as
recorded in the inventory (the inventory item name minus
`<category>_<suffix>`):

```json
{
  "categories": {
    "citation": {
      "year": { "type": "numeric" },
      "coordinate_linkage": {
        "type": "closed",
        "values": ["no", "n", "yes", "y"],
        "note": "closed code set recorded in the dictionary _enumeration"
      }
    },
    "publ_body": {
      "element": { "type": "closed", "values": ["section", "subsection"] }
    }
  }
}
```

One entry per inventory item of the category. Omitting an item is a
generator error, so the file stays an explicit record of every
generated item.

Per-item fields:

| field | required | meaning |
|---|---|---|
| `type` | yes | `string`, `numeric`, `closed`, or `range` |
| `values` | for `closed` | the closed codes, in the order the dictionary records them |
| `min`, `max` | for `range` | the write-time clamp bounds, `min` < `max` |
| `variant` | no | override the generated variant name |
| `rust` | no | override the generated type name (`closed`/`range`) |
| `note` | no | emitted as a doc comment on the variant and the type |

Value-type semantics:

- `string` maps to `CharString`. The item must be a `char` item.
- `numeric` maps to `Numeric`. The item must be a `numb` item.
- `closed` maps to a generated typed enum with one variant per code.
  The codes must match the inventory `_enumeration` when the
  inventory records one; the generator fails on a mismatch.
- `range` maps to a generated struct that clamps in `new`, the
  write-time check (the `cell::CellAngle` pattern). The item must be
  a `numb` item.

Drift checks that never block the run:

- a plain `string`/`numeric` annotation on an item whose inventory
  record carries a closed or range enumeration prints a warning;
- a `closed` annotation on an item with no inventory enumeration
  prints a dictionary-drift warning.

## What the generator writes

Per annotated category:

- `src/data_dict/core_cif/<category>/mod.rs` — the category module:
  the single-value enum (`<Category>Terms`, when the category has
  single-value items) and/or the loop enum
  (`<Category>LoopItem`, when it has loop items), each with the
  `CifTerm`, `SingleValueTerm`/`LoopValueTerm`, and (single-value
  only) `Display` impls, following
  `src/data_dict/core_cif/cell/mod.rs`;
- one file per `closed` or `range` item,
  `src/data_dict/core_cif/<category>/<type_snake>.rs`: a typed enum
  (Display plus `From` into `UnquotedString`/`CharString`/`Value`)
  or a clamping `Float` wrapper (Display plus `From` into
  `Value`/`f64`/`f32`), re-exported from the module.

These files are overwritten on every run. The content is a pure
function of the two inputs: categories and items sort
alphabetically, and the match-arm line breaking mirrors `cargo fmt`
at the default 100-column width.

## What is append-only

- `src/data_dict/core_cif/mod.rs` — the `pub mod <category>;`
  lines. The generator inserts missing lines in sorted position and
  never removes or reorders existing lines. The file may hold only
  `pub mod` lines; anything else aborts the run.
- `src/data_dict/tags.rs` — the central tag registry. The generator
  rewrites the file from the existing sections plus the annotated
  categories, in sorted order: the TAGS sections, the `ALL` list,
  and the two registry tests. The tag lists grow only; nothing
  registered before is removed or reordered. Hand edits to the
  registry are not expected: it is generator-owned.

## Doc comments (inventory schema 2, roadmap D18)

The inventory records two text fields per item: the DDL1
`_definition` and the `_units`/`_units_detail` pair.
`dict_inventory_gen` records them. A regeneration carries the
recorded `_enumeration` values forward, so the curation pin of
`tests/dict_inventory.json` (the hand-pinned recorded
enumerations) survives a rerun.

The generator turns the recorded text into doc comments in four
places:

| place | source |
|---|---|
| the type files (`codes.rs`, `ranges.rs`) | the item `_definition` on the enum or struct header, the annotation `note` below it |
| the `mod.rs` term enums | the `The `_<item>` item.` line plus the `_definition` and `_units` on each variant |
| the `TAGS` constants in `tags.rs` | the item `_definition` on each constant |
| the category `mod.rs` header | one module doc per category, plus the `Terms`/`LoopItem` enum headers |

The inject step docs the hand-maintained files (the struct and
row files the generator does not emit):

- the category struct and row struct headers;
- the `Terms`/`LoopItem` enum headers of the annotated
  categories;
- one doc per struct field, from the item record.

The field key resolution: the field name (the `.`, `-`, and `/`
characters folded to `_`, a trailing `_` stripped) matches the
item names of the core and shell inventories. The `_su` items
are excluded (decision D2: the base item carries the value).
The names dedup across the two sources. The struct fields scope
to their own category. The term enum variants scope to the
category file. A key that resolves to exactly one item gets
docs. A key that resolves to zero items or to two or more
stays untouched and prints to stdout. The printed list is the
drift report.

The DDL1 text is escaped for rustdoc before the wrap: angle
brackets become HTML entities (`&lt;I&gt;`). A bare `http://`
URL becomes a backtick span. The definitions are data, not the
crate's markdown.

Gates: `#![warn(missing_docs)]` at the root of both crates.
The check is `RUSTFLAGS="-D missing_docs" cargo check
--workspace`. The codegen no-write gate (`--no-write`, zero
`would-write` lines) keeps the emitted docs in step with the
inventory on every run. `cargo doc --workspace --no-deps`
reports zero doc warnings.

## Reuse path for T5

1. Add one table to `codegen/annotations.json` per new category
   (the `diffrn` and `diffrn_*` families), with one entry per
   inventory item: `type` for every item, `values` for each closed
   code set the dictionary records, `min`/`max` where the item gets
   a write-time range check.
2. `cargo run --example dict_codegen`. The generator writes the
   category modules, inserts the `pub mod` lines, and grows the tag
   registry.
3. `cargo test` (the inventory test maps every registry tag to the
   recorded items) and `cargo run --example dict_report` (the T5
   categories must reach full coverage).
4. Commit the annotation tables, the generated modules, and the
   registry growth in one change, as T4 did.

A rerun over unchanged inputs writes nothing: every output is
deterministic, so the second run reports `unchanged` for each file
and `git diff` stays clean.

## Verification notes

- The generated code is rustfmt-stable at the current toolchain; a
  `cargo fmt --check` after a run should not touch the generated
  files. Pre-existing `cargo fmt --check` diffs in unrelated files
  (`src/parser`, `src/canonical`, `space_group`) come from a newer
  rustfmt than the repo's last format pass and are out of tier
  scope.
- The registry tests (`tags::tests`) pin the section counts and the
  `ALL` total; the inventory test pins every registry tag to the
  recorded dictionary items.

## The T6 step (dual inventory, matrix kind, drift table)

T6 (the shell-dictionary tier) extends the same step with three
additions. All of them are inert when
`tests/dict_inventory_shell.json` is absent: a run over the standard
inventory alone stays byte-identical to the T4/T5 behavior.

1. **The dual-inventory union.** The generator loads
   `tests/dict_inventory_shell.json` when the file exists. The item
   set is the standard inventory plus the shell items whose
   lowercased name is not in the standard inventory. Two naming
   conventions:

   - categories that the shell adds or extends are written
     lowercased (for example `reflns_class`); the standard
     inventory is authoritative for the case of the categories it
     records;
   - the `_su` items are excluded from the generated module and the
     registry (decision D2: the uncertainty stays in the plain
     `Numeric` value; the shell inventory records the skipped set in
     `counts` and `items`).

2. **The `matrix` annotation kind.** The annotation table adds
   `matrix` as a value class, written as `("matrix", "numb")`: the
   item must be `numb`, the Rust payload is the `Matrix` type from
   `crate::grammar`, and no per-item type file is generated (the
   `Matrix` type is shared). The writer binds the value through
   `crate::data_dict::matrix::to_value` (the nested CIF 2.0 row
   lists) or `Matrix::from_list` with the recorded dimension for
   the numbered-family spelling. Two exceptions stay `numeric` with
   a doc note: `refln_F_complex` (no Complex payload yet, roadmap
   section 9) and `refln_form_factor_table` (no Table payload yet).

3. **The drift table.** `codegen/annotations.json` may hold a
   `drift` object: legacy registry tag strings, per category, for
   items the recorded dictionaries no longer define. The generator
   keeps them in the regenerated `tags.rs` sections. The recorded
   drift today is `space_group_symop_sg_id` (the DDL1 changelog
   records the removal; `space_group_id` stays a section-level tag
   of the hand-maintained `space_group` module).

Hand-maintained categories (the T0–T3 modules) are never annotated:
the generator would overwrite their custom types. Their T6 items are
added by hand to the module and to the existing `tags.rs` section;
the generator preserves the hand-maintained sections on every run.

The T6 verification set:

- `cargo test --test dict_inventory_shell` — the recorded counts,
  the tag coverage of the implemented T6 items, the `_su` exclusion,
  and the no-extras check (the drift table is the only allowed
  exception);
- `cargo run --example dict_report -- --shell` — the per-category
  T6 table (total, `_su` skipped, implemented, missing); the gate
  passes at zero missing;
- `tests/fixtures/18_shell.cif` and its generated expected JSON
  (the shell items in the CIF 1.1 form, with the classic numbered
  matrix family of the `atom_sites` category).
