#!/usr/bin/env bash
# Mission dict-items-review-d8 : read-only review of D8 on dev-0.2
# in the main worktree. Report goes to the session dir.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
cd "$MAIN" || exit 1
echo "mission dict-items-review-d8: starting in $MAIN (branch $(git branch --show-current))"
exec rushi run dict-items-review-d8 "$(cat "$ORCH/tasks/review-d8.md")"
