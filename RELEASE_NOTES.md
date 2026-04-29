# v4.7.0

## What's New

**Zero-Config A2A Peer Discovery** — Agents on the same LAN now discover each other automatically via mDNS. Peer discovery is enabled by default in the TUI and surfaces discovered peers in the bus log. (#115, #111)

**Windows Computer Use Tool** — Full desktop automation for Windows: screen capture, mouse/keyboard simulation, app control, and snapshot inspection via a persistent PowerShell session. Includes structured input validation and response types.

**DeepSeek Provider** — Native support for DeepSeek models with streaming, tool calling, and response parsing. SRP-split into 12 focused modules.

**Context Management Tools** — Three new tools for context window control:
- `context_summarize` — request cached summaries for arbitrary turn ranges
- `context_pin` — pin conversation turns so they survive context compression
- `context_budget` — inspect remaining context window capacity

**Incremental & Oracle-Replay Derivation Policies** — `DerivePolicy` gains `Incremental` (builds summary index progressively) and `OracleReplay` (replays from a saved oracle trace). A new `SummaryIndex` with LRU eviction backs the incremental path.

**Actor System** — New `src/actor/` module with mailbox, envelope, receipt, runtime, and dead-letter handling. Foundation for future agent bus improvements.

**TetherScript Plugin Examples** — Six example `.tether` scripts (guardrails, PR summary, release notes, task scoring, triage, test output) ship under `examples/tetherscript/`. Forage opportunity ranking now optionally scores via TetherScript.

**Autochat Persona Model Selection** — The autochat LCB persona now selects models automatically via `DelegationState`, wired into both swarm and ralph execution paths.

**TUI Enhancements**
- LaTeX block and inline math rendering in chat view
- Copy shortcuts for replies and full transcript (including SSH environments)
- Large paste summarisation via sidecar buffer
- A2A message stream visible in the TUI bus log
- Session tail-loading for large conversation histories (#98)
- Stream context errors now retry with automatic compaction

**Voice Tooling** — Voice input encoder module exposed as public. ALSA diagnostics silenced and panic-safe via RAII guard. SSH image paste UX improved with guided error messages.

## Bug Fixes

- **TUI mDNS discovery** — Now enabled by default; previously required manual opt-in (#106)
- **ALSA stderr suppression** — Replaced with RAII guard to prevent panics on voice input cleanup
- **K8s tool descriptions** — Clarified namespace and deployment parameter descriptions
- **PowerShell session reuse** — Persistent session now correctly reuses the same process across computer_use calls
- **TUI stream errors** — Context-length errors during streaming now trigger compaction and retry instead of failing
- **Large session loading** — Sessions are now tail-loaded to avoid UI stalls (#98)
- **A2A mDNS robustness** — Sanitized hostnames and fixed bind ordering for reliable LAN discovery

## Changes

- **SRP refactor of `delegation.rs`** — Split from 576 lines into 10 focused files, all under the 50-line effective limit
- **SRP refactor of `index` and `index_produce`** modules — Split into cache, types, tests, and build_context submodules
- **Voice input module** — Encoder submodule made public; stderr guard extracted for reuse
- **CI** — Jenkins pipeline now builds PR Windows executables as artifacts
- **Documentation** — New A2A peer discovery RFC and public agents guide (`docs/a2a-public-agents.md`, `docs/a2a-spawn.md`); PR #112 review corrections applied
- **212 files changed**, +8,527 / −1,019 lines across 42 commits
