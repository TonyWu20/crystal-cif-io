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
3. The write path truncates floats. `Number` formats a `Float`
   as `{:<6.3}`, so a round trip loses digits. The parse path
   keeps full `f64` precision (`docs/ground-truth.md` gap 5 is
   closed).
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
| D3 | matrix value type (66 shell items) | A: a `Matrix` value type, no `Value` variant. B: plain `Vec<Vec<Numeric>>` everywhere | A. See section 12 for the design |
| D4 | code shape for bulk items | A: hand-written enums per category. B: a codegen step that reads the inventory | A for T2–T3 (335 items), B for T4–T6 (657 items) |
| D5 | `todo!()` panics | A: implement all 51 payloads in T0. B: return a `Result` and defer the payload | A. They sit in finished categories, so the writes must work |
| D6 | legacy `data_types` writer API | A: remove it. B: keep it aligned with `CifTerm` | A. The crate is pre-1.0. Nothing external uses it |
| D7 | write-path float truncation (`{:<6.3}`) | A: format with the `f64` shortest-round-trip Display in T0. B: keep the truncation | A. The parse path already keeps full precision |
| D8 | CIF 2.0 list values (`[...]`), the carrier for `Matrix` items | A: add dialect detection, a `Value::List` variant, and the 2.0 string restriction. B: keep the crate 1.1-only and skip list parsing | A, scoped to lists only (tables stay out). See section 13 |
| D9 | CIF 2.0 syntax completion: the seven `known_gap` corpus files | A: implement every pinned feature (tables, triple-quoted strings, nested frames, Unicode, VT/FF). B: keep the `known_gap` pins and skip | A, scoped to the seven pinned cases, no new dictionary items. See section 14 |
| D9a | VT/FF as 1.1 line terminators | A: accept both in every dialect, a recorded 1.1 deviation. B: accept in the 2.0 dialect only | A. It follows the bracket-lead precedent (13.3 item 2, fixture 17). Fixture 26 pins it |
| D10 | dictionary completeness: research plus a builder layer | A: research the item dependencies into `docs/dict-completeness.md`, then a per-category builder on the `bon` crate. B: skip the research, hand-pick profiles. C: read-side validation only, no builder | A. The user chose the `bon` builder with compile-time required-field checking. The builder wave (D11) waits for the research review |

## 6. Phases

### T0 — foundation

No new items. The goal is a safe base and a verification loop.

1. Commit the dictionary inventory as test data.
   Generate it from `cif_core.dic` v2.4.5 (and the shell for T6).
   Add a generator example.
2. Add a test that maps every `CifTerm` tag to the inventory.
   Fail on an unknown tag. Print the per-category coverage.
3. Stop the write-path float truncation (D7).
4. Add a `Matrix` value type (D3, section 12).
   No `Value` variant today. Matrix items bind in the
   data-dictionary layer.
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

Start only after D1 approves the shell scope. Reading CIF 2.0
files needs D8 first. Writing 2.0 list values also needs D8.

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
| D1 | baseline scope | B: the standard 734 items now. The 347 shell items stay T6, a later separate wave (two steps: T2–T5, then T6) | 2026-10-08 |
| D2 | explicit SU items | B: keep the uncertainty inside `Numeric`. Skip the `_su` items. Reopen when paired-value checks need them | 2026-10-08 |
| D3 | matrix value type | A: a `Matrix` value type in the value layer, no `Value` variant (section 12) | 2026-10-08 |
| D4 | codegen vs hand-written | A: hand-written for T2–T3. B: a codegen step for T4–T6. The codegen lands in T4 | 2026-10-08 |
| D5 | `todo!()` strategy | A: implement all 51 payloads in T0. Wire `MeasurementReflnLoopItem` into `CifTerm` | 2026-10-08 |
| D6 | legacy `data_types` API | A: remove it. The crate is pre-1.0, nothing external uses it | 2026-10-08 |
| D7 | write-path float truncation | A: format `Float` with the `f64` shortest-round-trip `Display` in T0 | 2026-10-08 |
| D8 | CIF 2.0 list values | A: dialect detection, a `Value::List` variant, the 2.0 string restriction. Scoped to lists only (section 13). The parser work moved up from the T6 wave to its own branch, `dict-items/d8` (2026-10-09). T6 consumes it for the matrix items | 2026-10-08 |
| D4a | codegen implementation | `examples/dict_codegen.rs` with `codegen/annotations.json` (JSON: the generator reuses the existing `serde_json`; no new dependencies). The format and the reuse path are documented in `docs/codegen.md`. The registry and pub-mod lists stay append-only. T4 carries no range items; its five closed sets become typed enums | 2026-10-08 |
| D9 | CIF 2.0 syntax completion | A: all seven features (section 14). VT/FF line terminators in every dialect. Comments and `;` text members inside 2.0 containers. 2.0 table values. 2.0 triple-quoted strings. 2.0 nested save frames. 2.0 Unicode. The `1.25e+03` exponent form is already legal under the 1.1 `<Float>` production, so it needs no change. The seven `known_gap` pins flip to full IR pins. The iucr and cod pins stay byte-identical. The work lands on branch `dict-items/d9` (based on `dev-0.2`) | 2026-10-13 |
| D9a | VT/FF in every dialect | A: accept VT (`0x0B`) and FF (`0x0C`) as line terminators in every dialect. A recorded 1.1 deviation: the 1.1 spec excludes both (paras. 22, 28). It follows the bracket-lead precedent (13.3 item 2). Fixture 26 pins the 1.1 behavior | 2026-10-13 |
| D10 | dictionary completeness guard | A: research first (`docs/dict-completeness.md`, the `cell` worked example, the core structure categories, a `codegen/depends.json` extractor), then a per-category builder on the `bon` crate (3.x). The builder enforces the required set at compile time. New code style rule: no redundant comments where the code states the intent. Research runs on branch `dict-items/d10` (based on `dev-0.2`), mission `orchestrate/mission_d10.sh` | 2026-10-13 |

The current wave covers T0, T2, T3, T4, T5. T6 (and the D8 parser
work) start only when D1 approves the shell scope.

Wave status (2026-10-09, D8): the CIF 2.0 list-value parser work
moved up from the T6 wave to branch `dict-items/d8` (based on
`dev-0.2` at adb0b69, the orchestration commit), section 13
scope: dialect detection from the
`##CIF_2.0` magic code, `Value::List`, the 2.0 unquoted-value
restriction, `Matrix::from_list`, the canonical `list` value kind
and schema version 2, and the four section 13.6 fixtures. The T6
shell scope (the matrix items that consume the parser) still waits
on D1 approval.

Wave status (2026-10-13, D9): the CIF 2.0 syntax-completion work
runs on branch `dict-items/d9` (based on `dev-0.2`), section 14
scope: VT/FF line terminators (D9a), comments and text members
inside 2.0 containers, 2.0 table values, 2.0 triple-quoted
strings, 2.0 nested save frames, and 2.0 Unicode. The exponent
form `1.25e+03` is already legal, so section 14 records it with
no change. The seven `known_gap` pins flip to full IR pins. The
iucr, cod, and cod_sg pins stay byte-identical. No push and no
merge until the reviews finish.

Wave status (2026-10-09): T0, T2, T3, T4, T5 are merged into
dev-0.2 (final merge ab7db3f). All 734 items of the standard
dictionary are implemented. The recorded inventory holds 732.
The two missing items sit in the drift list. Four review sessions
finished clean. No P0 findings. The one P1 (a non-compiling
intermediate merge tree from the line-union merges) was repaired
on the branch (38a4956) and verified by the reviews. Open P2
notes: sessions/dict-items-review-t2..t5/report.md.

Shell wave: approved by the user on 2026-10-09 (D1 shell scope
confirmed). Started the same day as the shell-wave orchestration
(orchestrate/orchestrate_shell.sh): D8 first, then T6, then two
review sessions. D8 owns the parser dialect, Value::List, the
Matrix list constructor, the canonical list kind, and fixtures
16, 17, neg_cif2_*. T6 owns the shell inventory, the 347-item
set (D2: the _su items are skipped), the Matrix payload binding,
and fixture 18_shell.cif.

Shell wave status (2026-10-09, complete): D8 is merged into
dev-0.2 (merge 14870bd). T6 is merged (merge 9169ab5). The
shell inventory holds 1139 items in 91 categories, matching the
roadmap pin. Two review sessions finished clean with no P0.
The four P1 findings were fixed on dev-0.2 the same day:
the 2.0 quoted run allows interior blanks, the 2.0 list allows
a blank before the closer (both per the COMCIFS EBNF, pinned
by fixture 16), the 1.1 bracket-lead deviation is recorded in
13.3, and the shell generator closed-set corruption is fixed
(audit_schema, diffrn_radiation_wavelength_determination).
Accepted P2 notes: the T0 to T3 hand-extended modules keep
plain values for some closed and range items. The codegen
union rule keeps the standard inventory authoritative for
closed sets. The MatrixError struct-to-enum change is a
pre-1.0 breaking change. Reports:
sessions/dict-items-review-d8/report.md,
sessions/dict-items-review-t6/report.md.

## 11. Sources

- IUCr CIF 1.1 working specification (file syntax, common semantics):
  mirror at `cristal.org/DU-SDPD/nexus/iucr2/iucr-top/cif/spec/version1/`.
- IUCr core CIF dictionary, DDL1 v2.4.5:
  `ftp.iucr.org/cifdics/cif_core.dic`.
- Retrieval status (2026-10-08): `ftp.iucr.org` is unreachable from
  the build machine. The DDL1 inventory source is the IUCr-managed
  archive at `github.com/COMCIFS/DDL1-legacy-dictionaries`
  (`dictionaries/cif_core.dic`, `_dictionary_version` 2.5.0-dev.1),
  committed as `tests/dict/cif_core.dic` (see `tests/dict/SOURCE.md`).
  T0 records the actual item counts in the inventory. If they differ
  from the 734 of v2.4.5, the coverage gate uses the recorded counts.
- IUCr core dictionary shell v3.4.0-dev:
  `github.com/COMCIFS/cif_core` (`cif_core.dic`).
- Current state of the crate: `src/data_dict`, 51 `todo!()` sites,
  `MeasurementReflnLoopItem` without a `CifTerm` impl.

## 12. `Matrix` value type design (D3)

### 12.1 What a matrix is in a file

A matrix never appears as one token in a CIF 1.1 file. The
standard dictionary writes it as a numbered item family. A 3 x 3
matrix is nine items. A vector is three items. For example,
`_atom_sites_fract_tran_matrix_11` through `_33`.

The core shell adds 66 items with a `Matrix` container. The
dictionary fixes each shape with `_type.dimension` (for example
`[3,3]`). Examples are `cell.metric_tensor`,
`space_group_symop.R`, `space_group_symop.Seitz_matrix` (4 x 4),
`cell.vector_a`, `atom_site.cartn_xyz`.

The shell dictionary also expects a list value syntax (`[...]`).
That syntax exists only in CIF 2.0. The crate parses CIF 1.1.
So the list form is a later parser feature. Today, a matrix item
binds as a typed view over loop values and item families.

### 12.2 The Rust type

The type lives in the public value layer, next to `Numeric`.

```rust
// src/grammar/matrix.rs
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    rows: Vec<Vec<Number>>,
}

impl Matrix {
    /// All rows must share one length, with at least one row.
    pub fn try_new(rows: Vec<Vec<Number>>) -> Result<Self, MatrixError>;

    pub fn rows(&self) -> &[Vec<Number>];
    pub fn shape(&self) -> (usize, usize);
    pub fn row(&self, i: usize) -> Option<&[Number]>;
    pub fn col(&self, i: usize) -> Option<Vec<Number>>;

    /// Shape-checked views for items whose shape the dictionary fixes.
    pub fn as_3x3(&self) -> Option<[[&Number; 3]; 3]>;
    pub fn as_vector(&self) -> Option<Vec<Number>>;
}
```

Design points:

1. The element type is `Number`, not `Numeric`. The dictionary
   links matrix uncertainty to a `_su` companion item. It does not
   attach it per element. This matches D2 (B).
2. No `Value` variant. A `Value` is one CIF 1.1 token. A matrix
   is not one.
3. Typed wrappers live in `data_dict`, following the `CellAngle`
   pattern. For example, `SymopSeitz(Matrix)` checks the 4 x 4
   shape.
4. `try_new` rejects ragged rows. It returns a `MatrixError` in
   the style of `CrystalSystemCifError`.

### 12.3 Item binding per tier

- T2–T5: matrix-like data is item families. The items stay plain
  `numb` items. No `Matrix` is needed.
- T3 sugar (optional): a struct assembles the 3 x 3 transform
  matrix and 3-vector from the twelve numbered
  `atom_sites_fract_tran_*` items.
- T6: shell matrix items hold a `Matrix` payload. For example,
  `CellTerms::Metric_tensor(Matrix)`. The writer emits the
  element family for a 1.1 target. It emits a list value for a
  2.0 target.

### 12.4 Canonical JSON

No change today. The schema keeps its five value kinds. A matrix
item encodes through its 1.1 file form (loop rows or single
items). If the crate parses CIF 2.0 list values later, the schema
gains `kind: "matrix"` with nested rows. The version moves to 2.

### 12.5 Verification

- Unit tests cover `try_new` rejection of ragged rows, the shape
  views, and `Display` round trips.
- The inventory JSON records the shape of the 66 items. It reads
  the shape from the shell `_type.dimension`.
- A groundtruth fixture pins the classic family form. For
  example, the `atom_sites` transform items parse to numbers, as
  today.

## 13. CIF 2.0 list values (D8)

### 13.1 Why

The `Matrix` items of the core shell carry their value as a list.
The CIF 2.0 grammar defines the list syntax. The crate must parse
it to read 2.0 files and to write 2.0 targets. This section fixes
the scope and the acceptance criteria.

### 13.2 The grammar facts

The CIF 2.0 EBNF (published with the Bernstein et al. 2016 paper,
mirrored in the COMCIFS repository) gives:

- `list = '[' values ']'`, values separated by white space.
- A list may hold zero or more values. It may nest.
- `table = '{' key:value pairs '}'` also exists. No core
  dictionary item uses a table.
- A 2.0 file starts with the magic code `#\#CIF_2.0` as its first
  characters, after any byte order mark.
- A 2.0 unquoted value may not contain `[`, `]`, `{`, or `}`.
  The 1.1 parser accepts those characters today. The dialect
  switch must enforce the 2.0 restriction.

### 13.3 Scope

In:

1. Dialect detection from the file start. A `#\#CIF_2.0` magic
   code enables the 2.0 value rules. Anything else stays 1.1.
   Two 1.1 extensions are recorded (review d8 P1-3, P2-1): the
   unquoted lead set gains `[` and `]`, which deviates from the
   1.1 spec paragraph 19 (fixture 17 pins it). BOM elision
   applies to every dialect, not only 2.0 files.
2. A `Value::List(Vec<Value>)` variant. Nested lists are legal.
3. The 2.0 restriction on unquoted values. A `[`, `]`, `{`, or
   `}` inside an unquoted value is a parse error in the 2.0
   dialect. In the 1.1 dialect it is a plain character, as today.
4. `Matrix` binding: a matrix item reads a `List` of `Numeric`
   values, checks rectangularity, then checks the dictionary shape
   (`_type.dimension`).

Out:

1. Table values. No core dictionary item uses one.
   (D9 covers them, section 14.)
2. Triple-quoted strings (`"""`, `'''`), a 2.0 string form.
   (D9 covers them, section 14.)
3. Other 2.0 syntax. The magic code stays an inert comment line
   in 1.1 parsing.

### 13.4 The design

- `Value` gains one variant: `List(Vec<Value>)`. The grammar stays
  general. A list holds any values. The data-dictionary layer
  applies the matrix type.
- The parser holds a dialect flag. The value rule becomes:
  `list | <current choice>`, where `list` parses only in the 2.0
  dialect. The unquoted-string rule drops the four bracket
  characters in the 2.0 dialect.
- `Matrix` gains a constructor from list values. It rejects
  non-numeric elements and ragged shapes.

### 13.5 Canonical JSON

The schema gains one value kind:

```json
{ "kind": "list", "values": [ { "kind": "numeric", "...": "..." } ] }
```

New fixtures carry `schema_version: 2`. Existing fixtures keep
version 1. The harness accepts both versions. A matrix-shaped list
encodes as a nested list of `numeric` values. No dedicated
`matrix` kind is needed. The shape is dictionary data.

### 13.6 Fixtures

- `16_cif2_lists.cif`: a `#\#CIF_2.0` file with nested lists, a
  3 x 3 matrix list, an empty list, and a list of quoted strings.
- `neg_cif2_unbalanced_bracket.cif`: an unclosed `[`.
- `neg_cif2_bracket_in_unquoted.cif`: a 2.0 file with `[` inside
  an unquoted value.
- `17_cif1_brackets_stay_strings.cif`: a 1.1 file where
  `[1,2,3]` stays an unquoted `char_string`. This pins the
  dialect split.

### 13.7 Acceptance criteria

1. `parse_cif` reads every fixture in 13.6 into the canonical
   shape.
2. All 21 pre-D8 fixtures parse unchanged. Their expected
   JSON is byte-identical.
3. A 2.0 file with a `[` inside an unquoted value fails at the
   parse stage with a `Rich` error.
4. `Value::List` is exported from the crate root.
5. The groundtruth harness runs with two schema versions.

### 13.8 Effort

About two days. The dialect flag and the value rule are half a
day. The JSON kind and the harness change are half a day. The
four fixtures and their expected files are half a day.

D8 is independent of T2–T5. It builds on the T0 base. T6 needs it
only for reading 2.0 files or writing 2.0 targets.
## 14. CIF 2.0 syntax completion (D9)

### 14.1 Why

The D8 wave (section 13) left seven `known_gap` pins in the
`cif_api` corpus. Those files hold the upstream COMCIFS
`cif_api` reference implementation's edge cases: translated EOLs,
container comments, tables, triple-quoted strings, nested frames,
and Unicode. D9 brings the parser to the full 2.0 syntax so every
one of the 21 corpus files parses.

### 14.2 The features and designs

| feature | upstream source | design |
|---|---|---|
| VT/FF line terminators (every dialect, D9a) | `test_parse_10.c` drives `10.cif` with explicit extra-EOL options | `is_eol` accepts `\u{0b}` and `\u{0c}` in every dialect. A recorded 1.1 deviation, like the bracket-lead one (13.3 item 2, fixture 17). Fixture 26 pins it |
| Comments and `;` text members in 2.0 containers | the scanner `BODY_POSTDELIM` start state after `[` or `{` | `container_ws(dialect)`: in 2.0 a `#` with no leading blank starts a comment. The list production restructures its three white space regions so chumsky backtracking cannot drop a consumed comment. A `;` text field stands as a list member. The 1.1 list stays unchanged. Fixture 24 |
| 2.0 table values | the `KEY` token (a quoted string plus a colon) and the table-open and table-close tokens | a `Value::Table(Vec<(Value, Value)>)` variant. A table is `{`, white space separated pairs, `}`. A pair is a quoted key, a colon, and any value. The four container delimiters now end a 2.0 value token (the upstream tokenizes all four separately). In 1.1 the brace forms stay plain tokens. Fixture 21 |
| 2.0 triple-quoted strings | the `SQ3`/`DQ3` delimiter and `NOTSQ3`/`NOTDQ3` content forms | a `'''` or `"""` production. The content runs to the first closing delimiter. Content keeps its quote runs (up to two delimiters) and holds line terminators. The empty six-delimiter form is legal. Canonical kind stays `char_string`, with two new styles. 1.1 falls through to the embedded-quote rule, unchanged. Triple-quoted table keys are legal. Fixture 22 |
| 2.0 nested save frames | the C reference accepts nesting (`max_frame_depth = -1`); the bison EBNF drops it | a 2.0-only recursive frame content. The IR `SaveFrame` gains a `frames` list. The canonical frame object gains a `frames` array only when it is non-empty, so the iucr and cod pins stay byte-identical. `validate` extends its tag-scope and frame-code-scope checks recursively. 1.1 keeps spec para. 6 (no nesting). Fixture 23 |
| 2.0 Unicode | the 2.0 character set is UTF-8 | non-ASCII is legal in 2.0 block codes, frame codes, tag names, values, and comments. 1.1 stays ASCII (fixture 20 pins value rejection; the name-character rule pins codes and tags). Fixture 25 |
| the `1.25e+03` exponent form | the 1.1 `<Float>` production | already legal: the production is `{<Digit>}+'.'<Digit>+<Exponent>`. The crate accepts it and pins it as the float `1250` inside the `10.json` pin. No change |

The nested-frame choice: the upstream bison skeleton dropped nesting
from the EBNF ("dropped from CIF 2"), but the C reference accepts
it, and the `nested.cif` corpus file pins it. This crate follows
the C reference.

### 14.3 Canonical JSON

- The value kinds gain `table`:
  `{"kind": "table", "pairs": [{"key": <value>, "value": <value>}]}`.
  The pairs are an array, not an object, so key order survives.
- The `char_string` style set gains `triple_single_quoted` and
  `triple_double_quoted`.
- A frame object gains a `frames` array only when it holds nested
  frames. The array holds the same frame-object shape, so nesting
  is recursive.
- Every 2.0 pin carries `schema_version: 2`.

### 14.4 Negative fixtures

- `neg_cif1_table.cif`: a brace pair with spaces in a 1.1 file.
  The space-separated tokens are not one value in 1.1.
- `neg_cif2_unclosed_table.cif`: an unclosed `}`.
- `neg_cif1_triple_quoted.cif`: a line-spanning triple-quoted
  string in a 1.1 file. A 1.1 quoted string cannot cross a line.
- `neg_cif1_unicode_block_code.cif`: a non-ASCII block code in a
  1.1 file.
- `neg_cif1_unicode_tag.cif`: a non-ASCII tag name in a 1.1 file.

### 14.5 Acceptance criteria

1. Every one of the 21 `cif_api` files parses. The `known_gap`
   pins are gone.
2. The iucr, cod, and cod_sg pins stay byte-identical after
   regeneration.
3. Fixtures 21, 22, 23, 24, and 25 parse to their pinned JSON.
   The five 14.4 fixtures fail at the parse stage with a `Rich`
   error.
4. `cargo test` is green with no new warnings.
