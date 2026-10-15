# crystal-cif-io

Parse and write crystallographic CIF (Crystallographic Information
File) data in Rust. The workspace carries two crates.
`crystal-cif-fmt` handles the IUCr CIF 1.1 file syntax and the
CIF 2.0 value rules. It owns the parser, the IR types, and the
canonical JSON. `crystal-cif-io` provides the typed dictionary
layer for the IUCr core CIF dictionary. It carries the typed
block view (`CoreCif`) and a per-category builder layer. Each
builder needs the required items of one category. A missing
item fails to compile at the `build()` call.

## Usage

Add the dependency:

```toml
[dependencies]
crystal-cif-io = "0.3"
```

The read, edit, and write workflows need only this dependency.
`crystal-cif-io` re-exports the IR value and block types of
`crystal-cif-fmt` that they name. It re-exports the `SpaceGroup`
type of `crystallographic-group` under `group`. Add
`crystal-cif-fmt` only for direct IR control: loop columns,
save-frame trees, 2.0 containers, canonical JSON, dialect
detection. Add `crystallographic-group` only for group machinery
beyond `group::SpaceGroup`.

The tour serves the read, the edit, and the write of a `.cif`
file. It needs the `crystal-cif-io` dependency only. The main
sections work on the typed view of `crystal-cif-io`. The
value-layer sections at the end drop one level to the IR of
`crystal-cif-fmt`.

The snippets carry over across sections. `cifs` and `cif`
come from the read step of section 2. The appendix letters
carry over the `doc` of letter A. Every snippet compiles
against the current crates.
Parser and IR types live in `crystal-cif-fmt`. Dictionary and
builder types live in `crystal-cif-io`. The `space_group_symop`
builder names the `SpaceGroup` type of
`crystallographic-group`, re-exported under `group`. The main
sections import the re-exported types from `crystal_cif_io`. The
value-layer sections import `crystal_cif_fmt` directly. Each
snippet shows its own imports.

The tour rides on a real file. The excerpt below is trimmed
from `crystal_cif_fmt/tests/real_data/cod/1000000.cif`, COD entry 1000000
(IUCr Acta C 56, 1073, 2000). The full file is pinned in the
test corpus. Copy the full file to `1000000.cif` next to your
scratch. The tour reads from that path. The excerpt previews
the file's shape.

```cif
# This file is available in the Crystallography Open Database (COD),
data_1000000
loop_
_publ_author_name
'Phan Thanh, S.'
'Marrot, J.'
_publ_section_title
;
[H~3~N(CH~2~)~5~NH~3~].AlP~2~O~8~H, a one-dimensional aluminophosphate
;
_journal_name_full               'Acta Crystallographica, Section C'
_chemical_formula_sum            'C5 H17 Al N2 O8 P2'
_space_group_IT_number           14
_symmetry_space_group_name_H-M   'P 1 21/n 1'
_cell_angle_alpha                90.00
_cell_angle_beta                 95.1470(10)
_cell_angle_gamma                90.00
_cell_formula_units_Z            4
_cell_length_a                   7.8783(2)
_cell_length_b                   10.46890(10)
_cell_length_c                   16.0680(4)
_cell_measurement_reflns_used    5007
_cell_measurement_temperature    296(2)
_cell_measurement_theta_max      29.83
_cell_measurement_theta_min      2.32
_cell_volume                     1319.90(5)
loop_
_atom_site_label
_atom_site_fract_x
_atom_site_fract_y
_atom_site_fract_z
_atom_site_U_iso_or_equiv
_atom_site_thermal_displace_type
_atom_site_calc_flag
_atom_site_refinement_flags
_atom_site_occupancy
_atom_site_type_symbol
P1 .55909(13) .78402(9) .02396(7) .0280(3) Uani d . 1 P
P2 .09646(13) 1.05563(11) .13485(6) .0290(3) Uani d . 1 P
Al1 .24723(13) .95928(11) -.02995(7) .0246(4) Uani d . 1 Al
```

### 1. The error story

The tour uses `.expect` at every fallible call. The inputs are
the pinned corpus files. They parse, they type, and the
builders build. The `expect` calls state that fact. Production
code propagates instead.

Two shapes cover every fallible call. A `Result` carries the
failure of a step. An `Option` marks an item absent in the
file.

| step | call | the failure |
|---|---|---|
| parse the file typed | `parse_core_cif` | a `CoreCifParseError`: a rendered syntax report, or every block that fails to type |
| parse the file raw | `parse_cif` | a `Vec<Rich<char>>` of syntax errors, rendered by `rich_error` |
| type a block | `CoreCif::from_block` | a `Vec<CifError>`. The read collects every value-class and shape mismatch. It never stops at the first |
| check required items | `CoreCif::validate` | a `Vec<CifError>` of the missing required items |
| build a category | `Cell::builder() ... build()` | a `Result` over the category error type. Missing needed fields fail to compile at the `build()` call. Cross-checks fail at run time |
| read a typed field | `cif.refine.ls_matrix_type` | an `Option`. The item is absent in the file |

The error types of both crates derive `thiserror`. They carry
the message and the `source` chain. `CifError` names the tag.
The builder errors name the item.

### 2. Reading a data block

The typed layer reads a `.cif` file into plain structs.
`parse_core_cif` is the read entry point. It returns one
`CoreCif` per data block. The IR document and the per-block
typing stay inside the call. `CoreCif` is the view. It carries
the 62 categories of the standard dictionary view (roadmap D12i). The waves of roadmap
D12j roll them out in tier order. The wave 1 groups type the
cell, space group, formula, and atom_site items. The file's own
items also type into the later-wave groups. Foreign tags and save frames stay in the
`extra` residue. The parse is tolerant.

```rust
use crystal_cif_io::parse_core_cif;

let source = std::fs::read_to_string("1000000.cif").unwrap();
let cifs = parse_core_cif(&source).expect("the file parses and types");
let mut cif = cifs.into_iter().next().expect("the block is present");

// The wave 1 categories hold the file's items.
println!("{}", cif.cell.length_a.as_ref().expect("a length a"));
println!("{}", cif.cell.angle_beta.as_ref().expect("an angle beta").angle());
println!(
    "group {}, {}",
    cif.space_group.it_number.as_ref().expect("an IT number"),
    cif.space_group
        .symmetry_name_h_m
        .as_ref()
        .expect("the H-M symbol")
        .content(),
);
println!("{}", cif.chemical_formula.sum.as_ref().expect("the sum spelling"));
println!("{}", cif.atom_site[0].label.as_ref().expect("a label").content());
```

`parse_core_cif` collects every value-class and shape
mismatch of every block. It never stops at the first. A block
that fails to type reports every one of its errors. Foreign
tags land in `extra`. Loop rows fill their row structs.

```rust
match cif.validate() {
    Ok(()) => println!("the required sets are complete"),
    Err(gaps) => {
        for gap in gaps {
            eprintln!("{gap}");
        }
    }
}
```

The check reports every missing required item. The file lacks
the `adp_type` column of `atom_site`. Each of its 35 rows
reports one gap. The file also lacks the `chemical_formula`
structural spelling. The check lists 36 gaps.

The file's later-wave categories type too. Its
`_exptl_absorpt_correction_type` records the real-data code
`semi-empirical`. It sits one past the DDL1 closed set. The
closed-value gate keeps the marked extension for it (roadmap
D12m). The `_refine_ls_shift/su_max` value `<0.001` records
as the inequality payload of the plain field (roadmap D12l).

```rust
use crystal_cif_io::data_dict::Ineq;

println!("{}", cif.exptl.absorpt_correction_type.as_ref().expect("a correction type"));
println!("{}", cif.refine.ls_hydrogen_treatment.as_ref().expect("a treatment"));

match cif.refine.ls_shift_su_max {
    Some(Ineq::LessThan(bound)) => println!("su_max < {bound}"),
    Some(Ineq::Value(value)) => println!("su_max = {value}"),
    None => {}
}
```

The three lines print `semi-empirical`, `constr`, and
`su_max < 0.001`.

### 3. Editing the typed block

Edits are plain mutations on the struct. The fields are
`Option<T>`. A `None` field writes nothing. The types reject
out-of-range values. `validate` reports the required-item
gaps.

```rust
use crystal_cif_io::{CharString, Numeric, UnquotedString};
use crystal_cif_io::data_dict::Ineq;
use crystal_cif_io::data_dict::core_cif::geom_angle::GeomAngleRow;
use crystal_cif_io::data_dict::core_cif::refine::{MatrixType, NonNeg};

fn unquoted(s: &str) -> CharString {
    CharString::Unquoted(UnquotedString::new(s.to_string()))
}

// A single-value item takes the value of its type.
cif.chemical_formula.structural = cif.chemical_formula.sum.clone();

// A closed-set item takes the enum of its closed set.
cif.refine.ls_matrix_type = Some(MatrixType::Diagonal);

// The less-than assertion records in the inequality payload of
// the plain field.
cif.refine.ls_shift_su_max =
    Some(Ineq::LessThan(NonNeg::from_numeric(Numeric::from(0.001))));

// A loop category edits through its row vector.
let mut row = GeomAngleRow::default();
row.atom_site_label_1 = Some(unquoted("P1"));
row.atom_site_label_2 = Some(unquoted("Al1"));
row.atom_site_label_3 = Some(unquoted("P2"));
row.angle = Some(Numeric::from(51.2));
cif.geom_angle_rows.push(row);
```

The structural assignment rebinds the field to the sum
spelling. It resolves the formula gap. The 35 adp_type gaps
stay. The check reports the remaining state.

### 4. Writing the block back

The write step is `cif.to_cif()`. It emits the typed
categories in a fixed order, then the `extra` residue. Save
frames stay whole in the residue. `write_core_cif` writes a
multi-block file. The pre-write check reports the gaps. It
does not block the write.

```rust
match cif.validate() {
    Ok(()) => println!("the required sets are complete"),
    Err(gaps) => {
        for gap in gaps {
            eprintln!("{gap}");
        }
    }
}

std::fs::write("1000000_out.cif", cif.to_cif()).unwrap();
```

Save frames are residue too. A frame keeps its code and items
whole in `extra`.

```rust
use crystal_cif_io::{
    DataBlock, DataBlockHeading, DataBlockMember, SaveFrame, SaveFrameHeading,
};

for member in &cif.extra {
    match member {
        DataBlockMember::DataItems(_) => println!("a residue item"),
        DataBlockMember::SaveFrame(frame) => {
            println!("a residue frame {}", frame.heading());
        }
    }
}

let framed = DataBlock::from_heading_members((
    DataBlockHeading::new("1000000".into()),
    vec![DataBlockMember::SaveFrame(SaveFrame::from_heading_items((
        SaveFrameHeading::new("P1".to_string()),
        Vec::new(),
    )))],
));
let framed_cif = CoreCif::from_block(&framed).expect("the frame block types");
assert_eq!(framed_cif.extra.len(), 1, "the frame stays in the residue");
let round = framed_cif.to_block();
assert_eq!(round.members().len(), 1, "the frame writes back");
```

### 5. Builders (compile-time completeness)

The builder layer takes a complete category as input. Needed
fields are checked at compile time: a missing field fails to
build at the `build()` call. Derived items are not input. The
builder computes them and cross-checks a supplied value within
the relative tolerance. A mismatch rejects with a `Result`,
never a panic.

Breaking change, pre-1.0 (roadmap D12h): `build()` returns the
document structs of section 2. The builder-private structs and
their `to_data_items` paths are removed. The `CifCategory`
trait `to_members` method is the materialization path now.

```rust
use crystal_cif_io::data_dict::core_cif::cell::{Cell, CellAngle};
use crystal_cif_io::{Matrix, Number, UnsignedInteger};

// The direct-six entry set builds the excerpt's cell. The
// supplied volume cross-checks against the computed value.
let cell = Cell::builder()
    .length_a(7.8783.into())
    .length_b(10.4689.into())
    .length_c(16.0680.into())
    .angle_alpha(CellAngle::from(90.0f64))
    .angle_beta(CellAngle::from(95.1470f64))
    .angle_gamma(CellAngle::from(90.0f64))
    .formula_units_Z(UnsignedInteger(4))
    .volume(1319.90.into())
    .build()
    .expect("the COD cell is consistent");

println!("volume {}", cell.volume.as_ref().expect("the computed volume"));

let va = Matrix::try_new(vec![vec![
    Number::Float(5.959.into()),
    Number::Float(0.0.into()),
    Number::Float(0.0.into()),
]])
.expect("a 1 x 3 matrix");
let vb = Matrix::try_new(vec![vec![
    Number::Float(0.0.into()),
    Number::Float(14.956.into()),
    Number::Float(0.0.into()),
]])
.expect("a 1 x 3 matrix");
let vc = Matrix::try_new(vec![vec![
    Number::Float(0.0.into()),
    Number::Float(0.0.into()),
    Number::Float(19.737.into()),
]])
.expect("a 1 x 3 matrix");

// The three-vector entry set computes the direct six.
let cell = Cell::vectors()
    .vector_a(va)
    .vector_b(vb)
    .vector_c(vc)
    .build()
    .expect("a valid cell");

println!("volume {}", cell.volume.as_ref().expect("the computed volume"));
```

`Cell` is the document struct of section 2. The builder start
functions sit on that struct. `Cell::builder` takes the direct
six. `Cell::vectors` takes the three lattice vectors.

The group builder resolves the excerpt's space group. One of
three unique identifiers. Group 14 resolves to the Hall symbol
`-P 2ybc`, the `P` centring type, and four operations. The
`crystallographic-group` crate resolves the group.

```rust
use crystal_cif_io::data_dict::core_cif::space_group::{ITNumber, SpaceGroup};

let group = SpaceGroup::by_it_number(ITNumber::new(14))
    .expect("a valid group");
println!(
    "Hall {}, {} centring, {} operations",
    group.name_hall.as_ref().expect("a Hall symbol").content(),
    group.centring_type.as_ref().expect("a centring type"),
    group.multiplicity.as_ref().expect("a multiplicity"),
);
```

The file's full symbol `P 1 21/n 1` adds the `n` glide. It is
no all-settings spelling. The `by_name_h_m` entry point
resolves the all-settings table spelling only.

The loop and formula builders return their document structs
too. The symop rows derive from the Seitz list of the resolved
group. The formula builder needs `structural`. The other
spellings are optional.

```rust
use crystal_cif_io::{CharString, UnquotedString};
use crystal_cif_io::data_dict::core_cif::chemical_formula::ChemicalFormula;
use crystal_cif_io::data_dict::core_cif::space_group_symop::Symop;
use crystal_cif_io::group::SpaceGroup as Resolved;

let symops = Symop::builder()
    .group(Resolved::try_from_str("-P 2ybc").expect("IT 14 resolves"))
    .build()
    .expect("the rows build");
println!(
    "{} symops, first {}",
    symops.len(),
    symops[0].operation_xyz.as_ref().expect("an xyz").content()
);

fn text(s: &str) -> CharString {
    CharString::Unquoted(UnquotedString::new(s.to_string()))
}

let formula = ChemicalFormula::builder()
    .structural(text("C5 H17 Al N2 O8 P2"))
    .sum(text("C5 H17 Al N2 O8 P2"))
    .build()
    .expect("the formula builds");
println!("{}", formula.structural.as_ref().expect("the spelling"));
```

The `atom_site` row builder takes one coordinate mode per row
and a displacement type. The `formula_units` helper derives
the structure `Z` and cross-checks a supplied value. Each
`build()` returns the category document struct: `Cell`,
`SpaceGroup`, `Vec<Symop>`, `Vec<AtomSite>`,
`ChemicalFormula`.

The per-category input spec and the item dependency map live
in `docs/dict-completeness.md` and `codegen/depends.json`.

### 6. Matrices


`Matrix` is the dictionary value for matrix items. Build it from
rows or from a parsed 2.0 list value.

```rust
use crystal_cif_io::{Matrix, Value};

let matrix = Matrix::try_new(vec![
    vec![1.into(), 2.into()],
    vec![3.into(), 4.into()],
])
.expect("a 2 x 2 matrix");
assert_eq!(matrix.shape(), (2, 2));

// From a parsed 2.0 list value:
let list = Value::List(vec![Value::List(vec![
    Value::Numeric(1.into()),
    Value::Numeric(2.into()),
])]);
let matrix = Matrix::from_list(list.as_list().expect("a list"), Some((1, 2)))
    .expect("a 1 x 2 matrix");
```

`from_list` rejects non-numeric elements and ragged shapes.
`as_3x3` and `as_vector` return `None` when the shape does not
fit. Errors are `MatrixError`.

### 7. Canonical JSON and validation


`canonical::document_json` encodes a document as the canonical
IR. `validate` runs the spec-level checks and returns the
violations. This is where `crystal-cif-fmt` enters. Canonical
JSON is a feature of the fmt crate. Add the `crystal-cif-fmt`
dependency for this section.

```rust
use crystal_cif_fmt::{canonical, parse_cif};

let source = std::fs::read_to_string("1000000.cif").unwrap();
let doc = parse_cif(&source).expect("the file parses");
let json = canonical::document_json(&doc, "1000000.cif", 2);
println!("{}", serde_json::to_string(&json).unwrap());

for violation in canonical::validate(&doc) {
    eprintln!("violation: {violation}");
}
```

The schema version is 1 for 1.1 value kinds and 2 when the
document carries 2.0 kinds (`list`, `table`, the triple-quote
char-string styles, the frame `frames` array). The `validate`
checks flag duplicate block names and duplicate tags.

### 8. CIF 2.0 specifics


Lists, tables, triple-quoted strings, nested save frames, and
non-ASCII text parse only in the 2.0 dialect. The 1.1 dialect
keeps the ASCII character set.

The pinned 2.0 test file
`tests/real_data/cif_api/complex_data.cif` mixes all three value
containers. Save it as `complex_data.cif`.

```cif
#\#CIF_2.0

data_complex_data

_list_of_lists [
  []
  [foo bar ]
  [ x y z ]
]

_table_of_tables {
  'English':{
    'one':one
    'two':two
  }

  'French':{
    'one':'un'
    'two':"deux"
  }
}

_hodge_podge [
  ?
  { 'a':10 'b':11 'c':[? 12] }
  [. . {} {'alice':Cambridge 'bob':Harvard 'charles':.}]
]
```

```rust
use crystal_cif_io::parse_cif;

let cs = std::fs::read_to_string("complex_data.cif").unwrap();
let xdoc = parse_cif(&cs).expect("the 2.0 file parses");
let xblock = xdoc.get_data_block_by_name("complex_data").expect("present");
let hodge = xblock.find_single_value_by_tag("hodge_podge").expect("present");

for value in hodge.value().as_list().expect("a list") {
    println!("{value}");
}
```

The mixed list prints `?`, a nested table, and a list of
values. Table pairs keep their order. The key is a value,
usually a char string.

## The value layer reference (`crystal-cif-fmt`)

The main tour works on the typed view. It imports the
re-exported types from `crystal_cif_io`. These sections drop one
level. They show the IR of `crystal-cif-fmt` directly: the
document shape, the value variants, the loops, and the save
frames. They import `crystal_cif_fmt` directly. That is the path
that adds the `crystal-cif-fmt` dependency. Their snippets carry
over the `doc` of letter A. Use them when you read
a block that no category types, write a custom reader, or
inspect a 2.0 container.

### A. Parsing and dialect

`parse_cif` is the strict entry point. `Ok` means the parser
consumed the whole input. Errors are a `Vec<Rich<char>>`.
`rich_error` renders each one as an ariadne report against the
source text.

```rust
use crystal_cif_fmt::{parse_cif, rich_error, CifDialect};

let source = std::fs::read_to_string("1000000.cif").unwrap();

match CifDialect::detect(&source) {
    CifDialect::Cif1_1 => println!("CIF 1.1 value rules"),
    CifDialect::Cif2_0 => println!("CIF 2.0 value rules"),
}

let doc = match parse_cif(&source) {
    Ok(doc) => doc,
    Err(errors) => {
        for error in &errors {
            rich_error(error, "1000000.cif", &source);
        }
        std::process::exit(1);
    }
};
```

The dialect comes from the file start. The 2.0 magic code is
`##CIF_2.0` or the escaped `#\#CIF_2.0` spelling. Anything else
stays 1.1. The excerpt carries no version code. It stays 1.1.

### B. Document shape


A document holds data blocks. A block holds members. A member is
either data items or a save frame. The excerpt parses to one
block, `1000000`, with 19 members: keyword items and loops.
Real 1.1 structures carry few save frames. Appendix F shows
frames on a real 2.0 container file.

```rust
use crystal_cif_fmt::{DataBlockMember, DataItems};

for block in doc.data_blocks().unwrap() {
    println!("block {}", block.heading());
    for member in block.members() {
        match member {
            DataBlockMember::DataItems(items) => match items {
                DataItems::SingleValue(line) => println!("  item {}", line.tag()),
                DataItems::MultiValues(loops) => println!("  loop"),
            },
            DataBlockMember::SaveFrame(frame) => {
                println!("  frame {}", frame.heading());
            }
        }
    }
}
```

`get_data_block_by_name` fetches one block by its code, without
the `data_` prefix.

### C. Reading keyword fields


A single-line item is a `SingleLineData` pair: a tag and a value.
Look up by the item name, without the leading underscore. The
tag display adds the underscore.

```rust
use crystal_cif_fmt::Value;

let block = doc
    .get_data_block_by_name("1000000")
    .expect("the block is present");

let line = block
    .find_single_value_by_tag("cell_length_a")
    .expect("the tag is present");

println!("{} = {}", line.tag(), line.value());

let it = block
    .find_single_value_by_tag("space_group_IT_number")
    .expect("the tag is present");

println!("{}", it.value());
```

The value is a `Value`. Pick the variant you need:

```rust
let value = line.value();
if let Some(num) = value.as_numeric() {
    println!("{num}");
}
if let Some(text) = value.as_char_string() {
    println!("{}", text.content());
}
```

### D. Value variants


| variant | meaning | accessor |
|---|---|---|
| `Numeric(Numeric)` | a number, with an optional standard uncertainty | `as_numeric()` |
| `CharString(CharString)` | a quoted or unquoted string | `as_char_string()` |
| `TextField(TextField)` | a `;` semi-enclosed text block | `as_text_field()` |
| `List(Vec<Value>)` | a CIF 2.0 list value | `as_list()` |
| `Table(Vec<(Value, Value)>)` | a CIF 2.0 table value, ordered pairs | `as_table()` |
| `Inapplicable` | the `.` value | match |
| `Unknown` | the `?` value | match |

`CharString` covers the unquoted, single-quoted, double-quoted,
and the two triple-quoted 2.0 forms. `content()` returns the text
between the delimiters for every form. `TextField::raw_string()`
returns the lines joined with a newline.

The excerpt spans the variants. `_cell_angle_beta` is
`95.1470(10)`: a numeric with a standard uncertainty.
`_chemical_formula_sum` is a quoted char string.
`_publ_section_title` is a text field.

```rust
let beta = block.find_single_value_by_tag("cell_angle_beta").expect("present");
let num = beta.value().as_numeric().expect("a numeric value");
println!("{} esd={:?}", num, num.std_uncertainty());

let formula = block.find_single_value_by_tag("chemical_formula_sum").expect("present");
println!("{}", formula.value().as_char_string().expect("quoted").content());

let title = block.find_single_value_by_tag("publ_section_title").expect("present");
println!("{}", title.value().as_text_field().expect("a text field").raw_string());
```

`std_uncertainty` holds the esd in last-digit units. `95.1470(10)`
stores `10`. The display keeps the shortest round-trip form.

### E. Loops


A `loop_` section parses to `DataItems::MultiValues`. The type is
`LoopColumns`: one `LoopColumn` per tag. A column holds one value
per loop row, in row order.

```rust
use crystal_cif_fmt::Value;

let column = block
    .find_loop_column_by_tag("atom_site_label")
    .expect("the loop column is present");

println!("{} with {} rows", column.tag(), column.as_ref().len());
for value in column.values() {
    println!("{}", value.as_char_string().expect("a label").content());
}
```

`find_loop_column_by_tag` returns a clone of the column. When you
hold the `LoopColumns` value, use its `find_loop_column_by_tag`
for a borrow and `find_loop_column_mut_by_tag` for a mutation.

Row access across tags: a row `i` is `values[i]` of every
column. The loop body stores the values row by row. The
`LoopColumns` finders borrow instead of cloning. The walk
counts every loop. Unrelated loops report 0. The excerpt's
`atom_site` loop carries three rows: `P1`, `P2`, `Al1`.

```rust
use crystal_cif_fmt::DataItems;

for member in block.members() {
    if let DataBlockMember::DataItems(items) = member {
        if let DataItems::MultiValues(loops) = items {
            let labels = loops.find_loop_column_by_tag("atom_site_label");
            let symbols = loops.find_loop_column_by_tag("atom_site_type_symbol");
            // labels and symbols are Option<&LoopColumn>.
            let count = labels.map(|c| c.as_ref().len()).unwrap_or(0);
            let sym_count = symbols.map(|c| c.as_ref().len()).unwrap_or(0);
            println!("{count} label rows, {sym_count} symbol rows");
        }
    }
}
```

### F. Save frames

Real 1.1 structures carry few save frames. The pinned 2.0 test
file `tests/real_data/cif_api/simple_containers.cif` exercises
them: three blocks, five frames, one nested frame. Save it as
`simple_containers.cif`.

```cif
#\#CIF_2.0

# Tests multiple blocks in the same CIF, save frames in blocks, and data and
# save frame name scoping.

data_block1

save_s1
_location 'block1/s1'
save_

_location 'block1'

save_s2
_location 'block1/s2'
save_

data_block2

data_block3

save_s1
_location 'block3/s1'
save_

_location 'block3'

save_s3
save_
```

A save frame wraps items under a code. `heading()` returns the
frame code. Its display carries the `save_` prefix.
`data_items()` returns the items directly inside the frame.
`frames()` returns nested frames, which parse only in the 2.0
dialect. Frame names scope per block: `block1` and `block3` each
carry a `save_s1`.

```rust
use crystal_cif_fmt::{parse_cif, DataBlockMember, SaveFrame};

let cs = std::fs::read_to_string("simple_containers.cif").unwrap();
let cdoc = parse_cif(&cs).expect("the container file parses");

fn walk(frame: &SaveFrame, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{indent}frame {}", frame.heading());
    for child in frame.frames() {
        walk(child, depth + 1);
    }
}

for block in cdoc.data_blocks().unwrap() {
    for member in block.members() {
        if let DataBlockMember::SaveFrame(frame) = member {
            walk(frame, 0);
        }
    }
}
```

The walk prints `save_s1` and `save_s2` under `block1`. It
prints `save_s1` and `save_s3` under `block3`.

### G. Materializing typed terms


The dictionary layer types items. Each typed item implements
`CifTerm` (the tag) plus `SingleValueTerm` or `LoopValueTerm`
(the value or column). The traits turn items into data.

```rust
use crystal_cif_io::data_dict::core_cif::cell::{
    CellTerms, MeasurementReflnLoopItem,
};
use crystal_cif_io::data_dict::{LoopValueTerm, SingleValueTerm};
use crystal_cif_fmt::{
    CifDocument, DataBlock, DataBlockHeading, DataBlockMember, DataItems, LoopColumns,
};

let line = CellTerms::Length_a(7.8783.into()).to_single_value_data();

let h = MeasurementReflnLoopItem::Measurement_refln_index_h(vec![1.into(), 2.into()]);
let loop_items = DataItems::MultiValues(LoopColumns::new(vec![h.to_loop_column()]));

let block = DataBlock::from_heading_members((
    DataBlockHeading::new("1000000".into()),
    vec![
        DataBlockMember::DataItems(line),
        DataBlockMember::DataItems(loop_items),
    ],
));

let doc = CifDocument::new(None, Some(vec![block]));
println!("{doc}");
```

The loop column stands in for the excerpt's 5007-row
measurement loop. Two rows keep the example short. The writer
renders the 1.1 lexical form. The header is the escaped
version-code spelling `#\#CIF_1.1`. Closed code sets and range
checks live in the dictionary types. For example, `CellAngle`
clamps to 0..180 on construction.

## Features

- CIF 1.1 file syntax (IUCr 2014 specification).
- CIF 2.0 value rules for files with a 2.0 version code.
  `CifDialect::detect` detects the dialect from the file start. It
  accepts the IUCr `##CIF_2.0` spelling and the escaped
  `#\#CIF_2.0` spelling.
- A parser built on `chumsky` and `ariadne`. Errors render as
  ariadne reports with the source text labeled.
- A canonical IR (`CifDocument`) and a canonical JSON encoder
  (`crystal_cif_fmt::canonical`) with spec-level checks.
- `Value::List` for the CIF 2.0 list value, including nested
  lists.
- `Matrix` for matrix-shaped values, with dictionary shape checks.
- Typed dictionary items in `crystal_cif_io::data_dict`. The set
  covers all 732 IUCr core dictionary items recorded in
  `tests/dict_inventory.json` plus 262 core shell items (roadmap
  T6). Decision D2 skips the 191 `_su` items.
- Closed code sets and range checks for the dictionary items
  the dictionary constrains. The closed-set enums agree with
  the recorded dictionary. The gate
  `crystal_cif_io/tests/closed_values.rs` keeps them in step.
- The error types of both crates derive `thiserror`. They carry
  the message and the `source` chain.
- The IR value and block types of `crystal-cif-fmt` are
  re-exported at the crate root. The `SpaceGroup` type of
  `crystallographic-group` is re-exported under `group`. The
  read, edit, and write workflows stay inside this crate.
- The typed parse entry `parse_core_cif` returns one `CoreCif` per
  data block. The write step is `CoreCif::to_cif` for one block
  and `write_core_cif` for a multi-block file.
- A typed block view (`crystal_cif_io::CoreCif`) over the 62
  standard dictionary categories (roadmap D12i). Read a data
  block into typed structs, `validate` the required sets, and
  save the block to a file. Foreign tags and save frames stay
  in the `extra` residue.
- A per-category builder layer (`crystal_cif_io::builder`).
  One builder per equivalent minimum set. Needed fields are
  checked at compile time. Derived items compute at build and
  cross-check a supplied value. The space group resolves through
  the `crystallographic-group` crate.
- An inequality payload (`Ineq<T>`) records a `<N` assertion on
  a numeric item. The `refine_ls_shift/su_max` field carries
  it (roadmap D12l).
- The dictionary completeness study (`docs/dict-completeness.md`).
  The item dependency map lives in `codegen/depends.json` (94
  categories, 1185 items).

## Status

Version 0.2.0 released 2026-10-13 (the `dev-0.2` to `main`
merge). The dev line `dev-0.2` now carries the crate split of
decision D13. `crystal-cif-io` is at 0.3.0.
`crystal-cif-fmt` is at 0.1.0.

The typed document layer (D12) is complete on the dev line.
The 62 standard dictionary categories type into `CoreCif`.
Waves 2 through 5 roll them out in roadmap tier order (D12j).
The closed-value gate keeps the enum sets in step with the
recorded dictionary. `docs/roadmap-dict-items.md` records
every decision.

## Testing

The test suite pins the parser along several axes. Each axis owns
its corpus and its test. `docs/test-axes.md` carries the full
table.

```sh
cargo test
```

After a fresh copy of the repository, fetch the `cif_api` corpus
first:

```sh
python3 tests/real_data/cif_api/fetch.py
cargo test
```

The CI workflow runs the fetch before `cargo test`
(`.github/workflows/ci.yml`).

## Development examples

| command | purpose |
|---|---|
| `cargo run -p crystal-cif-fmt --example gt_generate` | regenerate the expected JSON for the fixture and real-data corpora |
| `cargo run -p crystal-cif-io --example dict_report` | print the dictionary coverage table |
| `cargo run -p crystal-cif-io --example dict_report -- --shell` | add the T6 shell view to the coverage table |
| `cargo run -p crystal-cif-io --example dict_codegen` | generate the typed category modules from the inventory and annotations |
| `cargo run -p crystal-cif-io --example dict_inventory_gen` | regenerate the standard dictionary inventory |
| `cargo run -p crystal-cif-io --example dict_inventory_shell_gen` | regenerate the shell dictionary inventory |
| `cargo run -p crystal-cif-io --example dict_coverage_gen` | regenerate the dictionary coverage baseline |

## Documentation

| document | topic |
|---|---|
| `docs/refactor-chumsky-ariadne.md` | parser design (`chumsky` + `ariadne`) |
| `docs/ground-truth.md` | the ground-truth harness and JSON schema |
| `docs/roadmap-dict-items.md` | the dictionary item roadmap |
| `docs/test-axes.md` | the test-suite axes |
| `docs/codegen.md` | the code generation step |
| `docs/dict-completeness.md` | the dictionary completeness study (D10a) |
| `docs/typed-structs.md` | the typed document layer (D12) |
| `docs/cod-api.md` | the COD REST API notes |

Third-party data attribution lives in `NOTICE.md`.

## License

MIT. See `LICENSE`.
