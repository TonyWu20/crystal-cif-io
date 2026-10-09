#!/usr/bin/env bash
# D8 + T6 shell-wave orchestrator for the dict-items roadmap.
#
# Stages:
#   1. D8 CIF 2.0 list values (worktree + mission, merge, cargo gate)
#   2. T6 core shell additions (worktree + mission, merge, cargo gate)
#   3. two review sessions in the main worktree (dev-0.2):
#      dict-items-review-d8 + dict-items-review-t6 in parallel
#
# D8 lands before T6 because the T6 Matrix items read and write
# 2.0 list values. Both missions are solo merges, so no union
# resolution is needed; merge_union is used for uniformity and
# still resolves conflicts by line union if they ever appear.
#
# Resumable: completed stages are recorded in orchestrate/STATE.
#   Rerun this script after fixing a failure; merged branches stay
#   merged. Freezable: create orchestrate/HOLD to pause between
#   stages. Killable: kill $(cat orchestrate/PID_SHELL).
#   Detached mission PIDs live in orchestrate/pids/.
set -u
source /home/tony/programming/crystal-cif-io/orchestrate/common.sh

echo $$ > "$ORCH/PID_SHELL"
log "shell orchestrator started (pid $$)"

# ---------- stage 1: D8 ----------
if state_get d8_merged; then
  log "stage 1 (d8): skipped, already merged"
else
  hold
  mission dict-items-d8 || abort "could not launch dict-items-d8"
  p_d8=$(cat "$PIDDIR/dict-items-d8.pid")
  log "stage 1: dict-items-d8 pid=$p_d8"
  if ! wait_pid "$p_d8" dict-items-d8; then
    abort "D8 mission FAILED. Inspect orchestrate/logs/mission_dict-items-d8.out and the worktree .worktrees/dict-items-d8, then rerun the orchestrator."
  fi
  merge_union "dict-items/d8" "D8 CIF 2.0 list values (dialect, Value::List, Matrix list constructor, canonical list kind, fixtures 16/17)" \
    || abort "D8 merge failed"
  cargo_gate d8 || abort "D8 gate red: cargo test failed in the main worktree (log: orchestrate/logs/cargo_test_d8.log)"
  state_set d8_merged
  ping "D8 merged into dev-0.2 and cargo test is green. Launching T6."
fi

# ---------- stage 2: T6 ----------
if state_get t6_merged; then
  log "stage 2 (t6): skipped, already merged"
else
  hold
  mission dict-items-t6 || abort "could not launch dict-items-t6"
  p_t6=$(cat "$PIDDIR/dict-items-t6.pid")
  log "stage 2: dict-items-t6 pid=$p_t6"
  if ! wait_pid "$p_t6" dict-items-t6; then
    abort "T6 mission FAILED. Inspect orchestrate/logs/mission_dict-items-t6.out and the worktree .worktrees/dict-items-t6, then rerun the orchestrator."
  fi
  merge_union "dict-items/t6" "T6 core shell additions (347 items, shell inventory, codegen, fixture 18)" \
    || abort "T6 merge failed"
  cargo_gate t6 || abort "T6 gate red: cargo test failed in the main worktree (log: orchestrate/logs/cargo_test_t6.log)"
  state_set t6_merged
  ping "T6 merged into dev-0.2 and cargo test is green. Launching the D8 and T6 review sessions."
fi

# ---------- stage 3: reviews ----------
if state_get shell_reviews_done; then
  log "stage 3 (shell reviews): skipped, already done"
else
  hold
  mission dict-items-review-d8 || abort "could not launch dict-items-review-d8"
  p_rd8=$(cat "$PIDDIR/dict-items-review-d8.pid")
  mission dict-items-review-t6 || abort "could not launch dict-items-review-t6"
  p_rt6=$(cat "$PIDDIR/dict-items-review-t6.pid")
  log "stage 3: review pids: d8=$p_rd8 t6=$p_rt6"
  rc_reviews=0
  wait_pid "$p_rd8" dict-items-review-d8 || rc_reviews=1
  wait_pid "$p_rt6" dict-items-review-t6 || rc_reviews=1
  state_set shell_reviews_done
  if [ $rc_reviews -eq 0 ]; then
    ping "SHELL WAVE DONE: D8 and T6 merged into dev-0.2, cargo test green, both review sessions finished. Reports: sessions/dict-items-review-d8/report.md, sessions/dict-items-review-t6/report.md. Read the verdicts, then commit fixes if any P0/P1 findings."
  else
    ping "Shell-wave reviews finished with at least one failed session (see orchestrate/logs/mission_dict-items-review-*.out). Reports where they exist: sessions/dict-items-review-d8/report.md, sessions/dict-items-review-t6/report.md. Rerun the orchestrator after fixing."
  fi
fi

log "shell orchestrator finished"
