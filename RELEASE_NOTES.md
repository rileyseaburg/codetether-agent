# v4.5.2

## What's New

- **Parallel read-only tool dispatch** — read-only tool calls now execute concurrently, reducing agent latency on multi-tool turns.
- **AgentBus integration for sub-agents** — spawned sub-agents are wired to the shared [`AgentBus`](src/bus/global.rs), enabling inter-agent event propagation out of the box.
- **Two-tier chat render cache** — frozen message prefixes are reused during streaming, eliminating redundant reformatting on every SSE chunk.

## Bug Fixes

- **Streaming markdown O(n²)** — each SSE burst no longer triggers a full reparse; bursts are coalesced into a single redraw per frame and debounced.
- **TUI cache drain & syntect waste** — fixed over-eager cache eviction, redundant syntax-highlight allocations, and double-wrapped render calls.
- **Browser eval wrapper** — hardened the CDP eval wrapper against malformed responses and improved release-script reliability.
- **Release verification** — stabilized the CI test suite and release verification pipeline.

## Changes

### Performance

| Area | Improvement |
|------|-------------|
| Tool output | Tool results capped at 64 KB by default to prevent context blowout |
| TUI scroll | Incremental scroll-height memo avoids recomputation on unchanged lines |
| TUI render | Per-message formatted-line cache skips unchanged messages during repaints |
| TUI streaming | SSE bursts coalesced into one redraw per frame with debounced markdown reparse |

### Refactoring

- **Session module** — the monolithic `session/mod.rs` (2 700+ lines) was decomposed into 14 focused submodules under SRP: `lifecycle`, `persistence`, `events`, `title`, `types`, `prompt_api`, and `helper/` (compression, confirmation, prompt events, stream, token, etc.).
- **Bedrock provider** — the 1 354-line `bedrock.rs` was split into 12 single-concern modules: `auth`, `sigv4`, `body`, `convert`, `discovery`, `estimates`, `eventstream`, `response`, `retry`, `stream`, `aliases`, and `tests`.
- **TUI state** — the 1 200-line `state.rs` was extracted into 15 ≤100-line modules (`scroll`, `history`, `input_cursor`, `message_cache`, `model_picker`, `session_nav`, `steering`, etc.).
- **TUI chat view** — `ui/main.rs` (1 000+ lines) refactored into 25 focused `chat_view/` submodules (≤50 lines each): `render`, `scroll`, `streaming`, `format_cache`, `cursor`, `suggestions`, `status`, etc.
- **Browser tool** — DOM and navigation actions extracted into dedicated `dom/read`, `dom/write`, `nav/lifecycle`, `nav/page` submodules.
- **Chat sync** — split into `config`, `config_types`, `minio_client`, `s3_key`, `batch_upload`, `archive_reader`, and `worker`.

### Documentation & Hygiene

- Added `# Examples` rustdoc blocks to all remaining undocumented public items across the crate.
- Lint fixes and formatter changes applied across 70 files.
- Removed stray `src/foo.rs` from the index.
- Deleted the legacy `tui_old.rs` (11 964 lines) and 7 fully-ported extraction stubs.

### Stats

307 files changed · +16 735 / −26 017 (net −9 282 lines — smaller, more focused codebase)
