# dict-items orchestration

Orchestrates the impl and review waves of
`docs/roadmap-dict-items.md` as detached headless rushi missions.
See `rushi docs headless-missions` for the pattern this follows.

## Layout

| path | role |
|---|---|
| `orchestrate.sh` | the master orchestrator (resumable, freezable) |
| `common.sh` | shared helpers: ping, state, mission, wait, merge_union, cargo gate |
| `mission_<tier>.sh` | one mission per impl tier: provisions the git worktree, then `exec rushi run dict-items-<tier>` |
| `mission_review-<tier>.sh` | one mission per review tier: `exec rushi run dict-items-review-<tier>` in the main worktree |
| `tasks/*.md` | the self-contained task text each mission execs (sessions start from zero context) |
| `STATE` | runtime state, one line per completed step (`t0_merged=1`, ...) |
| `HOLD` | create to freeze the orchestrator between stages; remove to resume |
| `PID` | the orchestrator process id |
| `logs/` | `orchestrator.log`, per-mission `mission_*.out`, per-gate `cargo_test_*.log` |
| `pids/` | one file per detached mission: `mission_<name>.pid` |

## Wave plan

1. T0 (worktree `.worktrees/dict-items-t0`, branch `dict-items/t0`)
   -> merge into dev-0.2 -> `cargo test` gate.
2. T2 + T3 + T4 in parallel (worktrees + branches `dict-items/t2..t4`)
   -> merge in t2->t3->t4 order -> `cargo test` gate.
3. T5 (worktree, branch `dict-items/t5`, reuses the T4 codegen)
   -> merge -> `cargo test` gate.
4. four review sessions in the main worktree (dev-0.2); each writes
   `sessions/dict-items-review-<tier>/report.md`.

Each worktree symlinks `target/` and `sessions/` to the main repo,
so build artifacts and session logs live in the main tree.

## Failure handling

- A failed mission pings the main session (`roadmap-impl`) with an
  ABORT line and the orchestrator exits. Merged branches stay
  merged; rerun `orchestrate.sh` to resume from `STATE`.
- The review sessions run in parallel on one target dir; cargo's
  lock serializes their builds.

## Ping design

Per-wave pings (T0 merged, all impl tiers merged, reviews done),
plus an immediate ping on any failure. No per-mission pings on
success. Pings are fire and forget: the poke is a backgrounded
`rushi run` so the pipeline never blocks on the main session's loop
(a blocking poke into a dead loop starts the loop and waits for its
turn to end).
