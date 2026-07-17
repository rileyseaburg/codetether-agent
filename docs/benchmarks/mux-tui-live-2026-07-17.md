# Live mux TUI benchmark — 2026-07-17

## Scope and method

This benchmark used the installed `codetether 4.7.4` binary on Linux
6.8.0-134-generic, 32 logical CPUs, and 56 GiB RAM. The exact binary SHA-256
was `750b87a4f511542125cc6f1d94f184bd2b2a01cab3b7158c1882d79d49d48a44`.
The host had unrelated compilation load, so lifecycle timings are ranges rather
than laboratory latency claims. CPU was sampled with `pidstat`; RSS came from
`/proc/PID/status`, and PSS/private memory came from `smaps_rollup`.

The empty `/tmp` workspace established only the idle floor. The loaded runs used
the actual CodeTether repository and asked agents to inspect mux/TUI and A2A/mDNS
code, make 8+ tool calls per task, inspect live processes, run focused tests, and
produce detailed reports without editing files. One `--no-a2a` TUI completed two
real tasks; one default-A2A TUI completed one real task.

## Lifecycle results

| Measurement | Result |
|---|---:|
| Detached mux server start, 7 runs | 69–154 ms; median 107 ms |
| Loaded `--no-a2a` attach + immediate detach, 5 runs | 30–40 ms |
| Loaded default-A2A attach + immediate detach, 5 runs | 20–30 ms |
| Attach-client peak RSS | 23.0–24.3 MiB |
| Idle mux server | 23.9 MiB RSS; 11.9 MiB PSS; 7.3 MiB private; 9 threads |
| Idle mux server CPU, 5 seconds | 0.0% |

Both agents remained alive and continued their work after `Ctrl+B, D`. Reattach
worked while the agents were active. All benchmark-named mux sessions were
stopped afterward; pre-existing user sessions were not changed.

## Memory growth under real work

Values below are for the TUI process only, excluding its small mux server.

| Current-binary state | RSS KiB | PSS KiB | Private KiB | Threads | CPU |
|---|---:|---:|---:|---:|---:|
| Fresh idle, `--no-a2a` | 49,172 | 25,366 | 17,220 | 13 | 0.4% |
| After real task 1 | 74,088 | 42,117 | 34,396 | 13 | 0.2% |
| After real task 2 | 78,348 | 43,097 | 37,972 | 13 | 0.2% |
| Fresh idle, default A2A | 56,132 | 24,261 | 19,260 | 15 | 0.7% |
| After real A2A/mDNS task | 79,700 | 44,827 | 38,296 | 15 | 1.8% attached |

Two concurrently active real-work stacks (two servers plus two TUIs) consumed
202,624 KiB RSS, 94,252 KiB PSS, and 78,612 KiB private memory. They temporarily
reached 95 threads and 77 file descriptors while tools were active, and used
17.2% of one CPU in that 10-second active-work sample. The `--no-a2a` TUI alone
temporarily rose from 13 to 81 threads during tool execution, then returned to
13.

The two real turns grew the `--no-a2a` TUI by 29,176 KiB RSS and 20,752 KiB
private memory over fresh idle. This confirms that empty-session measurements
substantially understate a working agent's footprint.

## A2A/mDNS result and old processes

The current default-A2A process did not reproduce a full-core spin during its
roughly seven-minute loaded run. Its `mDNS_daemon` used 0.5% in a steady attached
sample and 5.7% over the final 30-second detached sample. It held 8–10 sockets.

Four pre-existing hot TUIs were materially different: `/proc/PID/exe` identified
all of them as deleted/replaced binaries, not the installed benchmark binary.

| PID | Deleted binary hash prefix | Private KiB | Swap KiB | Sockets | CPU | Hot thread |
|---:|---|---:|---:|---:|---:|---|
| 2414900 | `2f74d88c` | 159,420 | 56 | 129 | 101.0% | `mDNS_daemon` 100.0% |
| 2557232 | `6b9b5020` | 100,192 | 35,560 | 127 | 100.4% | `mDNS_daemon` 99.8% |
| 2527160 | `6b9b5020` | 98,932 | 42,572 | 127 | 100.6% | `mDNS_daemon` 99.8% |
| 3129126 | `6b9b5020` | 161,844 | 1,180 | 128 | 99.6% | `mDNS_daemon` 99.0% |

The legacy full-core problem is conclusively inside `mDNS_daemon`, accompanied
by about 127–129 sockets per TUI. It is not valid to attribute that exact failure
to the current hash without a longer soak: the current build stayed far below a
full core, but its mDNS cost did rise after real work.

## Correctness observations

- Reattach to an alternate-screen TUI sometimes restored only a partial frame.
  Pressing an arrow key forced a complete redraw. Attach was fast, but the first
  visible frame was not always self-restoring.
- `cargo test mux --lib` was attempted as the focused test. It could not run
  because the existing tree references missing
  `src/tui/app/navigation/agent_focus_swarm_tests.rs`. No broad `cargo build` or
  `cargo check` was run.
- The benchmark agents made no source edits. The only repository change from
  this benchmark is this results artifact.

## Conclusions

The mux server itself is lean and idle-efficient. Real agent history roughly
doubled private TUI memory in this short run, so capacity planning should use
loaded sessions: about 38 MiB private and 78 MiB RSS per current TUI here, plus a
small server. Loaded attach/detach remained 20–40 ms.

Old deleted binaries should be restarted: they each waste one core and retain
roughly 97–158 MiB private memory. For current builds, mDNS still deserves a
longer multi-hour soak and socket-count guardrail; `--no-a2a` remains the lowest
overhead control when peer discovery is unnecessary.
