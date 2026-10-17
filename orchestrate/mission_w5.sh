#!/usr/bin/env bash
# Mission dict-items-w5 : D12 wave w5 (tier T5 typed structs).
# Provisions the worktree (idempotent), runs the rushi session,
# pings the driving session with done or failed.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-w5
BRANCH=dict-items/w5
DRIVER=typed-structs-for-entries
if [ -d "$WT" ]; then
  cd "$WT" || exit 1
  current=$(git branch --show-current)
  if [ "$current" != "$BRANCH" ]; then
    echo "worktree $WT is on branch $current, expected $BRANCH" >&2
    exit 1
  fi
else
  cd "$MAIN" || exit 1
  if git show-ref --verify -q "refs/heads/$BRANCH"; then
    git worktree add "$WT" "$BRANCH" || exit 1
  else
    git worktree add "$WT" -b "$BRANCH" dev-0.2 || exit 1
  fi
fi
ln -sfn "$MAIN/target" "$WT/target"
ln -sfn "$MAIN/sessions" "$WT/sessions"
cd "$WT" || exit 1
echo "mission dict-items-w5: worktree $WT ready (branch $BRANCH)"
if rushi run dict-items-w5 "$(cat "$ORCH/tasks/d12w5.md")"; then
  setsid nohup rushi run "$DRIVER" "dict-items-w5: D12 wave w5 (tier T5) done. Branch dict-items/w5 is ready for review (do not merge it here). See sessions/dict-items-w5/summary.md." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run "$DRIVER" "dict-items-w5: mission FAILED. See orchestrate/logs/mission_dict-items-w5.out and sessions/dict-items-w5." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-w5: session run finished, ping queued"
