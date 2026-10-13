# crystal-cif-io

Parse and write crystallographic CIF (Crystallographic Information
File) data in Rust. The crate handles the IUCr CIF 1.1 file syntax
and the CIF 2.0 value rules. It also provides a typed dictionary
layer for the IUCr core CIF dictionary.

## Usage

Add the dependency:

```toml
[dependencies]
crystal-cif-io = "0.2"
```

The API tour below carries over across snippets. `doc` comes from
the parsing step. `block` comes from the keyword-fields step.
Every snippet compiles against the current crate. The types are
re-exported at the crate root, so the snippets need no module
paths.

The tour parses this file:

```cif
##CIF_1.1
data_cell
    _cell_length_a         5.0
    _cell_volume           123.4
loop_
    _atom_site_label
    _atom_site_type_symbol
    _atom_site_fract_x
    A1 C 0.1
    B2 O 0.4
save_protein
    _cell_volume           99.0
save_
```

### 1. Parsing and dialect

`parse_cif` is the strict entry point. `Ok` means the parser
consumed the whole input. Errors are a `Vec<Rich<char>>`.
`rich_error` renders each one as an ariadne report against the
source text.

```rust
use crystal_cif_io::{parse_cif, rich_error, CifDialect};

let source = std::fs::read_to_string("data.cif").unwrap();

match CifDialect::detect(&source) {
    CifDialect::Cif1_1 => println!("CIF 1.1 value rules"),
    CifDialect::Cif2_0 => println!("CIF 2.0 value rules"),
}

let doc = match parse_cif(&source) {
    Ok(doc) => doc,
    Err(errors) => {
        for error in &errors {
            rich_error(error, "data.cif", &source);
        }
        std::process::exit(1);
    }
};
```

The dialect comes from the file start. The 2.0 magic code is
`##CIF_2.0` or the escaped `#\#CIF_2.0` spelling. Anything else
stays 1.1.

### 2. Document shape


A document holds data blocks. A block holds members. A member is
either data items or a save frame.

```rust
use crystal_cif_io::{DataBlockMember, DataItems};

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

### 3. Reading keyword fields


A single-line item is a `SingleLineData` pair: a tag and a value.
Look up by the item name, without the leading underscore. The
tag display adds the underscore.

```rust
use crystal_cif_io::Value;

let block = doc
    .get_data_block_by_name("cell")
    .expect("the block is present");

let line = block
    .find_single_value_by_tag("cell_length_a")
    .expect("the tag is present");

println!("{} = {}", line.tag(), line.value());
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

### 4. Value variants


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

### 5. Loops


A `loop_` section parses to `DataItems::MultiValues`. The type is
`LoopColumns`: one `LoopColumn` per tag. A column holds one value
per loop row, in row order.

```rust
use crystal_cif_io::Value;

let column = block
    .find_loop_column_by_tag("atom_site_label")
    .expect("the loop column is present");

println!("{} with {} rows", column.tag(), column.as_ref().len());
for value in column.values() {
    println!("{}", value.as_char_string().expect("a quoted label").content());
}
```

`find_loop_column_by_tag` returns a clone of the column. When you
hold the `LoopColumns` value, use its `find_loop_column_by_tag`
for a borrow and `find_loop_column_mut_by_tag` for a mutation.

Row access across tags: a row `i` is `values[i]` of every
column. The loop body stores the values row by row. The
`LoopColumns` finders borrow instead of cloning.

```rust
use crystal_cif_io::DataItems;

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

### 6. Save frames


A save frame wraps items under a code. `heading()` returns the
code, without the `save_` prefix. `data_items()` returns the
items directly inside the frame. `frames()` returns nested
frames, which parse only in the 2.0 dialect.

```rust
use crystal_cif_io::{DataBlockMember, SaveFrame};

fn walk(frame: &SaveFrame, depth: usize) {
    let indent = "  ".repeat(depth);
    println!("{indent}frame {}", frame.heading());
    for item in frame.data_items() {
        // The same DataItems API as block-level members.
    }
    for child in frame.frames() {
        walk(child, depth + 1);
    }
}

for member in block.members() {
    if let DataBlockMember::SaveFrame(frame) = member {
        walk(frame, 0);
    }
}
```

### 7. Building and writing


The dictionary layer types items. Each typed item implements
`CifTerm` (the tag) plus `SingleValueTerm` or `LoopValueTerm`
(the value or column). The traits turn items into data.

```rust
use crystal_cif_io::data_dict::core_cif::cell::{
    CellTerms, MeasurementReflnLoopItem,
};
use crystal_cif_io::data_dict::{LoopValueTerm, SingleValueTerm};
use crystal_cif_io::{
    CifDocument, DataBlock, DataBlockHeading, DataBlockMember, DataItems, LoopColumns,
};

let line = CellTerms::Length_a(5.0.into()).to_single_value_data();

let h = MeasurementReflnLoopItem::Measurement_refln_index_h(vec![1.into(), 2.into()]);
let loop_items = DataItems::MultiValues(LoopColumns::new(vec![h.to_loop_column()]));

let block = DataBlock::from_heading_members((
    DataBlockHeading::new("cell".into()),
    vec![
        DataBlockMember::DataItems(line),
        DataBlockMember::DataItems(loop_items),
    ],
));

let doc = CifDocument::new(None, Some(vec![block]));
println!("{doc}");
```

The writer renders the 1.1 lexical form. The header is the
escaped version-code spelling `#\#CIF_1.1`. Closed code sets and
range checks live in the dictionary types. For example,
`CellAngle` clamps to 0..180 on construction.

### 8. Matrices


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

### 9. Canonical JSON and validation


`canonical::document_json` encodes a document as the canonical
IR. `validate` runs the spec-level checks and returns the
violations.

```rust
use crystal_cif_io::canonical;

let json = canonical::document_json(&doc, "data.cif", 2);
println!("{}", serde_json::to_string(&json).unwrap());

for violation in canonical::validate(&doc) {
    eprintln!("violation: {violation}");
}
```

The schema version is 1 for 1.1 value kinds and 2 when the
document carries 2.0 kinds (`list`, `table`, the triple-quote
char-string styles, the frame `frames` array). The `validate`
checks flag duplicate block names and duplicate tags.

### 10. CIF 2.0 specifics


Lists, tables, triple-quoted strings, nested save frames, and
non-ASCII text parse only in the 2.0 dialect. The 1.1 dialect
keeps the ASCII character set.

```rust
let src = r#"#\#CIF_2.0
data_t
    _map { 'a':1 'b':2 }
"#;
let doc = parse_cif(src).expect("the 2.0 file parses");
let block = doc.get_data_block_by_name("t").expect("the block is present");
let line = block.find_single_value_by_tag("map").expect("the tag is present");

for (key, value) in line.value().as_table().expect("a table") {
    println!("pair: {} = {}", key, value);
}
```

Table pairs keep their order. The key is a value, usually a
char string.
## Features

- CIF 1.1 file syntax (IUCr 2014 specification).
- CIF 2.0 value rules for files with a 2.0 version code.
  `CifDialect::detect` detects the dialect from the file start. It
  accepts the IUCr `##CIF_2.0` spelling and the escaped
  `#\#CIF_2.0` spelling.
- A parser built on `chumsky` and `ariadne`. Errors render as
  ariadne reports with the source text labeled.
- A canonical IR (`CifDocument`) and a canonical JSON encoder
  (`crystal_cif_io::canonical`) with spec-level checks.
- `Value::List` for the CIF 2.0 list value, including nested
  lists.
- `Matrix` for matrix-shaped values, with dictionary shape checks.
- Typed dictionary items in `crystal_cif_io::data_dict`. The set
  covers all 732 IUCr core dictionary items recorded in
  `tests/dict_inventory.json` plus 262 core shell items (roadmap
  T6). Decision D2 skips the 191 `_su` items.
- Closed code sets and range checks for the dictionary items the
  dictionary constrains.

## Status

Version 0.2.0. The dictionary item roadmap is complete.
`docs/roadmap-dict-items.md` records the shell wave completion
(2026-10-09).

Seven CIFs in the `cif_api` corpus expose known 2.0 grammar gaps.
The gaps: tables, comments inside lists, triple-quoted strings,
nested save frames, non-ASCII values, VT/FF line terminators. The
`known_gap` pins live in `tests/real_data/expected/`.

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
| `cargo run --example gt_generate` | regenerate the expected JSON for the fixture and real-data corpora |
| `cargo run --example dict_report` | print the dictionary coverage table |
| `cargo run --example dict_report -- --shell` | add the T6 shell view to the coverage table |
| `cargo run --example dict_codegen` | generate the typed category modules from the inventory and annotations |
| `cargo run --example dict_inventory_gen` | regenerate the standard dictionary inventory |
| `cargo run --example dict_inventory_shell_gen` | regenerate the shell dictionary inventory |
| `cargo run --example dict_coverage_gen` | regenerate the dictionary coverage baseline |

## Documentation

| document | topic |
|---|---|
| `docs/refactor-chumsky-ariadne.md` | parser design (`chumsky` + `ariadne`) |
| `docs/ground-truth.md` | the ground-truth harness and JSON schema |
| `docs/roadmap-dict-items.md` | the dictionary item roadmap |
| `docs/test-axes.md` | the test-suite axes |
| `docs/codegen.md` | the code generation step |
| `docs/cod-api.md` | the COD REST API notes |

Third-party data attribution lives in `NOTICE.md`.

## License

MIT. See `LICENSE`.
