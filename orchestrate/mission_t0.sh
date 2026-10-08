#!/usr/bin/env bash
# Mission dict-items-t0 : one phase of the dict-items roadmap.
# Provisions the worktree, then execs the rushi session (one mission =
# one process, one exit code; the orchestrator waits on it).
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-t0
BRANCH=dict-items/t0
cd "$MAIN" || exit 1
if git show-ref --verify -q "refs/heads/$BRANCH"; then
  git worktree add "$WT" "$BRANCH" || exit 1
else
  git worktree add "$WT" -b "$BRANCH" dev-0.2 || exit 1
fi
ln -sfn "$MAIN/target" "$WT/target"
ln -sfn "$MAIN/sessions" "$WT/sessions"
cp -f "$MAIN/Cargo.lock" "$WT/Cargo.lock" 2>/dev/null
cd "$WT" || exit 1
echo "mission dict-items-t0: worktree $WT ready (branch $BRANCH)"
exec rushi run dict-items-t0 "$(cat "$ORCH/tasks/t0.md")"
