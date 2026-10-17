#!/usr/bin/env bash
# Mission dict-items-d12 : D12 wave 1 (the typed category structs).
# Provisions the worktree (idempotent: reuses an existing one),
# runs the rushi session, then pings the driving session with
# done or failed. One mission = one process.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-d12
BRANCH=dict-items/d12
# The D12 wave is driven by the typed-structs-for-entries session.
# The ping targets it, not the old roadmap-impl pipeline session.
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
echo "mission dict-items-d12: worktree $WT ready (branch $BRANCH)"
if rushi run dict-items-d12 "$(cat "$ORCH/tasks/d12.md")"; then
  setsid nohup rushi run "$DRIVER" "dict-items-d12: D12 wave 1 done. Branch dict-items/d12 is ready for review (do not merge it here). See sessions/dict-items-d12/summary.md." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run "$DRIVER" "dict-items-d12: mission FAILED. See orchestrate/logs/mission_dict-items-d12.out and sessions/dict-items-d12." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-d12: session run finished, ping queued"
