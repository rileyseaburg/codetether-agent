# v4.6.2

## What's New

- **GPT-5.5 model support** — Added OpenAI Codex provider support for the GPT-5.5 model family (#69).
- **LLM-routed search tool** — The search tool is now auto-registered in provider-backed sessions, enabling intelligent query routing without manual configuration.
- **macOS installer HTTP/TLS module** — Introduced dedicated `http.js` and `darwin_tls.js` modules, extracted from the monolithic installer for better modularity and testability.

## Bug Fixes

- **Bedrock provider test stability** — Resolved a crash in the Bedrock provider test suite.
- **macOS installer reliability** — Fixed open issues with the macOS installer flow and addressed review feedback from prior releases.

## Changes

- Refactored `npm/codetether/lib/installer.js` by extracting HTTP and TLS logic into separate modules, reducing installer complexity by ~130 lines.
- Added comprehensive installer test coverage (`installer.test.js`, 133 lines).
- Reorganized `src/provider/openai_codex.rs` for improved maintainability.
- **7 files changed**, 443 insertions(+), 262 deletions(−).
