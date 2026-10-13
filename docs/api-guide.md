# API guide

A practical tour of the public API. The examples carry over
across sections: `doc` comes from section 1, `block` from
section 3. The snippets compile against the current crate. The
types are re-exported at the crate root, so the snippets need no
module paths.

The examples parse this file:

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

## 1. Parsing and dialect

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

## 2. Document shape

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

## 3. Reading keyword fields

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

## 4. Value variants

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

## 5. Loops

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

## 6. Save frames

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

## 7. Building and writing

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

## 8. Matrices

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

## 9. Canonical JSON and validation

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

## 10. CIF 2.0 specifics

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
