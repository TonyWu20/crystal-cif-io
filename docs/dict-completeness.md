# Dictionary Completeness (D10)

This document records the item-dependency research for decision D10.
It reads the two pinned dictionaries and maps each relation kind to
the dictionary fields that encode it. The machine-readable result is
`codegen/depends.json`. The per-category builder input spec in
section 5 feeds the `bon` builder wave (D11).

## 0. Sources and artifacts

### Pinned dictionaries

| file | dictionary | version | update | items |
|---|---|---|---|---|
| `tests/dict/cif_core.dic` | `cif_core.dic` (DDL1) | `2.5.0-dev.1` | 2024-08-28 | 732 items in 63 categories |
| `tests/dict/cif_core_shell.dic` | CIF core shell (DDL 4.2.0) | `3.4.0-dev` | 2026-09-11 | 1139 items in 91 categories |

The sha256 of each file is recorded in `codegen/depends.json`.
The inventory test pins both dictionaries. A changed file fails the
tests.

### Artifacts

| artifact | role |
|---|---|
| `codegen/extract_depends.py` | stdlib-only extractor. Reads the two dictionaries, writes `depends.json` |
| `codegen/depends.json` | committed output. One record per item: source class, range, container, expression, cross-refs |
| `tests/depends_gen.rs` | cargo test. Reruns the extractor and fails on a byte diff. Checks inventory coverage |
| `tests/dict_inventory.json` | 732 DDL1 item names (authority for DDL1 spelling) |
| `tests/dict_inventory_shell.json` | 1139 shell item names (authority for shell spelling) |

Regenerate the file with:

```sh
python3 codegen/extract_depends.py
```

The gate is `cargo test --test depends_gen`. The test reruns the
extractor into `target/depends_rerun.json`. It fails when that file
differs from the committed `codegen/depends.json`. It also checks
that every inventory name appears in the file.

### Naming rules

Item names in `depends.json` follow the shell-inventory convention.
The first alias of the shell frame wins. A shell-native item without
an alias takes the `{category}_{object}` spelling. A shell item that
also exists in DDL1 keeps the DDL1 spelling. Two items keep a dot
in the name: `journal.data_validation_number` and
`symmetry.space_group_name_H-M`.

Shell frames that record a DDL1 item under a different shell
spelling land in `naming_exceptions` in the JSON. Each entry maps
the shell inventory name to the item that covers it. The cargo test
accepts an inventory name when it is in the file or in that list.
The list is generated, not hand-written.

## 1. Method: fields per relation kind (a)

Each dictionary encodes relations in a fixed field vocabulary.
The extractor reads these fields. The table below maps each
relation kind to the fields that carry it.

### DDL1 field vocabulary

| field | encodes |
|---|---|
| `_definition` | text definition. A formula (`=` inside) marks the item derived |
| `_type` | value type: `numb` or `char` |
| `_list` | loop flag: `yes`, `no`, or `both`. `both` means loop and single |
| `_list_mandatory` | loop items that must hold equal value counts |
| `_list_reference` | loop items that reference another loop |
| `_enumeration_range` | numeric range, for example `0.0:180.0` |
| `loop_ _related_item` | related-item pairs with a function: `alternate`, `conversion`, `replace`, or unlabeled |
| `_units` | units tag, for example `_angstrom` |
| `_example` | example values (skipped by the extractor) |
| `_deprecate` | deprecation note |

The DDL1 text rule: an opening `;` fence may carry inline content.
A bare `;` line closes the block.

### Shell field vocabulary

| field | encodes |
|---|---|
| `_type.purpose` | role: `Number` 227, `Describe` 161, `Measurand` 130, `Encode` 129, `State` 70, `SU` 41, `Link` 34, `Key` 22, `Composite` 2 |
| `_type.source` | source class: `Recorded` 340, `Assigned` 206, `Derived` 203, `Related` 67 |
| `_type.container` | container: `Single` 726, `Matrix` 66, `List` 19, `Array` 4, `Table` 1 |
| `_type.dimension` | matrix shape, for example `[3,3]` |
| `_type.contents` | contents: `Real`, `Integer`, `Complex`, `Text`, and more |
| `_enumeration.range` | numeric range (preferred over the DDL1 range) |
| `_enumeration_set.state` | closed value set |
| `_method.expression` | shell expression blocks. One frame may hold several blocks, one per purpose |
| `_name.linked_item_id` | the item this item links to (228 items carry one) |
| `_import.get` | the item is imported from a template file (344 items) |
| `_definition_replaced.by` | replacement item for a deprecated frame (13 items) |
| `_alias.definition_id` | alias list. The first alias is the crate spelling |

The shell text rule differs from DDL1. A bare `;` line opens and
closes the block.

One structural fact matters for the builder. No shell frame uses
`Loop` as a `_type.container` value. Loop-ness comes from the DDL1
`_list` tag. The extractor sets `container: "Loop"` only from DDL1
(121 items, `container_basis: "ddl1"`).

### Relation kinds and counts

| relation kind | encoded by | count |
|---|---|---|
| derived value | shell `_type.source: Derived` plus `_method.expression`, or a DDL1 `_definition` with `=` | 268 items |
| related item | shell `_type.source: Related` plus `_name.linked_item_id`, or DDL1 `loop_ _related_item` | 67 shell items, 115 DDL1 items (126 pairs) |
| closed set | `_enumeration_set.state` (shell) or DDL1 enumeration values | 63 items |
| range | `_enumeration.range`, shell preferred | recorded per item |
| deprecation | `_definition_replaced.by` | 13 items |
| import | `_import.get` (template files are not pinned in this repo) | 344 items |
| cross-ref | references in DDL1 `_definition` text and shell description or expression text | per-item `refs` array |

DDL1 related-item functions by count: `alternate` 73, `conversion`
16, `replace` 35, unlabeled 2.

## 2. Relation classes (b)

The extractor records one source class per item. The class comes
from the shell `_type.source` when the frame sets it. When the
frame sets no source, the class is `Derived` for a DDL1 definition
with `=` and `null` otherwise. The six working classes are:

| class | meaning | dictionary signal | items |
|---|---|---|---|
| Measured | a measured quantity | `_type.purpose: Measurand` | 130 |
| Derived | computed from other items | `_type.source: Derived` or DDL1 `=` definition | 268 |
| Assigned | chosen by the author, checked against a set or range | `_type.source: Assigned` | 206 |
| Recorded | reported text or context | `_type.source: Recorded` | 340 |
| context | measurement condition (temperature, wavelength, pressure, radiation) | a Recorded or Measurand item in a measurement frame | 7 in `cell`, more in `diffrn` |
| linked | uncertainty twin of another item | `_name.linked_item_id` to the base item, `_type.purpose: SU` | 228 carry a link |

The `_su` pattern: a base item plus its `_su` twin. The twin
imports its type through `_import.get`. The twin links to the base
through `_name.linked_item_id`. Per decision D2, the crate keeps
the uncertainty inside `Numeric`. The builder still records the
twin so a read side can pair the values.

Items with `source_class: null` are imported families. Their
shell frame carries no `_type.source`. The type resolves through
the imported template or the name-pattern fallback. A `null`
source class is a valid state, not a gap.

## 3. Worked example: `cell` (c)

The `cell` category holds 61 items: 22 from DDL1 and 39 shell-only.
Every DDL1 item also has a shell frame. The `cell_measurement_refln`
category adds 6 items: the four DDL1 loop items (`index_h`,
`index_k`, `index_l`, `theta`), the shell `hkl` matrix, and the
`theta_su` twin.

### Definition modes

Five complete ways define a cell. Four of the five carry a shell
expression, so the builder can derive them from the direct six
parameters.

| mode | items | shape |
|---|---|---|
| direct six | `length_a/b/c`, `angle_alpha/beta/gamma` | 6 x `Numeric` (angles in degrees, `0.0:180.0`) |
| orthogonal matrix | `orthogonal_matrix` | 3x3 `Matrix` (Rollett axial assignment) |
| three vectors | `vector_a/b/c` | 3 x 3-row `Matrix` |
| reciprocal six | `reciprocal_length_a/b/c`, `reciprocal_angle_alpha/beta/gamma` | 6 x `Numeric` |
| metric tensor | `metric_tensor` | symmetric 3x3 `Matrix`, six independent entries |

The mode items are alternative carriers of one quantity. The
builder takes one mode as input and computes the others.

### Derivation expressions

The shell records the expressions. The table lists each derived
item and its expression input.

| derived item | expression inputs |
|---|---|
| `volume` | `vector_a * (vector_b ^ vector_c)` (triple product). DDL1 gives the closed form `a*b*c*sqrt(1 - cos²a - cos²b - cos²c + 2*cos(a)*cos(b)*cos(c))` |
| `reciprocal_angle_a/b/c` | Buerger cosine rule on the direct angles |
| `reciprocal_length_a/b/c` | `Norm(reciprocal_vector_*)`. DDL1 gives `recip-a = b*c*sin(a)/V` |
| `reciprocal_vector_a` | `vector_b ^ vector_c / volume` (same for `b`, `c`) |
| `orthogonal_matrix` | Rollett matrix from lengths and angles |
| `vector_a/b/c` | `orthogonal_matrix * [1,0,0]` (same for `[0,1,0]`, `[0,0,1]`) |
| `metric_tensor` | outer products of the direct vectors |
| `reciprocal_metric_tensor` | outer products of the reciprocal vectors |
| `reciprocal_orthogonal_matrix` | `Inverse(Transpose(orthogonal_matrix))` |
| `convert_Uiso_to_Uij` | cosine matrix of the reciprocal angles |
| `convert_Uij_to_betaij` | `1.4142 * Pi * diag(reciprocal_length_*)` |
| `atomic_mass` | `sum(atom_type.number_in_cell * atom_type.atomic_mass)` over the atom-type loop |
| `formula_units_Z_prime` | `formula_units_Z / space_group_multiplicity` |

Cross-checks a builder should run:

- `volume` equals the closed form from the direct six.
- `volume` equals the triple product of `vector_a/b/c`.
- the norm of `reciprocal_vector_a` equals `reciprocal_length_a`.
- `reciprocal_orthogonal_matrix` equals the inverse transpose of
  `orthogonal_matrix`.
- the metric tensor built from the vectors equals the
  `metric_tensor` item when both are present.

### Cross-category edges

The `refs` arrays carry these edges out of `cell`:

| from | to | kind |
|---|---|---|
| `formula_units_Z` | `chemical_formula_structural`, `chemical_formula_moiety`, `chemical_formula_sum` | DDL1 related (assigned against the formula) |
| `formula_units_Z_prime` | `space_group_multiplicity` (via the `space_group` category) | division |
| `measurement_wavelength` | `diffrn_radiation_wavelength` | default source |
| `measurement_temperature` | `diffrn_ambient_temperature` | deprecated replacement |
| `measurement_pressure` | `diffrn_ambient_pressure` | deprecated replacement |
| `length_*`, `angle_*` | `refln_index_h/k/l`, `diffrn_refln_index_h/k/l`, `diffrn_reflns_transf_matrix_*` | hkl index consistency |
| `atomic_mass` | the `atom_type` loop | derived sum |

### Deprecations

The shell deprecates the `cell_measurement.*` context items. The
replacement moves to the `diffrn` category.

| deprecated item | replaced by |
|---|---|
| `measurement_temperature` | `_diffrn.ambient_temperature` |
| `measurement_wavelength` | `_diffrn_radiation.wavelength.value` |
| `measurement_pressure` | `_diffrn.ambient_pressure` |

`measurement_reflns_used` keeps a range of `3:`. It is not
deprecated.

### Read the cell records

This snippet runs against the committed file:

```python
import json

doc = json.load(open("codegen/depends.json"))
cell = next(c for c in doc["categories"] if c["name"] == "cell")
for i in cell["items"]:
    print(i["name"], i["source_class"], i["range"], i["container"])
```

## 4. Core structure categories (d)

### `space_group` (18 items)

Four DDL1 items: `IT_number`, `crystal_system`, `name_H-M_alt`,
`name_Hall`. The shell adds 14 items, one of them the dotted
legacy item `symmetry.space_group_name_H-M` (kept with its dot).
The category is mostly closed sets:

| item | class | set or range |
|---|---|---|
| `Bravais_type` | Assigned | closed, 14 codes (`aP` .. `cF`) |
| `centring_type` | Assigned | closed, 9 codes (`P` .. `H`) |
| `crystal_system` | Assigned | closed, 7 names |
| `IT_coordinate_system_code` | Assigned | closed, 37 codes |
| `Laue_class` | Assigned | closed, 11 symbols |
| `IT_number` | Assigned | range `1:230` |
| `multiplicity` | Derived | range `1:192`, equals the symop count |
| `name_H-M_ref`, `name_H-M_alt`, `name_Hall`, `name_Schoenflies` | Assigned / Recorded | symbols; `name_H-M_ref` refs the other three |
| `Patterson_name_H_M`, `point_group_H_M` | Assigned | symbols |
| `transform_Pp_abc`, `transform_Qq_xyz` | Recorded | reference-setting matrices; both ref `reference_setting` |
| `name_H_M_alt_description` | Recorded | refs `name_H-M_alt` |

`reference_setting` refs the `IT_number`, the Hall name, and the
two transform items.

### `space_group_symop` (7 items, all shell)

| item | class | shape |
|---|---|---|
| `operation_xyz` | Recorded | Symop text |
| `operation_description` | Recorded | text |
| `id` | Derived, range `1:192` | the symop ordinal |
| `R` | Derived | 3x3 rotation matrix |
| `RT` | Derived | 3x3, refs `R` |
| `Seitz_matrix` | Derived | 4x4, `SeitzFromJones(operation_xyz)` |
| `T` | Derived | 3-vector, translation row of the Seitz matrix |

The symop loop closes the `space_group.multiplicity` check: the
loop row count equals the multiplicity.

### `atom_site` (70 items, 7 loop groups)

Loop groups (DDL1 `_list` families): `aniso_B_11..33`,
`aniso_U_11..33`, `Cartn_x/y/z`, `fract_x/y/z`,
`label_component_0..6`.

| fact | value |
|---|---|
| `adp_type` | Assigned, closed: `Uani`, `Uiso`, `Uovl`, `Umpe`, `Bani`, `Biso`, `Bovl`, `betaani`. DDL1 records the same set without `betaani` (7 values) |
| `fract_x/y/z`, `cartn_x/y/z` | imported families (`templ_attr.cif` frames `fract_coord`, `cartn_coord`). Source class `null` is expected |
| `fract_xyz`, `cartn_xyz` | Derived, `Matrix [3]`, built from the single x/y/z items |
| `occupancy` | Derived, range `0.0:1.0` |
| `U_iso_or_equiv`, `B_iso_or_equiv` | Derived, range `0.0:` |
| `aniso_ratio` | Derived, range `1.0:` |
| `refinement_flags` | deprecated, replaced by `refinement_flags_posn`, `refinement_flags_adp`, `refinement_flags_occupancy` |
| `refinement_flags_posn` | closed, 16 codes (`.` .. `DGRS`) |
| `type_symbol` | Related. Links to `atom_type.symbol` |
| `aniso_type_symbol` | Related. Links through `aniso_label` to `atom_type.symbol` |
| `calc_attached_atom`, `aniso_label` | Related, link to `label` |
| `chemical_conn_number` | Related, links to `chemical_conn_atom.number` |
| `site_symmetry_order` / `site_symmetry_multiplicity` | Derived. Range `1:48` / `1:192`. The multiplicity refs `space_group_multiplicity` and the order |
| `Wyckoff_symbol` | imported from `templ_enum.cif` (`wyckoff_letter`) |
| `thermal_displace_type` | closed, 7 codes (`Uani` .. `Bovl`), no `betaani` |
| `attached_hydrogens` | Recorded, range `0:8` |
| `constraints`, `restraints` | Recorded. Ref the `refine_ls_number_*` items |

The `_su` twins (`fract_x_su`, `occupancy_su`, the `xyz_su`
matrices, the `B/U` mean twins) import their type and link to the
base item.

### `chemical_formula` (8 items)

| item | class | note |
|---|---|---|
| `structural` | Assigned | refs `chemical_formula_analytical` |
| `moiety`, `sum`, `analytical`, `iupac` | Assigned | formula spellings |
| `weight` | Derived, range `1.0:` | refs all five formula items plus `exptl_crystal_density_diffrn` |
| `weight_meas` | Recorded, range `1.0:` | the measured value |
| `weight_meas_su` | linked twin of `weight_meas` | |

`cell.formula_units_Z` closes the loop: `Z` assigns against these
formula items. `weight` ties the formula to the measured crystal
density.

## 5. `bon` builder input spec (e)

The D11 wave adds one `bon` builder per category. Each builder
struct takes the author-supplied items as input. The derived items
are not builder input. The builder computes them or cross-checks
them. The `CellTerms` enum in `src/data_dict/core_cif/cell/mod.rs`
is the existing shape the spec follows.

Reading the spec tables:

| role | meaning |
|---|---|
| required | builder field, `bon` validates presence at build |
| optional | builder field, absent by default |
| derived | not a builder input. The builder computes it from the required inputs |
| su | the `_su` twin, optional, paired with its base field |
| loop | a separate loop struct (one `Vec` per loop item, equal lengths) |
| enum | a closed set, typed as an `enum` per decision D4a |

### Builder shape

Equivalent minimum sets own their own builders. The crate adapts
to different habits. One builder per minimum set. Wave 1 ships
the `cell` as two builders (the direct six, three vectors). The
`space_group` builder takes one unique identifier (`IT_number`,
`name_Hall`, or `name_H-M`) and resolves the rest through the
`crystallographic-group` crate (0.4). The table lookups by
number, Hall symbol, or H-M symbol. `general_positions` gives
the Seitz operation lists. Items the research marked `required`
that the crate computes become builder outputs. The builder
cross-checks them when the author supplies a value.

`formula_units_Z` is not manual input. The structure-level
computation derives it from the `atom_site` loop, the formula,
and the `space_group` multiplicity.

### `cell` builder input

| item | role | value |
|---|---|---|
| `length_a`, `length_b`, `length_c` | required | `Numeric` |
| `angle_alpha`, `angle_beta`, `angle_gamma` | required | `CellAngle` (0:180) |
| `formula_units_Z` | optional (cross-check) | `Numeric` (range `0:`). The structure-level computation derives `Z` from the `atom_site` loop, the formula, and the `space_group` multiplicity |
| `formula_units_Z_prime` | optional | `Numeric` (range `0:`) |
| `formula_units_Z_details`, `formula_units_Z_prime_details`, `special_details` | optional | `CharString` |
| `measurement_reflns_used` | optional | `Numeric` (range `3:`) |
| `measurement_temperature`, `measurement_wavelength`, `measurement_pressure`, `measurement_radiation` | optional (deprecated) | `Numeric` / `CharString` |
| `measurement_theta_min`, `measurement_theta_max` | optional | `Numeric` (0:90) |
| `volume` | derived | triple product or closed form |
| `vector_a/b/c` | input of the second `cell` builder | 3 x 3-row `Matrix` |
| `orthogonal_matrix`, `metric_tensor` | derived, cross-check inputs | 3x3 `Matrix` |
| `reciprocal_length_*`, `reciprocal_angle_*`, `reciprocal_vector_*`, `reciprocal_metric_tensor`, `reciprocal_orthogonal_matrix` | derived | from the mode chosen |
| `convert_Uiso_to_Uij`, `convert_Uij_to_betaij` | derived | 3x3 `Matrix` |
| `atomic_mass` | derived | from the `atom_type` loop |
| the 24 `_su` items | su | `Numeric` / `Matrix` twins |

The loop struct is the existing `MeasurementReflnLoopItem`
(`index_h/k/l`, `theta`). The shell adds an `hkl` matrix form for
the same loop.

### `space_group` builder input

| item | role | value |
|---|---|---|
| `crystal_system`, `Bravais_type`, `centring_type` | computed | enums, from the resolved group |
| `IT_number`, `name_Hall`, `name_H-M` | one entry (any one of the three) | `UnsignedInteger` (1:230) / `CharString`. Each resolves uniquely through `crystallographic-group` |
| `name_H-M_ref`, `name_H-M_alt` | optional | `CharString` |
| `name_Schoenflies`, `Patterson_name_H_M`, `point_group_H_M`, `Laue_class` | optional | `CharString` / enum |
| `IT_coordinate_system_code` | optional | enum (37) |
| `reference_setting`, `transform_Pp_abc`, `transform_Qq_xyz`, `name_H_M_alt_description` | optional | `CharString` / 3x3 `Matrix` |
| `multiplicity` | computed | the core position set length of `general_positions` |
| `symmetry.space_group_name_H-M` | optional (legacy) | `CharString`, dotted tag |

### `space_group_symop` loop input

One loop struct. `operation_xyz` is required per row. `id` is
the row ordinal. The Seitz lists come from `general_positions`
of the resolved group. `R`, `RT`, `T`, and `Seitz_matrix` derive
from them. The builder cross-checks the row text against the
Seitz list.

### `atom_site` loop input

| item | role | value |
|---|---|---|
| `label` (or `label_component_0..6`) | required | `CharString` |
| `type_symbol` | required | `CharString` (links to `atom_type.symbol`) |
| `fract_x/y/z` or `cartn_x/y/z` | required | `Numeric` (exactly one coordinate mode) |
| `occupancy` | optional | `Numeric` (0:1, derived default 1.0) |
| `adp_type` | required | enum (8) |
| `aniso_U_11..33` / `aniso_B_11..33` | required when aniso | `Numeric` |
| `U_iso_or_equiv` / `B_iso_or_equiv` | optional | `Numeric` |
| `aniso_label`, `aniso_ratio`, `aniso_type_symbol` | optional (overlap disorder) | `CharString` / `Numeric` / `CharString` |
| `calc_flag` | optional | enum (`d`, `calc`, `c`, `dum`) |
| `refinement_flags_posn`, `refinement_flags_adp`, `refinement_flags_occupancy` | optional | enums |
| `description`, `constraints`, `restraints`, `disorder_group`, `disorder_assembly`, `fract_symmform`, `attached_hydrogens` | optional | `CharString` / `Numeric` |
| `Wyckoff_symbol` | optional | `CharString` |
| the `_su` twins | su | `Numeric` / `Matrix [3]` |

The coordinate mode is exclusive: a row carries fractional or
Cartesian coordinates, not both. The builder enforces this. The
aniso or iso displacement choice is one required enum field. The
builder rejects a row without a displacement type.

### `chemical_formula` builder input

| item | role | value |
|---|---|---|
| `structural` | required | `CharString` |
| `moiety`, `sum`, `analytical`, `iupac` | optional | `CharString` |
| `weight` | derived | from the formula and `exptl_crystal_density_diffrn` |
| `weight_meas` | optional | `Numeric` (1.0:) |
| `weight_meas_su` | su | `Numeric` |

This spec covers the five categories the research reviews. The
remaining categories follow the same table rules. `depends.json`
is the source for the D11 wave. The per-category `source_class`,
`range`, `container`, `closed_values`, `expression`, and `refs`
fields drive the builder code generation.
