#!/usr/bin/env bash
# Mission dict-items-w3 : D12 wave w3 (tier T3 typed structs).
# Provisions the worktree (idempotent), runs the rushi session,
# pings the driving session with done or failed.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-w3
BRANCH=dict-items/w3
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
echo "mission dict-items-w3: worktree $WT ready (branch $BRANCH)"
if rushi run dict-items-w3 "$(cat "$ORCH/tasks/d12w3.md")"; then
  setsid nohup rushi run "$DRIVER" "dict-items-w3: D12 wave w3 (tier T3) done. Branch dict-items/w3 is ready for review (do not merge it here). See sessions/dict-items-w3/summary.md." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run "$DRIVER" "dict-items-w3: mission FAILED. See orchestrate/logs/mission_dict-items-w3.out and sessions/dict-items-w3." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-w3: session run finished, ping queued"
