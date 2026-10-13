# Mission: the README typed tour (D12 wave 1)

## Context (you start from zero context)

The repo is `crystal-cif-io`, a two-crate workspace
(`crystal-cif-fmt`: parser and IR; `crystal-cif-io`:
`data_dict`, `builder`). You are in the worktree
`/home/tony/programming/crystal-cif-io/.worktrees/dict-items-d12-readme`
on branch `dict-items/d12-readme`, based on the unmerged branch
`dict-items/d12`.

The D12 wave 1 (branch `dict-items/d12`) adds the typed
document layer: the `CoreCif` view in `crystal_cif_io::core_cif`,
the five category structs in
`crystal_cif_io::data_dict::core_cif`, and the five `bon`
builders now return the doc structs (breaking, pre-1.0).

Read first:

- `docs/typed-structs.md` section 1: the show-to-users list.
- The current `README.md` tour.
- The real API in `crystal_cif_io/src/core_cif.rs` and
  `src/builder/`: match the tour code to the actual
  signatures, not to the docs.

## The work

1. Update the README tour to the typed API:
   - parse, `CoreCif::from_block`, read struct fields,
     `validate`, `to_block`, write back.
   - builder usage: `build()` returns the doc structs
     (`Cell`, `SpaceGroup`, `Symop`, `ChemicalFormula`). The
     `bon` setters are unchanged.
   - the `extra` residue: foreign tags and save frames.
2. Replace every builder-example that used the deleted
   builder-private structs. The tour code must compile against
   the branch API.
3. One line notes the breaking change, pre-1.0.
4. Keep the existing tour structure and style.
5. No source change: docs only. Run `cargo check` to prove the
   tour snippets match the API.

## Done means

- One or two commits on branch `dict-items/d12-readme`.
- `sessions/dict-items-d12-readme/summary.md`: what changed,
  which sections, the snippet list.
- Do not merge. The orchestrator merges after review.
