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

Parse a file and read a value:

```rust
use crystal_cif_io::{parse_cif, rich_error};

let source = std::fs::read_to_string("data.cif").unwrap();

let doc = match parse_cif(&source) {
    Ok(doc) => doc,
    Err(errors) => {
        for error in &errors {
            rich_error(error, "data.cif", &source);
        }
        std::process::exit(1);
    }
};

for block in doc.data_blocks().unwrap() {
    if let Some(value) = block.find_single_value_by_tag("cell_length_a") {
        println!("{} = {}", value.tag(), value.value());
    }
}
```

`parse_cif` is the strict entry point. `Ok` means the parser
consumed the whole input. The error type is `Vec<Rich<char>>`.
`rich_error` renders each error as an ariadne report with the
source text labeled.

Build a document and write it:

```rust
use crystal_cif_io::{
    data_dict::{core_cif::cell::CellTerms, SingleValueTerm},
    CifDocument, DataBlock, DataBlockHeading, DataBlockMember,
};

let line = CellTerms::Length_a(5.1.into()).to_single_value_data();
let block = DataBlock::from_heading_members((
    DataBlockHeading::new("demo".into()),
    vec![DataBlockMember::DataItems(line)],
));
let doc = CifDocument::new(None, Some(vec![block]));

print!("{doc}");
```

The API guide (`docs/api-guide.md`) walks through the public
surface. It covers keyword fields, loop columns, save frames,
value variants, typed dictionary items, matrices, canonical JSON,
and the CIF 2.0 specifics.

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
| `docs/api-guide.md` | the public API tour |
| `docs/refactor-chumsky-ariadne.md` | parser design (`chumsky` + `ariadne`) |
| `docs/ground-truth.md` | the ground-truth harness and JSON schema |
| `docs/roadmap-dict-items.md` | the dictionary item roadmap |
| `docs/test-axes.md` | the test-suite axes |
| `docs/codegen.md` | the code generation step |
| `docs/cod-api.md` | the COD REST API notes |

Third-party data attribution lives in `NOTICE.md`.

## License

MIT. See `LICENSE`.
