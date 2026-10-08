#!/usr/bin/env bash
# Mission dict-items-t3 : one phase of the dict-items roadmap.
# Provisions the worktree (idempotent: reuses an existing one),
# then execs the rushi session. One mission = one process, one exit
# code; the orchestrator blocks on it.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh
WT=$MAIN/.worktrees/dict-items-t3
BRANCH=dict-items/t3
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
echo "mission dict-items-t3: worktree $WT ready (branch $BRANCH)"
exec rushi run dict-items-t3 "$(cat "$ORCH/tasks/t3.md")"
