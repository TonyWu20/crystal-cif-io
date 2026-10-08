#!/usr/bin/env bash
# Mission dict-items-review-t3 : read-only review of tier t3 on dev-0.2
# in the main worktree. Report goes to the session dir.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
cd "$MAIN" || exit 1
echo "mission dict-items-review-t3: starting in $MAIN (branch $(git branch --show-current))"
exec rushi run dict-items-review-t3 "$(cat "$ORCH/tasks/review-t3.md")"
