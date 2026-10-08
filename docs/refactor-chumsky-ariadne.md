# Refactor the parser to `chumsky` + `ariadne`

Status: **Phase 2 implemented** (2026-10-07). This is Phase 2 of the
CIF work. Phase 1 (groundtruth) is done.
See [`ground-truth.md`](ground-truth.md).
Decision recorded 2026-10-06. Implementation recorded 2026-10-07.
Post-review defect fix (unreachable semantic diagnostics) recorded
2026-10-07. Chemrust layer removed 2026-10-07.

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

## Chemrust layer removed (2026-10-07)

The `chemrust-core` optional dependency (and its companions
`crystallographic-group` and `nalgebra`) was removed entirely.
This dissolved the `--no-default-features` defect above: with no
optional dependency left, the crate builds with no features and
the old defect can no longer occur.

- Deleted the four `chemrust_impl.rs` files in `src/grammar/` and
  `src/data_dict/core_cif/` (cell, atom_site, space_group).
- Removed the `chemrust-core` feature and the three optional
  dependencies from `Cargo.toml`.
- Made `space_group/crystal_system.rs` dependency-free: dropped the
  ungated `crystallographic_group` import and the chemrust
  `From<CrystalSystem>` conversion. The CIF-native `CrystalSystemCif`
  enum, its `Display`, and its `TryFrom` stay.
- Removed the now-dead `default_audit_data` helper (its only caller
  was the deleted `chemrust_impl`).

The crate is now a pure CIF parse, IR, canonical-JSON, and data-
dictionary layer. It no longer converts to chemrust types. The
data-dictionary entries keep their CIF-native term types; the
chemrust conversion will be re-added, if at all, as its own future
layer.

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

## Post-commit cleanup (2026-10-07)

After the Phase-2 commit, the remaining clippy findings were resolved in
two follow-up commits:

- **Cosmetic fixes (14).** `cargo clippy --fix` applied all 14
  machine-applicable lints (redundant references, an elided lifetime,
  no-op `f64` conversions, a `first()` call, a char-array trim, and a
  dropped `clone` on a `Copy` combinator). All are
  behavior-preserving.
- **`Violation` as a `Display` impl.** The hand-written `to_string`
  method on `Violation` in `src/canonical.rs` was replaced with a
  `Display` impl. The formatted strings are byte-identical, so the
  `v.to_string()` call sites in the tests and examples keep the same
  output.

`cargo clippy` is now clean. `cargo test` stays green and
`gt_generate --force` still writes nothing.

## Dependency changes (2026-10-07)

- Dropped `chrono`. Its only user was the deleted
  `default_audit_data` helper.
- Dropped the orphaned `indexmap` optional dependency. No feature
  referenced it after the `chemrust-core` feature was removed.
- Replaced `castep-periodic-table` with `elements_rs 0.2.8`,
  default features off so serde is not pulled in.
  - `elements_rs` 0.1.x through 0.2.7 are GPL-3.0. Version 0.2.8 is
    MIT, which matches this crate's license. 0.2.8 is therefore the
    version used.
  - `atom_site/label_symbol.rs` now holds `elements_rs::Element`
    instead of the castep `ElementSymbol`. The `TypeSymbolCode`
    variant `ElementSymbol` is renamed `Element` to match. `Element`
    prints its symbol via `Display`, so `TypeSymbol` output is
    unchanged.

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
