# Dictionary codegen step (roadmap decision D4)

The codegen step generates the typed category modules under
`src/data_dict/core_cif/<category>/` for the bulk item tiers (T4–T6,
roadmap `docs/roadmap-dict-items.md` section 6). It landed in T4 and
is reused by T5 (the `diffrn` categories) as-is.

The step has two inputs and a small set of outputs:

| input | role |
|---|---|
| `tests/dict_inventory.json` | the recorded item list. It drives which items exist per category, their value class (`char`/`numb`), the loop flag, and the `_enumeration` metadata |
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
