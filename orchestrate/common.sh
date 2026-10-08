# Shared helpers for the dict-items orchestration.
# Sourced by orchestrate.sh and the mission scripts.
set -u

MAIN=/home/tony/programming/crystal-cif-io
ORCH=$MAIN/orchestrate
STATE=$ORCH/STATE
LOGDIR=$ORCH/logs
PIDDIR=$ORCH/pids
MAIN_SESSION=roadmap-impl

mkdir -p "$LOGDIR" "$PIDDIR" "$ORCH/tmp"
touch "$STATE"

log() {
  echo "[$(date '+%F %T')] $*" >> "$LOGDIR/orchestrator.log"
}

# state_set <key> / state_get <key>
state_set() {
  grep -q "^$1=1$" "$STATE" 2>/dev/null || echo "$1=1" >> "$STATE"
}
state_get() {
  grep -q "^$1=1$" "$STATE" 2>/dev/null
}

# ping <msg...> : poke the main session (event trigger, works live or dead)
ping() {
  local msg="$*"
  log "PING: $msg"
  if ! rushi run "$MAIN_SESSION" "dict-items-orch: $msg" >> "$LOGDIR/orchestrator.log" 2>&1; then
    log "WARN: ping to $MAIN_SESSION failed"
  fi
}

# hold : pause while $ORCH/HOLD exists (the orchestrator user freezes it)
hold() {
  if [ -f "$ORCH/HOLD" ]; then
    log "HOLD: orchestrator paused. Remove orchestrate/HOLD to resume."
    ping "orchestrator paused at orchestrate/HOLD (resume: rm orchestrate/HOLD)"
    while [ -f "$ORCH/HOLD" ]; do sleep 15; done
    log "HOLD released, continuing"
  fi
}

# abort <msg> : fail-stop, notify the main session
abort() {
  log "ABORT: $1"
  ping "ABORT: $1"
  exit 1
}

# mission <name> : detach a mission script, print its PID.
# <name> is the rushi session name, e.g. dict-items-t2 or review-t2.
# Mission scripts live in $ORCH as mission_<suffix>.sh.
mission() {
  local name="$1" script
  case "$name" in
    dict-items-*) script="$ORCH/mission_${name#dict-items-}.sh" ;;
    *)            script="$ORCH/mission_$name.sh" ;;
  esac
  [ -f "$script" ] || { log "FATAL: missing mission script $script"; return 1; }
  (
    setsid nohup bash "$script" </dev/null >> "$LOGDIR/mission_$name.out" 2>&1 &
    echo $! > "$PIDDIR/$name.pid"
  )
  cat "$PIDDIR/$name.pid"
}

# wait_pid <pid> <name> : 0 if the mission exited 0
wait_pid() {
  local pid="$1" name="$2" rc=0
  wait "$pid" || rc=$?
  if [ $rc -eq 0 ]; then
    log "$name: mission exited 0"
  else
    log "$name: mission exited rc=$rc (last lines of the mission log)"
    tail -n 10 "$LOGDIR/mission_$name.out" >> "$LOGDIR/orchestrator.log"
  fi
  return $rc
}

# merge_union <branch> <summary> : merge into dev-0.2 in the main worktree.
# Conflicts are resolved with a line union (git merge-file --union),
# which is safe for the append-only shared files of this project.
merge_union() {
  local branch="$1" summary="$2" f rc=0
  cd "$MAIN" || return 1
  git merge --no-ff "$branch" -m "Merge $branch: $summary" >> "$LOGDIR/orchestrator.log" 2>&1
  if [ $? -eq 0 ]; then
    log "$branch: merged cleanly"
    return 0
  fi
  log "$branch: merge conflicts, union-resolving"
  for f in $(git diff --name-only --diff-filter=U); do
    git show ":1:$f" > "$ORCH/tmp/base" 2>/dev/null || : > "$ORCH/tmp/base"
    git show ":2:$f" > "$ORCH/tmp/ours"
    git show ":3:$f" > "$ORCH/tmp/theirs"
    if git merge-file -q -p --union "$ORCH/tmp/ours" "$ORCH/tmp/base" "$ORCH/tmp/theirs" > "$ORCH/tmp/merged" 2>>"$LOGDIR/orchestrator.log"; then
      cp "$ORCH/tmp/merged" "$f"
      log "$branch: union-resolved $f"
    else
      cp "$ORCH/tmp/ours" "$f"
      log "$branch: WARN: union merge lost lines in $f, kept ours"
    fi
    git add "$f"
  done
  git commit --no-edit >> "$LOGDIR/orchestrator.log" 2>&1 || rc=1
  if [ $rc -eq 0 ]; then log "$branch: merge committed (with union resolutions)"; fi
  return $rc
}

# cargo_gate <stage> : cargo test in the main worktree. 0 = green.
cargo_gate() {
  local stage="$1"
  local out="$LOGDIR/cargo_test_$stage.log"
  if (cd "$MAIN" && cargo test > "$out" 2>&1); then
    log "gate $stage: cargo test GREEN"
    tail -n 3 "$out" >> "$LOGDIR/orchestrator.log"
    return 0
  fi
  log "gate $stage: cargo test RED (tail)"
  tail -n 30 "$out" >> "$LOGDIR/orchestrator.log"
  return 1
}
