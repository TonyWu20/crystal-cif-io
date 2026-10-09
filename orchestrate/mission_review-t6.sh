#!/usr/bin/env bash
# Mission dict-items-review-t6 : read-only review of T6 on dev-0.2
# in the main worktree. Report goes to the session dir.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
cd "$MAIN" || exit 1
echo "mission dict-items-review-t6: starting in $MAIN (branch $(git branch --show-current))"
exec rushi run dict-items-review-t6 "$(cat "$ORCH/tasks/review-t6.md")"
