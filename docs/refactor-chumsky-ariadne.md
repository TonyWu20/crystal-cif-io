# Refactor the parser to `chumsky` + `ariadne`

Status: **Phase 2 implemented** (2026-10-07). This is Phase 2 of the
CIF work. Phase 1 (groundtruth) is done.
See [`ground-truth.md`](ground-truth.md).
Decision recorded 2026-10-06. Implementation recorded 2026-10-07.
Post-review defect fix (unreachable semantic diagnostics) recorded
2026-10-07.

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

Dependencies to add: `chumsky 0.13.0` and `ariadne 0.6.0` with the
`auto-color` feature. Both are the latest stable releases, checked on
crates.io on 2026-10-06.

Version note: `castep_cell_fmt` pins `chumsky 0.10.1`. This crate uses
`chumsky 0.13.0` instead. The chumsky changelog records additions only
between 0.10 and 0.13. The 0.10 pattern ports over with minor edits.
In practice the port hit real API shape changes: `text::any()` is
private in 0.13 (use the prelude `any()`), `one_of` takes three type
parameters, `repeated()`/`separated_by()` return `IterParser` and need
`.collect::<Vec<_>>()`, and a `validate` closure returns the output
and cannot fail the parse (use `filter` for hard failures).
The `chumsky 1.0.0-alpha` pre-release line is not chosen.

## Binding acceptance criteria

The groundtruth is the gate. All of the following must hold:

1. `cargo test ground_truth_verify` passes for all 18 fixtures
   (12 positive, 6 negative).
2. The `known_gap` in `09_realistic_structure.json` is closed.
   Mid-line `.` and `?` must encode as `inapplicable` and `unknown`,
   not `char_string`. Then remove the flag by hand.
3. After the flag is removed, `cargo run --example gt_generate
   -- --force` writes nothing. The expected files stay byte-stable.
4. Spec-level checks in `src/canonical.rs` (`validate`) still run and
   pass. The negative fixtures keep their `stage` and `violation`.

All four criteria hold as of 2026-10-07.

## Open decisions, resolved on 2026-10-07

| item | resolution |
|---|---|
| Public parse entry | add free `parse_cif` / `rich_error` in the `castep_cell_fmt` style. Drop the winnow `PResult` entry. |
| IR types | keep the current public IR. `src/canonical.rs` stays unchanged. |
| `src/grammar/` | delete the winnow parser after the switch. Keep the IR types. |
| pest leftovers | delete `src/cif.pest` and the `pest*` dependencies. |

## Rules fixed during implementation

- `Float` is now `f64` (was `f32`). Every fixture number is stable
  under the change.
- An unquoted value token may not begin with a reserved stem
  (`loop_`, `save_`, `data_`, `stop_`, `global_`, any case).
  A stem with a code (for example `save_foo`) starts a block or save
  frame and is never a value. Fixture `12_mixed_case` requires the
  rule.
- The `known_gap` of fixture `09` is closed. Mid-line `.` and `?`
  now encode as `inapplicable` and `unknown`.
  The `known_gap` flag and its note are removed by hand.
- **Post-parse semantic enrichment pass** (choice B, post-review
  defect fix). chumsky backtracking `filter`s truncate the
  `Rich::custom` diagnostics that `validate` emits when they rewind,
  so the three semantic messages never reach the caller. The `Err`
  branch of `parse_cif` now calls `semantic_diagnoses(input)`. That
  pass tokenizes the raw source, re-derives the three checks with
  byte-accurate spans, and appends them to the returned `Vec<Rich>`.
  A successful parse stays untouched. The `parse_cif` signature and
  `Err` contract are unchanged.

## Known pre-existing defect (out of scope)

`cargo test --no-default-features` fails on `HEAD`.
The `crystallographic_group` imports in `src/data_dict/core_cif/`
are not feature-gated behind `chemrust-core`. This defect predates
the parser work and is not fixed here.

## Post-review defect fix: unreachable semantic diagnostics (2026-10-07)

A post-review probe showed that the three semantic `Rich::custom`
diagnostics emitted inside the parser never reach the caller. chumsky
backtracking `filter`s truncate emitted errors on rewind, so only the
opaque `expected/found` errors surface. The fix is a post-parse
enrichment pass (choice B):

- The `Err` branch of `parse_cif` extends the returned `Vec<Rich>`
  with the output of `semantic_diagnoses(input)`. That function
  re-derives the three checks from the raw source with byte spans.
- A successful parse stays untouched. The `parse_cif` signature and
  `Err` contract are unchanged.
- `mod tests` in `src/parser/mod.rs` asserts the custom diagnostics
  reach the caller. That test detects the original defect.

The three checks:

| check | condition |
|---|---|
| reserved word as value | a bare `loop_`, `save_`, and so on stands where a value belongs |
| reserved stem begins a value | `save_foo`, `Data_bar` stand where a value belongs |
| empty loop | a `loop_` with no header tags |

Verification: `cargo test` is green (6 unit tests and
`ground_truth_verify` for all 18 fixtures). The demonstrated case
`data_d` / `_a save_foo` now surfaces the message
"a reserved word stem with a code, `save_foo`, may not begin a
value". `gt_generate --force` writes nothing.

## Phases

| phase | state |
|---|---|
| 1. Groundtruth: fixtures, expected IR, harness | done, commit `3adcd36` |
| 2. Parser refactor to `chumsky` + `ariadne` | done, 2026-10-07 |
| 3. Full CIF data-dictionary coverage | planned |

## Start prompt for the next session

> (consumed 2026-10-07, Phase 2 complete)
>
> Implement Phase 2 per `docs/refactor-chumsky-ariadne.md`. Build the
> chumsky + ariadne parser modeled on `castep_cell_fmt`. Acceptance is
> `cargo test ground_truth_verify` green for all fixtures plus
> closure of the fixture `09` gap, per the binding criteria in that
> document.
