#!/usr/bin/env bash
# Mission dict-items-review-d12 : read-only review of D12 wave 1
# on the unmerged branch dict-items/d12, in its worktree.
# Report goes to the session dir.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
DRIVER=typed-structs-for-entries
WT=$MAIN/.worktrees/dict-items-d12
if [ ! -d "$WT" ]; then
  echo "worktree $WT is missing; the impl mission must run first" >&2
  exit 1
fi
cd "$WT" || exit 1
echo "mission dict-items-review-d12: starting in $WT (branch $(git branch --show-current))"
if rushi run dict-items-review-d12 "$(cat "$ORCH/tasks/review-d12.md")"; then
  setsid nohup rushi run "$DRIVER" "dict-items-review-d12: review done. Report at sessions/dict-items-review-d12/report.md." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run "$DRIVER" "dict-items-review-d12: mission FAILED. See orchestrate/logs/mission_dict-items-review-d12.out and sessions/dict-items-review-d12." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-review-d12: session run finished, ping queued"
