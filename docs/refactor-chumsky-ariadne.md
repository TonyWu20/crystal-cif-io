# Refactor the parser to `chumsky` + `ariadne`

Status: **decided, not started.** This is Phase 2 of the CIF work.
Phase 1 (groundtruth) is done. See [`ground-truth.md`](ground-truth.md).
Decision recorded 2026-10-06.

## Decision

Replace the `winnow` parser in `src/grammar/` with a parser built on
`chumsky` and `ariadne`. The model is `castep_cell_fmt` in the
`castep-cell-io` repository.

## Rationale

- `ariadne` reports anchor errors to the source with line, column,
  and snippet labels. CIF users get actionable diagnostics.
- `chumsky` gives one explicit combinator per CIF production. The
  mid-line `.`/`?` gap of the current parser (fixture `09`) becomes a
  local, testable fix.
- `crystal-cif-io` and `castep-cell-io` share one parser and error
  style. This aligns the two crates in this workspace.

## Model to follow

Reference: `/home/tony/programming/castep-cell-io/castep_cell_fmt`

| file | role |
|---|---|
| `src/parser.rs` | one chumsky combinator per production. Entry point `parse_cell_file(input) -> Result<Vec<Cell<'a>>, Vec<Rich<'a, char>>>` |
| `src/error.rs` | `CResult<T>` result alias, `Error` enum via `thiserror` |
| `src/parser.rs::rich_error` | renders an `ariadne` report with the source text labeled |

Dependencies to add: `chumsky 0.10.1` and `ariadne 0.6` with the
`auto-color` feature. Match the versions in `castep_cell_fmt`.

## Binding acceptance criteria

The groundtruth is the gate. All of the following must hold:

1. `cargo test ground_truth_verify` passes for all 19 fixtures.
2. The `known_gap` in `09_realistic_structure.json` is closed.
   Mid-line `.` and `?` must encode as `inapplicable` and `unknown`,
   not `char_string`. Then remove the flag by hand.
3. After the flag is removed, `cargo run --example gt_generate
   -- --force` writes nothing. The expected files stay byte-stable.
4. Spec-level checks in `src/canonical.rs` (`validate`) still run and
   pass. The negative fixtures keep their `stage` and `violation`.

## Open decisions for the next session

| item | options |
|---|---|
| Public parse entry | keep `CifDocument::parse_from_str` (winnow `PResult`) or add a `castep_cell_fmt`-style free function returning `Vec<Rich<char>>` |
| IR types | keep the current public IR (`CifDocument`, `Value`, ...) or introduce new types and adapt `src/canonical.rs` |
| `src/grammar/` | delete the winnow parser after the switch |
| pest leftovers | `src/parser/mod.rs` (`CIFParser`, never constructed), `src/cif.pest`, and the `pest*` dependencies. Delete them, or keep `cif.pest` as a grammar reference |

## Phases

| phase | state |
|---|---|
| 1. Groundtruth: fixtures, expected IR, harness | done, commit `3adcd36` |
| 2. Parser refactor to `chumsky` + `ariadne` | decided, not started |
| 3. Full CIF data-dictionary coverage | planned |

## Start prompt for the next session

> Implement Phase 2 per `docs/refactor-chumsky-ariadne.md`. Build the
> chumsky + ariadne parser modeled on `castep_cell_fmt`. Acceptance is
> `cargo test ground_truth_verify` green for all 19 fixtures plus
> closure of the fixture `09` gap, per the binding criteria in that
> document.
