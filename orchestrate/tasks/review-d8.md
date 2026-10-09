# Mission: review D8 (CIF 2.0 list values, dict-items roadmap)

You are reviewing the merged D8 work of crystal-cif-io.
Work in the main repo at /home/tony/programming/crystal-cif-io
(branch dev-0.2). All impl tiers and D8 are merged here. Read
docs/roadmap-dict-items.md sections 12 and 13 first.

This is a read-only review: do not modify source, fixtures, or
tests. You may run cargo commands.

## Scope

Decision D8: dialect detection, the Value::List variant, the 2.0
unquoted-value restriction, the Matrix list constructor, the
canonical list kind, the dual-schema-version harness, and the
four section 13.6 fixtures.

## Checks (run all of them)

1. cargo test is green (run it; capture the summary).
2. Acceptance 1 (section 13.7): the four fixtures 16_cif2_lists,
   neg_cif2_unbalanced_bracket, neg_cif2_bracket_in_unquoted, and
   17_cif1_brackets_stay_strings parse into the canonical shape
   through ground_truth_verify.
3. Acceptance 2: the 12 existing 1.1 fixtures parse unchanged and
   their expected JSON is byte-identical to the pre-D8 state
   (compare against git history if needed).
4. Acceptance 3: a 2.0 file with a bracket in an unquoted value
   fails at the parse stage with a Rich error. Check the
   neg_cif2 expected files record the parse-stage failure.
5. Acceptance 4: Value::List is exported from the crate root.
   Check a small compile probe or the lib re-export.
6. Acceptance 5: the harness runs with both schema versions.
7. No new compiler warnings.
8. No conflict-marker leftovers in the tree.
9. D8 merge diff scope: the diff is limited to the value grammar,
   the parser dialect and value rule, the canonical list kind,
   the harness, the crate root export, and the four fixtures with
   their expected files. Flag any out-of-scope file.

## Output

Write your full report (check results, findings by severity P0
blocker / P1 important / P2 minor, with file and line references)
to sessions/dict-items-review-d8/report.md (your session dir).
End your final message with a one-paragraph verdict:
PASS, PASS-WITH-NOTES, or FAIL, plus your top findings.
