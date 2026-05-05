# v4.7.2

## What's New

- **TetherScript alpha.8** — Upgraded the TetherScript runtime with built-in `js_eval`, `browser_render`, `browser_eval_js`, and DOM helpers. Shipped four example plugins under `examples/tetherscript/`.
- **MCP Plugin Marketplace** — Discover, verify (SHA-256 + Ed25519), and hot-load community tools at runtime without rebuilding. Includes registry, verification, and hot-load modules (`src/plugin_marketplace/`).
- **OKR Marketplace & Knowledge Graph** — Live knowledge graph with typed queries (`src/knowledge_graph/`), plus an auction-based marketplace for OKR bidding with reputation scoring (`src/marketplace/`).
- **Mesh Federation & PR Auto-pilot** — Cross-agent mesh federation with cost-aware scheduling (`src/mesh/`) and automated PR creation, CI watching, and provenance blocks (`src/github_pr/`).
- **Self-Training Flywheel** — Distillation harvest and trainer modules (`src/distill/`) that feed real session data back into prompt optimization.
- **Voice-Native Pair Programming** — Voice command router and dictation mode (`src/voice/`) for hands-free agent interaction.
- **Project Memory Palace** — Persistent belief store that survives session boundaries (`src/memory/palace.rs`).
- **Audit-Grade Provenance** — PR verification blocks, CI verifier script (`script/verify_pr.sh`), and a GitHub Actions workflow for automated provenance checks.
- **Many-Worlds Speculative Development** — Speculative execution branches with a `collapse_controller` exposed via the CLI (`src/swarm/speculative.rs`).
- **Tenant BYOK Provider Keys** — Per-tenant bring-your-own-key provider configuration (`src/provider/tenant_keys.rs`).

## Bug Fixes

- **File picker traversal** — Fixed double `parent()` call and root self-reference in the TUI file picker (#187).
- **Moltbook API key leak** — Stopped writing the API key to stderr during provider initialization (#183).
- **Poisoned RwLock recovery** — `BatchTool` now gracefully recovers from a poisoned `RwLock` instead of panicking (#180).
- **Auction NaN panic** — Guarded division-by-zero in the swarm marketplace auction (#179).
- **S3 sink clock panic** — Handled pre-epoch system clock timestamps without panicking (#178).
- **Forage RunArgs** — Added missing `branches` and `strategies` fields to `RunArgs` serialization.
- **Security audit sweep** — Patched 15 security and correctness bugs identified in the latest audit pass.

## Changes

- **69 files changed**, +1,967 / −85 lines.
- `Cargo.toml` version bumped to `4.7.2`.
- Alpha-8 TetherScript tests marked `#[ignore]` pending upstream builtin support.
- Claude prompt files added to `.gitignore`.
- `AGENTS.md` updated with TetherScript plugin authoring guide and testing patterns.
