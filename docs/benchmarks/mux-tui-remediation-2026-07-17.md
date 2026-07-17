# Mux TUI benchmark remediation — 2026-07-17

This ledger connects the live benchmark findings to bounded fixes and regression
checks. It supplements `mux-tui-live-2026-07-17.md`; it does not replace the raw
before measurements recorded there.

| Issue | Evidence | Remediation | Verification |
|---|---|---|---|
| MUX-001: partial frame after reattach | Alternate-screen sessions attached at the live byte offset, so a new terminal never received the retained escape sequence and screen state. | Reattach now replays the bounded PTY buffer (maximum 4 MiB), including alternate-screen state. | `alternate_screen_reconnect_replays_bounded_screen_state` |
| MUX-002: mDNS CPU/socket growth | Deleted binaries spun one `mDNS_daemon` thread at a full core and held 127–129 sockets; the current binary still showed rising detached mDNS CPU. | Discovery now requires 1–8 concrete, deduplicated interface addresses and disables unrestricted interface discovery. Text hostnames fall back to the listener's resolved IP without reopening auto-discovery. | `rejects_unbounded_interface_discovery`, `deduplicates_concrete_interfaces`, `rejects_excessive_interface_counts`, and three address-selection tests |
| MUX-003: working-history memory growth | Two real turns increased TUI private memory by 20.8 MiB; the formatted-line cache had an entry count but no byte admission bound. | The cache now accounts for retained line/span vector capacities and owned text, caps that rendering at 2 MiB, and rejects individually oversized renders. | `rendered_weight_counts_structures_and_text`, `oversized_render_is_not_retained`, `cumulative_render_weight_evicts_old_entries` |
| MUX-004: focused suite could not start | The navigation module referenced a missing `agent_focus_swarm_tests.rs`. | Restored a focused regression for unique swarm-agent focus entries. | `swarm_agents_join_focus_ring_once` |
| MUX-005: replay-buffer throughput | The 256 MiB debug benchmark retained a bounded 4 MiB but managed only 5.17 MiB/s. An isolated comparison showed eviction was already fast (19.9 GiB/s); most cost came from testing six ANSI prefixes at every byte. | Alternate-screen detection now visits only ESC bytes; a fixed-capacity ring also raised isolated eviction throughput from 19.9 to 24.7 GiB/s. | `wrapped_replay_preserves_byte_order` plus the repeated long-horizon benchmark |

The cache budget bounds one known duplicate representation; it does not cap the
conversation history itself. Live totals below therefore include intentionally
retained conversation, provider, and tool history as well as the bounded cache.

## Focused after-measurements

The production ring and terminal-mode modules were also compiled through
`artifacts/mux-buffer-ring-benchmark-2026-07-17.rs` while unrelated Cargo jobs
held the shared target lock. In the same unoptimized harness, the old deque
eviction path managed 19.9 GiB/s, the ring managed 24.7 GiB/s, and repeated
complete optimized-output runs managed 531–575 MiB/s. Compared with the
original complete buffer result of 5.17 MiB/s, ESC-filtered mode detection
improved debug throughput by roughly 103–111x while retaining the same 4 MiB
bound.

## Validation status

- The compiled lib-test harness passed five mux-buffer tests, three mDNS-scope
  tests, two cache-policy tests, and the restored swarm-focus test.
- The post-ring standalone harness compiled the production ring and terminal
  mode modules directly; wrapped ordering and split ANSI transitions passed.
- The standalone mDNS address harness passed all three concrete-address tests.
- The production format-cache policy and store passed three isolated tests,
  including cumulative eviction and oversized-render rejection.
- `cargo fmt --check` and `./check_file_limits.sh` passed.

## Exact-binary live proof

`cargo install --path .` installed SHA-256
`a1e10b00769edb349a033b5d1c0cb80edd2e3dbbdf186671a7a7b4aad77faadb`.
That exact binary created `codetether-fix-proof-20260717`, launched
`codetether tui --access-mode full` in this repository, and ran a real GPT-5.5
workspace audit with more than eight read-only tool calls across source, tests,
and benchmark artifacts.

- Three fresh clients received the retained alternate-screen frame immediately
  on attach, without an arrow key or other redraw input. The TUI and audit kept
  running after each `Ctrl+B`, `D` detach.
- Under the loaded audit, the TUI used 72.8–75.1 MiB RSS, 49.4–50.2 MiB private
  memory, zero swap, and 12–13 sockets. The small loaded interval is consistent
  with the 2 MiB cache bound; it is not presented as a total-history bound.
- Two 20-second samples put the mDNS thread at 0.3% and 0.6% CPU. This is over
  99% below the full-core deleted-binary behavior, while 13 sockets is about
  one tenth of the previous 127–129 sockets.
- The final whole-process 20-second average was 7.8% CPU while the agent was
  performing real workspace work.

No broad `cargo build` or `cargo check` was run.
