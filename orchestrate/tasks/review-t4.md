# Mission: review tier t4 (dict-items roadmap)

You are reviewing the merged tier t4 work of crystal-cif-io.
Work in the main repo at /home/tony/programming/crystal-cif-io
(branch dev-0.2). All impl tiers (T0, T2, T3, T4, T5) are merged
here. Read docs/roadmap-dict-items.md sections 6 (tier t4) and 7
(verification) first.

This is a read-only review: do not modify source code, fixtures,
or tests. You may run cargo commands.

## Scope

Tier t4 categories: audit_author (2), audit_conform (3), audit_contact_author (5), audit_link (2), citation (26), citation_author (3), citation_editor (3), publ (32), publ_author (6), publ_body (5), publ_manuscript_incl (3), journal (38), journal_index (3), computing (7), database (15) = 153 items

## Checks (run all of them)

1. cargo test is green (run it; capture the summary).
2. cargo run --example dict_report: every tier t4 category is at
   full coverage.
3. Conventions: the tier modules follow the cell module pattern
   (src/data_dict/core_cif/cell/mod.rs). Spot-check five items:
   closed code sets are typed enums, range constraints are
   write-time checks, tags are in the tag registry, and the
   _enumeration metadata in tests/dict_inventory.json matches the
   code.
4. The tier fixture none required for this tier (char single-value and loop items; say so if the impl session recorded a reason) exists with its expected JSON and passes
   ground_truth_verify.
5. No new compiler warnings in the cargo test output.
6. No conflict-marker leftovers in the tree (search the whole tree
   for lines that start with the three-character markers used by
   git conflict output, in src, tests and examples).
7. Tier t4 merge commit(s) on dev-0.2 (git log): the diff scope
   is limited to the tier's categories, the pub mod lines, the tag
   registry, and its fixture. Flag any out-of-scope file.

## Output

Write your full report (check results, findings by severity P0
blocker / P1 important / P2 minor, with file and line references)
to sessions/dict-items-review-t4/report.md (your session dir).
End your final message with a one-paragraph verdict:
PASS, PASS-WITH-NOTES, or FAIL, plus your top findings.
