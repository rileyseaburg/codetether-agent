# Phase 1 — Resumable Stream Protocol: Wire Contract

Status: DESIGN (not-run). Binds two ends:
- **Server (emitter):** `a2a_server/worker_sse.py` — FastAPI `StreamingResponse`,
  `event_generator()` (Python).
- **Client (consumer):** `src/a2a/worker/task_stream.rs` — `reqwest`
  `bytes_stream()` + manual `\n\n` framing (Rust).

## 1. Ground Truth (today)

Server emits frames of the form (worker_sse.py:1096/1155/1194/1206):

```
event: <type>\ndata: <json>\n\n
```

with `<type>` in {`connected`, `task_available`, `heartbeat`, ...}. There is
**no `id:` field** and the connect path reads **no `Last-Event-ID`**. SSE's
native replay (`id:` + `Last-Event-ID`) is entirely unwired.

## 2. Event Taxonomy (the key design decision)

Not all events need the same delivery guarantee. We split into two classes:

### Class A — Advisory, claim-gated (at-least-once, idempotent)
`task_available`. Delivery is *already* recovered out-of-band: tasks are
assigned by atomic `POST /tasks/claim`, and `task_stream.rs` runs a 30s
`poll_pending_tasks` recovery poll. A missed `task_available` is harmless — the
poll or another worker's claim covers it. **These events are NOT sequenced and
NOT replayed.** Assigning them ids would force replay of offers already claimed
by someone else.

### Class B — Sequenced state events (exactly-once, in-order)
Progress, result, status-transition, and control events tied to a specific
task/session. A gap here is real data loss (a missed terminal `result` strands
the client). **These get monotonic ids and live in the replay ring.**

`heartbeat` and `connected` are control/liveness — **never sequenced**
(replaying a stale heartbeat is meaningless).

## 3. Wire Format

### 3.1 Sequenced (Class B) frame
```
id: <stream_epoch>.<seq>
event: <type>
data: <json>

```
- `stream_epoch` — opaque token minted by the server per logical stream
  lifetime (e.g., per worker registration). Guards against seq reuse across
  server restarts: a client presenting a `Last-Event-ID` from a *different*
  epoch is told to cold-resync (§5, analogous to a stale TCP timestamp / PAWS
  reject — the sequence space is no longer valid).
- `seq` — strictly monotonic `u64` within an epoch, gap-free for Class B.

### 3.2 Advisory (Class A) and control frames — unchanged
```
event: task_available
data: <json>

```
No `id:`. Client never persists a cursor for these.

## 4. Server-Side Replay Ring (app-layer retransmission queue)

Per logical stream the server retains a bounded ring of recent **Class B**
frames keyed by `seq`:
- Bound: `min(N events, M bytes)` — whichever trips first (default N=1024,
  M=4 MiB). This is the *replay window*; like cwnd it is finite, and a client
  that falls behind it cannot be retransmitted to.
- Eviction: oldest-first once either bound is exceeded.
- `lowest_retained_seq` is tracked so the server can answer "is `Last-Event-ID`
  still replayable?" in O(1).

## 5. Resume Handshake

On reconnect the client sends:
```
Last-Event-ID: <stream_epoch>.<seq>
```
Server decision table:
| Condition | Action |
|---|---|
| epoch mismatch | emit `resync-required` control event, then live |
| `seq` >= `lowest_retained_seq` (within window) | replay `(seq, head]` in order, then live |
| `seq` < `lowest_retained_seq` (fell out of window) | emit `resync-required`, then live |

`resync-required` frame:
```
event: resync-required
data: {"reason":"epoch_mismatch"|"window_exceeded","head_seq":<u64>,"epoch":"<tok>"}

```
On receipt the client drops its cursor, performs a cold resync (full
`poll_pending_tasks` + reconcile current task state), and adopts the new epoch.

## 6. Client Cursor Commit Point

The cursor advances **only after a Class B event is fully processed**
(post-`process_buffer` side effects committed), never on mere receipt. This is
the at-least-once -> exactly-once boundary: a crash between receive and process
re-requests the same `seq` on reconnect and reprocesses idempotently. Class A
events never move the cursor.

## 7. Backward Compatibility

- A server that emits no `id:` (today's behavior) => client treats every event
  as Class A/control, never persists a cursor, never sends `Last-Event-ID`.
  Identical to current behavior. **No flag day.**
- A client that ignores `id:` => degrades to today's reconnect-cold. Safe.
- Rollout: server starts emitting `id:` on Class B; old clients ignore it; new
  clients begin resuming. Purely additive.

## 8. Module Split (Rust client, <=50 lines each, SRP)

| File | Responsibility |
|---|---|
| `src/a2a/stream/event_id.rs` | `EventId { epoch: String, seq: u64 }` parse/format of `id:` |
| `src/a2a/stream/cursor.rs` | durable last-*processed* cursor (load/commit) |
| `src/a2a/stream/classify.rs` | map event type -> Class A / B / control |
| `src/a2a/stream/resume_request.rs` | inject `Last-Event-ID` on reconnect builder |
| `src/a2a/stream/resync.rs` | parse `resync-required`, drive cold-resync decision |

Server side (Python, `a2a_server/`):
| File | Responsibility |
|---|---|
| `replay_ring.py` | bounded ring, `lowest_retained_seq`, append/replay |
| `stream_epoch.py` | epoch minting + `Last-Event-ID` parse/decision table |

## 9. Acceptance Experiments (must be run before "done")

1. **Gap recovery within window:** start stream, emit Class B seqs 1..100,
   `ss -K` the flow after seq 50 is processed, reconnect; assert client replays
   51..100 exactly once, cursor monotonic, zero loss.
2. **Window exceeded -> cold resync:** stall client, let server evict past the
   client cursor, reconnect; assert `resync-required{window_exceeded}` and a
   single cold reconcile, no partial replay.
3. **Epoch change (server restart):** restart server mid-stream; assert
   `resync-required{epoch_mismatch}`, no seq collision, clean re-adoption.
4. **Advisory class untouched:** drop a `task_available`; assert the 30s
   recovery poll still claims it and no replay is attempted for it.

All four labeled with evidence level when executed (target: focused-CI-like via
a local FastAPI fixture + a killed-socket harness).
