# crystal-cif-io
A crate to parse, edit and write crystallographic `cif` format. Targeting to read/write most basic atom coordinates and lattice parameters first.

## Testing

The CIF 1.1 syntax groundtruth (reference `.cif` fixtures, expected
canonical-IR JSON, and the parser-agnostic harness) lives in
[`tests/fixtures`](tests/fixtures/). The schema and known gaps of the
current parser are documented in
[`docs/ground-truth.md`](docs/ground-truth.md).

```sh
cargo test                                        # run everything
cargo test ground_truth_verify                    # check parser against the groundtruth
cargo run --example gt_generate                   # write missing expected JSON
cargo run --example gt_generate -- --force        # overwrite expected JSON (guarded)
```

## Roadmap

The parser refactor (chumsky + ariadne, modeled on `castep_cell_fmt`)
lives in
[`docs/refactor-chumsky-ariadne.md`](docs/refactor-chumsky-ariadne.md).

The remaining CIF dictionary items for the CIF 1.1 standard carry their
own roadmap in
[`docs/roadmap-dict-items.md`](docs/roadmap-dict-items.md).
