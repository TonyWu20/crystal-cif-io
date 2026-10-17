# Mission: review D12 wave 1 (the typed document layer)

## Context (you start from zero context)

The repo is `crystal-cif-io`, a two-crate workspace
(`crystal-cif-fmt`: parser and IR; `crystal-cif-io`: `data_dict`
and `builder`). You are in the worktree
`/home/tony/programming/crystal-cif-io/.worktrees/dict-items-d12`
on branch `dict-items/d12`, which is unmerged. The diff under
review is `git diff dev-0.2..dict-items/d12` (three commits).

Read first:

- `docs/typed-structs.md`: the design, decisions D12a to D12h,
  D12i, D12j, D13a to D13d.
- `sessions/dict-items-d12/summary.md`: the mission's own
  summary, including the declared deviations.

This is a read-only review. Do not modify source, fixtures, or
tests. You may run cargo commands.

## Checks (run all of them)

1. `cargo test` green in the worktree, both crates. Capture the
   summary lines. No new compiler warnings.
2. Scope: the diff touches only `crystal_cif_io`
   (`src/data_dict/core_cif`, `src/core_cif.rs`, `src/lib.rs`,
   `src/builder`, `tests/`, `CHANGELOG.md`). No
   `crystal_cif_fmt` change. Flag any out-of-scope file.
3. Design conformance (typed-structs.md sections 4 to 7): the
   `CifCategory` trait in `data_dict/core_cif/mod.rs`; `CoreCif`
   in the crate root with the five groups plus `extra`;
   `from_block`, `to_block`, `validate`; the `CifError`
   variants `ValueClass`, `Shape`, `Missing` with
   expected/actual Display wording.
4. The five structs live in `<cat>/<cat>.rs`, all fields
   `Option<T>`, Default. Closed set items type to enums.
   Ranged items keep their clamped newtypes. Plain items keep
   the shared value type. Spot-check five fields per struct
   against the field roles in
   `docs/dict-completeness.md` section 5.
5. D12h: the five builders' `build()` returns the doc structs.
   No builder-private structs remain. The `bon` input surface
   (setters, names, required fields) is unchanged.
6. Residue: save frames and unknown tags land in `extra`,
   order kept. The `_su` items are skipped.
7. Tests: `tests/typed_round_trip.rs` exists and covers the
   round trip over the `09` and `13` fixtures, the residue,
   the `validate` missing counts, and the builder spot check.
   The dict-inventory gate still passes.
8. `CHANGELOG.md`: an Unreleased entry names the new layer and
   the breaking builder change.
9. No conflict markers anywhere in the tree.
10. Judge the three declared deviations (the tuple return of
    `from_members`, the loop gating on the required identity
    tags, the saturating cast) against decisions D12b and
    D12d. Each is consistent or a finding.

## Output

Write the full report to
`sessions/dict-items-review-d12/report.md` (your session dir):
the check results, findings by severity P0 blocker / P1
important / P2 minor, each with file and line references.
End your final message with a one-paragraph verdict: PASS,
PASS-WITH-NOTES, or FAIL, plus your top findings.
