# Mission: review tier t2 (dict-items roadmap)

You are reviewing the merged tier t2 work of crystal-cif-io.
Work in the main repo at /home/tony/programming/crystal-cif-io
(branch dev-0.2). All impl tiers (T0, T2, T3, T4, T5) are merged
here. Read docs/roadmap-dict-items.md sections 6 (tier t2) and 7
(verification) first.

This is a read-only review: do not modify source code, fixtures,
or tests. You may run cargo commands.

## Scope

Tier t2 categories: exptl (9), exptl_crystal (24), exptl_crystal_face (8), refln (31), reflns (17), reflns_class (11), reflns_scale (4), reflns_shell (23), refine (46), refine_ls_class (8), symmetry (4), symmetry_equiv (2) = 187 items

## Checks (run all of them)

1. cargo test is green (run it; capture the summary).
2. cargo run --example dict_report: every tier t2 category is at
   full coverage.
3. Conventions: the tier modules follow the cell module pattern
   (src/data_dict/core_cif/cell/mod.rs). Spot-check five items:
   closed code sets are typed enums, range constraints are
   write-time checks, tags are in the tag registry, and the
   _enumeration metadata in tests/dict_inventory.json matches the
   code.
4. The tier fixture tests/fixtures/14_reflns.cif (the refln loop form) exists with its expected JSON and passes
   ground_truth_verify.
5. No new compiler warnings in the cargo test output.
6. No conflict-marker leftovers in the tree (search the whole tree
   for lines that start with the three-character markers used by
   git conflict output, in src, tests and examples).
7. Tier t2 merge commit(s) on dev-0.2 (git log): the diff scope
   is limited to the tier's categories, the pub mod lines, the tag
   registry, and its fixture. Flag any out-of-scope file.

## Output

Write your full report (check results, findings by severity P0
blocker / P1 important / P2 minor, with file and line references)
to sessions/dict-items-review-t2/report.md (your session dir).
End your final message with a one-paragraph verdict:
PASS, PASS-WITH-NOTES, or FAIL, plus your top findings.
