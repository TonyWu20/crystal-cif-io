#!/usr/bin/env bash
# Dict-items roadmap orchestrator.
#
# Stages:
#   1. T0 foundation (worktree + mission, merge, cargo gate)
#   2. T2 + T3 + T4 in parallel (worktrees + missions, merge in
#      t2->t3->t4 order, cargo gate)
#   3. T5 (worktree + mission, merge, cargo gate)
#   4. four review sessions in the main worktree (dev-0.2)
#
# Resumable: completed stages are recorded in orchestrate/STATE.
#   Rerun this script after fixing a failure; merged branches stay merged.
# Freezable: create orchestrate/HOLD to pause between stages;
#   remove it to resume. Killable: kill $(cat orchestrate/PID).
#   Detached mission PIDs live in orchestrate/pids/.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh

log "orchestrator started (pid $$)"

# ---------- stage 1: T0 foundation ----------
if state_get t0_merged; then
  log "stage 1 (t0): skipped, already merged"
else
  hold
  mission dict-items-t0 || abort "could not launch dict-items-t0"
  p_t0=$(cat "$PIDDIR/dict-items-t0.pid")
  log "stage 1: dict-items-t0 pid=$p_t0"
  if ! wait_pid "$p_t0" dict-items-t0; then
    abort "T0 mission FAILED. Inspect orchestrate/logs/mission_dict-items-t0.out and the worktree .worktrees/dict-items-t0, then rerun the orchestrator."
  fi
  merge_union "dict-items/t0" "T0 foundation: inventory, tag test, float display, Matrix, todo! payloads, data_types removal, spelling" \
    || abort "T0 merge failed"
  cargo_gate t0 || abort "T0 gate red: cargo test failed in the main worktree (log: orchestrate/logs/cargo_test_t0.log)"
  state_set t0_merged
  ping "T0 merged into dev-0.2 and cargo test is green. Launching T2+T3+T4 in parallel."
fi

# ---------- stage 2: T2 + T3 + T4 in parallel ----------
if state_get w2a_merged; then
  log "stage 2 (w2a): skipped, already merged"
else
  hold
  mission dict-items-t2 || abort "could not launch dict-items-t2"
  p_t2=$(cat "$PIDDIR/dict-items-t2.pid")
  mission dict-items-t3 || abort "could not launch dict-items-t3"
  p_t3=$(cat "$PIDDIR/dict-items-t3.pid")
  mission dict-items-t4 || abort "could not launch dict-items-t4"
  p_t4=$(cat "$PIDDIR/dict-items-t4.pid")
  log "stage 2: t2 pid=$p_t2 t3 pid=$p_t3 t4 pid=$p_t4"
  failed=""
  for n in t2 t3 t4; do
    case $n in
      t2) p=$p_t2 ;; t3) p=$p_t3 ;; t4) p=$p_t4 ;;
    esac
    if state_get ${n}_merged; then
      log "stage 2: $n already merged, skipping wait"
      continue
    fi
    if wait_pid "$p" "dict-items-$n"; then
      merge_union "dict-items/$n" "$(case $n in t2) echo "T2 experiment, reflection, refinement";; t3) echo "T3 structure and chemistry";; t4) echo "T4 metadata and publication + codegen";; esac)" \
        || { failed="$failed merge-$n"; continue; }
      state_set ${n}_merged
      log "stage 2: $n merged"
    else
      failed="$failed $n"
      log "stage 2: dict-items-$n FAILED"
    fi
  done
  if [ -n "$failed" ]; then
    abort "wave T2/T3/T4 incomplete:$failed (see logs). Merged branches stay merged; fix the failed worktree and rerun the orchestrator."
  fi
  cargo_gate w2a || abort "w2a gate red: cargo test failed after the T2/T3/T4 merges (log: orchestrate/logs/cargo_test_w2a.log)"
  state_set w2a_merged
fi

# ---------- stage 3: T5 ----------
if state_get t5_merged; then
  log "stage 3 (t5): skipped, already merged"
else
  hold
  mission dict-items-t5 || abort "could not launch dict-items-t5"
  p_t5=$(cat "$PIDDIR/dict-items-t5.pid")
  log "stage 3: dict-items-t5 pid=$p_t5"
  if ! wait_pid "$p_t5" dict-items-t5; then
    abort "T5 mission FAILED. Inspect orchestrate/logs/mission_dict-items-t5.out and the worktree .worktrees/dict-items-t5, then rerun the orchestrator."
  fi
  merge_union "dict-items/t5" "T5 raw diffraction (diffrn*)" \
    || abort "T5 merge failed"
  cargo_gate t5 || abort "T5 gate red: cargo test failed (log: orchestrate/logs/cargo_test_t5.log)"
  state_set t5_merged
  ping "All impl tiers T0,T2,T3,T4,T5 merged into dev-0.2 and cargo test is green. Launching 4 review sessions."
fi

# ---------- stage 4: reviews ----------
if state_get reviews_done; then
  log "stage 4 (reviews): skipped, already done"
else
  hold
  mission dict-items-review-t2 || abort "could not launch dict-items-review-t2"
  p_r2=$(cat "$PIDDIR/dict-items-review-t2.pid")
  mission dict-items-review-t3 || abort "could not launch dict-items-review-t3"
  p_r3=$(cat "$PIDDIR/dict-items-review-t3.pid")
  mission dict-items-review-t4 || abort "could not launch dict-items-review-t4"
  p_r4=$(cat "$PIDDIR/dict-items-review-t4.pid")
  mission dict-items-review-t5 || abort "could not launch dict-items-review-t5"
  p_r5=$(cat "$PIDDIR/dict-items-review-t5.pid")
  log "stage 4: review pids: r2=$p_r2 r3=$p_r3 r4=$p_r4 r5=$p_r5"
  rc_reviews=0
  for n in t2 t3 t4 t5; do
    case $n in
      t2) p=$p_r2 ;; t3) p=$p_r3 ;; t4) p=$p_r4 ;; t5) p=$p_r5 ;;
    esac
    wait_pid "$p" "dict-items-review-$n" || rc_reviews=1
  done
  state_set reviews_done
  if [ $rc_reviews -eq 0 ]; then
    ping "ALL DONE: T0,T2,T3,T4,T5 merged into dev-0.2, all cargo tests green, 4 review sessions finished clean. Reports: sessions/dict-items-review-t2/report.md, -t3, -t4, -t5. Read the verdicts, then commit fixes if any P0/P1 findings."
  else
    ping "Reviews finished with at least one failed session (see orchestrate/logs/mission_dict-items-review-*.out). Reports where they exist: sessions/dict-items-review-*/report.md. Rerun the orchestrator after fixing."
  fi
fi

log "orchestrator finished"
