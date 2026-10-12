# crystal-cif-io
A crate to parse, edit and write crystallographic `cif` format. Targeting to read/write most basic atom coordinates and lattice parameters first.

## Testing

The CIF 1.1 syntax groundtruth (reference `.cif` fixtures, expected
canonical-IR JSON, and the parser-agnostic harness) lives in
[`tests/fixtures`](tests/fixtures/). The schema and known gaps of the
current parser are documented in
[`docs/ground-truth.md`](docs/ground-truth.md).

Real crystallographic files are pinned by the real-data harness in
[`tests/real_data`](tests/real_data/): IUCr published examples
(`tests/real_data/iucr/`), Crystallography Open Database entries
(`tests/real_data/cod/`, fetched via the COD REST API; see
[`docs/cod-api.md`](docs/cod-api.md)), one entry per space group
1..230 (`tests/real_data/cod_sg/`), and the 21 edge-case CIFs of
the COMCIFS [`cif_api`](https://github.com/COMCIFS/cif_api)
reference implementation (`tests/real_data/cif_api/`; provenance
and license in `tests/real_data/cif_api/SOURCE.md`). Those CIFs
are fetched at test time, not committed: the CI workflow
(`.github/workflows/ci.yml`) and the local fetch script
(`python3 tests/real_data/cif_api/fetch.py`) download them from
a pinned upstream commit and verify the blob SHAs. Provenance
lives in `tests/real_data/SOURCE.md` and the per-corpus
`SOURCE.md` files.

The suite is organized as axes: syntax ground truth, real-file
pins, space-group coverage, lexical/structural feature coverage,
and dictionary-item coverage. The design and the deferred axes
are in [`docs/test-axes.md`](docs/test-axes.md).

The real-data corpora are third-party data. The COD corpora come
from the Crystallography Open Database
(`https://www.crystallography.net/`), the IUCr corpus holds IUCr
published example files, and the cif_api corpus is the test data
of the COMCIFS reference implementation (LGPL-3.0). The credit,
the license notices, and the required article citations live in
[`NOTICE.md`](NOTICE.md).

```sh
cargo test                                        # run everything
cargo test ground_truth_verify                    # check parser against the groundtruth
cargo test space_group_coverage feature_coverage dictionary_item_coverage
cargo run --example gt_generate                   # write missing expected JSON
cargo run --example gt_generate -- --force        # overwrite expected JSON (guarded)
cargo run --example dict_coverage_gen             # re-record the dict-coverage baseline
```

## Roadmap

The parser refactor (chumsky + ariadne, modeled on `castep_cell_fmt`)
lives in
[`docs/refactor-chumsky-ariadne.md`](docs/refactor-chumsky-ariadne.md).

The remaining CIF dictionary items for the CIF 1.1 standard carry their
own roadmap in
[`docs/roadmap-dict-items.md`](docs/roadmap-dict-items.md).
