# Transport as a First-Class Layer — Implementation Plan

**Audience:** Reader is fluent in TCP/IP Illustrated. We speak in RTO, cwnd,
SACK, Nagle, delayed-ACK, zero-window, `TCP_INFO`, QUIC streams. No primitives
re-explained.

## Progress Ledger

- **Phase 1 (resumable stream protocol):** DONE. Client primitives
  (`src/a2a/stream/`) + worker wiring; Python server (`replay_ring`,
  `stream_epoch`, `stream_emit`, `sequencer_store`, `stream_resume_handshake`)
  with per-worker persistent sequencer. Proven by a live uvicorn-socket
  acceptance test (gap replay over real TCP). Commits up to `d48ff9d`.
- **Phase 2 (bounded backpressure):** DONE. Client `StagingBuffer` (1 MiB cap,
  overflow->reconnect); server bounded `asyncio.Queue` (maxsize 1024,
  drop-on-full, recoverable via replay ring). Commits `87699554`, `dcab2cf`.
- **Phase 3 (socket control):** DONE (option a). `socket_opts.rs` sets
  `tcp_nodelay`, keepalive (idle/interval/retries), `tcp_user_timeout` via
  reqwest 0.13 builder. Commit `f7ca2981`. NOTE: `TCP_INFO` sampling (Phase 4)
  still needs the raw socket handle -- reqwest's builder does not expose it, so
  Phase 4 may require a custom hyper connector.
- **Phase 5 (lifecycle state machine):** DONE. `backoff` (decorrelated jitter,
  500ms..60s), `breaker` (opens after 5 consecutive failures), `lifecycle`
  (Connecting/Live/Backoff/Dead). Wired into `worker_server_loop` replacing the
  fixed 5s sleep. Commit `14548e02`.
- **Phase 4 (TCP_INFO observability):** CORE DONE, integration pending.
  `tcp_info::sample(fd)` reads RTT/cwnd/retrans/total_retrans via
  `getsockopt(SOL_TCP, TCP_INFO)` on Linux, proven by a live-loopback test
  (`snd_cwnd >= 1` on a real connection). Commit `13dec849`. REMAINING: reqwest
  hides the `RawFd`, so periodic sampling + a telemetry/TUI surface need a custom
  hyper connector that exposes the configured `TcpStream`.
- **Phase 6 (QUIC/WebTransport):** not started.

## 0. Problem Statement

The A2A worker transport is `reqwest::Client` → `response.bytes_stream()` →
hand-rolled `\n\n` SSE framing (`src/a2a/worker/task_stream.rs`). Consequences:

- **No socket visibility.** `reqwest` owns the `TcpStream`. We cannot read
  `TCP_INFO` (`tcpi_rtt`, `tcpi_rttvar`, `tcpi_retrans`, `tcpi_snd_cwnd`,
  `tcpi_total_retrans`). A retransmit storm or a path that has collapsed cwnd to
  1 MSS is indistinguishable from "the model is slow."
- **No socket control.** No `TCP_NODELAY` (so Nagle ⊗ delayed-ACK can inject
  ~40ms stalls on small framed writes), no `SO_KEEPALIVE`/`TCP_KEEPIDLE`, no
  `TCP_USER_TIMEOUT` (so a black-holed path waits out the full RTO backoff
  ladder before the app learns anything).
- **No resumption.** On `Err`/`None` we `return` and the caller reconnects with
  a fresh `GET`. No `Last-Event-ID`, no cursor. Events emitted during the gap
  between FIN/RST and the new SYN are lost. SSE's own `id:`/`Last-Event-ID`
  replay is unused.
- **Unbounded backpressure.** `buffer.extend_from_slice(&chunk)` is an unbounded
  `Vec<u8>`. A fast producer + slow `process_buffer` grows memory without bound;
  the only flow control is TCP's own rwnd, which never gets to assert a
  zero-window because we drain the kernel buffer into userland unconditionally.

Goal: make **throughput, latency (incl. tail), connection lifecycle, and
resumption** first-class — both *observable* (TUI + telemetry) and
*recoverable* (protocol-level resume) — and stop using Rust as
Python-with-a-fast-runtime by reclaiming socket-level control.

## 1. Design Principles

- **Transport-agnostic resume first.** Resumption must work over plain
  SSE/HTTP1.1 before we introduce QUIC. Do not couple recovery to a transport.
- **Layered, SRP, ≤50 lines/file.** Each concern is its own module.
- **Observe before optimize.** Land `TCP_INFO` sampling + a real transport-health
  surface before tuning cwnd/buffers, so claims are evidence-backed.
- **Bounded everywhere.** Every buffer and channel has a capacity and an
  explicit overflow policy.

## 2. Phased Plan

### Phase 1 — Resumable Stream Protocol (transport-agnostic, highest leverage)

Fixes real data loss today; no transport change required.

**Wire contract (over existing SSE):**
- Server assigns a strictly monotonic `id:` per event (per logical stream).
- Server retains a **bounded replay ring** (N events or M bytes, whichever
  first) keyed by id. This is an application-layer analogue of the TCP
  retransmission queue: we can replay only what is still within the window.
- Client persists `last_event_id` after each successfully *processed* event
  (not merely received — commit point is post-`process_buffer`).
- On reconnect, client sends `Last-Event-ID: <n>`. Server replays `(n, head]`
  from the ring, then resumes live. If `n` has fallen out of the ring (cwnd
  analogy: client fell outside the replay window), server responds with a
  `resync-required` control event → client does a full cold resync.

**Module split:**
- `src/a2a/stream/event_id.rs` — monotonic id type + parse/format.
- `src/a2a/stream/replay_ring.rs` — bounded ring (capacity + byte budget).
- `src/a2a/stream/cursor.rs` — client-side durable last-processed cursor.
- `src/a2a/stream/resume_request.rs` — `Last-Event-ID` injection on reconnect.
- `src/a2a/stream/resync.rs` — `resync-required` control-event handling.

**Acceptance:** kill the TCP connection mid-stream (RST via `ss -K` or a proxy
that drops the flow); client reconnects, replays the gap, zero events lost while
within the ring; verify lost-when-outside-ring triggers cold resync.

### Phase 2 — Bounded Backpressure

Replace unbounded `Vec<u8>` accumulation with a bounded staging buffer +
bounded `tokio::sync::mpsc` to the processor. When the processor lags, stop
draining the body stream → kernel recv buffer fills → TCP rwnd shrinks →
**we let TCP advertise a smaller/zero window to the sender**, pushing
backpressure onto the wire instead of into the heap. This is the correct,
end-to-end flow-control story.

**Module split:**
- `src/a2a/stream/staging.rs` — bounded frame staging + overflow policy
  (block vs. drop-oldest; default block to engage rwnd).
- `src/a2a/stream/pump.rs` — body-stream → channel pump with capacity awareness.

**Acceptance:** slow consumer under a fast producer shows bounded RSS and an
observable rwnd reduction on the sender side (`ss -ti` `rcv_space`/`wnd`).

### Phase 3 — Socket Control Layer

Reclaim the `TcpStream`. Use a custom connector built on `socket2` so we set,
before connect: `TCP_NODELAY`, `SO_KEEPALIVE` + `TCP_KEEPIDLE`/`KEEPINTVL`/
`KEEPCNT`, `TCP_USER_TIMEOUT`, and send/recv buffer sizes. Two options:
- (a) `reqwest`/`hyper` with a custom connector that hands back a configured
  socket (keeps HTTP stack, gains socket knobs); or
- (b) drop to `hyper` client directly for the stream path for full control.

Prefer (a) first — minimal blast radius, still gets us the knobs and the raw
`TcpStream` handle for Phase 4.

**Module split:**
- `src/a2a/stream/socket_opts.rs` — `socket2` option set (one responsibility).
- `src/a2a/stream/connector.rs` — connector that applies opts + yields handle.

**Acceptance:** confirm `TCP_NODELAY` set (no 40ms Nagle/delayed-ACK stall on
small frames — measure inter-frame gap), keepalive probes visible in capture,
`TCP_USER_TIMEOUT` causes prompt failure on a black-holed path instead of full
RTO backoff.

### Phase 4 — Transport Observability (`TCP_INFO` + capture)

With the socket handle from Phase 3, sample `TCP_INFO` on Linux via
`getsockopt(TCP_INFO)` on an interval: expose `rtt`, `rttvar`, `snd_cwnd`,
`total_retrans`, `retrans`, `rcv_space`. Feed `crate::telemetry` and a new
first-class TUI transport-health view (replaces the client-only
`latency.rs` framing). Optional deeper tier: `pnet`/`etherparse` segment capture
behind a feature flag for retransmit/zero-window forensics (privileged; opt-in).

**Module split:**
- `src/a2a/stream/tcp_info.rs` — `TCP_INFO` getsockopt wrapper (Linux cfg).
- `src/telemetry/transport.rs` — transport metric struct + registry hook.
- `src/tui/transport_view.rs` — RTT/cwnd/retrans/throughput widget.

**Acceptance:** induce loss with `tc qdisc netem loss 5%`; widget shows
`total_retrans` climbing and cwnd dipping, correlated with throughput drop —
proving we can now distinguish path degradation from model latency.

### Phase 5 — Connection Lifecycle State Machine

Make reconnect explicit instead of implicit-in-a-loop. States:
`Connecting → Live → Degraded → Resuming → Backoff → (Connecting | Dead)`.
Jittered exponential backoff (decorrelated jitter), circuit breaker on repeated
hard failures, and `Degraded` driven by Phase 4 signals (e.g., sustained
`retrans` + RTT inflation). Rendered as first-class protocol state in the TUI.

**Module split:**
- `src/a2a/stream/lifecycle.rs` — state enum + transitions (SRP, ≤50).
- `src/a2a/stream/backoff.rs` — decorrelated-jitter backoff.
- `src/a2a/stream/breaker.rs` — circuit breaker.

**Acceptance:** flap the server; observe bounded reconnect rate (jitter, no
thundering herd), breaker opens after threshold, state transitions logged and
rendered.

### Phase 6 — QUIC / WebTransport Path (endgame)

`quinn` + `h3`/`h3-webtransport`. Buys multiplexed independent streams (no
HTTP/1.1 head-of-line blocking — a stalled frame on one logical channel no
longer blocks others behind it in the same TCP bytestream), stream-level flow
control, and **connection migration** (survives NAT rebinding / IP change via
connection IDs) — making "restart streams" a transport feature, not an app
hack. Phases 1–2 (event-id resume, bounded backpressure) carry over directly;
QUIC supplies per-stream framing and migration underneath.

**Acceptance:** multiplex two logical streams; stall one (app-level), confirm
the other keeps delivering (no HoL). Force a path/IP change; confirm connection
migration keeps the session alive without a cold resync.

## 3. Sequencing Rationale

1 → 2 → 3 → 4 → 5 → 6. Resume (1) stops data loss now and is transport-agnostic.
Backpressure (2) stops the unbounded-heap footgun and gets flow control onto the
wire. Socket control (3) is the prerequisite handle for observability (4).
Observability (4) makes every later optimization evidence-backed. Lifecycle (5)
operationalizes recovery. QUIC (6) is the structural endgame but must inherit a
proven resume/backpressure model, not invent one.

## 4. Non-Goals (this plan)

- Kernel-bypass (DPDK/io_uring zero-copy RX) — out of scope; revisit only if
  Phase 4 shows userland copy is the bottleneck.
- Replacing axum on the server control plane — only the stream path changes.

## 5. Evidence Discipline

Every phase's acceptance is a reproducible local experiment (`tc netem`,
`ss -ti`, `ss -K`, packet capture). Claims labeled: not-run / static-local /
focused-CI-like / live. No phase is "done" without its named experiment output.
