# CodeTether Codex provider

This crate owns Codex provider orchestration: configuration, credential selection,
backend selection, completion entrypoints, and compatibility helpers. The
application imports its provider and catalogs through the existing
`provider::openai_codex` facade.

Implementation dependencies have separate owners:

- `codetether-codex-auth`: OAuth exchange and credential refresh coordination.
- `codetether-codex-protocol`: message/request encoding and response parsing.
- `codetether-codex-transport`: HTTP/WebSocket execution and transport state.
- `codetether-provider-core`: provider contracts, model catalogs, shared HTTP pool.

The provider does not depend on the application. Tests use the shared tool-message
constructor directly. Runtime reasoning overrides are owned here and exposed
through the application facade, preserving one process-wide state instance.

The migration has source comparisons and structural verification under
`artifacts/consolidation-provider-codex/`. Compilation and tests have not been
run under the user's prohibition on Cargo and rustc commands.
