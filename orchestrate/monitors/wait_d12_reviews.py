#!/usr/bin/env python3
# Monitor: wait for the four D12 wave-2 review pokes to exit,
# then ping the main session with the closeout line.
import os, subprocess, time
ORCH = '/home/tony/programming/crystal-cif-io/orchestrate'
MAIN = '/home/tony/programming/crystal-cif-io'
def pids():
    out = []
    for t in ('t2', 't3', 't4', 't5'):
        with open(os.path.join(ORCH, 'pids', 'review-%s.pid' % t)) as f:
            out.append(int(f.read().strip()))
    return out
def alive(p):
    try:
        os.kill(p, 0)
        return True
    except OSError:
        return False
for _ in range(7200):
    if not any(alive(p) for p in pids()):
        break
    time.sleep(10)
subprocess.run('bash -c "source %s/common.sh && log \\\"all four D12 review pokes exited\\\""' % ORCH, shell=True)
msg = 'The four D12 wave-2 review reports are written. Read sessions/dict-items-review-t2/report.md, -t3/, -t4/, -t5/. Resolve the findings, then close out D12: the summary and the ping.'
subprocess.run(['rushi', 'run', 'typed-structs-for-entries', msg], cwd=MAIN)
