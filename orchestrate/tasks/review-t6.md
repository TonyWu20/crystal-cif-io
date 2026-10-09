# Mission: review T6 (core shell additions, dict-items roadmap)

You are reviewing the merged T6 work of crystal-cif-io.
Work in the main repo at /home/tony/programming/crystal-cif-io
(branch dev-0.2). All impl tiers, D8, and T6 are merged here.
Read docs/roadmap-dict-items.md sections 4, 6 (T6), 7, 9, 10,
and 12.3 first.

This is a read-only review: do not modify source, fixtures, or
tests. You may run cargo commands.

## Scope

The T6 tier: the shell inventory, the 347-item set (minus the
D2-skipped _su items), the Matrix payload binding (section 12.3),
the codegen-driven modules, the shell inventory test, the dict_report
shell coverage, and the 18_shell.cif fixture.

## Checks (run all of them)

1. cargo test is green (run it; capture the summary).
2. The standard inventory test and the shell inventory test both
   pass. dict_report against the shell inventory shows every T6
   category at full coverage.
3. Conventions: spot-check five T6 items. Closed code sets are
   typed enums, range constraints are write-time checks, tags are
   in the registry, and the shell inventory metadata matches the
   code. Check the Matrix items bind the Matrix payload, not a
   Value variant.
4. The _su items are skipped per decision D2, and the skip is
   recorded in the T6 exit report or the session log.
5. The 18_shell.cif fixture exists with its expected JSON and
   passes ground_truth_verify.
6. The T0 to T5 modules are untouched by the T6 diff. The
   standard inventory test stayed green.
7. No new compiler warnings.
8. No conflict-marker leftovers in the tree.
9. T6 merge diff scope: the diff is limited to the shell
   inventory, the T6 category modules, the codegen files, the
   pub mod lines, the tag registry, the dict_report example, and
   the fixture with its expected file. Flag any out-of-scope file.

## Output

Write your full report (check results, findings by severity P0
blocker / P1 important / P2 minor, with file and line references)
to sessions/dict-items-review-t6/report.md (your session dir).
End your final message with a one-paragraph verdict:
PASS, PASS-WITH-NOTES, or FAIL, plus your top findings.
