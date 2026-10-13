#!/usr/bin/env bash
# Mission dict-items-d10 : D10 dictionary completeness research.
# Provisions the worktree (idempotent: reuses an existing one),
# runs the rushi session, then pings the main session with done
# or failed. One mission = one process.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-d10
BRANCH=dict-items/d10
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
cp -f "$MAIN/Cargo.lock" "$WT/Cargo.lock" 2>/dev/null
cd "$WT" || exit 1
echo "mission dict-items-d10: worktree $WT ready (branch $BRANCH)"
if rushi run dict-items-d10 "$(cat "$ORCH/tasks/d10.md")"; then
  setsid nohup rushi run write-readme "dict-items-d10: research done. Branch dict-items/d10 is ready for review (do not merge it here)." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run write-readme "dict-items-d10: mission FAILED. See orchestrate/logs/mission_dict-items-d10.out and sessions/dict-items-d10." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-d10: session run finished, ping queued"
