#!/usr/bin/env bash
# Mission dict-items-d9 : D9 CIF 2.0 syntax completion.
# Provisions the worktree (idempotent: reuses an existing one),
# fetches the cif_api corpus, then runs the rushi session. One
# mission = one process, one exit code. On exit the script pings
# the main session (write-readme) with done or failed. The ping
# is fire and forget, per orchestrate/common.sh.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-d9
BRANCH=dict-items/d9
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
echo "mission dict-items-d9: worktree $WT ready (branch $BRANCH)"
python3 tests/real_data/cif_api/fetch.py || exit 1
if rushi run dict-items-d9 "$(cat "$ORCH/tasks/d9.md")"; then
  setsid nohup rushi run write-readme "dict-items-d9: mission done. Branch dict-items/d9 is ready for review (do not merge it here)." >> "$LOGDIR/orchestrator.log" 2>&1 &
else
  setsid nohup rushi run write-readme "dict-items-d9: mission FAILED. See orchestrate/logs/mission_dict-items-d9.out and sessions/dict-items-d9." >> "$LOGDIR/orchestrator.log" 2>&1 &
fi
echo "mission dict-items-d9: session run finished, ping queued"
