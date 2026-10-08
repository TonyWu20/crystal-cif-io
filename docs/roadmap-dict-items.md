# CIF Dictionary Items — Implementation Roadmap

This document plans the implementation of the remaining CIF dictionary
items for the crate. It is the working plan for `src/data_dict`.

## 1. Baseline

The IUCr CIF 1.1 specification defines file syntax and common semantics.
It does not list item names. Item names come from the IUCr core
dictionary. The IUCr publishes two forms of the core dictionary.

| baseline | source | items | categories | note |
|---|---|---|---|---|
| DDL1 core (standard) | `ftp.iucr.org/cifdics/cif_core.dic`, v2.4.5, 2014-11-21 | 734 | 56 | "in active use" |
| core shell (extension) | `github.com/COMCIFS/cif_core`, v3.4.0-dev, DDL 4.2.0 | 1139 | 91 | successor dictionary |

This roadmap takes the DDL1 core dictionary as the standard baseline.
We call this the "v1.1 standard" item set. The core shell adds items.
We treat those additions as the optional final tier (T6).

## 2. Current state

`src/data_dict` models 89 of the 734 standard items.
Five categories are complete.

| category | done | of |
|---|---|---|
| `atom_site` | 54 | 54 |
| `cell` | 22 | 22 |
| `space_group` | 5 | 5 |
| `space_group_symop` | 3 | 3 |
| `audit` | 5 | 5 |

Defects in the current code:

1. 51 `todo!()` sites panic on write. 40 sit in `atom_site`, 8 in
   `cell`, 3 in `audit`.
2. `MeasurementReflnLoopItem` defines four loop items. It lacks a
   `CifTerm` impl, so the items are dead code.
3. `Numeric` stores `f32`. The refactor target is `f64`
   (`docs/ground-truth.md`, known gap 5).
4. `audit_Update_record` deviates from the lowercase dictionary
   spelling.
5. The legacy writer API in `data_dict/data_types` overlaps the
   `CifTerm` path. Nothing in the crate uses it yet.

## 3. Remaining items

The standard dictionary has 645 unimplemented items.
They span 49 categories.
The table below lists every category with its counts.

| category | items | done | missing | loop items | numb items |
|---|---|---|---|---|---|
| `atom_site` | 54 | 54 | 0 | 54 | 29 |
| `atom_sites` | 29 | 0 | 29 | 0 | 24 |
| `atom_type` | 22 | 0 | 22 | 22 | 17 |
| `audit` | 5 | 5 | 0 | 0 | 0 |
| `audit_author` | 2 | 0 | 2 | 2 | 0 |
| `audit_conform` | 3 | 0 | 3 | 0 | 0 |
| `audit_contact_author` | 5 | 0 | 5 | 0 | 0 |
| `audit_link` | 2 | 0 | 2 | 2 | 0 |
| `cell` | 22 | 22 | 0 | 0 | 20 |
| `cell_measurement_refln` | 4 | 0 | 4 | 4 | 4 |
| `chemical` | 25 | 0 | 25 | 0 | 11 |
| `chemical_conn_atom` | 7 | 0 | 7 | 7 | 6 |
| `chemical_conn_bond` | 3 | 0 | 3 | 3 | 2 |
| `chemical_formula` | 7 | 0 | 7 | 0 | 2 |
| `citation` | 26 | 0 | 26 | 26 | 2 |
| `citation_author` | 3 | 0 | 3 | 3 | 0 |
| `citation_editor` | 3 | 0 | 3 | 3 | 0 |
| `computing` | 7 | 0 | 7 | 0 | 0 |
| `database` | 15 | 0 | 15 | 0 | 0 |
| `diffrn` | 12 | 0 | 12 | 0 | 8 |
| `diffrn_attenuator` | 3 | 0 | 3 | 3 | 1 |
| `diffrn_detector` | 7 | 0 | 7 | 0 | 3 |
| `diffrn_measurement` | 6 | 0 | 6 | 0 | 0 |
| `diffrn_orient_matrix` | 10 | 0 | 10 | 0 | 9 |
| `diffrn_orient_refln` | 9 | 0 | 9 | 9 | 9 |
| `diffrn_radiation` | 9 | 0 | 9 | 0 | 4 |
| `diffrn_radiation_wavelength` | 5 | 0 | 5 | 2 | 2 |
| `diffrn_refln` | 33 | 0 | 33 | 33 | 25 |
| `diffrn_reflns` | 29 | 0 | 29 | 0 | 28 |
| `diffrn_reflns_class` | 8 | 0 | 8 | 8 | 6 |
| `diffrn_scale_group` | 2 | 0 | 2 | 2 | 1 |
| `diffrn_source` | 10 | 0 | 10 | 0 | 4 |
| `diffrn_standard_refln` | 4 | 0 | 4 | 4 | 3 |
| `diffrn_standards` | 6 | 0 | 6 | 0 | 6 |
| `exptl` | 9 | 0 | 9 | 0 | 6 |
| `exptl_crystal` | 24 | 0 | 24 | 1 | 13 |
| `exptl_crystal_face` | 8 | 0 | 8 | 8 | 8 |
| `geom` | 1 | 0 | 1 | 0 | 0 |
| `geom_angle` | 8 | 0 | 8 | 8 | 1 |
| `geom_bond` | 8 | 0 | 8 | 8 | 3 |
| `geom_contact` | 6 | 0 | 6 | 6 | 1 |
| `geom_hbond` | 11 | 0 | 11 | 11 | 4 |
| `geom_torsion` | 10 | 0 | 10 | 10 | 1 |
| `journal` | 38 | 0 | 38 | 0 | 0 |
| `journal_index` | 3 | 0 | 3 | 0 | 0 |
| `publ` | 32 | 0 | 32 | 0 | 0 |
| `publ_author` | 6 | 0 | 6 | 0 | 0 |
| `publ_body` | 5 | 0 | 5 | 5 | 0 |
| `publ_manuscript_incl` | 3 | 0 | 3 | 3 | 0 |
| `refine` | 46 | 0 | 46 | 0 | 35 |
| `refine_ls_class` | 8 | 0 | 8 | 8 | 7 |
| `refln` | 31 | 0 | 31 | 31 | 24 |
| `reflns` | 17 | 0 | 17 | 0 | 14 |
| `reflns_class` | 11 | 0 | 11 | 11 | 9 |
| `reflns_scale` | 4 | 0 | 4 | 4 | 3 |
| `reflns_shell` | 23 | 0 | 23 | 23 | 23 |
| `space_group` | 5 | 5 | 0 | 1 | 1 |
| `space_group_symop` | 3 | 3 | 0 | 1 | 1 |
| `symmetry` | 4 | 0 | 4 | 0 | 1 |
| `symmetry_equiv` | 2 | 0 | 2 | 1 | 1 |
| `valence_param` | 9 | 0 | 9 | 9 | 4 |
| `valence_ref` | 2 | 0 | 2 | 2 | 0 |

By value class (the 645 missing items):

| value class | count |
|---|---|
| char, single | 194 |
| char, loop column | 116 |
| numb, single | 169 |
| numb, loop column | 166 |

The dictionary marks 319 items with an `_enumeration` constraint.
A closed code set suits a typed enum. A range suits a write-time
check.

## 4. Tiers

The tiers follow the domain of the data. The suggested order starts
with the data users read most often. The order is open for discussion.

| tier | domain | categories | items |
|---|---|---|---|
| T2 | experiment, reflection, refinement | `exptl*`, `refine*`, `refln*`, `reflns*`, `symmetry*` | 187 |
| T3 | structure and chemistry | `atom_sites`, `atom_type`, `chemical*`, `geom*`, `valence_*` | 148 |
| T4 | metadata and publication | `audit_*`, `citation*`, `publ*`, `journal*`, `computing`, `database` | 153 |
| T5 | raw diffraction | `diffrn*` (15 categories), `cell_measurement_refln` | 157 |
| T6 | core-shell additions (optional) | shell-only items, all categories | 347 |

T6 note: the core shell adds 347 items that the standard dictionary
lacks. It also renames a few standard items. Of the 347, 123 fall in
the areas the crate already covers (`cell`, `space_group*`,
`atom_site*`, `audit*`, `cell_measurement*`). The other 224 reach
the remaining categories. The 123 include 61 SU items, 37 matrix
items, `beta_*` aniso items, Wyckoff items, and generator items.

```mermaid
flowchart LR
    T0[T0: foundation<br/>decisions + infra] --> T2
    T2[T2: exptl/refln/refine<br/>187 items] --> T3
    T3[T3: structure + chemistry<br/>148 items] --> T4
    T4[T4: metadata + publication<br/>153 items] --> T5
    T5[T5: raw diffraction<br/>157 items] --> T6
    T6[T6: core shell extras<br/>347 items, optional]
```

## 5. Design decisions needed

These need a decision before T0 starts. Record each decision in the
log at the end of this document.

| id | decision | options | suggestion |
|---|---|---|---|
| D1 | baseline scope | A: standard 734 only. B: standard 734 + shell 347 | B, in two steps: T2–T5 then T6 |
| D2 | explicit SU items (228 in the shell, 223 unimplemented) | A: model the `_su` items. B: keep the uncertainty inside `Numeric` and skip `_su` | B for now. Reopen when paired-value checks need them |
| D3 | matrix value type (shell items, 66) | A: add a `Matrix` value to the grammar. B: leave to T6 | A, in T0, so T6 stays mechanical |
| D4 | code shape for bulk items | A: hand-written enums per category. B: a codegen step that reads the inventory | A for T2–T3 (335 items), B for T4–T6 (657 items) |
| D5 | `todo!()` panics | A: implement all 51 payloads in T0. B: return a `Result` and defer the payload | A. They sit in finished categories, so the writes must work |
| D6 | legacy `data_types` writer API | A: remove it. B: keep it aligned with `CifTerm` | A. The crate is pre-1.0. Nothing external uses it |
| D7 | `f32` to `f64` for `Numeric` | A: in T0. B: later | A. Ground-truth gap 5 already marks it |

## 6. Phases

### T0 — foundation

No new items. The goal is a safe base and a verification loop.

1. Commit the dictionary inventory as test data.
   Generate it from `cif_core.dic` v2.4.5 (and the shell for T6).
   Add a generator example.
2. Add a test that maps every `CifTerm` tag to the inventory.
   Fail on an unknown tag. Print the per-category coverage.
3. Move `Numeric` from `f32` to `f64` (D7).
4. Add a `Matrix` value to the grammar (D3).
   Extend the canonical JSON schema and the groundtruth fixtures.
5. Implement the 51 `todo!()` payloads (D5).
   Wire `MeasurementReflnLoopItem` into `CifTerm`.
   This closes `cell_measurement_refln` (4 items).
6. Remove or align the `data_types` API (D6).
7. Fix the `audit_Update_record` spelling.

Acceptance: `cargo test` green. Coverage report shows 89 + 4 = 93 of
734 items (13%). Zero `todo!()` in `core_cif`.

### T2 — experiment, reflection, refinement (187 items)

- `exptl` (9), `exptl_crystal` (24), `exptl_crystal_face` (8)
- `refln` (31), `reflns` (17), `reflns_class` (11), `reflns_scale`
  (4), `reflns_shell` (23)
- `refine` (46), `refine_ls_class` (8)
- `symmetry` (4), `symmetry_equiv` (2)

Acceptance: the inventory test passes for these categories.
A new groundtruth fixture `14_reflns.cif` covers the loop form.
The coverage report shows the tiers done.

### T3 — structure and chemistry (148 items)

- `atom_sites` (29), `atom_type` (22)
- `chemical` (25), `chemical_formula` (7), `chemical_conn_atom` (7),
  `chemical_conn_bond` (3)
- `geom` (1), `geom_angle` (8), `geom_bond` (8), `geom_contact` (6),
  `geom_hbond` (11), `geom_torsion` (10)
- `valence_param` (9), `valence_ref` (2)

Acceptance: same as T2. The codegen (D4) lands in this tier if the
bulk work needs it.

### T4 — metadata and publication (153 items)

- `audit_author` (2), `audit_conform` (3), `audit_contact_author` (5),
  `audit_link` (2)
- `citation` (26), `citation_author` (3), `citation_editor` (3)
- `publ` (32), `publ_author` (6), `publ_body` (5),
  `publ_manuscript_incl` (3)
- `journal` (38), `journal_index` (3)
- `computing` (7), `database` (15)

Acceptance: same as T2.

### T5 — raw diffraction (157 items)

- `diffrn` (12) and 14 `diffrn_*` sub-categories (141 items)
- `cell_measurement_refln` ships in T0 with the `CifTerm` wiring.

Acceptance: same as T2. A new fixture `15_diffrn.cif` covers
diffrn loops.

### T6 — core shell additions (347 items, optional)

Start only after D1 approves the shell scope.

- 123 items extend the finished categories: SU items (61 in these
  categories), matrix items (37), `beta_*` aniso items, Wyckoff and
  generator items.
- 224 items reach the other categories (new `refln` items, new
  `refine_ls` items, `model_site`, `atom_type_scat`, the
  `atom_sites` transform matrices, `function`, `display_colour`).

Acceptance: the shell inventory test passes. The standard inventory
test stays green.

## 7. Verification

- `tests/dict_inventory.json` pins the item set per phase.
- The tag-inventory test stops tag-name drift.
- Each tier adds a groundtruth fixture in the canonical schema.
- The coverage report (`cargo run --example dict_report`) gates the
  phase.

## 8. Effort estimate

The estimate assumes the codegen of D4 for T4–T6.

| work | count | estimate |
|---|---|---|
| char single items | 194 | 0.25 h each |
| char loop columns | 116 | 0.5 h each |
| numb single items | 169 | 0.5 h each |
| numb loop columns | 166 | 0.5 h each |
| typed-enum items (closed code sets) | ~50 | 2–4 h each |
| range checks on `_enumeration` items | ~100 | 0.25 h each |
| codegen step (T4–T6) | 1 | 2 days |
| T0 foundation work | 1 | 2–3 days |

Most of T4 and T5 comes from the codegen. The annotation file
drives the output. Annotation work is about 0.1 h per item.

## 9. Out of scope

- Other IUCr dictionaries (mmCIF, neutron, powder, dft, cnsd).
- The derived expressions of the shell (`function` category math).
- Cross-item validation rules beyond the spec checks in
  `src/canonical.rs`.

## 10. Decision log

Record decisions here as they are made.

| id | decision | outcome | date |
|---|---|---|---|
| D1 | baseline scope | open | — |
| D2 | explicit SU items | open | — |
| D3 | matrix value type | open | — |
| D4 | codegen vs hand-written | open | — |
| D5 | `todo!()` strategy | open | — |
| D6 | legacy `data_types` API | open | — |
| D7 | `f32` to `f64` | open | — |

## 11. Sources

- IUCr CIF 1.1 working specification (file syntax, common semantics):
  mirror at `cristal.org/DU-SDPD/nexus/iucr2/iucr-top/cif/spec/version1/`.
- IUCr core CIF dictionary, DDL1 v2.4.5:
  `ftp.iucr.org/cifdics/cif_core.dic`.
- IUCr core dictionary shell v3.4.0-dev:
  `github.com/COMCIFS/cif_core` (`cif_core.dic`).
- Current state of the crate: `src/data_dict`, 51 `todo!()` sites,
  `MeasurementReflnLoopItem` without a `CifTerm` impl.
