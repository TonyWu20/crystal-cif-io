#!/usr/bin/env bash
# Mission dict-items-readme-d12 : the README typed tour, on a
# branch off the unmerged dict-items/d12. Provisions the
# worktree (idempotent), runs the rushi session, pings the
# driving session with done or failed.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-d12-readme
BRANCH=dict-items/d12-readme
BASE=dict-items/d12
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
    git worktree add "$WT" -b "$BRANCH" "$BASE" || exit 1
  fi
fi
ln -sfn "$MAIN/target" "$WT/target"
ln -sfn "$MAIN/sessions" "$WT/sessions"
cd "$WT" || exit 1
echo "mission dict-items-readme-d12: worktree $WT ready (branch $BRANCH, based on $BASE)"
if rushi run dict-items-readme-d12 "$(cat "$ORCH/tasks/readme-d12.md")"; then
  setsid nohup rushi run "$DRIVER" "dict-items-readme-d12: README tour done. Branch dict-items/d12-readme is ready for review (do not merge it here). See sessions/dict-items-d12-readme/summary.md." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run "$DRIVER" "dict-items-readme-d12: mission FAILED. See orchestrate/logs/mission_dict-items-readme-d12.out and sessions/dict-items-d12-readme." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-readme-d12: session run finished, ping queued"
