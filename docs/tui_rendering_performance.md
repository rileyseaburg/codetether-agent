# TUI Rendering Performance Analysis & Recommendations

Scope: `src/tui/` — the ratatui/crossterm terminal UI, event loop, and chat
render pipeline. This document maps the **current state** of each performance
concern, then gives **actionable recommendations** ranked by impact/effort.

---

## 1. Current State (what already exists)

The codebase is already well-optimized along several axes. Recommendations
below build on, rather than re-invent, this work.

| Concern | Existing mechanism | File |
|---|---|---|
| Redraw gating | `needs_redraw` flag + `Snapshot` diff of message/input/status/streaming/bus/queue counts | `event_loop/dirty.rs`, `event_loop/mod.rs` |
| Event coalescing | `coalesce::drain` batches queued `SessionEvent`s after the first | `event_loop/coalesce.rs`, `session_event_drain.rs` |
| Tick cadence | 100 ms tick interval; watchdog decoupled at 2 s | `event_loop/timers.rs` |
| Formatted-line cache | Thread-local LRU-ish map keyed by `(ts, len, width, role)`, cap 128 | `chat_view/format_cache.rs` |
| Full-buffer line cache | `take_cached_if_valid` zero-clone take/restore; frozen-prefix + streaming-suffix split | `chat_view/lines.rs`, `state/message_cache.rs` |
| Streaming reparse throttle | Reuse parsed lines until text grows ≥ 256 bytes | `chat_view/streaming.rs` |
| Message windowing | `recent()` keeps last `CHAT_RETAINED_MAX_ITEMS = 500` | `app/message_window.rs` |
| Async session load | `load_session_for_tui` tail-loads off the render thread | `app/session_loader.rs` |
| Flicker/crash guard | size guard + panic guard + I/O error tolerance | `app/safe_draw.rs` |

---

## 2. Gaps & Actionable Recommendations

### R1. Debounced rendering / frame-rate cap  *(high impact, low effort)*

**Problem.** Every branch of `select_once` and every terminal event sets
`needs_redraw = true` unconditionally (`select_loop.rs`), and the draw fires at
the top of the next loop iteration with **no minimum inter-frame interval**.
During fast token streaming or paste bursts this can drive draws far above the
useful ~30–60 fps, burning CPU and causing visible tearing on slow terminals.

**Recommendation.** Add a frame-rate floor. Track `last_draw: Instant` in
`App` state and gate the draw in `event_loop/mod.rs`:

```rust
const MIN_FRAME: Duration = Duration::from_millis(16); // ~60fps ceiling
if app.state.needs_redraw && app.state.last_draw.elapsed() >= MIN_FRAME {
    draw_ui(...)?;
    app.state.needs_redraw = false;
    app.state.last_draw = Instant::now();
}
```

When a redraw is requested but suppressed by the floor, the existing 100 ms
tick (or a short one-shot `tokio::time::sleep`) will pick it up — so nothing is
lost, bursts are simply coalesced into one frame. This is the single highest
ROI change. Keep the file under 50 lines by putting the gate in a tiny
`event_loop/frame_gate.rs` helper.

### R2. Virtual scrolling for the messages panel  *(high impact, med effort)*

**Problem.** `build_uncached` materializes **all** lines for every retained
entry (up to 500 messages), then `clamp_scroll` slices via
`Paragraph::scroll`. The whole `Vec<Line>` is built and held even though only
`messages_rect.height - 2` rows are visible. For long sessions this dominates
both the cold-rebuild cost and memory.

**Recommendation.** Introduce true viewport virtualization:
1. Maintain a per-entry **line-count index** (cheap to compute, cacheable).
2. From `chat_scroll` + visible height, binary-search the index to find the
   first/last entry intersecting the viewport.
3. Build lines only for that slice plus a small over-scan margin (e.g. ±1
   entry) for smooth scrolling.

This bounds per-frame work by **viewport size**, not history size. It composes
with the existing format cache (per-message lines are still memoized). Gate it
behind a threshold (`entries.len() > 100`) so short sessions keep the simpler
path. Note the module doc already states "1 line == 1 terminal row" (no wrap),
which makes the offset math exact and the index trivial.

### R3. Lazy / decoupled widget updates  *(med impact, low effort)*

**Problem.** The `Snapshot` dirty-check (`dirty.rs`) is **global** — any change
to messages, input, status, streaming, bus, audit, or queue marks the *entire*
UI dirty and triggers a full-frame redraw. A bus-log entry arriving while the
user is in the chat view forces a chat re-render even though that pane is
unaffected.

**Recommendation.** Move to **per-region dirty flags**. Replace the single
`needs_redraw` bool with a small bitset (`DirtyRegions { chat, input, status,
sidebar, bus }`). The `Snapshot` already tracks the relevant counters
separately — split `changed_since` to return which regions changed. Only the
active `view_mode`'s regions need force a draw; off-screen panes (e.g. bus log
while in chat) can defer. This reduces unnecessary full rebuilds and pairs well
with R2 (virtualized chat only rebuilds when the `chat` region is dirty).

### R4. Streaming-preview cache key hardening  *(med impact, low effort)*

**Problem.** The streaming reparse cache (`streaming.rs`) keys only on
`streaming_text.len()`. If a stream edit replaces bytes without changing length
(rare but possible with some providers' correction/rewrite tokens), the stale
parse is shown until the next 256-byte growth. Also, the format cache
(`format_cache.rs`) calls `m.clear()` (full flush) on overflow instead of
evicting the oldest entry — a width change on a long session repeatedly nukes
the whole cache.

**Recommendation.**
- Add a cheap content discriminator to the streaming key: hash the last N bytes
  or include a monotonically-incrementing "stream epoch" bumped on any
  non-append mutation.
- Replace the `m.clear()` flush in `format_cache.rs` with proper LRU eviction
  (track insertion order via a small ring buffer of keys, evict one). This
  preserves cache warmth across resize/width churn.

### R5. Async data loading for heavy panels  *(med impact, med effort)*

**Problem.** Session tail-load is already async (R-good), but other data
sources are pulled **synchronously inside the tick** (`tick.rs`):
`refresh_audit_snapshot(...).await` runs on the render path when in Audit view,
and git views (`git_capture.rs`, `git_log.rs`) capture on demand. A slow `git
log` or large audit file stalls the entire event loop (no frames drawn while
awaited).

**Recommendation.** Push these onto background tasks that publish results via
the existing `mpsc` channels / `background.rs` drain path, exactly like
`SessionEvent`. The tick should *request* a refresh (set a "refresh pending"
flag) and *consume* completed snapshots, never `.await` blocking I/O inline.
Show a lightweight "loading…" placeholder until the snapshot arrives — this
also improves perceived responsiveness.

### R6. Reduced flicker on resize & view switches  *(low impact, low effort)*

**Problem.** On resize, `reset_height_memo` is a no-op and `reset_format_cache`
must be called to avoid drawing lines built for the old width; if a frame slips
through before invalidation, mixed-width lines flash. View-mode switches also
clear `cached_message_lines` (`message_cache_invalidate.rs`), causing a cold
rebuild and a one-frame blank/jump.

**Recommendation.**
- On resize, invalidate **before** the next draw (hook `reset_format_cache` +
  `message_cache_invalidate::clear` into the resize event handler, not lazily).
- For view switches, keep the previous view's last buffer rendered until the
  new view's first buffer is ready (double-buffer the active pane), eliminating
  the blank frame.
- Confirm crossterm is run with a single synchronized output flush per frame
  (ratatui's `Terminal::draw` already diffs cells, so avoid any manual
  `execute!`/`stdout().flush()` outside the draw closure — those are the usual
  flicker culprits).

### R7. Bound the streaming-text buffer  *(low impact, low effort)*

**Problem.** `streaming_text` grows unbounded for very long single responses;
the formatter reparses the whole buffer each 256-byte step (O(n²) over a long
stream).

**Recommendation.** Freeze fully-rendered streaming lines into the frozen
prefix incrementally (the frozen-prefix machinery already exists in
`state/message_cache/frozen.rs`) and only keep the unstable tail in
`streaming_text`. This makes per-step parse cost O(tail) instead of O(total).

---

## 3. Priority Roadmap

| # | Recommendation | Impact | Effort | Order |
|---|---|---|---|---|
| R1 | Frame-rate cap / debounce | High | Low | **1st** |
| R3 | Per-region dirty flags | Med | Low | **2nd** |
| R6 | Resize/view-switch flicker fixes | Low–Med | Low | **3rd** |
| R4 | Cache key + LRU hardening | Med | Low | 4th |
| R2 | Virtual scrolling | High | Med | 5th |
| R7 | Incremental stream freezing | Med | Low | 6th |
| R5 | Async heavy-panel loading | Med | Med | 7th |

**Quick wins first** (R1, R3, R6, R4) — all low-effort, no architectural
change, each independently shippable and testable. R2/R5/R7 are the
structural follow-ups for very long sessions.

## 4. Verification guidance

- Add a `--bench-render` harness or `#[bench]`-style test that feeds N
  synthetic messages and asserts cold-rebuild line count + timing bounds.
- Instrument draws with `tracing::debug!(frame_ms, region)` behind
  `RUST_LOG=codetether::tui=debug` to measure before/after.
- Respect the 50-line file limit: each new helper (frame gate, dirty bitset,
  line index, LRU evictor) gets its own focused module under
  `src/tui/app/event_loop/` or `src/tui/ui/chat_view/`.
