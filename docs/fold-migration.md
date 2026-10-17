# Fold migration: one struct per CIF category (mission handover)

This document is the full handover for the fold-migration mission.
The mission session starts from zero context. Read this file first.
The codebase is the source of truth; this file tells you what to do
and where it lives.

## 1. Goal

Today one CIF category can occupy two fields on `CoreCif`: a struct
(the single-value items) plus a separate `_rows` loop vector. The goal
is that every category occupies exactly one field on `CoreCif`. The
category struct owns its loop rows. After the migration, no `CoreCif`
field ends in `_rows` as a sibling of a struct.

## 2. The dual-naming problem (concrete)

`atom_sites` is the canonical example. It is two fields today:

- `atom_sites: AtomSites` — the struct. It holds the scalar items such
  as `cartn_transform_axes`, `solution_hydrogens`, `special_details`.
- `atom_sites_rows: Vec<AtomSitesRow>` — the loop rows. Each row is the
  numbered transform-matrix and transform-vector columns.

Each field has its own `CifCategory` impl. `AtomSites::to_members`
emits the scalar tags. `Vec<AtomSitesRow>::to_members` emits the
`loop_` block. `CoreCif::from_block` threads two `from_members` calls.

## 3. The two category sets

The 24 dual-named categories (struct plus `_rows` vector). These are the
targets of the fold.

| set | categories | how they are built |
|---|---|---|
| wave 1 | `atom_sites`, `atom_type`, `chemical`, `chemical_conn_bond`, `geom_angle`, `geom_bond`, `geom_contact`, `geom_hbond`, `geom_torsion`, `valence_param` | hand-written modules |
| T4–T6 | `diffrn`, `diffrn_orient_matrix`, `diffrn_reflns`, `diffrn_standard_refln`, `audit_author`, `audit_conform`, `citation`, `citation_editor`, `computing`, `database`, `journal`, `journal_index`, `publ`, `publ_author` | codegen-generated |

The 9 bare-loop categories. They already occupy one field each (a bare
`Vec`). They are not dual-named.

`space_group_symop`, `atom_site`, `chemical_conn_atom`,
`cell_measurement_refln`, `exptl_crystal`, `exptl_crystal_face`,
`reflns_scale`, `refine_ls_class`, `symmetry_equiv`.

Default scope: fold the 24 dual-named categories. Leave the 9 bare-loop
categories as bare `Vec` fields. They already satisfy the one-field
invariant. Wrapping them is a separate, larger change. Do not wrap them
in this mission.

## 4. Hand-written vs codegen (what to touch)

The fold touches two code paths. They are different. Do both.

Hand-written wave-1 categories: edit the modules directly.

- `crystal_cif_io/src/data_dict/core_cif/<category>/<category>.rs`
- The struct and the row struct live in that one file.
- `impl CifCategory for <Category>` (scalar) and
  `impl CifCategory for Vec<<Category>Row>` (loop) are both there.

Codegen-generated T4–T6 categories: change the generator, then regenerate.

- The generator is `crystal_cif_io/examples/dict_codegen.rs`.
- Run it with `cargo run --example dict_codegen`.
- It writes `src/data_dict/core_cif/<category>/<type>.rs` and
  `mod.rs`, updates `src/data_dict/core_cif/mod.rs` and
  `src/data_dict/tags.rs`.
- Inputs: `tests/dict_inventory.json` and `codegen/annotations.json`.
- It is idempotent. On a clean tree it reports every file unchanged.
- The generator template is what currently emits the struct plus a
  separate `Vec<Row>` impl. Change the template so the struct owns
  `rows: Vec<Row>` and one `CifCategory` impl covers both.
- Regenerate. The diff shows the fold for every T4–T6 category at once.

The shared assembly is hand-written.

- `CoreCif` and `from_block` and `to_block` are in
  `crystal_cif_io/src/core_cif.rs`.
- `CifCategory` is defined at
  `crystal_cif_io/src/data_dict/core_cif/mod.rs:110`.

## 5. The target shape

For `atom_sites`, after the fold:

```rust
pub struct AtomSites {
    pub cartn_transform_axes: Option<CharString>,
    pub solution_hydrogens: Option<SolutionMethod>,
    // ... the other scalar items
    pub rows: Vec<AtomSitesRow>,
}
```

- `CoreCif` keeps `atom_sites: AtomSites` and drops `atom_sites_rows`.
- One `CifCategory` impl for `AtomSites`. `to_members` emits the scalar
  tags then the loop block. `from_members` routes each incoming member to
  a scalar field or to `rows`.
- The row struct `AtomSitesRow` is unchanged.

The field name inside the struct is `rows`. Do not prefix it with the
category name.

## 6. Open decisions (resolve, do not stall)

The mission may resolve these and record the choice in the decision log.
If a choice is risky or surprising, ping the main session before acting.

1. Rows field name: use `rows`. Default is `rows`.
2. Bare-loop scope: do not wrap the 9 bare-loop categories. Default.
3. If the codegen template change breaks the wave-1 hand-written files
   or the `tags.rs` pin, stop and ping the main session. Do not force it.

## 7. Step-by-step plan

Work in small commits. Keep the tree green at each step.

1. Pick one wave-1 category as the pilot. Use `atom_sites`.
2. Fold the pilot by hand. Add `rows` to the struct. Merge the two
   `CifCategory` impls into one on the struct. Drop the `Vec<Row>`
   impl. Update `CoreCif` to drop the `_rows` field. Update
   `from_block` and `to_block`. Migrate every call site that reads
   `atom_sites_rows` to `atom_sites.rows`.
3. Run the full suite. `cargo test -q --workspace` must be green.
4. Commit the pilot.
5. Roll the same fold out to the remaining wave-1 categories. Commit
   per category or per small group.
6. Change the codegen template for the T4–T6 categories. Regenerate with
   `cargo run --example dict_codegen`. Update `CoreCif` and the call
   sites. Run the full suite.
7. Confirm the generator is idempotent. Run it again. It must report
   every file unchanged.
8. Migrate the README tour and the doctest and the examples.
9. Run the full suite one last time. Record the decision in
   `docs/roadmap-dict-items.md` as the next D number.

## 8. Invariants and acceptance gates

The mission is done when all of these hold.

- `cargo build -q --workspace` is clean with no new warnings.
- `cargo test -q --workspace` is fully green.
- No `CoreCif` field ends in `_rows` for the 24 dual-named categories.
- One `CifCategory` impl per dual-named category.
- `cargo run --example dict_codegen` reports every file unchanged.
- The IR round-trip holds. `from_block` then `to_block` reproduces the
  document. The byte-pinned tests stay byte-identical.
- The README tour and the `lib.rs` doctest compile.
- The 9 bare-loop categories are untouched.

## 9. Where things live

| thing | location |
|---|---|
| `CoreCif`, `from_block`, `to_block` | `crystal_cif_io/src/core_cif.rs` |
| `CifCategory` trait | `crystal_cif_io/src/data_dict/core_cif/mod.rs:110` |
| category modules | `crystal_cif_io/src/data_dict/core_cif/<category>/` |
| codegen generator | `crystal_cif_io/examples/dict_codegen.rs` |
| codegen inputs | `tests/dict_inventory.json`, `codegen/annotations.json` |
| decision log | `docs/roadmap-dict-items.md` |
| tour example | `crystal_cif_io/examples/readme_tour.rs` |
| pinned round-trip tests | `crystal_cif_io/tests/typed_round_trip*.rs` |

## 10. How to finish and report

Commit in small green steps. When the gates in section 8 all hold, the
mission is complete. The orchestrator pings the main session on the
mission exit. The main session reads `sessions/dict-items-fold` for the
result.

## 11. Post-wave correction (recorded after the mission ran)

The wave completed. Two facts in this handover were not confirmed by the
code and are corrected here.

The T4-T6 `CifCategory` layer is hand-written, not codegen-generated.
`dict_codegen` emits only the type files, the `CifTerm` `mod.rs`, and the
registry files. It does not emit the `CifCategory` impl layer. So all 24
categories were folded with the hand-written transform. No codegen template
change was needed or made. Section 4 of this file is superseded on that
point.

The codegen idempotency gate is blocked by pre-existing drift, not by the
fold. The drift is: two annotation lists drift from the union inventory
(`diffrn_radiation_wavelength_determination` lacks `estimated` and
`refined`; `publ_requested_category` lacks `TE`); `update_core_cif_mod`
aborts on the D12 `CifCategory` trait block in `core_cif/mod.rs`; and the
templates predate the D13 split, so a full regeneration rewrites about 160
files with stale imports. This drift is a separate open task. See the D17a
row in `docs/roadmap-dict-items.md`.
