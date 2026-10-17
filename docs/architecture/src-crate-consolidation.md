# Source-to-crate consolidation

This is the workspace-wide goal. The session runtime is one completed part, not completion of this inventory. No Cargo or rustc commands may be executed. Compiler invocation through rustdoc or wrappers is also excluded. Earlier compilation and test results below are historical evidence from before this restriction; subsequent changes receive static checks only and are not claimed to compile or pass runtime tests. Counts are a lexical snapshot of files on disk, including tests and potentially unreachable modules; they are not a claim that every file compiles into every target.

## Completion conditions

- Each subsystem has an explicit owning crate or a justified thin application adapter.
- Existing partial extractions are completed; implementations are moved rather than copied.
- Crate dependencies flow toward contracts and domain services, without importing the application crate.
- Each changed module has one responsibility and respects the 50-code-line rule.
- Existing public paths remain compatible where practical. Preserve behavior through source comparisons and permitted static checks; record compilation/runtime validation as unverified for changes made under the no-compiler restriction.
- Every pending inventory row receives an evidenced disposition before this broader goal can be complete.

## Extraction order

1. Complete broken or inactive existing extractions, beginning with vector storage.
2. Audit lower-level services and contracts: configuration, platform, approval, telemetry, search/indexing, and provider adapters.
3. Separate orchestration from transport: sessions, agent execution, bus, swarm, A2A, Ralph, and forage.
4. Move cohesive presentation/transport adapters after their services have stable boundaries: tools, TUI, CLI, HTTP, MCP, and mux.
5. Audit residual entrypoints and apparently unreachable modules; verify the final dependency graph and compatibility facades.

## Source inventory (owners updated as extraction progresses)

| Source owner | Rust files | Approximate code lines | Disposition |
| --- | ---: | ---: | --- |
| tool | 1231 | 45393 | Shared contracts, classification and pure patch engine extracted; concrete tools, filesystem adapters and registry remain for audit |
| tui | 1208 | 42457 | Pending cohesion and dependency audit |
| session | 916 | 27990 | Partial extraction; inspect remaining implementation |
| provider | 141 | 3096 | Shared contracts/HTTP/images/context limits, Codex OAuth/lifecycle/request, message and response protocol/HTTP and WebSocket transport, and Google/Bedrock auth, transcript conversion, body policy, native requests, responses, streaming, and Bedrock execution/discovery/refresh and OpenAI/DeepSeek/Z.AI/Anthropic/Moonshot/Google/StepFun/OpenRouter/Vertex Anthropic/Vertex GLM/GLM-5/Copilot implementations extracted; other adapters/registry pending |
| a2a | 355 | 12904 | Wire contracts, HTTP client, stream/QUIC, discovery and task scope extracted; gRPC, server and remaining worker ownership remain pending |
| cli | 198 | 10322 | Pending cohesion and dependency audit |
| swarm | 228 | 5643 | Shared runtime/domain/prompt policy extracted; application supplies session, provider, tools, worktrees and pod operations; remaining ownership audit ongoing |
| server | 213 | 8264 | Pending cohesion and dependency audit |
| mux | 252 | 7209 | Pending cohesion and dependency audit |
| cognition | 146 | 2103 | Domain/store/backend services, native model/router ownership and cadence extracted; client assembly, registry/signing adapters, publication and tick composition pending |
| mcp | 71 | 1715 | Shared protocol, transport, client state, catalogs, bus HTTP and native search extracted; application retains policy, registry wiring and server tool adapters |
| ralph | 108 | 2639 | Ownership audited: shared scheduling, state/persistence, quality/relay/outcome policy and bus protocol; application retains live provider/session/tool/worktree/event adapters |
| bus | 17 | 54 | Ownership audited: shared runtime/training and separate S3 adapter extracted; only compatibility exports and application global bus selection remain |
| lsp | 13 | 280 | Ownership audited: protocol, clients, managers, diagnostics and cache in lsp-core; application retains identity wrapper, trait adapters and integration tests |
| rlm | 16 | 185 | Ownership audited: shared execution extracted; only compatibility APIs, local-router setup and provider/session-bus adapters remain |
| config | 1 | 1 | Configuration crate compatibility facade |
| forage | 57 | 1400 | Ownership audited: shared selection, seeding, evidence and rubric services; application retains lifecycle, interactive/engine/policy adapters and reporting |
| runtime_policy | 33 | 796 | Ownership audited: shared evaluation and approval policy in runtime-policy; application retains config-loading entry points, prior-context guard, sandbox probe and compatibility/test modules |
| main | 1 | 1590 | Entrypoint/adapter; enforce thin boundary |
| telemetry | 5 | 48 | Core extracted; thin application identity/startup adapters |
| indexer | 8 | 146 | Core extracted; CLI and provider-resolution adapters retained |
| agent | 33 | 599 | Catalog, contracts, prompts, message/image transformations and tool-result execution helpers extracted; session loop, prior-context policy and swarm adapters remain |
| platform | 1 | 1 | Native services extracted; Linux application and GNU/MSVC Windows platform type checks passed |
| approval | 13 | 167 | Ownership audited: approval-core owns records, ledger, pending registry, decision dispatch and grants; application retains SessionEvent delivery, test isolation and facades |
| autochat | 5 | 68 | Shared requests, context, transport, model selection and delegation extracted; registry and local RLM execution adapters remain |
| vectordb | 13 | 409 | Core extracted; provider/backend adapters retained |
| benchmark | 12 | 303 | Ownership audited: shared services and suite scheduling extracted; remaining modules translate Ralph/provider/telemetry state and retain runtime ownership |
| crash | 1 | 10 | Shared crash service extracted; adapter supplies application version and memory-guard setup |
| github_pr | 10 | 120 | Shared PR policy/context/HTTP extracted; CLI body input, A2A credential acquisition and output adapters retained |
| moltbook | 1 | 1 | Client, wire types, HTTP helpers, credentials and content helpers extracted; compatibility facade retained |
| review | 11 | 200 | Review contracts, prompt construction, and parsing moved to approval-core; runtime state, runner, and tool-policy adapters retained |
| actor | 14 | 352 | Inactive tracked subtree: no application-root declaration or explicit external callers; retained for separate disposition, not migrated |
| worker_server | 1 | 9 | Ownership audited: state, endpoints and serving in worker-http; application adapts CLI address arguments |
| search | 9 | 92 | Shared planning/execution pipeline and contracts extracted; model registry resolution and concrete tool dispatch retained as host adapters |
| image_clipboard | 2 | 33 | Portable image operations extracted; native clipboard access remains an application adapter |
| voice | 6 | 20 | Ownership audited: commands, routing policy, SSH detection and speech catalog extracted; only compatibility exports, live transport selection and native CPAL probing remain |
| worktree_cli | 6 | 144 | Entrypoint/adapter; enforce thin boundary |
| cleanup_cli | 4 | 137 | Entrypoint/adapter; enforce thin boundary |
| session_router_tmp | 1 | 79 | Audit reachability before moving |
| lib | 1 | 56 | Entrypoint/adapter; enforce thin boundary |
| cloudevents | 1 | 12 | Ownership audited: event contract and parsing in a2a-worker-core; only Axum header conversion and compatibility API remain |
| memory | 2 | 2 | Project-belief persistence and context rendering extracted to cognition-core; compatibility exports remain |
| browser | 2 | 31 | Partial extraction; inspect remaining implementation |
| startup_vault | 1 | 30 | Entrypoint/adapter; enforce thin boundary |
| native_cli | 1 | 18 | Entrypoint/adapter; enforce thin boundary |
| noninteractive_env | 1 | 14 | Pending cohesion and dependency audit |
| startup_crash | 1 | 14 | Entrypoint/adapter; enforce thin boundary |
| provider_cli_alias | 1 | 7 | Entrypoint/adapter; enforce thin boundary |
| audit | 1 | 1 | Existing crate facade |
| distill | 1 | 1 | Existing crate facade |
| event_stream | 1 | 1 | Existing crate facade |
| k8s | 1 | 1 | Existing crate facade |
| knowledge_graph | 1 | 1 | Existing crate facade |
| marketplace | 1 | 1 | Existing crate facade |
| mesh | 1 | 1 | Existing crate facade |
| okr | 1 | 1 | Existing crate facade |
| plugin_marketplace | 1 | 1 | Existing crate facade |
| provenance | 1 | 1 | Existing crate facade |
| secrets | 1 | 1 | Existing crate facade |
| tls | 1 | 1 | Existing crate facade |
| util | 1 | 1 | Existing crate facade |
| workspace_scan | 1 | 1 | Existing crate facade |
| worktree | 1 | 1 | Existing crate facade |
| worktree_stub | 1 | 1 | Existing crate facade |

## Existing crate inventory (baseline plus completed extractions)

| Crate | Rust files | Workspace member | Missing declared root modules |
| --- | ---: | --- | --- |
| codetether-a2a-core | 47 | yes | none |
| codetether-a2a-discovery | 19 | yes | none |
| codetether-a2a-http | 6 | yes | codetether-a2a-core |
| codetether-a2a-stream | 33 | yes | codetether-telemetry |
| codetether-a2a-worker-core | 36 | yes | none |
| codetether-agent-core | 44 | yes | none |
| codetether-approval-core | 41 | yes | none |
| codetether-audit | 1 | yes | none |
| codetether-autochat-core | 73 | yes | none |
| codetether-benchmark | 42 | yes | none |
| codetether-bus-core | 134 | yes | none |
| codetether-bus-http | 21 | yes | none |
| codetether-bus-s3 | 30 | yes | none |
| codetether-browser | 102 | yes | none |
| codetether-cognition-native | 60 | yes | none |
| codetether-companion-capture | 6 | yes | none |
| codetether-companion-core | 12 | yes | none |
| codetether-companion-protocol | 8 | yes | none |
| codetether-companion-relay | 65 | yes | none |
| codetether-crash | 40 | yes | none |
| codetether-cognition-core | 185 | yes | none |
| codetether-config | 71 | yes | none |
| codetether-context-policy | 310 | yes | none |
| codetether-distill | 15 | yes | none |
| codetether-event-stream | 32 | yes | none |
| codetether-github-pr | 35 | yes | none |
| codetether-goal-core | 71 | yes | none |
| codetether-indexer | 101 | yes | none |
| codetether-k8s | 80 | yes | none |
| codetether-knowledge-graph | 11 | yes | none |
| codetether-lsp-core | 95 | yes | none |
| codetether-marketplace | 4 | yes | none |
| codetether-mesh | 3 | yes | none |
| codetether-forage-core | 44 | yes | none |
| codetether-okr | 72 | yes | none |
| codetether-paths | 5 | yes | none |
| codetether-platform | 56 | yes | none |
| codetether-plugin-marketplace | 4 | yes | none |
| codetether-provenance | 52 | yes | none |
| codetether-image | 13 | yes | none |
| codetether-provider-core | 120 | yes | none |
| codetether-provider-anthropic | 50 | yes | none |
| codetether-provider-deepseek | 25 | yes | none |
| codetether-provider-moonshot | 23 | yes | none |
| codetether-provider-google | 30 | yes | none |
| codetether-provider-stepfun | 42 | yes | none |
| codetether-provider-openrouter | 52 | yes | none |
| codetether-google-auth | 6 | yes | none |
| codetether-provider-vertex-anthropic | 46 | yes | none |
| codetether-provider-vertex-glm | 42 | yes | none |
| codetether-provider-glm5 | 49 | yes | none |
| codetether-provider-copilot | 53 | yes | none |
| codetether-bedrock-auth | 29 | yes | none |
| codetether-bedrock-protocol | 117 | yes | none |
| codetether-provider-bedrock | 27 | yes | none |
| codetether-bedrock-refresh | 15 | yes | none |
| codetether-codex-auth | 28 | yes | none |
| codetether-codex-protocol | 72 | yes | none |
| codetether-codex-transport | 51 | yes | none |
| codetether-provider-codex | 173 | yes | none |
| codetether-provider-openai | 72 | yes | none |
| codetether-provider-zai | 120 | yes | none |
| codetether-question | 30 | yes | none |
| codetether-ralph-core | 148 | yes | none |
| codetether-rlm | 448 | yes | none |
| codetether-runtime-policy | 37 | yes | none |
| codetether-search | 18 | yes | none |
| codetether-secrets | 81 | yes | none |
| codetether-session-runtime | 20 | yes | none |
| codetether-swarm-core | 276 | yes | none |
| codetether-telemetry | 62 | yes | none |
| codetether-theme | 13 | yes | none |
| codetether-tls | 4 | yes | none |
| codetether-tool-core | 57 | yes | none |
| codetether-tetherscript-values | 4 | yes | none |
| codetether-provider-tetherscript | 11 | yes | none |
| codetether-gemini-web-protocol | 74 | yes | none |
| codetether-gemini-web-transport | 13 | yes | none |
| codetether-provider-gemini-web | 35 | yes | none |
| codetether-provider-metrics | 16 | yes | none |
| codetether-provider-bonsai | 80 | yes | none |
| codetether-moltbook | 36 | yes | none |
| codetether-mcp-core | 81 | yes | none |
| codetether-mcp-transport | 36 | yes | none |
| codetether-model-catalog | 30 | yes | none |
| codetether-util | 3 | yes | none |
| codetether-vectordb | 14 | yes | none |
| codetether-voice-core | 12 | yes | none |
| codetether-worker-http | 18 | yes | none |
| codetether-worktree | 105 | yes | none |
| codetether-worktree-isolation | 4 | yes | none |

## First extraction: vector storage

The core files declared in src/vectordb/mod.rs:33-45 own vector representation, local hashing, similarity search, typed records, and JSON persistence. They use only standard/library dependencies. The existing crates/codetether-vectordb/src/lib.rs already describes that boundary but its implementation modules are missing and its manifest is not a workspace member.

Move those implementation modules and their four core tests into the existing crate. Keep src/vectordb/provider.rs and src/vectordb/auto/ in the application because they select or adapt application providers and optional inference backends. Re-export the core crate through src/vectordb/mod.rs so callers and provider implementations retain their existing types and paths. Wire the workspace, root dependency, and lockfile without invoking Cargo.

Baseline: the existing native application test runner passes all 10 vectordb tests (four core, two provider, four automatic-backend tests). Post-move verification passed: 4 core unit tests, 6 core doc tests, and the 6 retained application provider/backend tests.

## Progress

- Shared session runtime: previously extracted and verified; broader session and workspace consolidation remains pending.
- Approval ledger: extracted and verified; process-local grants and live delivery remain application adapters. Other approval SRP work remains part of the inventory.
- Vector storage: core implementation moved into the existing crate, workspace and lockfile wired, compatibility paths preserved. All 13 moved modules retain identical non-comment code. Native compilation and tests passed without Cargo. Evidence: artifacts/consolidation-vectordb-I7NQxi/.

## Coupling found for subsequent extractions

- Approval storage now uses `codetether_paths` directly in `crates/codetether-approval-core/src/store.rs`; live event delivery remains in `src/approval/live/request.rs`.
- `src/approval/decision_kind.rs:46` combines a parsed decision with process-local grant effects. Keep those effects with the application adapter until the grant ownership boundary is explicit.
- Configuration now refers to `codetether_theme::Theme` in `crates/codetether-config/src/config/ui.rs`; the optional renderer lives in `codetether-theme`, and the TUI keeps a compatibility facade.
- `src/provider/mod.rs` combines provider implementations, registry composition, credentials initialization, and protocol re-exports. The existing provider-core and ZAI crates are starting points; the rest still require consolidation.

### Approval ledger extraction

The durable request/decision/receipt types and JSONL store now live in `crates/codetether-approval-core`. `src/approval` re-exports that API and retains live session delivery, session grants, and decision routing. `ApprovalStore::open_default` directly calls the same `codetether_paths::data_dir` function previously reached through `Config`; the journal event/read API crosses the new crate boundary without changing serialized records. All other moved implementation bodies were checked against their captured baseline. Six native unit tests, two doc examples, and 24 application integration tests pass. The integration checks cover CLI, MCP, HTTP, session events, and TUI delivery. Evidence: artifacts/consolidation-approval-ThIBZN/. No Cargo commands were executed.

### Configuration and theme extraction

All 70 configuration files moved under `codetether-config`; their logic is unchanged apart from replacing application RLM/theme paths with the corresponding standalone crates. `src/config/mod.rs` is a compatibility facade. The configuration crate builds with renderer-free theme data. The application enables the optional `ratatui` feature, retaining the original theme methods through `src/tui/theme.rs`.

Theme fields and all 20 methods were compared against the original; a native before/after probe produced identical preset serialization, named-color conversion, convenience constructors, and role styles. All 31 configuration unit tests, both new crate doc examples, and 50 application integration tests pass without Cargo. Source hashes were checked after compilation and testing. Evidence: artifacts/consolidation-config-theme-Zd0z4Z/.

The subsequent LSP audit identified an extractable protocol core and application-owned client orchestration. The core is now extracted as described below; the remaining client implementation and the rest of the inventory remain in scope.

### LSP module cohesion

Before introducing a crate boundary, split the existing mixed responsibilities:

- `crates/codetether-lsp-core/src/types/config.rs`, `overrides.rs`, `catalog.rs`, and `linter_catalog.rs` own connection configuration, override precedence, and built-in defaults.
- Protocol data lives in dedicated request, response, notification, document, initialization, geometry, symbols, completion, and diagnostic modules.
- `crates/codetether-lsp-core/src/types/install.rs` coordinates availability checks; `run_install.rs` executes installation and `verify_install.rs` reports availability afterward. These paths are preserved but installation is never invoked by the consolidation checks.
- `src/lsp/client/` separates connection lifecycle, initialization, document synchronization, navigation, response parsing, manager caches, and linter configuration. The original client and manager types remain at their existing public paths.

The transport now separates process startup, framed message reads, header parsing, writes, stderr capture, requests, and diagnostics. A subsequent client/orchestration extraction still needs an explicit application/client identity boundary (the current handshake uses the application package version), and a decision about repository-owned smoke fixtures. Moving directories alone would leave these responsibilities coupled. The broad consolidation goal remains active.

The preparatory LSP split introduced 55 focused implementation modules, each at most 49 code lines. Native application compilation and 29 selected application tests passed after the client/type split; native compilation and 22 focused LSP tests passed against the final transport split. The real rust-analyzer test was excluded because it can launch Cargo. Formatting, whitespace checks, and the source line-budget ratchet passed. No Cargo commands or Python were executed. Evidence: `artifacts/consolidation-lsp-modules-ekokul/`.

### LSP protocol core extraction

`codetether-lsp-core` now owns protocol data, language-server/linter configuration, stdio framing, process-group cleanup, language detection, workspace-folder derivation, and URI helpers. Its 47 moved files retain their implementation tokens after namespace/import normalization; the new crate root is the only additional file. All 48 files satisfy the 50-code-line limit. The root LSP module re-exports the existing public paths.

The application still owns client/manager orchestration, initialization identity, cache policy, and the repository smoke fixture. These are explicitly pending, rather than counted as a finished LSP subsystem. Keeping initialization in the application preserves its package version in the handshake without making the protocol crate depend on the application manifest.

Native core verification passes 11 unit tests and eight doc examples. One moved workspace-folder example was corrected to index the returned vector. Manifest/lockfile wiring and dependency direction were checked with a native TOML parser. Native application compilation and 21 application LSP/preflight regression tests also pass. This includes real TypeScript diagnostics, process-cache behavior, cold-server retention, failed-server eviction, and pre-approval error rejection. Evidence: `artifacts/consolidation-lsp-core-zljcN0/`. No Cargo commands or Python were executed.

### Telemetry extraction

`codetether-telemetry` owns token and tool counters, provider-history aggregation, snapshots, costs, memory guards, RSS monitoring, and kernel TCP observations. Provider aggregation, token counters/snapshots, swarm completion, and file-change constructors were split into focused modules before extraction. All 62 crate files satisfy the 50-code-line limit.

The application retains five short identity/startup adapter files (48 code lines in total). They register the application package version before arming allocation guards or starting the RSS watchdog. Report builders read this shared host identity; allocation/reporting logic and all process-wide counter state remain owned by the telemetry crate. A2A re-exports the kernel TCP sampler instead of supplying a type dependency back into the application.

Source comparison preserves 171 existing items after path/visibility normalization; two report builders intentionally read the injected host version, and the RSS report regression now checks that version. Eleven core tests and 26 documentation examples pass. Native whole-application type validation, the application identity/singleton adapter test, and all three startup-adapter doc examples also pass. Formatting, line limits, source hashes, workspace wiring, and dependency direction were verified without Cargo. Evidence: `artifacts/consolidation-telemetry-qJ0oJp/`.

### Codebase indexer extraction

`codetether-indexer` now owns index data, workspace scanning, graph assembly, import/symbol parsing, local embeddings, remote embedding batching/retries, and persistence paths. The original monolithic module was split before extraction. Graph definitions/imports/final sorting and scan/read/record/embed steps have distinct owners. All 101 core source files and eight application adapter files respect the 50-code-line limit.

The application retains CLI options/output and an embedding adapter that delegates to `ProviderRegistry::from_vault` and the selected provider. The core requests a provider only after scanning and only for remote embeddings. Paths now call the same `codetether_paths` implementation previously reached through configuration. Existing public indexer paths remain available.

The original ten tests pass after the module split. Four new contract tests cover retry preservation, retry exhaustion, vector-dimension rejection, and skipping remote resolution for local/graph-only builds. All 14 core tests and 11 doc examples pass. An eight-case original-versus-refactored comparison produces identical vectors, statistics, source records, and graph edges for a mixed-language fixture with hidden/pruned files and size filters. Native whole-application type validation passes. Both JSON and text CLI output and the persisted payload match the original, excluding generation timestamps. Manifest/lockfile wiring and the transitive local dependency graph are verified. Evidence: `artifacts/consolidation-indexer-7CABnp/`. No Cargo commands or Python were executed.

### Shared provider contract and model metadata

The `Provider` trait now lives alongside its protocol types in `codetether-provider-core`. Its 11 method declarations/defaults retain identical tokens. Bedrock/Cerebras/GLM/Vertex aliases and the Codex model catalog, capability cache, effort parsing, and tier parsing have one shared owner. Application adapter paths re-export or delegate to the core. Request construction and default identity reporting therefore use the same aliases and live catalog without a core dependency on concrete application adapters.

The 198-input Bedrock alias comparison matches exactly. Core verification passes 34 unit tests and 41 documentation examples. Two stale Opus expectations in moved examples were corrected to match the existing alias behavior. A new process-isolated application regression verifies that writing a live catalog through the adapter affects both the reported model identity and the constructed request. Native application compilation and all 26 selected provider regressions pass, including the isolated live-catalog/request-identity check. Evidence: `artifacts/consolidation-provider-contract-oapgHb/`.

A declared-dependency audit found and corrected a missing `tracing` dependency in the new indexer manifest. Minimal native builds now pass for provider-core, ZAI, indexer, and all seven earlier extracted/configuration support crates. This strengthens the prior broad-extern verification without invoking Cargo.

### OpenAI-compatible provider extraction

`codetether-provider-openai` now owns OpenAI-compatible chat completion, SDK and raw-SSE streaming, embeddings, model discovery, and message conversion. Request construction, dispatch, response conversion, and typed deltas were split before extraction; all 72 provider files meet the 50-code-line budget. The application provider module is a compatibility re-export.

Bounded body readers and shared TCP options now live in provider-core, with the original provider/A2A paths re-exported. This removes the OpenAI adapter dependency on A2A. The core declares the streaming HTTP feature it requires.

Both the original and extracted transports pass the same 22 provider tests, including five loopback-only tests for MiniMax fallback, typed/raw streaming, optional authorization, sorted embedding responses, and capped model discovery. The original baseline also runs the socket-helper test, which remains covered by provider-core. All 32 previously separated OpenAI modules have identical implementation tokens after namespace, import-order, and optional array-comma normalization. Core validation passes 38 tests and 43 documentation examples; the provider example also passes. Native whole-application type validation also passes. All request/error/authentication paths in the loopback suite pass against both implementations. Evidence: `artifacts/consolidation-openai-provider-RGgmfq/`. No Cargo commands or Python were executed.

## Remaining verification integration

The release-verification workflow currently runs tests for the root package (`.github/workflows/release-verify.yml`), which does not automatically execute moved unit tests in dependencies. Before declaring workspace consolidation complete, update platform-appropriate CI coverage for the extracted crate tests as well as auditing all remaining source owners. Local native test evidence does not replace this pending CI integration.

### Existing event-stream crate module consolidation

The event-stream root now exports separate chat-event, stream-event, and broadcast modules. S3 configuration, environment lookup, client construction, endpoint selection, signing/digests, file reading, request headers, transport dispatch, response handling, and archive records have separate owners. Public paths and serialized data definitions remain unchanged. The 32 Rust files contain at most 42 code lines each.

The original and split implementations both pass the same 12 tests, including loopback HTTP uploads/errors, wire encoding, deterministic authorization, broadcast fan-out/lag, configuration, and stub behavior. All 16 documentation examples pass, and documentation builds without warnings. Fifteen original method bodies match exactly; expanding the new environment-lookup helper also matches the original body. Upload dispatch is decomposed into header and response helpers. The pre-existing canonical-request digest uses DefaultHasher; this structural change deliberately preserves it, and its correctness needs separate review. Native whole-application test-target type validation passes. Evidence: artifacts/consolidation-event-stream-FWBEXX/. No Cargo commands or Python were executed.

### Remaining crate-internal module audit

The event-stream evidence includes a fresh lexical audit of existing crate files. It identifies oversized files in worker-core, browser, companion-relay, distill, k8s, knowledge-graph, OKR, provenance, ZAI, RLM, and secrets. Reachability must be checked before acting: ZAI's legacy.rs is not declared by its crate root, and RLM explicitly selects oracle/validator/mod.rs and oracle/storage/mod.rs instead of the adjacent historical .rs files. These historical duplicates and the active oversized modules still require evidenced disposition; counting small crate roots alone does not complete the consolidation goal.

### Existing OKR crate module consolidation

The OKR crate now separates objective, key-result, run, approval-decision, and outcome domains. Data definitions, construction, validation, progress, and lifecycle behavior have distinct owners. The repository separately owns state, paths, initialization, per-record CRUD, queries, cache invalidation, and statistics. Public root exports and the persistence module remain compatible. All 72 Rust files are at most 44 code lines.

All 51 original method bodies and 13 serialized data definitions match their original tokens. Both implementations pass the same 19 tests, including disk reload/cache behavior, invalid-record handling, all query dimensions, aggregate counts, workflow transitions, and progress boundaries. All 61 documentation examples pass; documentation builds without warnings. Native whole-application test-target type validation passes. Evidence: artifacts/consolidation-okr-xnVIsz/. No Cargo commands or Python were executed.

### Retired unreachable historical copies

Removed codetether-provider-zai/src/legacy.rs and codetether-rlm/src/oracle/{storage,validator}.rs. Fresh native compiler dependency lists, including each crate's test target, exclude all three; those lists remain byte-identical after removal. ZAI already selects its focused transport modules, and RLM explicitly selects storage/mod.rs and validator/mod.rs. The files had no pre-existing worktree changes; their hashes and original copies are retained with the verification evidence and temporary baseline, respectively. Updated the active oracle README to point at the compiled validator module. Active oversized RLM modules and the other audited crate owners remain pending.

### Existing Kubernetes crate module consolidation

The Kubernetes crate root now exposes focused manager, pod, and data modules. Manager initialization separates in-cluster selection from kubeconfig fallback. Pod operations separate launch orchestration, manifest/environment/label construction, create-conflict retries, deletion, state projection, container lifecycle, and logs. Existing deployment and lease-coordination modules are unchanged. All 80 Rust files respect the 50-code-line limit.

The five public data definitions, 11 unchanged manager method bodies, and all 27 existing non-root modules match the original. Four manager methods were decomposed: initialization, pod launch, deletion, and state lookup. Both versions pass the same 26 tests, including in-memory lease/API contracts and isolated initialization. The captured pod manifest is identical after normalizing only its generated lease-owner identifier. All 17 documentation examples pass, and documentation builds without warnings. Native whole-application test-target type validation passes. Evidence: artifacts/consolidation-k8s-zLGAQJ/. No Cargo commands or Python were executed; no live cluster was contacted.

### Existing secrets crate module consolidation

The secrets root now exports separate manager, configuration, provider-record, and global-access modules. Client state, default construction, configured connection, Kubernetes login, cached key reads, complete provider reads, writes, listing, reachability, and cache invalidation have focused owners. Existing login, environment resolution, retries, and renewal modules retain their behavior; auth_refresh only needs explicit imports after removing the root's implementation imports. All 77 crate files meet the 50-code-line limit (maximum 46).

All 18 original method bodies, eight global functions, and two public serialized data definitions match tokens. Thirty-eight existing non-root files are byte-identical; the remaining auth_refresh file has identical method bodies and updated imports. The original and split implementations pass the same 24 tests, covering renewal, persisted login profiles, redacted diagnostics, cache behavior, writes, errors/retries, Kubernetes reauthentication, and isolated global registration. All 24 documentation examples pass, documentation builds without warnings, and two focused tests through the application's actual secrets re-export pass. Evidence: artifacts/consolidation-secrets-PAYoyZ/. No Cargo commands or Python were executed; Vault requests were confined to local fixtures.

Whole-application test-target validation initially failed because concurrently added question-UI code referenced a missing test file, and the source-size check reported event_handlers/mod.rs growing from 50 to 51 code lines. Both initial failures are retained in the secrets evidence. Those UI changes have since been completed independently: the later RLM oracle pass rebuilt the native dependency chain, passed whole-application type validation with the extracted secrets library, and passed the global file-size check.

### RLM oracle routing, parsing, and grep consolidation

The active oracle root, trace validator, FINAL schema, grep oracle, and grep validation now delegate to cohesive modules. Query classification, literal inference, ordered pattern definitions, answer interpretation, match comparison, verdict construction, confidence configuration, routing, and batching have separate owners. The inactive answer_format extraction is completed with the original active parser behavior; its obsolete strategy copy is retired. Public oracle paths remain available. All 73 changed Rust files have at most 45 code lines.

The original and refactored versions pass the same 37 oracle tests. Full captured routing and consensus verdicts match after normalizing generated trace identifiers and timestamps; pattern-inference outputs also match. Twenty-three of 28 methods retain identical bodies, five methods are decomposed, and all eleven type definitions retain their structure. Thirty-four focused documentation examples pass. The full documentation build retains the same pre-existing capability.rs link warning to RlmRouter::should_route; no new warning is introduced by these modules. Native configuration and LSP consumers have been rebuilt for the new RLM library. Application test-target type validation passes after rebuilding its configuration and LSP consumers. Evidence: artifacts/consolidation-rlm-oracle-kXkoRW/. No Cargo commands or Python were executed.

Remaining RLM work includes the template catalog, storage/manager paths, and the other oversized active modules listed in the crate audit. This oracle pass does not complete the workspace-wide goal.

### RLM template catalog, storage, and host contracts

The 27-template catalog now separates immutable definitions, ordered indexing, rendering, lookup, iteration, and generated-query metadata. Configuration defaults and output-capability registries have dedicated owners. Host-facing tool, message, response, provider, rewriter, and event interfaces are separated while retaining the traits module API. Storage separates construction, upload attempts, pending counts, capacity warnings, per-record sync, upload accounting, and JSONL bucket paths/writers. All 78 changed Rust files have at most 44 code lines.

Both versions pass all 65 RLM unit tests. The complete catalog, parameter metadata, generated queries, and iteration order match exactly. All 27 template definitions match tokens, as do 23 method bodies, 11 free functions, 13 type definitions, and three trait contracts. The remaining sync_pending method delegates to per-record helpers; mixed valid/invalid spool, local retention, and JSONL bucket regressions pass. Thirty-two targeted documentation checks pass, and the full documentation build passes with warnings denied. The earlier capability link warning is fixed. Configuration and LSP consumers rebuild successfully; whole-application test-target type validation passes. Evidence: artifacts/consolidation-rlm-data-uIGLOJ/. No Cargo commands or Python were executed.

Ten RLM files still exceed the module budget, all under chunker or router. Their current paths and counts are in remaining-oversized.json; they remain part of the active consolidation goal.

### RLM routing and chunking module completion

Separated semantic-boundary classification, chunk flushing, content-type detection, preparation state, prompt creation, result measurement, fallback sections, routing pressure, and truncation hints. The iterative loop now has dedicated request, response, state initialization, and orchestration modules; progress, cancellation, provider selection, and rewriting have independent owners. All 330 Rust files in the RLM crate now meet the 50-code-line limit. This completes the ten oversized files left by the previous RLM pass.

Both original and split implementations pass all 69 RLM tests. Exact snapshots match for ten loop scenarios, primary/subcall requests, conversations, trace events, prepared prompts, routing decisions, Unicode truncation, context fallback, chunk metadata, and result formatting (elapsed-time fields normalized). Nineteen function bodies are unchanged; four entry points were decomposed. Public chunker/router documentation checks and the full strict documentation build pass. Configuration and LSP consumers rebuild successfully, and full application test-target type validation passes. Evidence: artifacts/consolidation-rlm-routing-yAj4BG/. No Cargo commands or Python were executed.

The fresh crate audit still flags oversized files in worker-core, browser, companion-relay, distill, knowledge-graph, provenance, and the newly added question crate. Their ownership and module cohesion, along with the remaining application src owners and CI integration, remain part of the active workspace goal.

### Worker-core and provenance module consolidation

Worker-core now separates advertised interfaces, capability environment parsing, cognition configuration/snapshots, agent-name extraction, and activity classification. Provenance separates serialized identity/origin/execution data, construction, updates, identity sanitization, commit-message trailers, asynchronous/blocking execution, and Git context setup. Both crates retain their existing public APIs and are entirely within the file budget: 27 worker files (maximum 43 code lines) and 47 provenance files (maximum 48).

All 22 functions, eight methods, and six data definitions match their original implementation tokens (with the moved runtime-identity path normalized). Four worker tests and twenty provenance tests pass against both versions. Worker classification/interface snapshots and provenance author/commit-message snapshots match. Git integration runs only in disposable repositories with hooks disabled. Fifteen worker and eighteen provenance documentation examples pass; both full documentation builds pass with warnings denied. The worktree consumer rebuilds successfully and whole-application test-target type validation passes. Evidence: artifacts/consolidation-worker-provenance-qEWXlN/. No Cargo commands or Python were executed.

### Distill and knowledge-graph module ownership

Distill now groups its harvesting policy, record format, parser, and filesystem reader under `harvest/`; training configuration, results, version selection, and the current stub entry point live under `trainer/`. Knowledge-graph groups node/edge definitions under `graph/model/`, with separate storage and borrowed lookup modules. Public `harvest`, `trainer`, and `graph` paths remain available through re-exports. The training entry point still performs no training or writes. Its existing version selection, including file entries and unreadable-directory fallback, is preserved.

Six contract tests pass against the original and reorganized libraries. They cover filtering, malformed records, filesystem failures, version selection, stub side effects, caller ordering/duplicates/dangling references, and query behavior. Sixteen token comparisons preserve data definitions and implementations, including reconstruction of the extracted version helper. Six distill and seven graph documentation examples pass, and both documentation builds pass with warnings denied. All 26 Rust files across these two crates are within 50 code lines (maximum 44). Native application test-target type validation and global source-file checks are recorded with the evidence in `artifacts/consolidation-distill-graph/`. No Cargo commands or Python were executed.

The workspace goal remains incomplete. Companion-relay, browser, and question still need module audits, and the remaining application owners and workspace CI coverage remain pending. Small files alone are not evidence of coherent responsibility boundaries; the flat provenance module layout also needs a grouping review.

### Companion-relay handler and runtime modules

Companion-relay now groups device pairing, command dispatch, reply acknowledgement, and frame-upload handling beneath `device/`; owner creation, routing, requests, replies, and stop handling beneath `owner/`; queue validation and lifecycle transitions beneath `replies/`; and in-flight records, state, and initialization beneath `runtime/`. HTTP authentication, body limits, lock order, and the upload sequence recheck remain unchanged. Twelve source comparisons reconstruct the extracted helpers and confirm the original implementations; 36 other production files, including routing and body validation, remain byte-identical. All 60 Rust source files are at or below 50 code lines; five integration-test files bring the crate inventory to 65.

Ten tests pass against both original and reorganized source: seven unit tests and three HTTP tests. Coverage includes UTF-16 reply limits, paused/stopped/unpaired admission, acknowledgement identity, expiry boundaries and event publication, initial state, owner/device separation, direct private-handler replies, and rejection when a device pauses during a frame body upload. The global source line-budget check also passes. Native relay binary type validation and documentation checks are recorded in `artifacts/consolidation-companion-relay/`; the application does not depend on this standalone relay crate.

Baseline tests exposed two existing facts that this structural refactor preserves. `DeviceCommand` explicitly serializes `reply: null`, so the old HTTP test expectations were updated to include it. Also, `paths::session` omits `reply` and `typed` from its action whitelist although both handlers exist. The new HTTP test captures their current 404 behavior; a separate private-handler test exercises their implementation. Enabling those routes is a distinct behavior change, not silently included here. The initial failing test evidence is retained with the passing results.

Browser, question, the provenance grouping review, remaining application owners, and workspace CI coverage remain part of the active goal. No Cargo commands or Python were executed.

### Provenance responsibility grouping

The crate root now declares nine responsibility modules instead of 25 flat implementation modules. Claim application belongs under `claim`; commit signing policy and repository enrichment under `git`; author/committer environment under `identity`; hook installation, paths, permissions, generated script, and message updates under `hook`; optional trailer emitters under `trailers`; and configuration, persistence, atomic publication, and principal display under `runtime_identity`. Forgejo identity derivation separately owns its normalization and configuration helpers. Existing root exports are preserved.

This pass includes the current Forgejo identity and durable runtime-identity implementations from the worktree, preserving their behavior. All 102 functions, methods, and data declarations match after module-path and visibility normalization. All 20 existing tests pass before and after, including concurrent identity-file creation, signature and Forgejo contract vectors, and disposable-repository commit tests. Author identity snapshots match. All 52 Rust files remain within the line budget (maximum 48 code lines).

The existing snapshot test's `serde_json` usage was missing from the crate's test dependencies. The manifest and existing lockfile package entry now declare that dependency. A native library build uses only declared production dependencies; documentation examples and strict documentation rendering pass; the worktree dependent crate rebuilds. The native application test-target check and other evidence are recorded in `artifacts/consolidation-provenance-groups/`. No Cargo commands or Python were executed.

Browser and question module audits, remaining application owners, and workspace CI integration remain pending under the full active goal.

### Browser offline operations and request boundaries

Browser offline tracing now owns its result model, redirect traversal, one-hop HTTP/cookie handling, URL resolution, and cookie serialization beneath `auth_trace/`. Cookie comparison owns its model, file boundary, pure comparison, and composite-key index beneath `cookie_diff/`. CORS header analysis is grouped with its operation. DOM request payloads are grouped into selector/scope, input/keyboard/upload, and text-targeting modules, preserving existing request exports. Error conversions and stack formatting are children of the public error module. The loopback HTTP test server is separate from its assertions.

The public `BrowserError` enum remains a single cohesive sum type. Its containing file falls from 69 to 61 code lines; the enum declaration itself is 55 lines including error-format attributes. Splitting variants would change the public API rather than improve ownership. This is an explicit retained size exception under the existing-file ratchet; every new file is within 50 code lines. The crate now contains 102 Rust files.

Twelve no-feature tests pass against both implementations; seventeen TetherScript-enabled tests pass against both implementations. The original feature-dependent HTTP test is excluded from the no-feature baseline comparison, and its final registration is correctly gated to TetherScript. New coverage checks zero allowed redirects, carried cookies, complete cookie-record differences and duplicate indexing, error formatting/conversions, and DOM request serialization. The HTTP tests use loopback listeners, never an external service or real browser process.

All 172 declaration and extraction checks pass after module-path and visibility normalization and explicit reconstruction of the extracted redirect hop. Feature-enabled documentation tests and strict rendering pass; both feature configurations build natively. Application test-target type validation is recorded in `artifacts/consolidation-browser-groups/`. The no-feature library retains its existing unused-code warnings, recorded in the native logs. No Cargo commands or Python were executed.

Question, the remaining application owners, and workspace CI coverage remain pending under the full active goal.

### Question contract, draft, and rendezvous modules

The question crate separates request wire models, legacy decoding, validation, and read access under `request/`; editing actions, private state, views, transitions, and submission under `draft/`; and task-local capability, pending-state ownership/cleanup, async admission, and frontend operations under `runtime/`. Public request, draft, action, and runtime function paths remain unchanged. The application tool and TUI continue using those paths without implementation changes.

All ten tests pass against the original and reorganized implementations. Coverage includes Unicode header limits, trimmed duplicate labels, legacy defaults, single/multiple/custom answers, cursor wrapping, previous-question state, exact-session routing, unavailable/headless execution, spawned-task isolation, cancellation, and the completed-slot race boundary. Thirty-eight declaration, extraction, and singleton checks reconstruct parsing/submission helpers and verify unchanged logic and task-local/pending declarations. All 30 Rust files are within 50 code lines (maximum 34).

Documentation examples and strict documentation rendering pass. The full application test-target native type check and the global source-file budget check are recorded with the test results in `artifacts/consolidation-question-groups/`. No Cargo commands or Python were executed.

The existing-crate module pass does not complete the workspace goal. Remaining application source owners and workspace CI coverage still require their recorded disposition and verification.

### Platform service extraction

`codetether-platform` owns the former native platform implementation. `src/platform/mod.rs` is a compatibility facade, preserving existing Windows and Unix application paths. The crate keeps browser discovery, Windows input/window/GDI operations, and voice adapters in their existing domains; browser candidate discovery and registry probing now have separate child modules. The DPI guard is public at the crate boundary because existing application callers use it.

Native allocation checking and the portable BGRA encoder are shared under `capture/`. This removes three reverse dependencies from Windows capture code into the application tool layer. `src/tool/computer_use/capture_limits.rs` retains preview/file policy and delegates pixel allocation checks; the preview regression test imports the encoder instead of including Windows source by path. Existing voice placeholders remain unchanged; this refactor does not implement WASAPI capture or playback.

Ninety-five moved function/method/type and allocation-contract comparisons pass after path and visibility normalization and ordinary formatting of the existing allocation function. Four common portable capture/encoding/key tests pass against both versions; the baseline also runs its existing file-budget test. All 56 crate Rust files are within 50 code lines. The Linux library, four documentation examples, strict documentation rendering, and application test-target type validation are recorded in `artifacts/consolidation-platform-extract/`.

The initially missing Windows dependency artifacts were subsequently built directly with `rustc` from cached, lockfile-verified sources. Both `x86_64-pc-windows-gnu` and `x86_64-pc-windows-msvc` now pass type validation for the original implementation, the extracted crate, and a compile-only consumer of the existing application paths, including the newly public DPI guard. The earlier target-validation gap is resolved; desktop/audio execution and full Windows application builds were not run. No Cargo commands or Python were executed.

### Windows platform verification

Direct compilation followed the current lockfile and target-conditioned manifests, including feature propagation. The checked library graph contains 38 Windows-target libraries (including the platform crate) and six host libraries for procedural macros; the build-time `autocfg` helper was also built natively. All 44 dependency archive checksums match `Cargo.lock`. Crate build scripts were inspected and invoked as native executables; their compiler probes invoke `rustc`, without invoking Cargo or Python.

The Windows GNU and MSVC checks compile the actual gated source, rather than substituting stubs. Both original and extracted implementations pass, as does a consumer that references the public browser, window/input, encoding, voice, and DPI APIs through a compatibility facade. The verification is metadata-only: it establishes Windows source/type compatibility, not successful linking, runtime device behavior, or whole-application Windows test coverage. Evidence: `artifacts/consolidation-platform-windows/`. Application source ownership and workspace CI coverage remain pending under the full goal.

### Agent catalog, prompt, and contract extraction

`codetether-agent-core` owns serializable agent profiles, tool metadata/activity, execution-result contracts, the in-memory catalog, built-in profile definitions, prompt templates/composition, project instruction discovery, and VS Code tool summaries. Its runtime dependencies are provider-core (for the existing usage type), serde, and serde_json. It has no application, session, tool-runtime, or swarm dependency. Catalog definitions live under `catalog/`, separately from `builtin/` prompt assembly and `types/` contracts.

The application retains `Agent` construction/execution, provider/tool ownership, policy dispatch, session context, and swarm implementations. Existing `agent::types`, `agent::registry`, `agent::builtin`, and build-guidance paths are compatibility re-exports. The instruction-append helper and worktree guidance are exposed across the crate boundary because existing session and swarm callers require them.

All 13 catalog, instruction-discovery, prompt-quality, and workspace-tool-summary tests pass against the original and extracted implementations. All 73 function/method/type/constant declaration comparisons pass, with only module paths and necessary visibility normalized; the build prompt resource is byte-identical. Thirty-eight documentation examples pass with none ignored, and strict documentation rendering passes. All 36 crate Rust files stay within the line limit (maximum 48); the remaining 33 application agent files total approximately 662 code lines.

Native full-application test-target compatibility and source-file checks are recorded with the extraction evidence in `artifacts/consolidation-agent-core/`. No Cargo commands or Python were executed. The remaining execution and application source owners and CI coverage continue under the full active goal.

### Tool interface and result extraction

`codetether-tool-core` owns the async `Tool` contract, `ToolResult` data/construction/truncation/structured errors, output-budget resolution, model-facing feedback, image metadata normalization, and generic alias/invalid-tool adapters. Invalid-tool context, protocol implementation, message formatting, suggestions, and edit distance have separate child modules. Application paths re-export the same API; concrete tools and signed-plugin/default/provider registry construction remain in the application. The application-specific read-only allowlist and policy integration also remain there.

The core depends on provider-core for the existing image content type and util for the existing UTF-8 truncation helper, plus standard serialization/async-trait dependencies. It does not depend on the application registry, session execution, policy layer, or concrete tool implementations. All 37 declaration/extraction comparisons pass, including reconstruction of invalid-tool feedback and a token comparison of the complete retained application registry implementation.

All 12 tests pass before and after extraction. They cover alias delegation, ordered/Unicode invalid-tool suggestions, result serialization, structured failures, metadata preservation, UTF-8 truncation, image normalization, and feedback. Sixteen documentation examples pass. Truncation behavior is preserved: the requested byte limit applies to the retained prefix, and the marker is additional text; the documentation now states this accurately. All 26 crate Rust files are within 50 code lines (maximum 44).

Strict documentation rendering, the full application test-target native type check, and source-file checks are recorded in `artifacts/consolidation-tool-core/`. No Cargo commands or Python were executed. Concrete tool and registry/application migrations and workspace CI coverage remain under the active goal.

### Verification restriction and LSP response ownership

The user subsequently prohibited rustc as well as Cargo. No compiler, rustdoc,
compiler wrapper, or Python was invoked for this stage. Earlier execution logs
remain historical evidence only; they do not validate later source changes.

The stateless completion, hover, location, document-symbol, and workspace-symbol
decoders now live under `crates/codetether-lsp-core/src/response/`. URI parsing
lives under that crate's existing `uri/` module. `src/lsp/client.rs` imports the
decoders, preserving its callers and public client methods. This completes the
response-decoding portion of the protocol-core extraction without adding a
dependency. Connection/cache ownership, diagnostics orchestration, and server
initialization remain in the application; initialization currently embeds the
application's package version and needs an explicit identity contract before
it can move without changing behavior.

Nine static source comparisons pass: all six parser signatures/bodies are
unchanged except visibility, the application client changes only its imports,
and the core root/URI module changes only register and export the moved code.
The original implementation files are removed. All seven new Rust files are
within 50 code lines (maximum 46). The application file-limit check and
`git diff --check` pass. Compilation, runtime tests, and six new documentation
examples are **not run**. Evidence and source hashes:
`artifacts/consolidation-lsp-response/`.

The full workspace goal remains active. These source comparisons do not prove
Rust type correctness, dependency resolution, or runtime behavior, and the
remaining application owners and workspace CI coverage are still pending.

### DeepSeek transport and shared HTTP pool extraction

`codetether-provider-deepseek` owns the DeepSeek adapter, grouped into provider
state, request conversion, response interpretation, ordered model metadata, and
HTTP execution. Its public provider type, public configuration fields, and
constructor remain available through `src/provider/deepseek/mod.rs`. Registry
dispatch and credential loading remain in the application.

The process-wide provider HTTP pool now belongs to provider-core. The application
shared-HTTP module re-exports it, so existing adapters and the extracted DeepSeek
adapter share one lazy singleton. The socket-options dependency already belonged
to provider-core. The core manifest now explicitly declares once_cell and the
reqwest rustls feature used by this implementation; the new adapter's dependency
and lockfile entries use existing package versions.

All 35 moved functions, methods, and data declarations preserve their tokens after
normalizing module paths and visibility. The shared HTTP singleton/accessor also
matches, and the Provider trait implementation retains its attributes and body.
All 25 adapter Rust files and the moved HTTP file meet the 50-code-line limit
(maximum 47). Source checks also verify the two compatibility facades, removal of
the old implementation, and manifest/lockfile dependency wiring. The application
file-limit check and `git diff --check` pass.

No compilation, runtime tests, or documentation tests were run for this stage.
No Cargo, rustc, rustdoc, compiler wrapper, or Python was invoked. Source
equivalence supports behavior preservation but does not prove type correctness
or dependency feature resolution. Evidence: `artifacts/consolidation-deepseek/`.
Remaining adapters, orchestration owners, and workspace CI coverage remain under
the active consolidation goal.

### Portable image ownership and Chat transport conversion

`codetether-image` now owns the image attachment type, data-URL normalization and
limits, rejection classification/messages, RGBA-to-PNG encoding, and bounded
image fitting. Its only dependencies are the existing base64, image, and tracing
libraries. `session::ImageAttachment` and the clipboard parsing/fitting API
remain re-exports of the same types/functions. Native clipboard operations,
including the Windows TUI bridge, remain in the application; only their portable
pixel encoder moved.

Provider-core now owns the Chat image decorator and its tool-reply splitting and
companion ordering helpers. This code depends only on provider contracts and JSON;
it does not decode images or depend on the image crate. The application wrapper
retains its nine test files against concrete Google, Moonshot, OpenRouter,
Copilot, and StepFun serializers.

Twenty-nine static declaration comparisons pass, including the eight existing
portable image tests and the shared Chat conversion functions. All nine provider
integration-test files are byte-identical. Session/capture source changes match
the expected re-exports and moved helper imports. Seventeen new Rust files have
at most 39 code lines. Global file-limit and whitespace checks pass. Compilation,
runtime tests, documentation tests, and clipboard execution were not run; no
Cargo, rustc, rustdoc, compiler wrapper, or Python was invoked. Evidence:
`artifacts/consolidation-image-core/`.

The provider audit also found that `src/provider/zai.rs` still implements a
separate active provider and no application source refers to
`codetether_provider_zai`. The existing Z.AI crate therefore does not establish
a completed migration. Reconciliation must preserve current construction,
request, streaming, and model-discovery behavior before switching the facade;
it remains an explicit pending part of the provider inventory. The shared-helper
stage below subsequently connects the application to part of that crate without
claiming the provider migration is complete.

### Z.AI shared catalog and streaming helper ownership

The active adapter now imports the existing crate's catalog merge, output tracker,
streaming request builder, raw capture/fault logic, and reasoning-delta helpers.
Duplicate application sources and their duplicate tests are removed. Fourteen
crate source/test files are grouped under `catalog/`, `capture/`, `delta/`,
`output/`, and `stream_request/` without changing their public module paths.

Eighty-four declaration comparisons match both original implementations after
normalizing application-qualified type paths. Application source changes match
only the expected imports/re-exports; request execution, discovery, conversion,
and stream decoding remain untouched. The grouped files stay within the line
budget (maximum 47 code lines), and file-limit/whitespace checks pass. Compilation,
runtime tests, and documentation tests were not run. Evidence:
`artifacts/consolidation-zai-shared/`.

The full Z.AI provider migration remains pending because inspected behaviors
differ: constructor URL normalization, buffered request fields, packed tool-result
handling, and ordinary-text joining. Directly substituting the crate's current
provider would change the active application's behavior. Those differences and
their source locations are recorded in the evidence README for reconciliation.

### Z.AI history format reconciliation

The shared `history` module now owns role dispatch, ordinary text joining, tool
reply selection, assistant text/tool-call assembly, and historical reasoning.
`HistoryFormat::SingleReply` preserves the application's newline separator and
first-content-part tool handling. `ExpandedReplies` preserves the injected
client's concatenated text and packed-result expansion. Each caller supplies
its existing argument normalizer. Both use one assistant serializer.

The application converter is now a delegate; its other code and existing tests
remain unchanged. Three extracted assistant blocks match their original tokens
after reconstructing helper calls. The injected-client normalizer remains
byte-identical. Ordinary/tool format branches were inspected against both
originals; three regression tests document these differences but were not run.
All twelve changed/new crate Rust files stay within 50 code lines (maximum 37).
Standalone rustfmt formatted the new source; static source, line-limit, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python was
invoked. Compilation and all executable tests remain unverified for this stage.
Evidence: `artifacts/consolidation-zai-history/`.

Constructor, request-parameter, discovery, and stream-decoding reconciliation
remain pending before the full Z.AI provider can move behind one application
facade. The broader workspace goal remains active.

### Z.AI buffered request reconciliation

The crate's request module separates common buffered payload encoding, optional
generation controls, tool-schema conversion, and format selection. The
application uses `BufferedFormat::Application`, preserving omission of explicit
stream/top-p/stop fields. The injected-client wrapper uses `ExplicitControls`,
preserving its existing fields. Each caller retains its history conversion and
thinking policy; HTTP dispatch, retries, headers, and response decoding are
unchanged.

Static checks compare common application assembly, tool conversion, optional
control statements, and the exact adapter replacement. Six grouped Rust files
contain at most 33 code lines. The global file-limit and whitespace checks pass;
standalone rustfmt formatted the source. One regression test was added but not
run. Compilation, runtime tests, and documentation tests remain unverified.
No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Evidence:
`artifacts/consolidation-zai-request/`.

Constructor and remaining normalization/discovery/response/stream ownership
still require reconciliation before the full Z.AI migration is complete.

### Z.AI thinking and argument policy ownership

Thinking decisions, explicit instruction detection, and JSON configuration now
live in the crate's thinking/ module. Tool argument normalization and primitive
field recovery live in tool_arguments/. The application imports the thinking
functions and delegates its history argument normalizer. The unused private
salvage wrapper was removed; no caller needs that application-local method.

Five moved function bodies retain the active application's tokens after required
visibility/name/path changes. The rest of the application matches only the
expected imports and delegates. Source review of the former crate implementation
found the same thinking predicates and argument branches; the shared version
retains the application's compact serialization fallback and regex panic text.
This is source evidence, not an executed equivalence claim.

All seven changed/new Rust files contain at most 28 code lines. Static source,
global line-limit, and whitespace checks pass; standalone rustfmt formatted the
source. Existing tests were retained but not run. No Cargo, rustc, rustdoc,
compiler wrapper, or Python ran. Evidence: artifacts/consolidation-zai-policy/.
Constructor, discovery, response, and streaming ownership remain pending.

### Z.AI discovery ownership

The crate's discovery/ module owns application HTTP discovery, application entry
decoding, and result reporting. The application method delegates with its
existing client, credential, and endpoint. Empty-list fallback, endpoint
selection, and failure/success logging retain their implementation.

Injected-client discovery and model decoding are grouped in the same domain but
retain their distinct contracts: application IDs are trimmed and filtered,
server display names are retained, and vision is not inferred; injected IDs are
preserved as names and vision is inferred. Two new tests describe these
differences and have not been run.

Static reconstruction inlines the extracted helpers to match the original
application discovery method. The rest of the application is unchanged apart
from delegation, and injected discovery/decoding retain their tokens. All seven
grouped Rust files contain at most 40 code lines. File-limit and whitespace
checks pass. Standalone rustfmt formatted the source; no Cargo, rustc, rustdoc,
compiler wrapper, or Python ran. Compilation and executable tests remain
unverified. Evidence: artifacts/consolidation-zai-discovery/.

Constructor, completion HTTP, response interpretation, and SSE state ownership
remain pending under the full active goal.

### Z.AI buffered completion ownership

Application HTTP execution and response interpretation now live under the
crate's completion/ module, separated into sending, parsing, content, finish
reason, and usage modules. The injected completion method moved into the same
domain unchanged. The application retains request construction and delegates
execution. Five duplicate application wire structs were removed after shape
comparison with the crate's existing types.

The active response contract is preserved: empty content is omitted, reasoning
diagnostics remain, cached input is subtracted from ordinary prompt tokens, and
full unstructured buffered HTTP errors are retained. Injected-client response
handling remains distinct where its contract differs.

Twelve static checks cover extracted blocks, response orchestration, wire types,
and caller boundaries. Nine grouped files have at most 43 code lines. Source
limits and whitespace checks pass. Two regression tests were added but not run.
Standalone rustfmt formatted the files; no Cargo, rustc, rustdoc, compiler
wrapper, or Python ran. Compilation and runtime tests remain unverified.
Evidence: artifacts/consolidation-zai-completion/. Constructor and SSE ownership
remain pending before the complete Z.AI migration can be claimed.

### Z.AI streaming ownership

The application now delegates streaming HTTP execution and SSE decoding to the
crate. Its streaming/ domain groups HTTP sending/errors, byte decoding,
buffer/frame/choice handling, state, and usage accounting. The injected-client
streaming implementation uses the same decoder. Application stream-state,
wire, assembly, and terminal helpers were removed, and three inline stream
regression tests moved into the crate.

The shared decoder retains the accounting difference found during inspection:
application streams report total prompt tokens and retain a supplied zero cache
count; injected streams subtract cached input. The state default retains the
injected behavior and the application entry point explicitly selects total
prompt accounting. A new regression test describes the difference.

Twelve static checks cover preserved modules, state/frame changes, application
HTTP and usage code, non-stream methods, and migrated tests. Eighteen grouped
files contain at most 44 code lines. Decoder equivalence was reviewed against
the original loop but has not been executed. File-limit and whitespace checks
pass; standalone rustfmt formatted the source. No Cargo, rustc, rustdoc,
compiler wrapper, or Python ran. Compilation and all tests remain unverified.
Evidence: artifacts/consolidation-zai-streaming/.

The provider shell, constructor, request assembly, and remaining inline
application tests still need migration before Z.AI is fully consolidated.

### Z.AI application provider migration

The application provider shell and its twelve remaining tests now live under
the crate's application/ module. Construction, state, debug formatting, model
delegation, history/tool preparation, capability selection, request preparation,
trait delegation, and tests have separate owners. The application Z.AI module
is only a compatibility export; registry and credential selection remain outside
the provider crate.

The application-compatible type preserves unmodified endpoint strings, newline
history, first-only packed tool replies, its buffered fields and response rules,
and total-input SSE usage. The existing injected-client type keeps its distinct
API and conventions. Both adapters use crate-owned protocol services. Twenty-five
data/method/test comparisons preserve the original application source after
necessary boundary and formatting changes; the default endpoint matches.

All 120 crate Rust files are reachable through the lexical module graph,
including test and conditional declarations, with no missing module paths or
unreferenced files. Every file is within 50 code lines; the 25 introduced by this
stage have at most 26. Unreferenced duplicate model-merge files and an old capture
test were archived and removed. Source and evidence:
artifacts/consolidation-zai-shell/.

Standalone rustfmt, source comparisons, the module-path audit, global line
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper,
or Python ran. Compilation and executable tests remain unverified. Z.AI source
ownership is consolidated; other provider/application owners and workspace CI
coverage remain part of the active goal.

### Anthropic-compatible provider ownership

The new codetether-provider-anthropic crate owns provider state/configuration,
Messages API request conversion, caching policy, model catalogs, HTTP execution,
response interpretation, and SSE processing. It depends on provider-core for
contracts and the existing HTTP pool, and on the portable image crate for image
fitting. The application retains provider selection, credentials, and its
session/cache-prefix integration test.

The application exports the same AnthropicProvider path and Vertex image
converter path. The crate exposes message encoding for the retained integration
test, whose source is unchanged. All 108 compared declarations retain their
implementation tokens after module-path and necessary visibility normalization.
All 48 crate Rust files are reachable in the lexical module graph and remain
within the 50-code-line limit (maximum 49).

Static source, module-path, dependency-wiring, file-limit, and whitespace checks
pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation,
runtime tests, provider HTTP behavior, and documentation examples remain
unverified. Evidence: artifacts/consolidation-anthropic/. Other provider and
application owners and workspace CI coverage remain pending.

### Moonshot provider ownership

The new codetether-provider-moonshot crate owns direct Moonshot completion.
Construction, request policy, role/tool conversion, response wire types,
HTTP/error handling, content/usage/finish interpretation, and ordered model
metadata have separate modules. The application re-exports MoonshotProvider,
including its message converter used by the existing cross-provider image tests.

Seventeen static checks compare declarations and reconstruct complete message
conversion and buffered completion from their extracted helpers. Model order,
metadata, K2 temperature/thinking policy, and unsupported structured-streaming
behavior are preserved. All 23 crate Rust files are reachable in the lexical
module graph and contain at most 41 code lines.

Standalone rustfmt, source comparisons, module paths, global file limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, runtime tests, HTTP behavior, and documentation examples remain
unverified. Evidence: artifacts/consolidation-moonshot/. Other provider and
application owners remain part of the active workspace goal.

### OpenRouter provider ownership

The new codetether-provider-openrouter crate owns client configuration, history
and sanitized tool schemas, reasoning configuration, bounded live model
discovery, extended error envelopes, buffered completion, and the existing
stateful SSE parser. The application re-exports OpenRouterProvider and the
capability, reasoning-level, and runtime-config modules used by sessions/TUI.

Seventy-four static checks preserve declarations and existing helpers, then
reconstruct message conversion, buffered completion, and model discovery.
Response logging order, body limits, timeouts, and streaming parsing are
retained. All 52 Rust files are reachable in the lexical module graph and
contain at most 49 code lines. An unreferenced coalescing implementation was
archived and removed rather than enabled as part of the extraction.

Standalone rustfmt, source comparisons, module paths, global line limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, runtime tests, HTTP behavior, and documentation examples remain
unverified. Evidence: artifacts/consolidation-openrouter/. Other provider and
application owners remain pending under the full workspace goal.

### Google service-account authentication ownership

The new codetether-google-auth crate owns service-account credential parsing,
project selection, RSA signing state, JWT assertions, token exchange, and the
per-provider access-token cache. Vertex Anthropic and Vertex GLM retain their
own endpoint construction and configured HTTP clients and delegate authentication
to the shared service.

The original token methods are token-identical. Source reconstruction verifies
the complete shared algorithm, including its refresh threshold, expiry default,
cache lock pattern, and errors. Both provider integrations preserve credential
validation order and otherwise change only auth state/delegation and associated
imports/diagnostic access.

All six Rust files are reachable and contain at most 39 code lines. Static
source, module-path, global line-limit, and whitespace checks pass. Standalone
rustfmt formatted the source; no Cargo, rustc, rustdoc, compiler wrapper, or
Python ran. Compilation, tests, signing, and token exchange remain unverified.
Evidence: artifacts/consolidation-google-auth/. The remaining Vertex provider
implementation and workspace owners still require consolidation.

### Google Gemini provider ownership

The new codetether-provider-google crate owns provider construction and
validation, Google-compatible role conversion, thought-signature round trips,
request settings, HTTP/error handling, response wire types, cached-input usage,
and model metadata. The application re-exports its GoogleProvider type and
converter for the existing cross-provider image integration tests.

Twenty-four static checks preserve declarations and reconstruct complete
message conversion, buffered completion, and model order/data. All 30 crate
Rust files are reachable in the lexical module graph and contain at most
39 code lines. The existing unsupported structured-streaming behavior is
preserved; no embedding override was introduced.

Standalone rustfmt, source checks, module paths, global file limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, runtime tests, provider HTTP behavior, and documentation examples
remain unverified. Evidence: artifacts/consolidation-google/. Other provider
and application owners remain pending under the full workspace goal.

### StepFun provider ownership

The new codetether-provider-stepfun crate owns provider configuration, typed
wire data, role/image conversion, request preparation, model metadata, buffered
HTTP completion, and streaming processing. These responsibilities are grouped
under request, wire, response, transport, catalog, and streaming modules.
The application provider module is a compatibility export.

Thirty-two static checks preserve declarations and reconstruct request roles,
typed request preparation, buffered completion, streaming HTTP, data-line
processing, and event selection. All 42 files are reachable in the lexical
module graph and have at most 43 code lines.

The existing first-event-per-HTTP-chunk stream selection is preserved, including
discarding further decoded events in that chunk. Two tests describe that
behavior and terminal handling; they were not run. This is an explicit retained
behavior, not a streaming fix bundled with consolidation.

Standalone rustfmt, static source checks, module paths, global line limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, runtime tests, HTTP behavior, and documentation examples remain
unverified. Evidence: artifacts/consolidation-stepfun/. Other provider and
application owners remain part of the active workspace goal.

### Vertex Anthropic provider ownership

The codetether-provider-vertex-anthropic crate owns endpoint/client construction,
message and tool conversion, catalog helpers, buffered retries, response parsing,
and streaming HTTP/event processing. It reuses shared Google authentication and
the Anthropic image converter. The application retains compatibility exports.

Fifty static checks preserve declarations and reconstruct message conversion,
buffered retry handling, and streaming processing. All 46 Rust files are
lexically reachable and contain at most 50 code lines. Existing simplified SSE
behavior is preserved. Compilation, tests, credential exchange, HTTP behavior,
and documentation examples remain unverified. No Cargo, rustc, rustdoc, compiler
wrapper, or Python ran for this extraction. Evidence:
artifacts/consolidation-vertex-anthropic/. The broader workspace goal is active.

### Vertex GLM provider ownership

The codetether-provider-vertex-glm crate owns client construction, request role
conversion, model settings/catalog, wire types, response conversion, buffered
retries, and streaming admission/SSE processing. Google authentication remains
a shared dependency. The application module preserves the existing provider
export and no longer owns implementation.

Fifty-five source checks cover declarations, request construction, complete
retry orchestration, response content/finish/cache accounting, and SSE event
processing. Both retry paths retain token refresh after backoff; their distinct
HTTP error handling and streamed usage behavior are unchanged. All 42 files
are lexically reachable and contain at most 41 code lines. Textual workspace
and lockfile checks, standalone rustfmt, source limits, and whitespace checks
pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation,
dependency resolution, tests, HTTP behavior, and documentation examples remain
unverified. Evidence: artifacts/consolidation-vertex-glm/. Other providers and
application owners remain pending under the active workspace goal.

### GLM-5 vLLM provider ownership

The codetether-provider-glm5 crate owns configuration, role and tool argument
conversion, request settings, wire types, buffered HTTP/response handling, cache
accounting, and SSE decoding. Provider-core retains shared HTTP pooling and
model identity. The application preserves provider/default-model exports and
its configuration documentation.

Fifty-three static comparisons cover declarations, five moved test bodies,
request/response transformations, full completion orchestration, and streamed
usage/tool/finish events. All 49 Rust files are lexically reachable and contain
at most 34 code lines. Buffered/streamed request differences, usage-only events,
and configured-model versus request-model behavior are preserved. Workspace
and lockfile text checks, standalone rustfmt, source limits, and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, dependency resolution, runtime tests, HTTP behavior, and doc
examples remain unverified. Evidence: artifacts/consolidation-glm5/. Remaining
provider and application owners keep the full workspace goal active.

### Copilot provider ownership

The codetether-provider-copilot crate owns request conversion, initiation and
vision headers, model discovery, catalog admission/projection, fallback rows,
price approximations, transport errors, and response conversion. The remaining
application adapter supplies the application package version as User-Agent and
preserves public constructors, enterprise helpers, and image-test conversion.
This prevents a crate move from silently changing the client version header.

Sixty-three source comparisons cover the extracted implementation and moved
test bodies; fourteen adapter checks cover signatures, application identity,
and delegation. All 53 crate Rust files are lexically reachable and contain at
most 44 code lines. Global source limits, whitespace, and textual workspace/lock
wiring checks pass. Existing unsupported structured streaming, catalog rows,
and pricing approximations are preserved. Standalone rustfmt formatted the
source; no Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation,
dependency resolution, runtime tests, HTTP behavior, and documentation examples
remain unverified. Evidence: artifacts/consolidation-copilot/. Other provider
and application owners remain pending under the full workspace goal.

### Bedrock authentication ownership

The codetether-bedrock-auth crate owns credential and region lookup, live
bearer state, canonical URL encoding, IAM request signing/dispatch, and presigned
bearer token generation. A borrowed signing client retains the provider client,
auth state, and region. Application compatibility exports preserve credential
and token APIs; bearer/IAM routing, Vault refresh, audit integration, and the
remaining Bedrock protocol stay application-owned pending further extraction.

Thirty-seven static comparisons cover original methods, signing orchestration
and helpers, and moved golden-vector/URL test bodies. Ten integration checks
cover retained application methods, signing delegation, and compatibility
exports. All 29 files are lexically reachable and have at most 50 code lines.
Standalone rustfmt, global source limits, whitespace, and textual workspace/lock
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, dependency resolution, tests, credential loading, signing, HTTP
behavior, and doc examples remain unverified. Evidence:
artifacts/consolidation-bedrock-auth/. Bedrock and other application owners
remain pending under the full workspace goal.

### Bedrock transcript and pairing ownership

The codetether-bedrock-protocol crate owns role/tool/image conversion, adjacent
role merging, transcript tool exchange repair, final JSON pairing inspection
and repair, error annotation, and final-turn shaping. Image fitting delegates
to codetether-image. Application compatibility exports preserve the shared
Converse and InvokeModel conversion call sites and body-level repair gate.

One hundred sixteen static comparisons preserve declarations/imports and moved
test bodies. All 41 files are lexically reachable and contain at most 44 code
lines. Workspace/lock text checks, both request call sites, source limits,
standalone rustfmt, and whitespace checks pass. No Cargo, rustc, rustdoc,
compiler wrapper, or Python ran. Compilation, dependency resolution, runtime
tests, image processing, and documentation examples remain unverified.
Evidence: artifacts/consolidation-bedrock-protocol/. Bedrock body policy,
response decoding, runtime settings, transport, and application integration
remain pending, alongside the other owners in the full workspace goal.

### Shared context limits and Bedrock request policy

Provider-core now owns the ordered context-window lookup and million-token
predicate. The application re-exports the existing API. Twelve static checks
preserve branch precedence, values, and existing test bodies across five files
with at most 43 code lines. Evidence: artifacts/consolidation-provider-limits/.

Bedrock-protocol now owns Converse body construction, inference controls, cache
placement, tool-history declarations, output budgets, model estimates, and live
runtime overrides. TUI and provider exports share one override cell. Fifty-six
static checks preserve declarations, tests, and the split output-limit lookup;
all 66 protocol files are lexically reachable and have at most 49 code lines.
Application exports, TUI call sites, the singleton, dependencies, source limits,
standalone rustfmt, and whitespace checks pass their static gates. Evidence:
artifacts/consolidation-bedrock-policy/.

No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation,
dependency resolution, runtime tests, and documentation examples remain
unverified. Bedrock response/streaming/transport, refresh, and application
integration remain pending under the full workspace goal.

### Bedrock buffered response ownership

Bedrock-protocol now owns Converse and native InvokeModel response decoding,
wire representations, content/finish/usage conversion, service error payloads,
reasoning-content conversion, empty-output validation, and reasoning signature
audit recording. The audit hook references codetether-audit, preserving the
application singleton and its original event and spawn behavior. Application
exports and existing streaming audit/guard call sites remain compatible.

Twenty-six source comparisons preserve the original declarations, reconstructed
parser orchestration, audit hook, and five moved regression tests. All 90
protocol files are lexically reachable and have at most 49 code lines. Static
dependency/export/visibility checks, standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python
ran. Compilation, dependency resolution, tests, audit writes, runtime behavior,
and documentation examples remain unverified. Evidence:
artifacts/consolidation-bedrock-response/. Streaming, transport, discovery,
and refresh remain pending under the active workspace goal.

### Bedrock stream decoding ownership

Bedrock-protocol now owns incremental binary frame buffering, header parsing,
stream state, content/tool/reasoning events, usage metadata, and the byte-to-chunk
pump. The application retains signed requests, auth recovery, and initial HTTP
error admission. Its request method is token-identical and delegates decoding
through a compatibility import. Original framing and terminal semantics remain.

Twenty-four static comparisons cover the original implementations, split header
decoding, and seven moved regression tests. All 103 protocol files are lexically
reachable and contain at most 50 code lines. Dependency/export/visibility checks,
source limits, standalone rustfmt, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, dependency resolution,
tests, streaming behavior, and documentation examples remain unverified.
Evidence: artifacts/consolidation-bedrock-stream/. HTTP orchestration, discovery,
refresh, and native request conversion remain pending under the full goal.

### Bedrock native request ownership

Bedrock-protocol now owns native Anthropic Messages request bodies, block and
message conversion, tool definitions/history declarations, and native pairing
repair. The native request/response formats share a protocol owner. Application
exports preserve current body/converter paths; routing and HTTP execution stay
application-owned.

Twenty-four static comparisons preserve original declarations and eight moved
request/pairing tests. Integration checks preserve routing code/test and the
completion call site. All 117 protocol Rust files are lexically reachable and
have at most 50 code lines. Standalone rustfmt, source file limits, and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, dependency resolution, tests, runtime behavior, and documentation
examples remain unverified. Evidence: artifacts/consolidation-bedrock-native/.
HTTP orchestration, discovery, retry handling, and refresh remain pending under
the full workspace goal.

### Bedrock HTTP and discovery ownership

The codetether-provider-bedrock crate owns a client borrowing the application
HTTP client, auth state, and region. It owns runtime/management URLs, bearer/IAM
request dispatch, foundation/profile fetches, catalog filtering and metadata
projection, merging, and final sorting. The application retains completion
retry/recovery orchestration and SSO refresh and delegates HTTP/discovery calls
through compatibility methods.

Twelve static comparisons preserve dispatch methods and discovery ordering,
filters, projections, and modality extraction. All 11 files are lexically
reachable and contain at most 43 code lines. Workspace/lock references, borrowed
state, application delegation, source limits, standalone rustfmt, and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, dependency resolution, tests, HTTP behavior, actual catalog
contents, and documentation examples remain unverified. Evidence:
artifacts/consolidation-bedrock-client/. Completion orchestration, retry/refresh
coordination, and SSO refresh remain pending under the full workspace goal.

### Bedrock execution and recovery coordination

The Bedrock client crate now owns Converse/InvokeModel execution, HTTP streaming
admission, synthetic native streaming, retry policy, URL encoding, and error
mapping. A CredentialRecovery interface delegates refresh to the unchanged
application Vault/SSO recovery function. The client retains recovery guards,
attempt accounting, retry statuses, and delay behavior. Application methods
now delegate through the borrowed client.

Twenty-seven static comparisons cover loops, parameters, retry/error/URL
helpers, synthetic chunks, and existing test bodies. All 27 client files are
lexically reachable and have at most 44 code lines. Callback/dependency checks,
source limits, standalone rustfmt, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, dependency resolution,
tests, recovery execution, timing, HTTP behavior, and doc examples remain
unverified. Evidence: artifacts/consolidation-bedrock-execution/. SSO refresh
and remaining application ownership remain pending under the full goal.

### Bedrock SSO refresh ownership

The codetether-bedrock-refresh crate owns silent OIDC refresh, role credential
retrieval, token rotation handling, staleness/expiry assessment, bearer minting,
and metadata-preserving secret persistence. It calls the same shared secrets
service re-exported by the application. Startup, CLI, and mid-session recovery
keep their existing ensure_fresh/refresh_now call sites via compatibility exports.

Forty static comparisons preserve declarations and existing test bodies. All
15 files are lexically reachable and contain at most 44 code lines. Workspace
and lock entries, secret-service wiring, application entry points, source
limits, standalone rustfmt, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, dependency resolution,
tests, credential exchange, secret writes, and documentation examples remain
unverified. Evidence: artifacts/consolidation-bedrock-refresh/. Remaining
application adapter structure and other owners remain under the full goal.

### Bedrock application adapter cohesion

The remaining Bedrock adapter separates state/diagnostics, constructors, public
convenience methods, and authentication validation. Its root now declares
modules and compatibility exports. Assembly, routing, and recovery integration
remain appropriate application responsibilities; execution, protocol, auth
mechanics, and refresh/persistence have workspace crate owners.

Fourteen static comparisons preserve the struct, region default, moved methods,
and existing tests. The affected adapter files contain at most 33 code lines.
All 50 Bedrock application Rust files are lexically reachable, with no orphaned
files, and have at most 48 code lines. Standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python
ran. Compilation, tests, and documentation examples remain unverified. Evidence:
artifacts/consolidation-bedrock-adapter/. Other provider and application domains
remain under the full workspace goal.

### Codex OAuth ownership

The codetether-codex-auth crate owns OAuth credential types, PKCE/state
generation, authorization URLs, code/identity/refresh exchanges, expiry
calculation, and account claims. Existing provider associated methods delegate
with unchanged signatures. Cache state, cross-process refresh coordination,
and Vault persistence remain provider-owned pending further extraction.

Twenty-two static comparisons preserve method bodies, wire/credential types,
and constants. Wiring checks cover 12 delegates, workspace/lock references,
credential exports, explicit base64 extension imports, and 191 literal include
targets. All 16 files are lexically reachable and have at most 44 code lines.
Standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, dependency resolution,
tests, generated OAuth material, network exchanges, and doc examples remain
unverified. Evidence: artifacts/consolidation-codex-auth/. Remaining Codex
implementation and other owners stay under the full workspace goal.

### Codex credential lifecycle ownership

Codex-auth now owns shared credential state, freshness policy, process locking,
Vault parsing/persistence, peer-rotation adoption, merge helpers, and refresh
sequencing. CredentialManager borrows existing provider state; the transport
access-token cache and backend opt-in gate remain in the provider. Registry
credential parsing delegates to the shared implementation.

Twenty static comparisons preserve the original algorithms and state. Twelve
lifecycle files contain at most 44 code lines; all 28 auth-crate files are
lexically reachable. Delegates, borrowed state, secret-service imports,
dependencies, and 191 include targets pass wiring checks. Standalone rustfmt,
source limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrapper, or Python ran. Compilation, dependency resolution, tests, process
locking, Vault access, token refresh, and documentation examples remain
unverified. Evidence: artifacts/consolidation-codex-lifecycle/. Remaining Codex
request/response/streaming/transport and other owners stay under the full goal.

### Codex message protocol ownership

Codex-protocol owns chat and Responses encoding, image shapes, reasoning
checkpoint/event handling, request-projection tool repair, and the nonempty
input anchor. The application retains method and reasoning compatibility
paths. Encoding, repair, and anchor order are preserved; saved history is
not mutated by these projection helpers.

Forty static comparisons cover methods, formats, and moved tests. Sixteen
provider delegates keep their signatures and 191 literal include targets
resolve. All 27 crate files are lexically reachable and have at most 39 code
lines. Workspace wiring, standalone rustfmt, source limits, and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, dependency resolution, tests, wire behavior, and doc examples
remain unverified. Evidence: artifacts/consolidation-codex-protocol/. Response
parsing, streaming, transport state, and other workspace owners remain pending.

### Codex response protocol ownership

Codex-protocol now owns SSE framing, Responses event decoding and function-call
assembly, completion collection, output checkpoints, error classification, and
WebSocket event lifecycle decisions. Parser state, event handlers, collector,
and checkpoint/error modules are separate. The provider retains transport
connections, pooling, polling, retry orchestration, and compatibility methods.

Fifty-one static comparisons preserve original bodies, state, constants, and
moved tests. Twenty-five delegates retain their signatures and forwarding
arguments; 188 literal include targets resolve. All 63 protocol Rust files
are lexically reachable and contain at most 41 code lines. Manifest/lockfile
wiring, standalone rustfmt, source limits, and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation,
dependency resolution, tests, documentation examples, event ordering, and
connection reuse remain unverified. Evidence:
artifacts/consolidation-codex-response/. Remaining Codex transport/request
execution and other workspace owners remain under the active full goal.

### Codex request payload ownership

Codex-protocol now owns HTTP payloads, WebSocket create events, instruction
extraction/defaults, stable cache keys, service-tier insertion, reasoning
request objects, and backend request identity in nine separate modules.
The provider retains runtime configuration and model-option resolution, then
passes resolved values to the encoder. Request fields and backend differences
are preserved by source comparison.

Nine source/delegate comparisons and compatibility/dependency checks pass.
All 72 protocol Rust files are lexically reachable, and the nine new request
modules contain at most 40 code lines. Earlier response comparisons/delegates,
standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, dependency resolution,
tests, documentation examples, and runtime wire behavior remain unverified.
Evidence: artifacts/consolidation-codex-request/. Transport ownership and
other inventory rows remain part of the active workspace consolidation goal.

### Codex HTTP transport ownership

Codex-transport owns HTTP opening retries, retry classifications, delay
selection, diagnostics, and SSE response consumption. Provider factories
still acquire credentials and open backend-specific requests. The shared
transport depends on protocol parsing and provider-core timing, with no
application dependency. Driver and recovery compatibility paths remain.

Twenty-nine static source comparisons cover methods, retry states, constants,
and moved classification tests. All eleven transport Rust files are lexically
reachable and contain at most 46 code lines. Driver/dependency/compatibility
wiring and 188 literal include targets pass. Standalone rustfmt, source limits,
and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or
Python ran. Compilation, dependency resolution, tests, doc examples, delays,
network behavior, and stream ordering remain unverified. Evidence:
artifacts/consolidation-codex-transport/. WebSocket transport/state and other
workspace owners remain under the active consolidation goal.

### Codex WebSocket transport ownership

Codex-transport owns typed connections, JSON I/O, streamed frame handling,
deadlines, terminal recycling, session health, turn routing state, and bounded
idle pooling. Connection construction/send/receive/close use separate modules.
The provider retains authenticated connection setup and request assembly;
connection and state paths re-export shared owners. The routing-header constant
now has one shared owner. Public socket types use the same dependency version
and TLS features as the application.

Forty-seven source comparisons cover bodies, types, constants, and moved tests.
All 32 transport Rust files are reachable and have at most 48 code lines.
Compatibility/dependency checks and 186 include targets pass, as do earlier
HTTP source comparisons, standalone rustfmt, source limits, and whitespace
checks. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation,
dependency resolution, tests, doc examples, connection reuse, locking, and
eviction remain unverified. Evidence: artifacts/consolidation-codex-websocket/.
Remaining provider assembly/execution and other workspace owners stay under
the full active goal.

### Codex request execution ownership

Codex-transport now owns borrowed-client HTTP transmission, successful routing
capture, response validation/error formatting, retry-header metadata, upgrade
headers, request-shape diagnostics, and transport endpoint/version constants.
The provider keeps payload/model preparation, credential acquisition, OAuth
refresh, and backend selection. HttpTransport borrows existing client/state;
ChatGPT 401 refresh remains in its existing application sequence.

Sixteen source comparisons and seven adapter signature/forwarding checks pass.
All 44 transport files are lexically reachable and at most 48 code lines.
All 186 include targets resolve. Standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python
ran. Compilation, dependency resolution, tests, doc examples, request bytes,
OAuth refresh, and HTTP/WebSocket behavior remain unverified. Evidence:
artifacts/consolidation-codex-http-client/. Provider execution/assembly and
other owners remain under the full active goal.

### Codex model policy and discovery ownership

Provider-core model_policy now owns selector metadata, seed filtering, live
metadata conversion, backend eligibility, lazy reasoning-option resolution,
and model transport requirements. Transport owns the account /models request
and existing cache update. Application code retains backend gating, cache
checks before auth, credential acquisition, runtime reads, and listing fallback.

Seventeen source comparisons and eight adapter checks pass. Core has 97 and
transport has 45 lexically reachable Rust files. The eleven moved/new files
have at most 38 code lines, and 186 application include targets resolve.
Standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, dependency resolution,
tests, doc examples, live discovery, cache timing, and upstream availability
remain unverified. Evidence: artifacts/consolidation-codex-model-policy/.
Remaining provider orchestration and other owners remain in the active goal.

### Codex connection setup and stream startup ownership

Codex-transport now owns backend handshake endpoint selection, connection
setup with routing replay/capture, pool lookup, stream startup, and buffered
collection. WebSocketTransport borrows the existing provider health, routing,
and pool state. The provider retains credentials, model/payload preparation,
backend selection, and OAuth refresh. Connection, pooling, startup, and endpoint
operations are separate modules.

Six source comparisons and six adapter signature/forwarding checks pass. All
51 transport Rust files are reachable and at most 48 code lines. All 186
application include targets resolve. Standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python
ran. Compilation, dependency resolution, tests, doc examples, network setup,
pool reuse, and stream ordering remain unverified. Evidence:
artifacts/consolidation-codex-connection/. Provider orchestration and other
workspace owners remain under the full active consolidation goal.

### Codex unused compatibility wrappers

Removed 40 private forwarding methods with no remaining references in the
application Rust source/test roots, along with their include entries. Removed
the unused collector import file and obsolete implementation imports. Public
methods and still-used test/application wrappers remain; shared implementations
are unchanged. This leaves 145 literal Codex include targets, all resolving.

Verification scanned 5,886 remaining application Rust files with no identifier
references to removed methods. Original single-method/private status and source
hashes were checked. Shared protocol/core/transport module graphs, standalone
rustfmt, source limits, and whitespace checks pass. No Cargo, rustc, rustdoc,
compiler wrapper, or Python ran. Compilation, macro expansion, tests, and runtime
behavior remain unverified. Evidence: artifacts/consolidation-codex-facade/.
Older extraction verifiers expecting these wrappers are now historical; the
facade verifier records their intentional removal. Other workspace owners and
remaining provider orchestration stay under the active full goal.

### Shared tool-result message ownership

Tool-core messages now owns call-associated feedback/status construction and
structured image attachment. Session helper paths and the Codex image-test
alias re-export those constructors. This removes a session-owned implementation
from the provider/session boundary while keeping the same core types and image
feedback adapters. Constructor, retention, and image responsibilities use
separate modules.

Four source comparisons cover both constructors and the moved test bodies.
All 31 tool-core files are lexically reachable; the five new files have at most
36 code lines. Session exports, standalone rustfmt, source limits, and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, tests, documentation examples, and runtime integration remain
unverified. Evidence: artifacts/consolidation-tool-messages/. Remaining session
and provider orchestration, plus other owners, remain under the active goal.

### Codex provider orchestration ownership

Codetether-provider-codex owns the remaining provider, runtime configuration,
credential/backend selection, completion entrypoints, and tests. The application
path is a facade. Auth, protocol, transport, model policy, shared HTTP, and tool
message construction retain their existing shared owners. The new crate has
no application dependency. Necessary cross-crate catalog/method visibility and
test import/filter adaptations are documented in its extraction evidence.

171 whole-file token comparisons and an extracted test-server body comparison
pass. All 173 crate Rust files are reachable through module/include declarations
and have at most 50 code lines. Compatibility paths, singleton wiring, manifest/
lock entries, standalone rustfmt, source limits, and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation, dependency
resolution, tests, doc examples, and runtime behavior remain unverified. Evidence:
artifacts/consolidation-provider-codex/. Other provider implementations/registry
and all remaining inventory owners stay under the full active goal.

### Shared TetherScript value conversion

Codetether-tetherscript-values owns JSON/runtime value conversion and its existing
integer/resource tests. The tool path is a compatibility export, while provider
calls now use the shared crate directly. This removes provider dependence on
tool implementation modules. The root dependency is optional under the existing
tetherscript feature and uses the existing locked runtime package.

Eight function/test comparisons plus provider-call namespace correspondence pass.
All four crate files are lexically reachable and at most 43 code lines. Feature/
manifest/lock checks, standalone rustfmt, source limits, and whitespace checks
pass. No Cargo, rustc, rustdoc, compiler wrapper, Python, or TetherScript execution
ran. Compilation, tests, feature combinations, and runtime behavior remain
unverified. Evidence: artifacts/consolidation-tetherscript-values/. Provider
orchestration and the other inventory owners remain under the active full goal.

### TetherScript provider ownership

Codetether-provider-tetherscript owns the script provider and USAI adapter, with
separate authority, hook call, completion, model normalization, and trait modules.
The application path re-exports the script provider and USAI; the native Cerebras
constructor remains in application setup. Provider-core contracts and the shared
value-conversion crate replace application imports. USAI retains its existing
workspace example script include without copying or changing its contents.

Eleven whole-file token comparisons pass. All eleven modules are reachable and
have at most 50 code lines. Compatibility paths, optional feature/dependency
entries, lock metadata, embedded script resolution, standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper,
Python, TetherScript execution, or network request ran. Compilation, tests,
feature combinations, and runtime behavior remain unverified. Evidence:
artifacts/consolidation-provider-tetherscript/. Other provider implementations,
application registry setup, and remaining owners stay under the active goal.

### Gemini Web protocol ownership

Gemini-web-protocol owns prompt rendering, response selection, reasoning/UI
markup handling, tool parsing/validation, usage estimates, stream events, and
stateless wire/error helpers. The provider retains cookie/session-token handling,
HTTP requests, retries, and orchestration. Six compatibility modules and seven
method delegates preserve application paths. Existing opt-in capture behavior
moved unchanged and was not executed.

63 whole-file comparisons, seven method comparisons, and the remaining provider
body comparison pass with explicit namespace/visibility adaptations. All 72
crate files are lexically reachable and at most 49 code lines. Facade/dependency
checks, standalone rustfmt, source limits, and whitespace checks pass. No Cargo,
rustc, rustdoc, compiler wrapper, Python, or provider request ran. Compilation,
tests, documentation examples, and live protocol behavior remain unverified.
Evidence: artifacts/consolidation-gemini-web-protocol/. Gemini Web transport
and other owners remain under the full active consolidation goal.

### Gemini Web session transport ownership

Gemini-web-transport owns cookie parsing, home-page session-token extraction,
account routing, cache refresh/invalidation, and the shared origin. It borrows
the provider client, cookie string, and token-cache mutex. HTML parsing is a
separate helper; regex priority, diagnostics, TTL, and lock scope are retained.
The application still owns StreamGenerate execution and provider orchestration.

Seven source comparisons, four delegate checks, and the remaining provider-body
comparison pass. All seven transport modules are reachable and at most 40 code
lines. Borrowed-state/dependency checks, standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, Python,
cookie access, or HTTP request ran. Compilation, tests, live parsing, and cache
concurrency remain unverified. Evidence: artifacts/consolidation-gemini-web-session/.
The remaining Gemini Web transport and other owners stay under the active goal.

### Gemini Web HTTP execution ownership

Gemini-web-transport owns StreamGenerate construction and bounded retries, with
separate header metadata, timestamp, URL, send, and body-read helpers. Protocol
owns the model/mode mapping and fallback. The application retains client
construction, tool-protocol retry, completion assembly, and the Provider trait.

Nine reconstruction/source checks preserve helper operations, error propagation,
request/retry sequencing, model mapping, and the provider remainder. All 13
transport and 74 protocol modules are reachable and within 50 code lines.
Dependency/form-feature wiring, standalone rustfmt, source limits, and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrapper, Python, or network request
ran. Compilation, tests, request bytes, retry timing, and live behavior remain
unverified. Evidence: artifacts/consolidation-gemini-web-http/. Remaining provider
orchestration and other inventory owners stay under the full active goal.

### Gemini Web provider ownership

Codetether-provider-gemini-web owns the remaining provider orchestration,
construction, state, validated retries, completion/stream assembly, trait
entrypoints, and tests. The application path re-exports its provider type.
Protocol and transport edges now belong to the provider crate. Shared socket
defaults and cookie debug redaction are preserved.

28 method/test/state comparisons plus seven sibling-module comparisons pass.
The two trait forwarders are checked separately. All 35 crate files are reachable
and at most 46 code lines. Facade/dependency checks, standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper,
Python, cookie access, or provider request ran. Compilation, tests, documentation
examples, and runtime behavior remain unverified. Evidence:
artifacts/consolidation-provider-gemini-web/. Other providers and all remaining
workspace owners stay under the full active consolidation goal.

### Shared request-routing identity

Provider-core request_identity now owns metadata injection, canonical-block
recognition, and system-string handling. Metrics, cognition, and CLI callers
retain their compatibility import path. Registry/dispatch integration tests
remain in the application; shared delimiter constants support their existing
checks without duplicating production values.

Nine function/constant comparisons and compatibility checks pass. All 100 core
Rust files are reachable, and the three new identity modules have at most 34
code lines. Standalone rustfmt, source limits, and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrapper, or Python ran. Compilation, tests,
doc examples, and runtime dispatch remain unverified. Evidence:
artifacts/consolidation-request-identity/. Metrics instrumentation, registry
composition, and other owners remain part of the active full goal.

### Provider metrics wrapper ownership

Codetether-provider-metrics owns timed calls, stream observation, record
construction/logging, and MetricsProvider. It uses shared core identity and
the existing telemetry singleton. The application preserves the wrapper
path and retains identity/registry integration tests. Provider delegation,
timing, TTFT, terminal recording, and async submission bodies are unchanged.

Fourteen whole-file token comparisons pass. All fifteen crate files are
reachable and at most 47 code lines. Facade/singleton/dependency checks,
standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, or Python ran. Compilation, tests, docs, and runtime
metrics behavior remain unverified. Evidence: artifacts/consolidation-provider-metrics/.
Registry composition, native providers, and other owners remain under the
full active consolidation goal.

### Native Bonsai provider ownership

Codetether-provider-bonsai owns provider execution, configuration, loading,
sampling, native math, kernels, and tests. Application registration stays in
its original module. The facade preserves provider and cognition native paths;
necessary native types/fields are exposed across the crate boundary. Root
Candle/CUDA/TetherScript features forward to the new optional feature gates.

80 Rust-file token comparisons and two exact CUDA-file comparisons pass. All
80 Rust modules are reachable through declarations/includes across cfg branches
and contain at most 50 code lines. Facade/feature/dependency/asset checks,
standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrapper, Python, CUDA initialization, kernel compilation, model
load, TetherScript execution, or tests ran. Compilation, GPU parity, feature
combinations, and inference remain unverified. Evidence:
artifacts/consolidation-provider-bonsai/. Local CUDA, application registration,
and other workspace owners remain under the full active goal.

### Shared local-provider configuration and helpers

Provider-core local modules own the CUDA/non-CUDA shared configuration, local
model metadata, environment parsing, prompt formatting, and finish mapping.
Application paths re-export the shared owners; the CUDA runtime/cognition
connection remains in the application. Catalog tests moved with their code.

23 source comparisons cover both old config definitions, defaults, helpers,
catalog, and tests. The provider remainder/delegates also compare. All 112 core
files are reachable; the 12 new files have at most 40 code lines. Compatibility
checks, standalone rustfmt, source limits, and whitespace checks pass. No Cargo,
rustc, rustdoc, compiler wrapper, Python, CUDA, or model execution ran.
Compilation, feature combinations, tests, and runtime behavior remain unverified.
Evidence: artifacts/consolidation-local-provider-core/. Native adapter/runtime
and remaining workspace owners stay under the full active consolidation goal.

### Shared model API catalog ownership

Codetether-model-catalog owns API data types, fetching, lookup, credential-aware
filtering, recommendations, and ModelInfo conversion. Wire types and operations
are separate modules. The application models module is a public facade, and
credential checks use the same shared secrets manager. Existing URLs, defaults,
iteration/filtering, and recommendation order are retained.

28 type/method comparisons pass. All 30 files are reachable and at most 32 code
lines. Facade/manifest/lock checks, standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python
ran. Compilation, tests, fetching, Vault behavior, and data freshness remain
unverified. Evidence: artifacts/consolidation-model-catalog/. Registry setup,
local runtime integration, and other owners stay under the active full goal.

### Shared pricing and cost aggregation

Provider core owns model-price matching and cache multipliers; provider metrics
owns session-cost calculation over shared telemetry. The application pricing
module re-exports all four functions. Values, match order, and arithmetic are
preserved as source data without asserting current pricing accuracy.

Four function comparisons and compatibility/singleton-import checks pass. All
116 core and 16 metrics modules are reachable. The five new files have at most
34 code lines. Standalone rustfmt, source limits, and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrapper, Python, counter access, or pricing
request ran. Compilation, tests, runtime costs, and pricing freshness remain
unverified. Evidence: artifacts/consolidation-provider-pricing/. Other
provider and workspace owners remain under the active full goal.

### Tenant-key payload ownership

Secrets now owns tenant payload decoding, source/provider-name access, and
selected owned-string cleanup. PerTaskProviderKeys remains an application
registry adapter and delegates cleanup from Drop. Payload fields and defaults,
null/empty behavior, public adapter signatures, and registry construction are
retained. This does not expand credential-erasure guarantees.

Six grouped source/adapter checks pass. All 81 secrets files are reachable, and
the four new modules have at most 28 code lines. Standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper,
Python, credential lookup, or actual cleanup ran. Compilation, tests, drop-time
behavior, and registry integration remain unverified. Evidence:
artifacts/consolidation-tenant-key-payload/. Registry composition and other
workspace owners remain under the full active goal.

### Shared provider routing and endpoint policy

Provider-core routing modules own model-path parsing and provider resolution.
The application registry delegates against its existing provider map; registration,
configuration, and Vault composition remain in the application. Anthropic owns
legacy MiniMax endpoint normalization. Existing application paths are retained.
Alias mapping, error messages, nested model paths, and the existing first-map-value
default selection are preserved; no deterministic ordering is introduced.

Eight source comparisons cover moved functions, four existing tests, and the
complete retained registry with its delegation. Lexical module checks find all
120 core and 50 Anthropic files reachable. The six new files and both changed
crate roots meet the 50-code-line limit. Standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrapper, or Python ran.
Compilation, Rust tests, and runtime behavior remain unverified. Evidence:
artifacts/consolidation-provider-routing/. Other inventory owners remain under
the full active consolidation goal.

### Approval review contracts and text conversion

The existing approval-core crate now owns review subjects, verdicts, prompt
construction, and tolerant JSON verdict parsing. Its review module separates
prompt subject/rendering/instructions from parsing and verdict contracts. Public
application paths remain re-exports. The application retains process runtime
configuration, provider selection, swarm execution, and tool-policy integration
because those depend on application services. Review recommendations remain
advisory; this extraction does not change approval grants or tool policy.

Five existing parsing/prompt tests moved with the shared code. Tool-policy and
runner integration tests remain in the application. Twenty-three source
comparisons, module reachability (28 files), standalone rustfmt, source limits,
and whitespace checks pass. The eleven new modules have at most 36 code lines.
No dependency or lockfile changes were needed. No Cargo, rustc, rustdoc, compiler
wrapper, Python, reviewer agent execution, or Rust tests ran. Compilation, tests,
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-review-core/. The full workspace goal remains active.

### Search pipeline and host adapters

Codetether-search owns backend/result contracts, prompt and request construction,
plan parsing, argument normalization, text collection, and sequential pipeline
execution. A BackendExecutor trait accepts host tool execution using the existing
shared ToolResult contract. The application retains its provider-registry lookup,
concrete tool construction, and entrypoint adapter. Existing application module
paths re-export shared APIs. Provider fallback, choice clamping, execution order,
metadata conversion, memory search restriction, and the RLM-unavailable response
are retained in source.

Nineteen source comparisons pass, including the entire execution body with host
selection removed and backend dispatch delegated. The memory restriction test
moved unchanged. All eleven new modules are reachable and have at most 38 code
lines. Manifest/lock comparisons show only the new local package and root edge.
Standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, model request, backend execution, or Rust
tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-search/. The full workspace goal remains active.

### RLM provider bridge

Codetether-rlm now owns the shared-provider wrapper and request/response
conversion in four focused provider_adapter modules. Existing application paths
remain aliases. This removes provider conversion implementation from the host
without bringing cognition or session dependencies into the crate. Role fallback,
tool schemas, max-token limit, text collection, finish formatting, and existing
zero token counters are retained. Session-bus/context conversion remains an
application adapter. The legacy REPL and its tools remain pending consolidation.

Seven source comparisons pass, including retained context conversion. All 334
RLM crate files are lexically reachable; four new modules have at most 41 code
lines. The manifest/lockfile change adds only the existing shared-provider
dependency. Standalone rustfmt, source limits, and whitespace checks pass. No
Cargo, rustc, rustdoc, compiler wrappers, Python, model calls, or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-provider/. The full consolidation goal remains active.

### In-memory RLM REPL ownership

Codetether-rlm owns the in-memory REPL state, runtime labels, DSL output contracts,
context/line/pattern/slice operations, variable expansion, argument parsing, AST
queries, and DSL evaluation in eighteen focused modules. The application REPL
module re-exports these types and retains its legacy model executor and external
process driver. The long command and expression evaluators were split into
ordered helpers, with reconstructed bodies compared against the original.
Existing empty-context chunking and malformed-argument behavior was preserved.

Thirty-two source/reconstruction comparisons pass, including all original REPL
methods, four moved tests, and the full retained application remainder. The new
modules have at most 42 code lines; all 352 crate files are lexically reachable.
Standalone rustfmt, source limits, and whitespace checks pass. A formatting
error in the initial helper-generation script was corrected before validation.
No Cargo, rustc, rustdoc, compiler wrappers, Python, REPL scripts, model calls, or
Rust tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-repl/. Remaining legacy execution/tool code and other
workspace owners remain under the full active goal.

### RLM tool schemas and dispatch

Codetether-rlm now owns the eight RLM schemas, operation handlers, output limits,
and result contract. Schema builders and dispatch handlers are separate modules
per operation. The application tools module is a re-export. Sub-query sentinel
payloads still return to the host for asynchronous provider handling; this change
does not execute model calls or redefine slicing semantics.

Thirty-one source comparisons pass, including all schema literals, original
order, handler bodies, truncation, result contract, and six moved tests. All 380
RLM crate files are lexically reachable; 28 new modules have at most 34 code lines.
Source-limit and whitespace checks pass, and standalone rustfmt processed the
new code. Formatting caught use of the reserved word final as a module name;
the internal module was renamed final_answer before validation. No Cargo, rustc,
rustdoc, compiler wrappers, Python, tool execution, or Rust tests ran. Compilation
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-tools/. Legacy execution and other workspace owners
remain under the full active goal.

### External RLM REPL process ownership

Codetether-rlm owns the Bun/Node process wrapper in eight focused modules for
state, availability, startup, script generation, command I/O, final framing,
and lifecycle. The existing application ExternalRepl path is re-exported.
Bun selection with Node fallback, temporary script naming, generated JavaScript,
pipe handling, timeout, and explicit termination remain unchanged in source.
No dependencies were added because the crate already enables Tokio process I/O.

Ten source comparisons pass, including all seven original methods and the
complete retained application executor. All 388 crate files are lexically
reachable; new modules have at most 47 code lines. Standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers,
Python, Bun/Node child process, or Rust tests ran. Compilation and subprocess
behavior remain unverified. Evidence: artifacts/consolidation-rlm-external/.
The legacy executor and other workspace owners remain under the full goal.

### Legacy RLM trace and decoding helpers

Codetether-rlm legacy_support owns numbered-output parsing, file inference,
pattern-evidence inspection, latest grep match selection, canonical grep
payload construction, diagnostic truncation, code-block extraction, and
sub-query parsing/evaluation. The legacy executor retains private method
signatures and delegates with its existing trace or REPL state. Two existing
helper tests moved into the shared crate.

Twelve source comparisons pass, including the entire application remainder and
its delegate calls. A remaining Self qualifier in one moved helper was found
and corrected by comparison before final validation. All 399 RLM crate files
are lexically reachable; eleven new modules have at most 34 code lines.
Standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, model/REPL execution, or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-support/. Legacy loop decomposition and the broader
workspace goal remain unfinished and active.

### Legacy RLM prompt and request ownership

Six legacy_support/prompts modules own context summaries, system instructions,
sub-query context truncation, messages, and request defaults. DSL and structured
sub-query paths share the same request builder; their distinct trace behavior
remains in the executor. Text, thresholds, model selection, temperature, token
limit, and lack of tools/stop sequences remain unchanged in source.

Ten comparisons pass, including both former copies of the sub-query policy and
the full retained executor. All 405 crate files are lexically reachable, and
the new modules have at most 30 code lines. Standalone rustfmt, source limits,
and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
model calls, or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-rlm-prompts/. Legacy loop state
and host dependencies still need consolidation under the full active goal.

### Legacy final-payload normalization

Shared final_payload policy normalizes answers from grep evidence and records
the same trace step/final context event. The repeated recording block has one
helper with the original action labels; all three original log messages and
return cases remain. The application delegates using its existing trace state.
Three private forwarding methods with no remaining callers were removed.

Six source/reconstruction comparisons pass, including every correction branch
and the complete application remainder. All 407 crate files are lexically
reachable; two new modules have at most 40 code lines. Standalone rustfmt,
source limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, model calls, or Rust tests ran. Compilation and runtime
behavior remain unverified. Evidence: artifacts/consolidation-rlm-final/.
Legacy execution and other workspace owners remain under the full active goal.

### Legacy RLM sub-query completion and host modules

A shared subquery completion helper now owns provider invocation and text/token
collection for both DSL and structured paths. Application query_sequence,
query_dsl, and query_structured modules retain host orchestration and their
distinct trace recording. The top-level executor declares those modules and
retains state and the main analysis loop.

Eight source comparisons cover both completion implementations, host method
extractions, and the complete remaining executor. All 408 shared RLM files and
17 application RLM files are lexically reachable. The new shared helper has
23 code lines; all three host modules meet the 50-code-line rule. Standalone
rustfmt, source limits, and whitespace checks pass. No Cargo, rustc, rustdoc,
compiler wrappers, Python, provider calls, or Rust tests ran. Compilation and
runtime behavior remain unverified. Evidence: artifacts/consolidation-rlm-subquery/.
The full workspace goal remains active.

### Legacy structured-tool host handling

The application legacy executor now delegates structured final acceptance,
final rejection, sub-query sentinel handling, ordinary output, and unknown
tools to five focused host modules. These mutate the existing executor state.
The loop retains the same stop/continue decisions and appends tool results in
the same order. Public APIs and shared-crate dependencies are unchanged.

Eleven source comparisons pass, including original fragments, extracted bodies,
and the complete loop remainder with explicit control-flow adapters. All 22
application RLM files are lexically reachable; the five handlers have at most
41 code lines. Standalone rustfmt, source limits, and whitespace checks pass.
The extraction initially duplicated an if condition; formatting detected it.
Source comparison also detected an old scalar call argument after tuple
adaptation; both were corrected before final verification. No Cargo, rustc,
rustdoc, compiler wrappers, Python, executor invocation, or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-structured/. The main legacy loop and the full
workspace consolidation goal remain unfinished and active.

### Legacy DSL host decomposition

The legacy DSL branch now delegates assistant-message/code execution, verbose
result logging, final-answer policy, final rejection, and ordinary output
recording to five focused modules. An explicit DslControl value distinguishes
no final answer, rejected final answer, and accepted final answer. The outer
loop preserves its original continue/break decisions and message order.

Ten source comparisons pass, including extracted bodies, decision mapping,
and the complete retained executor. All 27 application RLM files are lexically
reachable; new modules have at most 38 code lines. Standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers,
Python, DSL execution, provider calls, or Rust tests ran. Compilation and
runtime behavior remain unverified. Evidence: artifacts/consolidation-rlm-dsl/.
Main-loop preparation, completion, and remaining workspace owners stay under
the full active goal.

### Legacy analysis lifecycle decomposition

Seven host modules now own initial messages, initial trace, context summary,
timed model request, optional rewriting, response trace, and final result
assembly. Main iteration still lives in the application, with setup/finish
delegation preserving original request defaults, tracing, and rewriter order.
The moved statistics reference was adjusted for the deeper module location.

Fourteen source comparisons pass, including saved sections, moved bodies, and
the complete remaining executor. All 34 application RLM files are lexically
reachable; new modules have at most 38 code lines. Standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers,
Python, provider requests, or Rust tests ran. Compilation and runtime behavior
remain unverified. Evidence: artifacts/consolidation-rlm-lifecycle/. The main
iteration and remaining workspace owners stay under the full active goal.

### Legacy executor shell and structured batches

Executor state, construction, settings/accessors, and compatibility methods now
have focused application modules. The public executor path re-exports its state
type. Structured batches preserve assistant/tool message order while delegating
individual calls to a bounded dispatcher. No application dependencies moved into
the RLM crate in this step.

Eighteen source comparisons pass, including original field types, all moved
methods, batch ordering, call-level stop decisions, and the remaining executor.
All 40 application RLM files are lexically reachable. Six new modules have at
most 46 code lines. Standalone rustfmt, source limits, and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrappers, Python, executor invocation, or Rust
tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-shell/. Main iteration decomposition and the full
workspace consolidation goal remain unfinished and active.

### Legacy analysis iteration decomposition

A private AnalysisRun owns per-run counters, messages, tools, timeout, and answer.
Preparation, one-step completion, structured handling, DSL handling, and the
public loop are separate modules. The loop keeps the same iteration condition,
error propagation, and early-stop behavior, then passes its state to the existing
result assembly. All remaining application RLM files now meet the 50-code-line
limit; this does not by itself complete migration of the legacy executor.

Seven reconstruction comparisons pass, including original preparation and
request ordering, structured/DSL control flow, and the public loop. All 46 host
RLM files are lexically reachable. The six new modules and root have at most
46 code lines. A leftover continue expression was corrected after inspection;
the verifier now checks that extracted step methods contain no loop jumps.
Standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, executor invocation, or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-iteration/. The full workspace goal remains active.

### Shared legacy RLM executor

The legacy executor now belongs to codetether-rlm in 32 existing implementation
modules plus its root and a narrow ResponseRewriter trait. Provider calls, loop
state, structured/DSL dispatch, tracing, and result assembly have one shared
owner. The application retains environment/local-router construction, a
compatibility wrapper, and the trait implementation for ToolCallRouter. Its
public constructor/builders/accessors/analyze signatures are preserved. The
root rlm::RlmExecutor bridge remains separate and retains its existing behavior.

Fifty-three source/API comparisons pass, including every moved implementation,
shared initialization, host setup order, wrapper delegation, and rewriter binding.
All 442 shared-crate files and 19 application RLM files are lexically reachable.
All files changed in this step meet the 50-code-line limit. Existing dependencies
were sufficient; no manifests or lock entries changed. Standalone rustfmt, source
limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers,
Python, model/router initialization, or Rust tests ran. Compilation, Send/Sync
bounds, feature combinations, and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-executor/. The full workspace goal remains active.

### Shared router host and compatibility executor

The standard ReplHost and router-backed executor now live in codetether-rlm.
Their tool definitions, dispatch, answer extraction, trace recording, and
analysis-result construction retain their source behavior. The shared router
entrypoint initializes the in-memory host for both the application adapter and
the shared executor. Application provider and SessionBus conversion remain at
the host boundary. All sixteen remaining application RLM files have an explicit
structural ownership disposition in host-boundaries.json.

Eleven source comparisons pass. All 448 shared-crate and 16 application files
are lexically reachable; six new shared modules have at most 48 code lines.
Standalone rustfmt, source limits, and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, provider requests, or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-rlm-router-executor/. The workspace-wide goal remains
active; this structural owner audit does not prove full-goal completion.

### Moltbook integration ownership

Codetether-moltbook now owns the client, wire models, credential loading and
persistence, registration/profile/post/feed operations, HTTP helpers, and
content generation. Separate client modules use the existing shared secrets
crate directly. The application module re-exports the public API. Endpoint
construction, auth headers, request/response handling, fallback rules, and
content strings retain their original source behavior.

Thirty-five comparisons pass, including all methods, types, constants, helpers,
and four moved tests. All 36 new files are lexically reachable and contain at
most 41 code lines. Manifest/lock checks confirm only the new local package and
root dependency edge. Standalone rustfmt, source limits, and whitespace checks
pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, Vault access, Moltbook
requests, or Rust tests ran. Compilation and live behavior remain unverified.
Evidence: artifacts/consolidation-moltbook/. The full workspace goal stays active.

### Crash reporter module boundaries

Consent prompting, startup, panic capture, spool storage, upload retry state,
paths, settings, and wire payloads now have focused application modules. The
flush loop and individual upload function remain in the root pending further
decomposition. No initialization ordering, consent policy, payload shape, or
network behavior was changed in the compared source.

Thirty-one source comparisons pass, including the retained root upload code
and the moved backoff test. All 29 crash files are lexically reachable; the
28 new modules have at most 49 code lines. Standalone rustfmt, source limits,
and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
panic hooks, report I/O, uploads, or Rust tests ran. Compilation and runtime
behavior remain unverified. Evidence: artifacts/consolidation-crash-modules/.
The crash extraction and full workspace goal remain active.

### Crash upload workflow decomposition

A private FlushAttempt now carries the original counters and transport error.
Report loading/discard, upload outcomes, final backoff updates, and legacy payload
retry have separate modules. The flush entrypoint retains report order, stop
thresholds, and the original client configuration. The root now contains wiring
and constants rather than upload orchestration.

Eight source comparisons pass, covering extracted bodies, state defaults,
loop-control mapping, and retained wiring. All 36 crash files are lexically
reachable; seven new modules have at most 39 code lines. Standalone rustfmt,
source limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, report I/O, uploads, or Rust tests ran. Compilation and runtime
behavior remain unverified. Evidence: artifacts/consolidation-crash-upload/.
The crash crate boundary and full workspace goal remain unfinished and active.

### Shared crash reporting service

Codetether-crash owns consent flow, settings, panic capture, report storage,
retry state, and upload processing. Module-declaration files group capture,
upload, and consent wiring while operation implementations remain bounded.
The application adapter supplies its own package version and memory-guard
installer; initialization still installs the panic hook, arms guards, then
checks consent before any pending upload. Shared Config and MemorySnapshot
types retain their existing identities.

Fifty-one source/wiring checks pass. All 40 crate files are lexically reachable
and at most 49 code lines. Manifest/lock comparisons verify only the new local
package and root dependency edge. Standalone rustfmt, source limits, and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
hook installation, consent prompt, report I/O, uploads, or Rust tests ran.
Compilation, callback future bounds, platform behavior, and runtime effects
remain unverified. Evidence: artifacts/consolidation-crash-crate/. The full
workspace goal remains active.

### Shared GitHub PR context and policy

Codetether-github-pr owns lifecycle transitions, CI status parsing, repository
and execution-signature context, signature merging, and provenance Markdown.
Modules separate types, Git discovery helpers, and rendering operations. The
application preserves public paths with re-exports and retains CLI handling
and A2A credential acquisition/HTTP orchestration.

Thirty-four source comparisons pass, including all extracted helpers/types,
existing test definitions, and the unchanged application client. All 25 crate
files are lexically reachable and at most 44 code lines. Manifest/lock checks
account for the new package/root edge and package ordering. Standalone rustfmt,
source limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, Git discovery, signing, credential requests, GitHub calls, or
Rust tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-github-pr/. HTTP/credential boundaries and the full
workspace goal remain unfinished and active.

### Shared GitHub PR HTTP operations

The shared PR crate now owns authenticated client construction, HTTP wire types,
branch PR lookup, creation, updates, and create-or-update selection. A borrowed
PullRequestSpec carries domain input without a CLI dependency. Application
modules separately own A2A credential acquisition, body-file/CLI input, and
CLI result metadata. Original context/body/credential/request ordering and
existing-update title/body semantics are retained in source.

Fourteen source/workflow comparisons pass. All 35 shared and 10 application PR
files are lexically reachable; thirteen new modules have at most 35 code lines.
The existing locked reqwest is added to the shared crate. Standalone rustfmt,
source limits, and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, Git discovery, credential requests, or GitHub operations ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-github-pr-http/. The full workspace goal remains active.

### Shared swarm result cache

Codetether-swarm-core now owns cache configuration, entries, task/content hashing,
lookup, insertion, entry-count eviction, JSON persistence, index loading and
statistics access. Focused modules separate these responsibilities. The
application retains default-directory selection and thin method forwarding,
preserving existing swarm::cache paths and constructor signatures. The shared
constructor accepts a lazy host directory fallback, avoiding a dependency on
application configuration.

Forty-nine static comparisons/checks cover moved methods/types, all five existing
cache tests and their helpers, application signatures/delegation, the unchanged
directory policy and the intended dependency changes. All 76 shared crate files
are lexically reachable; the new cache and adapter modules have at most 34 code
lines. Source limits, standalone rustfmt and whitespace checks pass. Existing
hashing, eviction, expiry and asynchronous cleanup behavior is preserved in
source; these checks do not establish runtime equivalence.

The existing locked anyhow, tracing and tempfile dependencies are declared,
Tokio filesystem support is enabled, and test-only Tokio macro/runtime features
are explicit. No Cargo, rustc, rustdoc, compiler wrappers, Python, cache runtime,
or Rust tests ran. Compilation and test outcomes remain unverified. Evidence:
artifacts/consolidation-swarm-cache/. Swarm orchestration and the full workspace
consolidation remain unfinished; the goal stays active.

The swarm module audit also identified an unreachable duplicate,
`src/swarm/executor_tokens.rs`. Its three estimator bodies match the reachable
`token_limits/estimation.rs` after substituting the existing 3.5 constant and
visibility. The unused file was removed; active estimators and constants were
unchanged. All 287 remaining application swarm files are lexically reachable,
including the explicitly validated `executor_module!` declaration template.
Evidence: duplicate.mjs, duplicate-check.json and app-modules.json in the same
artifact directory.

### Shared swarm configuration, protocol and outcomes

The shared swarm crate owns execution limits, SwarmConfig and its defaults,
ExecutionMode parsing, DecompositionStrategy, Actor/Handler lifecycle contracts,
ActorStatus, SwarmMessage, SwarmResult and SwarmArtifact. Configuration,
protocol and outcome modules separate those responsibilities; the application
root and actor_status preserve the previous public import paths. The old
application defaults file was removed.

SwarmMessage now names the existing shared ToolResult directly; the application
tool module already re-exports that same type. The shared crate declares
codetether-tool-core and async-trait dependencies. Its five transitive local
crate dependencies form no cycle and have no edge back to the application.

Twenty-six static source/wiring/dependency checks preserve the original fields,
derives, serde attributes, constants, CLI aliases, defaults, status formatting,
and actor method signatures. Sixteen new or replaced modules have at most 30
code lines. All 91 shared and 286 application swarm files are lexically
reachable; source limits, standalone rustfmt and whitespace checks pass. No
Cargo, rustc, rustdoc, compiler wrappers, Python, actors, or Rust tests ran.
Compilation, serialization and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-protocol/. Planning/execution ownership and the
full workspace consolidation remain pending; the goal stays active.

### Shared swarm decomposition planning

Codetether-swarm-core::planning owns candidate JSON extraction, model-response
wire types, two-pass task creation and dependency resolution, plan validation,
stage assignment, initial decomposition prompts and repair prompts. The
application parser delegates to this service; plan/prompt compatibility modules
preserve the existing orchestration call sites. Provider requests and mutable
orchestrator state remain application-owned and require further consolidation.

Thirty-five static source/wiring checks compare the extracted parser components,
all eight moved source/test modules, unchanged orchestrator methods and outer
declarations, and compatibility delegates. Nine existing test bodies move
unchanged except for shared-type import paths. Sixteen new or replaced modules
have at most 41 code lines. All 104 shared and 281 application swarm files are
lexically reachable, including cfg/test branches. Standalone rustfmt, source
limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers,
Python, provider requests, planner execution or Rust tests ran. Compilation and
runtime behavior remain unverified. Evidence: artifacts/consolidation-swarm-planning/.
The full workspace goal remains active.

### Shared swarm coordinator state

CoordinatorState owns the task and agent registries, successful completion IDs,
readiness/stage queries and statistics. Query, lifecycle and access modules keep
those responsibilities separate. The application Orchestrator delegates its
existing public methods and retains configuration, provider registry/model
selection and provider-backed decomposition. SubAgentMessage and
OrchestratorMessage are shared protocol contracts with compatibility exports
at the original application path.

Thirty-seven static source/wiring checks cover all moved methods, application
delegation, constructor/decomposition transitions, message fields, retained
provider policy and initial state. Inserting a single task retains prior tasks;
replacing a plan replaces only the task map. Unknown completion IDs are ignored,
and repeated known completions retain their existing counter/history behavior.
Seven new or replaced modules have at most 29 code lines. All 110 shared and
281 application swarm modules are lexically reachable. Standalone rustfmt,
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, provider calls or Rust tests ran. Compilation and runtime
semantics remain unverified. Evidence: artifacts/consolidation-swarm-state/.
The broader workspace goal remains active.

### Shared swarm decomposition flow

The shared planner now owns response validation, the single repair attempt,
required-parallel failure handling and optional single-task fallback. A
PlanCompletion trait supplies only asynchronous prompt-to-text completion. The
application binds its existing provider, model and temperature, retaining the
original request construction and credential/provider setup. The application
Orchestrator is divided into bounded setup, selection, access, lifecycle,
query and decomposition adapter modules; obsolete private plan/prompt facades
were removed. Its public paths and methods remain available.

Twenty-seven static source/ordering comparisons check unchanged request bytes,
provider policy, model selection, constructor order, state delegates, adapter
wiring and the reconstructed validation/repair/fallback flow. All 20 changed
modules are at most 47 code lines. All 120 shared and 286 application swarm files
are lexically reachable. Five new policy tests cover initial success, repair
success, optional fallback, required-parallel rejection and initial provider
failure without repair; those tests were not executed.

Standalone rustfmt, source limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, provider requests or Rust tests ran.
Compilation, async trait bounds and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-decomposition/. The full workspace goal remains
active; task execution and other remaining owners still require consolidation.

### Shared swarm result store

The shared swarm crate owns ResultStore, result/schema/notification contracts,
subscription patterns, typed queries, publication, expiry cleanup and task-scoped
handles. Focused modules separate data contracts, schema inference, publication,
queries, subscription bookkeeping, cleanup and handle behavior. The application
result_store module re-exports every previous public type and trait.

Thirty-nine static source comparisons check all types, implementations and six
existing test bodies. Twenty new or replaced modules have at most 39 code lines.
All 139 shared and 286 application swarm files are lexically reachable.
Lock acquisition, store-before-broadcast ordering, cloning, query predicates and
serialization expressions match the previous source. Expiry remains explicit
cleanup; notification listeners retain their own filtering responsibility.

Standalone rustfmt, source limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, result-store operations or Rust tests ran.
Compilation, asynchronous behavior and runtime outcomes remain unverified.
Evidence: artifacts/consolidation-swarm-result-store/. The broader consolidation
goal remains active.

### Shared speculative branch-collapse policy

The shared collapse module owns observation tracking, health counters, conflict
risk, score calculation, threshold and low-coherence pruning, survivor rescue,
promotion and policy/result contracts. Small modules separate these operations.
The application retains local Git probes and sampling threads and delegates
observations to the shared controller. Existing public types and methods remain
available; Debug output delegates to the shared controller representation.

Thirty-one static comparisons preserve the original state fields, defaults,
score calculations, observation/evaluation ordering, threshold decisions,
survivor rule, pruning/promotion logic and three existing tests. The local sample
method and Git helper bodies match the original source; the standalone Git
health probe is byte-identical. All 25 new or replaced modules have at most 49
code lines. All 161 shared and 288 application swarm files are lexically
reachable. Standalone rustfmt, file limits and whitespace checks pass.

No Cargo, rustc, rustdoc, compiler wrappers, Python, branch probes, collapse
policy functions or Rust tests ran. Compilation, timing and runtime decisions
remain unverified. Evidence: artifacts/consolidation-swarm-collapse/. Task
execution and the wider workspace consolidation remain pending; the goal stays
active.

### Shared swarm pre-flight validation policy

ValidationPolicy owns configuration checks, dependency graph/cycle validation,
token estimates and warnings, report contracts and report formatting. The
application SwarmValidator retains provider/credential discovery and workspace
probes, then passes their statuses and ordered issues to shared validation.
Existing public report/type paths remain re-exported. Six existing tests move
with the pure checks, with only imports and validator construction adjusted.

Thirty-one static comparisons check the 18 moved implementation modules, two
test modules, application probe/delegation order, shared report assembly and
six byte-identical host modules. All 24 new or replaced modules have at most
41 code lines. All 183 shared and 268 application swarm files are lexically
reachable. Standalone rustfmt, file limits and whitespace checks pass.

No Cargo, rustc, rustdoc, compiler wrappers, Python, provider/workspace probes,
validation functions or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-swarm-validation/. Task execution
and the wider workspace consolidation remain unfinished; the goal stays active.

### Shared swarm context budgets and exhaustion recovery

The shared context module owns byte-based token estimation, tool-output
truncation, middle-history removal and summarization, exhaustion evidence and
recovery advice. The application token_truncate and token_exhaustion modules
re-export the original APIs. A direct dependency on the existing provider-core
crate supplies the same message types already exposed by the application.

Fifteen static comparisons/checks cover all eight moved modules, both
compatibility facades, unchanged integration regression test bytes and the
intended manifest/lock dependency edge. All 11 new or replaced modules have
at most 43 code lines. Existing truncation limits, UTF-8 boundary handling,
summary ordering and arithmetic behavior remain unchanged. All 192 shared
and 260 application swarm files are lexically reachable. Standalone rustfmt,
file limits and whitespace checks pass.

No Cargo, rustc, rustdoc, compiler wrappers, Python, context routines or Rust
tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-context/. The full workspace goal remains active.

### Shared run policy and direct event ownership

The shared run_policy module owns failed-dependency partitioning, blocked-result
construction, aggregate output rendering, the no-work result and final success/
statistics calculation. Application modules retain the old call sites as thin
delegates and provide timing/coordinator state to statistics calculation. Two
existing tests move with dependency gating and output aggregation.

Twenty-six application swarm event consumers now import the existing types
directly from swarm-core or agent-core. The TUI re-exports already refer to
those same types. No TUI module imports remain under src/swarm; rendering and
event payloads are unchanged. Forty static source/import-identity checks cover
the migration, including host timing input order and compatibility delegates.
All 199 shared and 258 application swarm files are lexically reachable.
Standalone rustfmt, file limits and whitespace checks pass.

No Cargo, rustc, rustdoc, compiler wrappers, Python, event delivery or Rust tests
ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-run-policy/. Shared policy is further consolidated,
but backend execution and the broader workspace goal remain unfinished.

### Shared dependency-stage runtime coordination

The shared stage_runtime module owns ascending stage traversal, readiness checks,
empty-stage handling, failed-dependency gating, blocked-result ordering and
backend dispatch sequencing. StageRuntime and borrowed StageInput describe the
host boundary. The application Run implementation supplies cancellation checks,
coordinator task selection, blocked-task events, backend calls and ordered result
publication/statistics/retention. The old application stage implementation was
removed; its traversal entrypoint delegates to the shared coordinator.

Twenty static source/ordering checks compare original traversal, stage metadata
and task selection, backend calls, blocked-event forwarding, post-execution
recording and unchanged state/control/storage/statistics modules. Ten new or
changed modules contain at most 34 code lines. All 204 shared and 260
application swarm files are lexically reachable. Standalone rustfmt, file limits
and whitespace checks pass.

No Cargo, rustc, rustdoc, compiler wrappers, Python, runtime methods, backend
execution or Rust tests ran. Compilation, async trait bounds and runtime behavior
remain unverified. Evidence: artifacts/consolidation-swarm-stage-runtime/. Local
and Kubernetes backend ownership and the full workspace goal remain unfinished.

### Shared local sub-agent retry policy

The shared agent_retry module owns retry termination, attempt counts, capped
exponential backoff, retry diagnostics and AgentLoopExit. AgentAttempt supplies
one host invocation. The local adapter retains its Job, prompts, registry and
tool definitions and invokes the existing agent-loop helper unchanged. Old
private backoff/logging implementations are removed; the public exit path
remains a compatibility export.

Fourteen static source/wiring comparisons preserve retry conditions, sleep and
counter order, diagnostics, backoff formulas, request arguments and module
wiring. Sixteen new or changed modules have at most 46 code lines. The original
backoff test moves unchanged; three new tests specify immediate completion,
completion after two retries and exhaustion including the initial attempt.
No tests were run. All 216 shared and 257 application swarm files are
lexically reachable. Standalone rustfmt, source limits and whitespace checks pass.

No Cargo, rustc, rustdoc, compiler wrappers, Python, retries, provider calls or
Rust tests ran. Compilation, async trait bounds and runtime behavior remain
unverified. Evidence: artifacts/consolidation-swarm-agent-retry/. The full
workspace consolidation goal remains active.

### Shared deliverable policy and local attempt outcomes

The shared deliverable module owns terminal-status interpretation, verification
contract inference, contract prompt text and calibration of unsupported
preservation claims. Nine existing policy tests move with these implementations.
The shared agent_result module constructs local SubTaskResult/status pairs from
AgentOutcome and borrowed execution metadata. The local application adapter
provides task ID, instruction, step/timeout limits and the original elapsed value.
Tool-policy call sites retain crate-local compatibility names; remote result
assembly remains unchanged while using the same shared deliverable policy.

Eighteen static comparisons preserve all policy text, output/status rules,
result fields and local adapter wiring. Other tool-policy sources are byte-
identical. Seventeen new or changed modules have at most 44 code lines. All
229 shared and 249 application swarm files are lexically reachable.
Standalone rustfmt, file limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, policy/result functions or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-agent-outcomes/. The full goal remains active.

### Shared remote swarm protocol

RemoteSubtaskPayload, RemoteBranchProbe, environment/log prefixes, Base64/JSON
encoding and decoding, log extraction and finished-pod result interpretation
now belong to remote_protocol in the shared swarm crate. Application Kubernetes
and result modules re-export their previous names. The existing locked Base64
0.23.1 dependency is declared directly; no versions change. Pod management,
working-directory changes and remote agent execution remain application-owned.

Nineteen static comparisons preserve fields/serde defaults, constants, codec
logic, log selection and fallback errors, both existing test bodies and the
manifest/lock edge. Twelve new or replaced modules have at most 40 code lines.
All 239 shared and 249 application swarm files are lexically reachable.
Standalone rustfmt, file limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, remote processes, pod operations or Rust
tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-remote-protocol/. The full goal remains active.

### Shared remote outcomes and bounded remote process adapter

Remote outcome construction now lives beside the remote wire protocol. A
deferred elapsed-time callback preserves the original clock-read position in
both success and error result records. The application remote entrypoint uses
separate setup, resource, prompt and invocation modules, then stops the probe
before shared result construction and structured output emission. Provider
registry ownership remains in the entrypoint through final output.

Thirteen static source/ordering checks preserve environment/payload loading,
working-directory setup, provider/tool setup, prompt text, execution arguments,
result fields and final emission order. Both probe modules remain byte-identical.
Eight new or changed files contain at most 41 code lines. All 241 shared and
253 application swarm files are lexically reachable. Standalone rustfmt,
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, remote processes, pod operations or Rust tests ran. Compilation
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-remote-runner/. The wider goal remains active.

### Shared sub-agent turn driver

The shared agent_loop module owns provider-turn sequencing, parsed turn/tool
contracts, continuation policy, limit precedence and history-budget advancement.
AgentRuntime keeps steering, readiness, provider requests, textual tool-call
normalization, events and tool invocation in the application. State/request/
continuation compatibility paths retain the same data identities.

Twenty-eight static source and adapter comparisons preserve original driver
order, all exit branches, tool-call precedence, end-of-turn input handling,
response parsing, request construction and state fields. Fifteen new or changed
modules have at most 44 code lines. Existing application regression tests remain
in place. All 249 shared and 253 application swarm files are lexically
reachable; every shared crate file is within 50 code lines. Standalone rustfmt,
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, agent loops, provider/tool calls or Rust tests ran. Compilation,
async trait bounds and runtime behavior remain unverified. Evidence:
artifacts/consolidation-swarm-agent-loop/. The full goal remains active.

### Shared task preparation and lifecycle events

The context module now supplies one dependency-result formatter for both local
and Kubernetes stages. Cache policy owns read-only task selection and cached
result retargeting. Run policy owns expected-mutation maps, synthetic failures,
decomposed-task events and blocked-task event construction; host callbacks still
deliver events. Application call sites retain thin compatibility names.

Eighteen static comparisons verify source bodies, local/remote formatter
equivalence, public delegates and existing tests. Three tests move with cache
selection, mutation expectations and blocked events. Seventeen new or changed
modules have at most 32 code lines. All 256 shared and 252 application swarm
files are lexically reachable. Standalone rustfmt, file limits and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, task helpers,
event delivery or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-swarm-task-adapters/. The broader
goal remains active.

### Shared delegated prompt policy

The prompt_policy module owns capability/tool text, sourced constraint entries,
constraint-mode projection, quality/source-metric contracts, repository quality
loading and system-prompt composition. It reuses the existing shared agent-core
AGENTS.md loader and worktree-guidance constant. The application injects session
turn instructions through a callback applied at the original composition point.
Existing prompt integration tests remain at the application entrypoint.

Twenty-one static source/identity checks preserve strings, composition order,
repository lookup and other tool-policy files. Fourteen pure tests move with the
policy; one test uses an identity callback for the shared composer. Eighteen new
or changed modules have at most 43 code lines. All 271 shared and 239
application swarm files are lexically reachable. Standalone rustfmt, file limits
and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
prompt execution or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-swarm-prompt-policy/. The full
workspace goal remains active.

### Shared tool workspace path guard

The complete workspace-root argument guard moved from swarm into tool-core,
which owns tool contracts and argument policies. The shared guard handles
authoritative roots, declared path fields, batch recursion, lexical containment
and existing-path symlink checks. The application retains one compatibility
export at its original executor path. Nine existing tests move unchanged apart
from their module locations.

Eighteen static source/ownership checks preserve every guard/test body and the
intended test-only tempfile dependency. Fourteen new or changed modules have at
most 42 code lines. All 43 tool-core and 228 application swarm files are
lexically reachable. Standalone rustfmt, file limits and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrappers, Python, path checks or filesystem
tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-tool-path-guard/. The full consolidation goal stays active.

### Shared speculative branch planning and next-owner audit

SpeculativeRunner, BranchSpec and the ordered fallback strategies now live
under the shared collapse module. The application import path re-exports the
existing API. Branch-count clamping, naming and strategy selection are unchanged.
The CLI's dispatch TODO remains; this extraction does not claim speculative
execution is wired up. Seven static source/size checks pass. Seven new or changed
files have at most 46 code lines. All 276 shared and 228 application swarm
files are lexically reachable. Standalone rustfmt, file limits and whitespace
checks pass; no Cargo, rustc, compiler wrappers, Python, branch planning or Rust
tests ran. Compilation and runtime behavior remain unverified.

The next-owner audit found that bus envelopes and the registry depend on data-only
A2A contracts in src/a2a/types.rs (450 code lines, 35 type definitions).
That file imports serde and HashMap and has no crate/super references. Its
extraction is the prerequisite to a clean shared bus boundary. Evidence:
artifacts/consolidation-swarm-speculative/, including next-owner-audit.json.
The full workspace goal remains active.

### Shared A2A wire contracts

Codetether-a2a-core owns the 35 task, message, agent-card, RPC, security,
streaming and push-notification contracts previously implemented in
src/a2a/types.rs. The existing application path is a compatibility re-export.
Types are grouped into seven cohesive modules, with individual definitions and
focused RPC-constructor files. The crate depends only on serde and serde_json,
so bus and transport consumers can use it without importing application code.

Sixty-one static comparisons/checks preserve every field/derive/serde attribute,
all ten error-code constants, state predicates, RPC constructors and the protocol
version default. Manifest/lock checks account for the new package/member and
application edge only. All 47 shared files are lexically reachable and
have at most 38 code lines. An application implementation search found no remaining
implementations targeting moved types. Standalone rustfmt, file limits and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
serialization routines or Rust tests ran. Compilation, serde behavior and
runtime compatibility remain unverified. Evidence:
artifacts/consolidation-a2a-contracts/. The full goal remains active; bus
ownership and A2A transports remain pending.

### Shared bus contracts and message policy

Codetether-bus-core owns BusEnvelope, BusMessage, durability/partition policy,
SpeechAct, tool-schema conversions and bounded live payloads. Application paths
re-export the same types; AgentBus/BusHandle operations, registry, recorder,
durable storage and external sinks remain in the application.

The 21-variant BusMessage enum retains its exact field shape and ordering. Seven
declaration-only macro fragments group related variants and feed one enum
declaration, keeping files within the line limit without nesting or changing
variant payloads. The source checker compares each fragment and the final
accumulation chain to the original enum; this is not compiler macro expansion.

Twenty-six static source/dependency checks pass. Six speech/delivery tests move
with unchanged bodies apart from shared-type imports. All 20 shared files
are lexically reachable; new files have at most 33 code lines. Existing application
bus methods/tests and catalog publication are compared against the original.
Utility alias resolution was checked against the existing truncate module.
Standalone rustfmt, file limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, bus operations or Rust tests ran. Compilation,
macro expansion and runtime behavior remain unverified. Evidence:
artifacts/consolidation-bus-contracts/. The full goal remains active.

### Shared bus registry and recorder

The shared bus crate owns AgentRegistry, ready/shutdown registration effects,
BusRecorder and cursor-based history draining. Registry access, ephemeral cards,
ready-card/skill construction and skill-ID normalization are separate modules.
Recorder construction, append/eviction, recent queries and cursor draining are
separate modules. Application paths remain compatibility exports.

Thirty-five static source/composition checks preserve all registry fields and
methods, card defaults, skill projection, five registry test bodies, recorder
locking/eviction and cursor logic, and publication effects. All 40 shared
files are lexically reachable. Twenty-four changed files have at most 42 code
lines. Existing locked DashMap, UUID and tracing dependencies are declared;
no versions change. Standalone rustfmt, file limits and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrappers, Python, registry/recorder operations
or Rust tests ran. Compilation and concurrency behavior remain unverified.
Evidence: artifacts/consolidation-bus-registry-recorder/. Broadcast runtime,
durable storage and the full workspace goal remain unfinished and active.

### Shared durable bus log

DurableLog, partition selection, fire-and-forget append policy and the
FileDurableLog JSONL backend now belong to bus-core. File state/construction,
append/replay operations, partition sanitization and file-reading helpers are
separate modules. The application retains compatibility paths and the existing
AgentBus attachment/replay adapter until live bus extraction.

Fourteen static comparisons preserve background append, lock lifetime, flush
before line counting, offset/filter rules, path sanitization and both existing
path tests. Eight new or changed modules have at most 35 code lines. All
45 shared files are lexically reachable. Existing anyhow, async-trait and
Tokio dependencies are declared without version changes. Standalone rustfmt,
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, log I/O, background appends or Rust tests ran. Compilation
and runtime durability remain unverified. Evidence:
artifacts/consolidation-bus-durable-log/. The full goal remains active.

### Shared live bus runtime

AgentBus and BusHandle now belong to bus-core, together with every inherent
implementation: construction, publication, receiver filtering, lifecycle/task
messages, Ralph learning/progress, speech, voice and durable replay. Application
paths re-export the same types; global registration, relay and external sinks
remain application-owned pending further audit. Runtime state, operation groups
and existing tests are separate modules. Publication retains registry updates,
background durability scheduling, recording and broadcast in that order.

Forty-nine static checks compare all 35 inherent methods, both state layouts,
Debug/Default implementations, envelope construction and five existing test
bodies. All 78 bus-core Rust files are lexically connected, including test
modules and declaration includes. Changed runtime/facade files have at most
36 code lines. Existing Tokio test features are declared without lockfile
changes. Standalone rustfmt, the source line-budget gate and whitespace checks
pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, bus operations or Rust
tests ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-bus-runtime/. The full workspace goal remains active.

### Shared protocol relay and sink subscription boundary

RelayAgentProfile and ProtocolRelayRuntime moved to bus-core. Construction,
participant lifecycle and correlated handoff emission have separate modules.
The application keeps a compatibility export. All six relay methods and both
existing tests are compared to the original source; test execution is unverified.
Tokio's time feature is added only to the existing dev dependency for those tests.

The sink audit found a private-field access missed by the preceding live-runtime
extraction: BusS3Sink subscribed through AgentBus.tx. AgentBus now exposes
subscribe(), which returns the same broadcast receiver, and the sink uses that
public method. This correction is covered by a static delegation check.
The application global singleton remains local because it selects the active bus
for application entrypoints and child-session factories. Training-record
conversion and the S3 transport/configuration split remain pending.

Sixteen static checks pass. All 84 shared bus files are lexically connected;
nine changed shared/facade files have at most 25 code lines. The existing sink
only changes its subscription expression. Standalone rustfmt, source file limits
and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
relay/bus execution, credential lookup or network calls ran. Compilation and
runtime behavior remain unverified. Evidence: artifacts/consolidation-bus-relay/.
The full workspace consolidation goal remains active.

### Shared bus training projection

Training-record schemas, message projection, provenance, ordering, duplicate
filtering, complete-tool-group validation, parallel-call merging and JSON
serialization now belong to bus-core::training. Its public serialize_envelopes
function accepts a bus batch and returns JSON strings; internal record types
remain private. The existing public tool-catalog formatter remains accessible
through the application compatibility path. S3 credential resolution, archive
key construction, local spool, upload and startup remain application adapters.

The 21 message variants have separate rendering modules. A declarative routing
macro binds borrowed message fields and passes them to the matching renderer;
its source table is checked against the original match arms and current message
contracts. This checks source composition, not compiler macro expansion.
Metadata construction and all original renderer expressions are compared
separately, preserving field order, serde attributes and text formatting.

One hundred thirty-three static checks pass, covering the four record schemas,
21 projections, parameter types/routing, grouping/filtering/order, all retained
sink methods and ten moved regression test bodies. The retained sink now calls
the shared pipeline and otherwise preserves its method bodies. All 134 shared
bus files and 23 application bus files are lexically connected; new modules
have at most 50 code lines. The existing sink shrank to 418 code lines.
Standalone rustfmt, source file limits and whitespace checks pass. No Cargo,
rustc, rustdoc, compiler wrappers, Python, tests, credentials, uploads or bus
operations ran. Compilation, macro expansion and runtime behavior remain
unverified. Evidence: artifacts/consolidation-bus-training/. The broad
workspace consolidation goal remains active.

### Separate bus S3 adapter and bus ownership audit

The S3 implementation now belongs to codetether-bus-s3, keeping MinIO and
executor dependencies out of bus-core. Modules separate connection settings,
environment lookup, chat-sync fallback, Vault lookup, client construction,
bucket lifecycle, receive/timer handling, serialization/flush, upload, spool
files, archive keys and startup/executor initialization. Both configuration
and sink types move with every associated implementation; existing application
paths re-export them and the catalog formatter.

Thirty-eight static checks preserve credential source order and defaults,
endpoint normalization, bucket errors, batching/flush behavior, executor
initialization and the spool-before-upload/delete-only-after-success sequence.
Two existing config/endpoint tests and three executor tests move unchanged
apart from module visibility for a fixture helper. Tests did not run.
The new crate uses existing locked dependency versions; no existing lock
package changed except the root's added dependency edge.

All 30 adapter files and 17 remaining application bus files are lexically
connected, with no missing modules. Changed Rust files have at most 41 code
lines. The local path-dependency graph reaches six manifests without a cycle
or missing path. Standalone rustfmt, file limits and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrapper, Python, executor initialization,
credential lookup, file-spool operations, network calls or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-bus-s3/.

The remaining src/bus implementation is the application global bus selector;
its process-entrypoint/child-session role justifies retaining it locally. All
other remaining files are compatibility exports or module declarations. This
completes the source-ownership audit for bus, not the workspace-wide goal;
other inventory rows remain pending.

### Shared auto-chat requests, context and handoff transport

Auto-chat constants, capability requirements, request parsing, convergence
normalization, shared context contracts, merge/render rules, context bus
publication/draining and correlated handoff reception now belong to
codetether-autochat-core. Modules separate each policy and data contract.
Application compatibility exports preserve existing paths. The RLM execution
function remains a local adapter with the same body; model registry selection
and session delegation remain for subsequent ownership work.

Fifty-seven static checks compare constants, contracts, all moved functions
and methods, unchanged model/delegation adapters, and nine existing test
bodies. The core uses the already-extracted RLM FinalPayload contract; provider
execution stays outside. Existing dependency versions remain unchanged, and
the local path graph reaches six manifests without a cycle or missing path.
All 42 shared files and six application files are lexically connected. New and
changed Rust files have at most 33 code lines. Standalone rustfmt, source file
limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers,
Python, model calls, RLM executions, bus activity or Rust tests ran. Compilation,
doctests and runtime behavior remain unverified. Evidence:
artifacts/consolidation-autochat-context/. The workspace goal remains active.

### Shared auto-chat model rotation and selection

RelayModelRotation, provider priorities, fallback resolution, alias normalization,
model scoring and ordered preferences now belong to autochat-core. Separate
modules own rotation state, provider lookup, OpenRouter scoring, Copilot
selection and shared Codex defaults. A two-method ModelRegistry interface
lets the application retain its initialized ProviderRegistry and instrumented
provider handles without duplicating registration or changing listing order.
Existing application entrypoints remain wrappers with the original signatures.

Thirty-three static checks preserve all selection functions, constants,
rotation state, both wrapper calls, the registry forwarding methods and five
existing tests plus their fixture. The Codex catalog path is checked through
the existing re-export to the same provider-core catalog. Provider-core is
now a direct dependency; only that lock edge changes, with no version changes.
All 66 shared files and five remaining application files are lexically
connected. Changed files have at most 37 code lines. Standalone rustfmt, source
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, provider calls, model enumeration or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-autochat-models/. Delegation and local RLM execution
remain under review; the full workspace goal remains active.

### Shared relevance and delegation calibration

The new codetether-context-policy crate owns relevance metadata, bucket axes,
message feature extraction, recent-window projection, Beta posteriors,
delegation configuration, environment overrides, updates and candidate ranking.
Types and their implementations move together. Session paths re-export the
shared modules; auto-chat's stable LCB ordering and bucket construction move
to autochat-core using these same shared types. The local RLM executor remains
an application adapter because its constructor installs host tool-routing
configuration. Broader session ownership remains under audit.

Eighty-five static comparisons preserve every moved item and method, bucket
encodings, numerical formulas, field/serde layout, environment overrides and
22 existing test bodies. Tests were not executed. All 84 context-policy files
and 73 autochat-core files are lexically connected. Changed Rust files have
at most 38 code lines. Local dependency paths contain no cycles or missing
manifests; existing lock package versions are unchanged. Standalone rustfmt,
source file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, environment lookups, scoring execution or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-policy/. The full workspace goal remains active.

### Shared context derivation configuration

Persisted DerivePolicy variants/defaults, RequestOptions, implicit migration
selection, input budgets, completion output-limit configuration, incremental
budget overrides and reset-threshold precedence now belong to context-policy.
Stable delegation skill identifiers also move to that owner. Related budget
helpers are grouped within each policy module; individual regressions remain
separate files. Application paths are compatibility exports, while session
mutation, summary generation and provider execution remain local.

Fifty-three static source/contract checks preserve serde tags and fields,
policy defaults, stable skills, all budget formulas, override parsing/order and
15 existing test bodies. The model-window resolver is the same provider-core
implementation already used through the application facade. Shared tests add
only the existing serde_json dev dependency; no dependency version changes.
All 113 context-policy files are lexically connected. Changed Rust files have
at most 32 code lines. Standalone rustfmt, source file limits and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, environment
lookups or Rust tests ran. Compilation, doctests and runtime behavior remain
unverified. Evidence: artifacts/consolidation-context-derivation/. The full
workspace consolidation goal remains active.

### Shared hierarchical summary index

SummaryIndex, SummaryRange, SummaryNode and Granularity now belong to
context-policy::summary_index. Existing access, mutation, async cache-miss
production, range contracts and custom tree serialization retain their
module boundaries and implementations. The application index path re-exports
the shared API and retains workspace recall integration. Summary producers
and the live Session remain application-owned.

Fifteen static comparisons preserve every original index implementation and
ten existing test bodies, including LRU invalidation and the serialized
entry/legacy-map reader. All 122 context-policy files are lexically connected.
New and changed Rust files have at most 50 code lines. Existing anyhow and
Tokio dependencies are declared without version changes. Standalone rustfmt,
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, producer calls, cache operations or Rust tests ran. Compilation
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-summary-index/. The workspace goal remains active.

### Shared page contracts and origin-aware projection

PageKind, ResidencyLevel, classification and permitted fidelity paths now
belong to context-policy::pages. SummaryGap, MessageOrigin, DerivedContext,
summary interleaving, uncovered-range calculation and origin-aware tool
pairing repair now belong to context-policy::projection. Application paths
re-export these shared owners. Live session derivation still coordinates
selection and providers, while these transformations operate on shared data.

Thirty-seven static checks compare both enums and serde annotations, fidelity
rules, projection contracts, insertion/coverage/repair algorithms and thirteen
existing test bodies. Repair retains injection-before-dropping order and
message/origin updates in lockstep. Import ordering is canonicalized for
rustfmt; algorithm bodies remain token-compared. All 146 context-policy files
are lexically connected, and changed files have at most 47 code lines.
Standalone rustfmt, source file limits and whitespace checks pass. No Cargo,
rustc, rustdoc, compiler wrappers, Python, projection operations or Rust tests
ran. Compilation, doctests and runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-projection/. The full workspace goal remains
active, including the remaining context selection orchestration.

### Shared active-task selection and prepared-summary packing

Active-task anchoring, continuation recognition, task signatures, relevance
scores, dropped-range collection and prepared-summary selection now belong
to context-policy::selection. The packing algorithm receives the existing
application token-estimation function as an argument. The application retains
protected-entry selection and proactive-index acquisition, then calls the
shared rules. No model runtime is added to the shared policy crate.

Thirty-seven static checks preserve score weights, continuation strings,
selection algorithms, the unchanged coordinator body, exact estimator
forwarding and eleven moved test bodies. Eight packing integration tests and
their fixtures remain byte-for-byte unchanged in the application. All 162
shared files and 50 application context files are lexically connected. New
files have at most 46 code lines; the existing coordinator shrank to 107.

The module audit found incremental_observability.rs had neither a module
declaration nor callers. Its source and the initial audit finding are archived
in the evidence directory; the unused file is removed. Standalone rustfmt,
source file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, estimator calls, summary production or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-selection/. The workspace goal remains active.

### Shared context budget paths and plain pairing repair

The post-repair overflow clamp and below-budget pass-through now belong to
context-policy::projection. Application wrappers supply the same request-token
estimator, preserving system/tool overhead. Plain tool-pair repair and its
ExperimentalStats contract move to context-policy::transforms; the existing
application transform pipeline still composes its other strategies.

Twenty-three static checks preserve clamp bounds/provenance, below-budget
repair ordering, all pairing loops and counters, aggregate stats arithmetic,
exact estimator forwarding and four moved pairing tests. Clamp regression
and fixture source remain byte-identical in the application. All 176 shared
files and 50 application context files are lexically connected. Changed Rust
files have at most 36 code lines. Standalone rustfmt, file limits and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, token estimation,
transforms or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-context-budget-paths/. The
workspace consolidation goal remains active.

### Shared message transforms

Tool-output deduplication, assistant whitespace cleanup, stale-output
snipping, thinking-block pruning, duplicate-call argument handling and
sink/recent middle trimming now belong to context-policy::transforms.
Separate modules retain each policy's thresholds and helpers. Existing
application paths are facades; the application apply_all pipeline, order
and TetherScript repair feature gates remain byte-identical.

Sixty-seven static checks preserve every function/fixture body, constant,
marker and hash operation, plus 21 existing test bodies. The existing SHA-256
dependency is declared at its locked version; no package versions change.
All 231 shared files and 16 application experimental-transform files are
lexically connected. Changed Rust files have at most 48 code lines.
Standalone rustfmt, source file limits and whitespace checks pass. No Cargo,
rustc, rustdoc, compiler wrappers, Python, transforms, plugin calls or Rust
tests ran. Compilation, doctests and runtime behavior remain unverified.
Evidence: artifacts/consolidation-context-transforms/. The full workspace
consolidation goal remains active.

### Shared transform orchestration and reasoning conversion

The default transform pipeline now belongs to context-policy and accepts a
FnOnce reasoning-repair hook. The application supplies its existing
TetherScript adapter and retains the final tracing call so the original
logging target and filters remain unchanged. Plugin loading, authority setup
and feature gates remain application-owned. Pure message-to-hook JSON and
repaired-reasoning application move into shared reasoning conversion modules.

Fifteen static checks preserve pipeline ordering, callback placement, logging,
all conversion function bodies, plugin adapter files and both application
integration test files. JSON is promoted from a dev dependency to a runtime
dependency at the same version; Cargo.lock is unchanged. All 239 shared files
and 14 remaining experimental-transform application files are lexically
connected. Changed files have at most 32 code lines. Standalone rustfmt, source
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, plugin execution, transforms or Rust tests ran. Compilation
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-pipeline/. The broader session/workspace
consolidation goal remains active.

### Shared incremental derivation coordinator

The incremental coordinator now separates preparation, assembly and final
clamping in context-policy::derivation::incremental. An owned plan spans the
application's asynchronous prepared-index lookup. The host retains session
cloning, its below-budget gate, protected-entry seeding and index acquisition.
Shared preparation scores/orders the clone; shared assembly packs summaries
and repairs pairing; shared finalization applies the budget and provenance.

Ten static composition checks preserve operations and callback arguments on
each side of the existing asynchronous boundary. The live protected-entry
module remains byte-identical, and the existing packing integration harness
remains available under cfg(test). All 244 shared files and 50 application
context files are lexically connected. Changed files have at most 49 code
lines. Standalone rustfmt, source file limits and whitespace checks pass.
No Cargo, rustc, rustdoc, compiler wrappers, Python, index acquisition, token
estimation or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-incremental-coordinator/.
The broader session and workspace goal remains active.

### Shared reset preparation and reconstruction

Reset now has shared threshold/marker checks, active-tail splitting, ready
context construction, summary reconstruction and provenance finalization.
ResetPreparation distinguishes ready context, legacy fallback and a retained
prefix/tail summary plan. The application keeps provider/background summary
retrieval, dropped-range TOC generation and the legacy derivation call.
The prior application fallback helper is removed after its decisions move
into shared preparation.

Twenty-five static checks cover marker and reconstruction source, branch
correspondence, host callback/summary arguments, range finalization and module
wiring. Three original marker tests move unchanged; six new branch tests
cover threshold, marker, legacy, empty-prefix and summary-plan outcomes, but
none ran. All 261 shared files and 49 application context files are lexically
connected. Changed files have at most 44 code lines. Standalone rustfmt, source
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, provider calls, summaries or Rust tests ran. Compilation
and reset runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-reset/. The workspace goal remains active.

### Shared pin selection and state-header formatting

Pinned-index selection, recent/pinned budget reservation and deterministic
state-header formatting now belong to context-policy. Application adapters
supply immutable tracked-vector views, persisted constraint prefix lines,
and a lazy message-offset callback. The callback preserves per-included-pin
offset lookup rather than taking an eager session snapshot.

Sixteen static checks preserve all rule/function bodies after explicit input
substitution, including the eight-line cap, first-line UTF-8 excerpt and
saturating budget arithmetic. Four session-level regression tests remain
byte-identical and unrun. All 269 shared files and 49 application context files
are lexically connected; changed files have at most 35 code lines. Standalone
rustfmt, source file limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, session locks, database reads, formatting
operations or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-context-pins/. The broad
workspace consolidation goal remains active.

### Shared effective-policy resolution

Effective policy selection and persisted reset-threshold projection now
belong to context-policy. The application supplies its persisted policy and
message view, and retains the unknown-override warning through a callback
so the original log target and text stay unchanged. The obsolete private
implicit-policy facade is removed; shared implicit resolution remains active.

Six static comparisons preserve complete resolver order, repeated budget
lookup behavior, threshold mapping, warning forwarding and the unchanged
provider-dispatch function. All 271 shared files and 48 application context
files are lexically connected. New shared modules obey the 50-line limit,
and the existing application dispatcher shrank. Standalone rustfmt, source
file limits and whitespace checks pass. No Cargo, rustc, rustdoc, compiler
wrappers, Python, environment lookups, model calls or Rust tests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-effective-policy/. The full workspace goal remains active.

### Shared dispatch choice and focused host execution

Derivation dispatch selection now belongs to context-policy. Forced keep-last
recovery (including Some(0)) and oracle replay map to legacy execution; reset
thresholds and incremental budgets retain their payloads. Application modules
separate entrypoint/header injection, execution dispatch and each of the three
provider-backed branches. Existing public derive_with_policy paths are
preserved through re-exports. The former policy_dispatch module is replaced.

Thirteen static comparisons preserve the public signature, effective-policy
warning wrapper, strategy choices, every forwarded argument, budget resolution
and success-only state-header injection. Two pure selection regression tests
were added but not run. All 273 shared files and 53 application context files
are lexically connected, with no old dispatch references remaining. Changed
files have at most 37 code lines. Standalone rustfmt, source file limits and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
provider calls or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-context-dispatch/. The full
workspace goal remains active.

### Shared request assembly and final input guard

Stable system/history prefix construction, completion-request fields and the
final mandatory/optional input guard now belong to context-policy::request.
Application adapters retain proactive preparation, recall lookup, exact token
estimators and omission logging. Optional system/developer context is still
downgraded to user evidence and inserted before the active user turn.

Ten static comparisons preserve constructor order, all request fields, budget
reservation, rejection text, optional insertion logic, callback arguments and
the surrounding application call order. Guard regression files and provider
cache-prefix tests remain byte-identical and unrun. All 277 shared files and
52 application context files are lexically connected. Changed files have at
most 39 code lines. Standalone rustfmt, source file limits and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, estimator
calls, requests or Rust tests ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-context-request/. The full
workspace consolidation goal remains active.

### Shared context-overflow recovery

Context-limit error classification, prepared-context retry planning, the
[6, 3, 1] emergency keep-last schedule and request-option transitions now
belong to context-policy::request::recovery. The application derivation
adapter remains byte-identical, including its warning targets and text.
Existing classifier and completion-recovery paths are compatibility exports.

Twelve static checks preserve every matching rule, budget calculation, retry
schedule, option update and three moved test bodies. All 283 shared files and
52 application context files are lexically connected. New modules have at
most 38 code lines; the existing error-rendering module shrank. Standalone
rustfmt, source file limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, recovery calls or Rust tests ran. Compilation
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-recovery/. The full workspace goal stays active.

### Shared transcript rendering and text selection

Context-policy::transcript now owns RLM transcript rendering, stable role
labels, ordered text-block extraction and latest nonblank user text selection.
The application error and text helpers preserve existing paths through
re-exports. Filesystem-backed path discovery and truncation remain unchanged.
Content rendering, role labels and text selection are separate cohesive modules.

Twelve static checks compare the reconstructed renderer and each moved
function, verify compatibility exports and retained application functions,
and enforce the 50-code-line limit on every changed Rust file. All 287 shared
Rust files are lexically connected, with no missing declarations or orphan
files. Standalone rustfmt, source file limits and whitespace checks pass.
No dependencies changed. No Cargo, rustc, rustdoc, Python, compiler wrappers,
Rust tests, doctests or transcript functions were executed. Compilation and
runtime behavior remain unverified. Evidence:
artifacts/consolidation-context-transcript/. The full workspace goal remains active.

### Shared terminal compression fallback

Context-policy::compression::terminal now owns retained-tail selection,
payload-cap escalation, UTF-8-safe head/tail snippets, marker construction
and oldest-message eviction. The application supplies the existing request
estimator and warning callback. The warning stays in its original module
and runs after shrinking/eviction and before the final estimate. The shared
crate reuses codetether-util for the existing byte-safe prefix operation;
its path dependency and lockfile entry were updated without Cargo.

Twenty-one static comparisons cover the algorithms, constants, marker text,
estimator call order, callback wiring, retained user anchor and orphan-tool
eviction. Four existing application tests remain byte-identical and unrun.
All 296 shared files and 30 compression-adapter files are lexically connected;
new modules have at most 38 code lines. Standalone rustfmt, source file limits
and whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
Rust tests or compression operations ran. Compilation and runtime behavior
remain unverified. Evidence: artifacts/consolidation-terminal-compression/.
The full workspace goal remains active.

### Shared compression summary formatting

Context-policy::compression::summary owns summary-message installation,
cached oversized-message wrappers and dropped-range navigation. Its toc
submodule separates index construction from first-line excerpt selection.
Application paths remain compatibility exports; four index tests moved with
the owner without changing their bodies. Seventeen static comparisons cover
every moved function, constants, test bodies and exports. At this stage all
307 shared files and 29 application compression files were lexically connected;
new modules have at most 32 code lines. Standalone rustfmt, source file limits
and whitespace checks pass. Evidence: artifacts/consolidation-compression-summary/.

### Shared compression triggers and retry schedule

Context-policy::compression::budget owns BudgetCheck, trigger selection,
per-attempt budget/history fit and the [16, 12, 8, 6, 3, 1] retry schedule.
Application adapters keep context lookup and estimation in the original order,
and the async cascade, provider execution, events and logging remain unchanged.
Eleven static comparisons cover field shapes, thresholds, trigger precedence,
schedule, input mapping and the unchanged cascade. All 310 shared files and
29 application compression files are lexically connected. New modules have
at most 28 code lines; existing files respect the line-budget ratchet.
Standalone rustfmt, source file limits and whitespace checks pass. Evidence:
artifacts/consolidation-compression-policy/.

Neither stage executed Cargo, rustc, rustdoc, compiler wrappers, Python,
Rust tests, doctests, compression or provider operations. Compilation and
runtime behavior remain unverified. The full workspace goal remains active.

### Shared voice contracts, routing and speech catalog

Codetether-voice-core owns voice intents, command parsing/routing, SSH
marker detection, audio transport policy and the speech-model catalog.
Dictation separates contracts from parsing; the catalog separates model
types, entries, queries and tests. Application voice paths remain exports,
with only live process transport selection and native CPAL probing retained.
The shared crate depends only on the already-locked Serde version. Workspace
and lockfile changes were made without running Cargo.

Thirty static comparisons preserve every algorithm, serialized type shape,
catalog entry, native adapter, and eleven moved test bodies. Shared and
application module graphs contain 12 and 6 connected files, respectively,
without missing or unreferenced modules. Every changed voice file has at
most 33 code lines. Standalone rustfmt, source line limits and whitespace
checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python, Rust tests,
doctests or device probes ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-voice/. The voice source
ownership audit is complete; the full workspace goal remains active.

### Shared belief lifecycle and project memory

Codetether-cognition-core owns Belief and BeliefStatus, confidence updates,
extraction prompts, response decoding and project-memory persistence.
Contracts, lifecycle, response schema, prompts and decoding are separate
modules. Project memory separates load, save and context rendering. The
application retains the optional-thinker guard and live thinker call; all
original public belief and memory paths remain available.

Twenty-four static checks compare contracts, methods, exact prompts, decoder
composition, timestamp calls, UUID construction, persistence behavior and
dependency changes. Lockfile comparison ignores package-block order while
checking every block’s contents. Six lifecycle tests moved unchanged, and
one no-thinker test remains in the application unchanged; none ran. All 19
shared files are lexically connected. New modules have at most 45 code lines.
Standalone rustfmt, source file limits and whitespace checks pass. No Cargo,
rustc, rustdoc, compiler wrappers, Python, model calls, tests, doctests or
project-memory I/O operations ran. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-cognition-beliefs/. The full
workspace goal remains active.

### Inactive actor subtree audit

Neither application root declares src/actor, and an explicit-reference scan
across src, tests and crates found no external crate::actor, ActorRuntime or
actor module-path references. This differs from the active swarm Actor trait.
The 14 tracked actor files are retained, with source hashes recorded in
artifacts/consolidation-cognition-beliefs/actor-audit.json. This lexical audit
is not a compiler reachability proof or permission to activate the subsystem;
its final disposition remains outstanding.

### Shared persona, proposal, thought and workspace contracts

Cognition-core now groups identity/policy/lifecycle request records under
persona; proposal records, votes and governance defaults under proposal;
thought events, phase rotation, labels and tick data under thought; and
shared summary/attention records under workspace. Application paths remain
compatibility exports. Inherent phase and event-label methods moved with
owning types; their visibility permits existing application calls. No
locks, model execution or task-spawning code moved into these contracts.

Thirty-two static comparisons preserve serialized fields, defaults, method
bodies, stable labels and facade exports. Two workspace tests moved unchanged
and remain unrun. All 38 shared files and 199 application cognition files
are lexically connected. New modules have at most 34 code lines. Standalone
rustfmt, source limits and whitespace checks pass. No dependencies changed;
no Cargo, rustc, rustdoc, compiler wrappers, Python, tests, doctests or runtime
operations ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-contracts/. The full workspace goal stays active.

### Shared cognition store, voting, lineage and ranking policy

Cognition-core::beliefs::update owns duplicate confirmation, contradiction
handling, decay attention and extracted-belief merge events. proposal::voting
owns quorum, veto, required-role, majority, deadline and attention decisions.
Persona lineage/depth/reap-target modules own inheritance and traversal.
Workspace ranking owns belief, attention and uncertainty ordering; shared
text helpers retain the existing truncation and sentence-count behavior.
Application paths now export these operations. Async voting locks, workspace
refresh locks and existing governance/lineage tests remain unchanged.

Forty static comparisons preserve algorithms, constants, event payloads,
timestamps, UUID calls, signatures and facade exports. All 56 shared files
and 199 remaining application cognition files are lexically connected. New
modules have at most 43 code lines. Standalone rustfmt, source limits and
whitespace checks pass. No dependencies changed. No Cargo, rustc, rustdoc,
compiler wrappers, Python, Rust tests, doctests or policy operations ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-policy/. The full workspace goal stays active.

### Shared cognition scheduling and store services

Cognition-core now owns per-persona budget-window scheduling and idle
detection, asynchronous budget charging, bounded event/snapshot buffering,
recent-persona event selection, pending-vote sweeps and workspace refresh.
The services retain their existing Arc/RwLock signatures and locking order.
Main-loop scheduling, live thinker invocation, task startup and application
runtime construction remain application responsibilities. The core adds only
a dependency on the existing Tokio version with its sync feature.

Twenty-six static comparisons preserve all functions, constants, lock
scopes/order, selection ordering, event payloads, casts, saturation and time
calls. Two budget tests moved unchanged, with unchanged fixture functions.
All 67 shared files and 197 application cognition files are lexically
connected. New modules have at most 41 code lines. Standalone rustfmt,
source file limits and whitespace checks pass. No Cargo, rustc, rustdoc,
compiler wrappers, Python, tests, doctests or service operations ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-services/. The full workspace goal remains active.

### Shared cognition prompt and output presentation

Cognition-core::thought::presentation groups phase prompts, event formatting,
output normalization, placeholder detection, deterministic fallback text and
recent-context signal summaries in focused modules. All original application
helper paths are compatibility exports. Model invocation remains in the
unchanged application thinker adapter. No prompts or fallback rules changed.

Nineteen static comparisons preserve complete function/constant source,
prompt and fallback literals, normalization order and facade exports. The
original thinker adapter and two normalization tests remain byte-identical;
no tests ran. All 76 shared files and 197 application cognition files are
lexically connected. New modules have at most 42 code lines. Standalone
rustfmt, source file limits and whitespace checks pass. No dependencies
changed. No Cargo, rustc, rustdoc, compiler wrappers, Python, model calls,
Rust tests or doctests ran. Compilation/runtime behavior remain unverified.
Evidence: artifacts/consolidation-cognition-presentation/. The full workspace
goal remains active.

### Shared cognition persistence service

Cognition-core::persistence owns the versioned state schema, referenced-event
collection, payload trimming, evidence ordering, lock-ordered state capture,
file-size guarding, decoding and temporary-file/rename write sequence.
Application adapters supply data-directory resolution and log callbacks;
logging remains in the original persistence module. The save path callback
is evaluated after state capture, preserving the original operation order.
Tokio fs was enabled on the already-declared dependency.

Thirty static comparisons preserve schema, constants, helper algorithms,
save/load composition, error strings, callback placement, logs and adapter
signatures. Two persistence tests moved with only their internal import path
adjusted; neither ran. Initial-state loading and tick flushing remain
byte-identical. All 90 shared files and 200 application cognition files are
lexically connected. New modules and adapters have at most 48 code lines.
Standalone rustfmt, source limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, tests, doctests or cognition-state I/O ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-persistence/. The full workspace goal stays active.

### Shared cognition output builders and lineage projection

Cognition-core now owns default seed requests, proposal titles/construction,
proposal-created events, thought events, memory snapshots and snapshot
metadata. Status/control contracts and deterministic lineage graph projection
also moved. The application retains thinker endpoint normalization and the
lineage read lock, delegating projection while the guard is still held.

Twenty-seven static comparisons preserve all constructors, field shapes,
quorum/deadline calculations, UUID/time call sites, output text and graph
ordering. Existing persona and budget test sources remain byte-identical and
unrun. All 102 shared files and 200 application cognition files are lexically
connected. New files have at most 46 code lines. Standalone rustfmt, source
limits and whitespace checks pass. No dependencies changed. No Cargo, rustc,
rustdoc, compiler wrappers, Python, tests, doctests or constructors ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-builders/. The full workspace goal stays active.

### Shared execution contracts and request policy

Cognition-core::execution owns capability leases, invocation/receipt records,
outcomes, extraction request schemas and prompts, response decoding, existing
shallow argument validation, and five check-result event constructors. The
application executor is split into a live extraction coordinator and one
per-request dispatcher. The sequence remains allowlist, registry, schema,
lease creation/check, tool call, result clipping and event construction.

Twenty-five static comparisons cover contracts, lease methods, prompt text,
validation, decoding, event fields and the request-loop rewrite into early
returns. Four shared test bodies and one application no-thinker test are
unchanged and unrun. Proposal receipt consumers remain byte-identical. All
121 shared files and 203 application cognition files are lexically connected;
new executor modules have at most 40 code lines. Standalone rustfmt, source
limits and whitespace checks pass. No dependencies changed. No Cargo, rustc,
rustdoc, compiler wrappers, Python, tests, doctests, lease creation or model/tool
execution ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-execution/. The full workspace goal stays active.

### Shared verified-proposal dispatch and persona progress

Cognition-core::proposal::dispatch owns shared execution handles, the existing
Critical/Verified approval gate, pending-approval registration, verified sweep
and receipt/event construction. Application approval delegates through the
original method. Persona progress and allowed-tool lookup live beside persona
scheduling; idle reap policy preserves the existing direct-child behavior.

Twenty static comparisons preserve all bodies, lock scopes/order, error text,
approval checks, state transitions and event fields. Tick publication and
existing governance/idle test sources remain byte-identical and unrun. All
128 shared files and 203 application cognition files are lexically connected.
New modules have at most 35 code lines. Standalone rustfmt, source limits and
whitespace checks pass. No dependencies changed. No Cargo, rustc, rustdoc,
compiler wrappers, Python, tests, doctests, approval or dispatch operations
ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-dispatch/. The full workspace goal stays active.

### Shared persona lifecycle mutations

Cognition-core::persona::lifecycle now owns persona record assembly,
creation validation/insertion, child-request mapping, reap preparation and
lifecycle event construction. Application methods publish after the service
releases its persona write lock. Reap events are still constructed one at a
time immediately before publication; they are not prebuilt as a batch.

Seventeen static comparisons preserve inheritance, policy precedence, IDs,
errors, timestamps, lock-release order, event payloads and public signatures.
Existing lifecycle, persona, branching and depth test sources remain
byte-identical and unrun. All 134 shared files and 203 application cognition
files are lexically connected. New modules/adapters have at most 42 code
lines. Standalone rustfmt, source limits and whitespace checks pass. No
dependencies changed. No Cargo, rustc, rustdoc, compiler wrappers, Python,
Rust tests, doctests or persona operations ran. Compilation and runtime
behavior remain unverified. Evidence: artifacts/consolidation-cognition-lifecycle/.
The full workspace goal remains active.

### Shared cognition and thinker configuration

Cognition-core::configuration owns runtime options, typed environment readers
and environment option assembly. thinker owns backend/device enums, config
defaults, completion output, endpoint normalization and thinker environment
assembly. The application retains client construction, backend selection and
model loading; existing import paths delegate to the shared types/functions.

Thirty static comparisons preserve fields, defaults, parsing, environment
variable names and read order, Candle settings and per-backend timeouts. The
client declaration and construction/selection sources remain byte-identical.
All 146 shared files and 203 application cognition files are lexically
connected. New shared modules have at most 49 code lines. Standalone rustfmt,
source limits and whitespace checks pass. No dependencies changed. No Cargo,
rustc, rustdoc, compiler wrappers, Python, tests, doctests, model loading,
credential lookup or configuration functions ran. Compilation and runtime
behavior remain unverified. Evidence: artifacts/consolidation-cognition-configuration/.
The full workspace goal remains active.

### Shared thinker wire and conversion policy

Cognition-core::thinker::protocol owns OpenAI-compatible wire types, request
construction and decoded-payload conversion; Bedrock body/URL/output mapping;
and registry-provider model validation, request construction and output
conversion. Identity and model-catalog calls use their existing provider-core
owners directly. The application retains HTTP body decoding, networking,
retries, logs, credentials and provider/backend execution.

Twenty-seven static comparisons preserve wire fields, serde attributes,
content/reasoning precedence, request ordering, identity calls, usage casts
and model validation. HTTP orchestration/send/status files and existing
request-identity/model-validation tests remain unchanged and unrun. All 157
shared files and 200 application cognition files are lexically connected.
New modules have at most 48 code lines. Standalone rustfmt, source limits
and whitespace checks pass. The only dependency addition is the existing
provider-core path crate. No Cargo, rustc, rustdoc, compiler wrappers, Python,
network calls, tests, doctests or conversion functions ran. Compilation and
runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-protocol/. The full workspace goal stays active.

### Shared OpenAI-compatible thinker HTTP transport

Cognition-core::thinker::http owns the two-attempt loop, send/status
classification, response decoding and retry pacing. HttpObserver callbacks
keep diagnostics in the original application modules with the same targets
and fields. The empty application response module was removed. Reqwest JSON
and rustls features plus Tokio time were added using existing locked versions.

Twenty-three static comparisons preserve bearer handling, error text, request
and body-consumption order, backoff, callback routing and existing retry-test
source. All 165 shared files and 199 application cognition files are lexically
connected. Changed transport/logging modules have at most 37 code lines.
Standalone rustfmt, source limits and whitespace checks pass. No Cargo, rustc,
rustdoc, compiler wrappers, Python, tests, doctests, network requests or
credential operations ran. Compilation and runtime behavior remain unverified.
Evidence: artifacts/consolidation-cognition-http/. The full workspace goal
remains active.

### Shared Bedrock and resolved-provider completion services

Cognition-core::thinker::bedrock constructs and processes Converse requests
with a host-supplied signed sender and success observer. Application signing
and original log targets remain outside the core. thinker::provider accepts
an already-resolved provider and performs the existing validation/completion/
conversion sequence; registry and Vault lookup remain in the application.
Error snippets use the same shared byte-safe utility via a direct dependency.

Fourteen static comparisons preserve request order, body/status handling,
error strings, success-log timing, provider validation and output labeling.
Client construction and request-identity test sources remain unchanged and
unrun. All 168 shared files and 199 application cognition files are lexically
connected; new services/adapters have at most 28 code lines. Standalone
rustfmt, source limits and whitespace checks pass. No Cargo, rustc, rustdoc,
compiler wrappers, Python, tests, doctests, network, signing or Vault operations
ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-backend-services/. The full workspace goal
remains active.

### Separate native cognition model owner

Codetether-cognition-native owns Candle model loading, tokenizer/model/cache
state, generation, sampling, blocking dispatch, Bonsai integration and prefill
chunking. The application keeps compatibility exports and uses a borrowed
identity accessor in the FunctionGemma router. Native types reuse cognition-core
contracts and the existing Bonsai provider implementation. Native/GPU dependencies
are feature-gated in the new owner; root candle, CUDA, cuDNN, Accelerate, MKL
and FunctionGemma features forward to it while retaining previous flags.

Forty-eight static comparisons preserve moved source, model operations,
log targets, identity usage, feature forwarding and lockfile package contents.
All 11 existing native/prefill test bodies moved with import-only adjustments;
none ran, including the ignored full-model smoke test. All 37 native files
and 169 remaining application cognition files are lexically connected.
New modules have at most 50 code lines. Standalone rustfmt, source limits and
whitespace checks pass. No Cargo, rustc, rustdoc, compiler wrappers, Python,
model-file reads, model loads, device probes, GPU operations, tests or doctests
ran. Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-native/. The full workspace goal remains active.

### Shared cadence and thought-result construction

Cognition-core::driver owns the fixed-cadence loop, stop checks, minimum
interval and overrun rescheduling. The application supplies its existing
run_tick callback and shared handles. Model/default thought-result builders
and TickOutput now belong to the thought module; per-persona processing and
runtime task startup remain application code.

Thirteen static comparisons preserve cadence control flow, timing calls,
result fields, output shape and unchanged per-persona processing. All 171
shared files and 169 application cognition files are lexically connected.
New modules/adapters have at most 36 code lines. Standalone rustfmt, source
limits and whitespace checks pass. No dependencies changed. No Cargo, rustc,
rustdoc, compiler wrappers, Python, driver loops, tests or doctests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-driver/. The full workspace goal stays active.

### Shared FunctionGemma routing policy

Cognition-core::tool_routing owns parsed-call records, closed-block extraction,
JSON parsing, tool prioritization, prompt/schema formatting, loaded-model
identity injection and response inspection. The application parser supplies
its original warning callback; native inference and response mutation remain
in the application. No prompt or routing policy changed.

Twenty-three static comparisons preserve algorithms, constants, templates,
warning fields/target and application exports. Existing parse, malformed-parse
and prompt tests plus the inference adapter remain byte-identical and unrun.
All 180 shared files and 169 application cognition files are lexically connected.
New modules have at most 38 code lines. Standalone rustfmt, source limits and
whitespace checks pass. No dependencies changed. No Cargo, rustc, rustdoc,
compiler wrappers, Python, routing calls, tests or doctests ran. Compilation
and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-routing/. The full workspace goal stays active.

### Complete FunctionGemma router ownership

Active and disabled configuration contracts, environment loading, thinker
configuration mapping and response rewriting now belong to cognition-core.
Cognition-native owns the active router, inference, reformatting and the no-op
adapter. Application feature branches re-export these owners. The disabled
application branch names disabled_routing explicitly so feature unification
cannot select an active router accidentally. Existing 90-second timeout,
direct-parse ordering, configuration defaults/kill switch and log targets remain.

Thirty-five static comparisons preserve source, configuration and branch
behavior. All 9 existing router tests moved without body changes and remain
unrun. Core/native/application module graphs contain 185, 60 and 146 connected
files. New modules have at most 48 code lines. Standalone rustfmt, source
limits and whitespace checks pass. The only dependencies added are existing
Regex and Tokio time support. No Cargo, rustc, rustdoc, compiler wrappers,
Python, routing/inference operations, tests or doctests ran. Compilation and
runtime behavior remain unverified. Evidence:
artifacts/consolidation-cognition-router-owner/. The full workspace goal remains active.

### Shared CloudEvent task-ingress parsing

A2A-worker-core::cloudevents owns the normalized CloudEvent record and
structured/binary/legacy parsing. The application passes a header lookup
closure that retains its original Axum HeaderMap decoding and public API.
Structured-body precedence, lazy header lookups and legacy defaults remain.

Eight static comparisons preserve wire attributes, parsing order, defaults,
errors and adapter wiring. Both ingress contract tests remain byte-identical
and unrun. All 30 worker-core files are lexically connected. New modules and
adapter have at most 23 code lines. Standalone rustfmt, source limits and
whitespace checks pass. No dependencies changed. No Cargo, rustc, rustdoc,
compiler wrappers, Python, parsing operations, tests or doctests ran.
Compilation and runtime behavior remain unverified. Evidence:
artifacts/consolidation-cloudevents/. The full workspace goal stays active.

## Runtime policy decision ownership

The new codetether-runtime-policy crate owns RuntimeToolPolicy and all its
inherent implementations, decision contracts, explicit tool and command rules,
base decisions, action classification, shell read-only classification, and
stable metadata codes. Configuration comes from codetether-config. The
read-only tool allowlist now belongs to codetether-tool-core and is shared by
dispatch and permission policy through the existing application facade.

The application keeps invocation ordering and live boundaries: prior-context
checks, approved session commands, sandbox availability, approval requests,
workspace loading, justification, and ToolResult construction. Existing public
imports remain available. These remaining adapters still require a cohesion
audit; runtime-policy ownership is not marked complete.

Static source comparisons preserve rule precedence, command guards and
allowlists, configuration accessor order, enum variants and metadata strings.
The three action-classification tests moved unchanged with their module;
application integration and read-only dispatch tests remain in place and unrun.
The dependency closure has eight local crates and no application back-edge.
Lexical module graphs cover 15 shared-policy, 44 tool-core and 47
application-policy files with no missing or unreferenced modules.

Validation: 19 static checks, at most 43 code lines per new Rust file, standalone
rustfmt, global file-limit ratchet and git diff whitespace checks. Dependency
array order is ignored in lockfile comparisons; package versions and contents
are otherwise preserved, with one new local package and application edge.
No Cargo, rustc, Python, model, provider, shell-command classifier execution,
or Rust tests ran. Compilation and tests remain unverified. Evidence is in
artifacts/consolidation-runtime-policy/. The full consolidation goal remains active.

## Runtime policy invocation and diagnostics

Shared runtime-policy now owns the invocation sequence as well as individual
rules. Host session-command and sandbox callbacks are lazy: explicit command
denials run first, then session grants, read-only classification, other
command-prefix rules, sandbox policy and finally the action-aware default.
Callbacks preserve their former evaluation points; the application still
reads the session grant store and probes actual sandbox state.

The pure sandbox-availability decision, blocking-result construction, missing
justification policy and retry-example construction also moved into the shared
crate. Approval-store access, prior-context guarding, workspace loading,
invocation scope hashing, request metadata and schema advertising remain in
the application. The three retry-example tests moved with their implementation,
and the two existing sandbox decision tests and host preflight test are
unchanged. All tests remain unrun.

Evidence: artifacts/consolidation-runtime-policy-invocation contains 15 source
comparison checks, snapshots, lexical module graphs and file-limit/whitespace
outputs. The shared crate has 22 reachable Rust files, the application runtime
policy has 45 files / 1235 code lines, and new files have at
most 40 code lines. No dependency changed. No Cargo, rustc, Python, sandbox probe,
approval-store access or Rust test was executed. Compilation and runtime behavior
remain unverified. The whole-workspace goal remains active.

## Runtime approval metadata and schema ownership

Shared runtime-policy owns reviewer previews, full invocation detail, approval
error-field insertion and the justification-schema mode flag. The application
re-exports the same schema functions and metadata helpers, so all existing
callers share one flag. Mode storage/loading still use relaxed atomics in their
original order. The shared crate now directly depends on provider-core for
ToolDefinition, an existing transitive dependency; no new package was added.

The file-mutation preview support classifier belongs to tool-core. Concrete
proposed file reconstruction remains in the application. Metadata construction
uses the same classifier through its shared owner. Short summaries retain their
160-character clipping, full detail retains its original hidden-field filtering,
and approval metadata keys and insertion order are preserved. Five preview/detail
tests moved unchanged; schema tests remain in the application. All are unrun.

Validation: 15 static source/dependency checks, lexical graphs for 29 shared-policy,
45 tool-core and 41 application-policy files, no missing/unreferenced
modules or local dependency cycles, maximum 37 code lines per new Rust file,
standalone formatting, global file-limit ratchet and whitespace checks. Evidence
is in artifacts/consolidation-runtime-policy-metadata/. No Cargo, rustc or Python
ran; compilation and tests remain unverified. Approval request persistence and
scope construction remain for audit. The full workspace goal stays active.

## Approval interaction contracts

Approval-core::interaction now owns ExecPolicyAmendment, ReviewDecision,
LiveApprovalRequest and LiveApprovalDecision. Its eight focused modules
separate request records and builders, decision records, serialized review
choices, command amendments and prefix parsing. All inherent implementations
moved with their types. The application keeps compatibility exports and live
request delivery, pending-state management and session grants.

The prefix parser moved from runtime_policy into the same interaction owner
as its returned amendment contract. No dependency or manifest changed. Source
comparisons preserve serialization attributes, fields and variants, default
review choices, decision labels, denial normalization and prefix token handling.
Two application handoff test modules remain byte-identical and unrun.

Validation: 17 static comparisons, 36 reachable approval-core files, 13
application approval files and 41 runtime-policy files, no missing
or unreferenced lexical modules, at most 49 code lines per new module, standalone
rustfmt, global file-limit ratchet and whitespace checks. The shared approval
crate retains its existing two-crate local dependency closure. Evidence lives
in artifacts/consolidation-approval-interaction/. No Cargo, rustc, Python,
approval store or live request was executed. Compilation and tests remain
unverified, and the complete workspace consolidation goal remains active.

## Approval state ownership audit

Approval-core now owns process-local tuple grants, command-prefix proposals
and grants, pending request senders and ordering, pending queries and review
decision parsing/application. Each original singleton moved once; application
paths re-export it. The host still sends SessionEvent::ApprovalRequest and
retains the same cancellation guard and failed-delivery behavior.

The existing test reset functions remain gated behind cfg(test) or the
approval-core test-support feature. Only the application's dev-dependency
enables that feature. Normal application dependencies do not expose the reset
helpers. Approval-core adds the existing Tokio version with only its sync
feature; no package version changed.

All 13 remaining application approval files have a defined role:
SessionEvent delivery and its cancellation guard, compatibility exports, or
test setup/tests. They contain 167 code lines. This completes the approval
subsystem ownership audit, not the entire workspace consolidation goal.

Evidence in artifacts/consolidation-approval-state: 15 static source/dependency
checks; 41 reachable shared approval files and 13 reachable application
approval files; no missing/unreferenced lexical modules; maximum 47 code lines
per new Rust file; standalone formatting, file-limit ratchet and whitespace
checks. The original live handoff tests and environment isolation code are
byte-identical. No Cargo, rustc, Python, grant operations or live handoff ran.
Compilation and tests remain unverified.

## Pure patch engine ownership

Tool-core::patch owns the unified-diff parser, header/range/body decoding,
hunk records and construction, stable target grouping, in-memory application,
and approval resource formatting. Its twelve modules retain original parser
and matching behavior. The application patch tool calls shared exports and
continues to own filesystem access, path guards, approval enforcement and
execution orchestration. No dependencies changed.

Source comparisons preserve whitespace matching, insertion offsets, counted
hunk completion, malformed-header handling, sorted target identities and
result newline behavior. The hunk application regression tests moved unchanged.
Application preview, proposed-content and approval test files are byte-identical,
as are the application execution/proposed-content pipelines. All tests remain
unrun. Runtime policy can now share the same patch-resource implementation.

Evidence in artifacts/consolidation-patch-core: 29 static checks, lexical graphs
for 57 tool-core files and 24 patch-tool files, no missing/unreferenced modules,
a three-crate local tool-core dependency closure, maximum 37 code lines per new
module, standalone formatting, file-limit ratchet and whitespace checks. No
Cargo, rustc, Python, patch application or filesystem tool execution ran.
Compilation and tests remain unverified. The full workspace goal remains active.

## Runtime approval persistence and identity

Runtime-policy now owns approval request persistence, verification, session-command
checks, exact invocation identity and approval-gated result construction. It uses
the same approval-core stores and tool-core patch resource implementation as
application callers. The original hashing exclusions, grant/verification order,
request metadata and error behavior are retained. Five unused application
facades were removed after their callers moved. Configuration loading, prior
context guards and sandbox probes remain application adapters.

Validation evidence is in artifacts/consolidation-runtime-approval/: ten static
source and dependency comparisons and complete lexical module graphs. Existing
identity tests are unchanged and unrun. No Cargo, rustc or Python ran; compilation
and tests remain unverified. The full workspace goal is still active.

## Runtime-policy application boundary audit

Runtime-policy owns configured evaluation, workspace selection and tool-only
evaluation. The configured evaluator accepts a lazy sandbox probe and retains
the original decision → scope → approval → justification → request sequence.
The application checks prior context before calling it, and retains async
configuration-loading entry points so their timing and fallback are unchanged.
Redundant invocation-decision, session-command and result-approval facade files
were removed. The remaining 33 application files (796 code lines)
are integration tests, compatibility exports, configuration entry points, the
prior-context guard or the actual sandbox availability adapter. This completes
the runtime-policy ownership audit, not the overall workspace goal.

Eight static checks compare original algorithms, callback placement and async
entry points. Lexical module graphs include every test/cfg branch: 37 shared
runtime-policy files and 33 application files, without missing or
unreferenced modules. New files have at most 37 code lines. Standalone rustfmt,
file-limit ratchet and staged whitespace checks are the permitted validation;
compilation and all Rust tests remain unverified. No Cargo, rustc or Python
ran. Evidence: artifacts/consolidation-runtime-policy-adapters/.

## Agent message and tool execution helpers

Agent-core::execution owns tool-call and response-text extraction, image-tool
conversation enrichment, registered-tool result conversion and invalid-tool
fallback handling. Application helpers supply session messages and registered
tool names. Session mutation, goal/peer prompt composition, tool permissions
and the provider execution loop remain in the application. Agent-core gains
only the existing tool-core dependency.

Eleven static comparisons preserve call metadata, text and image order, tool
error strings, output-budget timing, host prompt composition and the unchanged
image-context test file. Lexical graphs cover 44 agent-core files and 33
application-agent files with no missing/unreferenced modules. New modules have
at most 29 code lines. Standalone formatting, file-limit and whitespace checks
were used; no Cargo, rustc, Python, tools or model calls ran. Compilation and
Rust tests remain unverified. Evidence: artifacts/consolidation-agent-execution/.
The complete workspace goal remains active.

## LSP diagnostic helpers

LSP-core owns document-specific publication waiting, TypeScript diagnostic
conversion, client capability declarations and document language IDs. Five
tests moved with their implementations; the paused-clock tests gain Tokio's
test-util feature through the core crate's dev-dependency. Existing application
paths re-export the shared helpers. Application lifecycle, initialization,
cache management and diagnostic request orchestration remain unchanged.

Full-client extraction still needs an explicit initialization identity
boundary: the current initialize method advertises the application package
version. This stage preserves that identity rather than silently reporting
the core crate's version. No language server, linter or compiler was started.

Evidence in artifacts/consolidation-lsp-diagnostics includes thirteen static
source/API/dependency checks, lexical graphs with 61 shared and 37
application LSP files, and no missing/unreferenced module files. New files have
at most 48 code lines. Standalone formatting, the file-limit ratchet and
whitespace checks are the permitted validation. Compilation and all Rust
tests remain unverified. No Cargo, rustc or Python ran. The broader workspace
consolidation goal remains active.

## LSP initialization service boundary

LSP-core::initialize now builds and sends initialize requests, handles errors,
stores returned capabilities, notifies initialization and sets transport state.
The application supplies its package version and the initialized-log observer.
This preserves application identity and the original tracing target without
binding the shared crate to the application's manifest. Request construction
and handshake sequencing live in separate focused modules.

Seven static checks compare all request fields and ordering, capability update
and observer timing, initialized notification/state changes and application
identity/log wiring. Lexical graphs cover 63 core and 37 application
LSP files with no missing/unreferenced modules. New files have at most 28 code
lines. No dependencies changed. Formatting, file-limit and staged whitespace
checks were used. No Cargo, rustc, Python, language server or handshake ran;
compilation and tests remain unverified. Evidence is retained in
artifacts/consolidation-lsp-initialize/. The full workspace goal remains active.

## Shared single-server LSP client

LSP-core owns the single-server transport/configuration/capability/document
state and all document, navigation, completion, symbol, shutdown and diagnostic
operations. Application LspClient wraps and dereferences that client, retaining
its existing constructors and no-argument initialization method. The wrapper
supplies the application version and original initialization logging. Other
client logs carry explicit original application targets.

The application manager still caches Arc wrappers, so reference-count eviction
semantics are unchanged. Cache timestamps, test configuration and diagnostic
snapshots are accessed through shared-client getters. The moved constructor
retains its original timestamp/lock construction order. Tokio fs is promoted
to a normal feature because the shared client owns document reads. No package
versions or lockfile dependencies changed.

Validation: 23 static source/state/adapter checks, original cache and diagnostic
algorithms compared with snapshots, 78 core and 23 application files
reachable without missing/unreferenced lexical modules, maximum 42 code lines
per new module, standalone formatting, file-limit and whitespace checks. No
Cargo, rustc, Python, language server or LSP request executed. Compilation and
all tests remain unverified. Evidence: artifacts/consolidation-lsp-client/.
The broader workspace consolidation remains active.

## Generic LSP cache ownership

LSP-core::client_cache owns cache storage, lazy eviction startup, idle sweeps
and timestamp refresh. CacheClient provides only the last-use mutex. The
application specializes the generic cache for its existing identity wrapper,
preserving the exact Arc allocation counted by eviction. Default construction
creates the same empty Arc/RwLock map and OnceLock without starting a task.

The 120-second idle limit, 30-second sweep, sole-owner test, poison recovery,
lock lifetime and timestamp-before-Arc-clone order remain unchanged. The
eviction log retains its original application target. Three cache tests moved
with import/type changes and remain unrun. No dependencies changed.

Seven static comparisons, lexical graphs (84 core and 21 application
files with no missing/unreferenced modules), maximum 46 code lines per new
file, standalone formatting, file-limit and whitespace checks are recorded in
artifacts/consolidation-lsp-cache/. No Cargo, rustc, Python, language server or
eviction task ran. Compilation and tests remain unverified. The full workspace
consolidation goal remains active.

## LSP manager ownership audit

LSP-core::manager now owns primary-server and linter selection, configuration
precedence, connection creation and initialization order, cache lookups,
capability/document forwarding, recovery and shutdown. Its ManagedClient
contract accepts the existing application identity wrapper. The public
application LspManager name aliases this generic manager with the same wrapper,
so returned Arc clients preserve identity and ownership. All moved logging
retains explicit original application targets.

The remaining 13 application LSP files (280 code lines) contain
identity constructors/initialization, cache and manager trait adapters,
compatibility exports, or real-server integration tests and their helpers.
This completes the LSP ownership audit. No server or integration test ran.

Fourteen static checks cover original method bodies, generic state, factory
delegation and alias compatibility. Lexical graphs cover 95 shared LSP files
and 13 application files with no missing/unreferenced modules. New
modules have at most 45 code lines. Standalone formatting, file-limit and
whitespace checks are recorded in artifacts/consolidation-lsp-manager/. No
dependencies changed. No Cargo, rustc or Python ran. Compilation and tests
remain unverified. The entire workspace goal remains active.

## Forage selection and presentation ownership

The new forage-core crate owns opportunity/rubric/summary records, status and
deadline weighting, alignment tokenization and matching, prompt construction,
text normalization and summary rendering. Selection uses one timestamp per
collection, the original score and remaining-work sort order, and a supplied
scoring callback. The application supplies its existing optional TetherScript
scorer at the same point in each candidate's evaluation.

Shared types retain their inherent methods. The console entry printer becomes
a free application function to keep console effects out of the shared crate.
Root forage tests remain unchanged; execution, audit, persistence, CLI arguments
and bus integration remain in the application for further consolidation.

Twenty-four static comparisons cover original fields, algorithms, prompts,
summary methods, per-candidate ordering, sorting, console output and manifest
changes. Lexical graphs cover 15 shared and 13 application files with
no missing/unreferenced modules. New Rust files have at most 50 code lines.
Formatting, source limits and staged whitespace checks are the permitted
validation. No Cargo, rustc, Python, scoring hook or forage execution ran.
Compilation and tests remain unverified. Evidence: artifacts/consolidation-forage-core/.
The full workspace consolidation goal remains active.

## Forage objective seeding and progress persistence

Forage-core now owns default/mission-derived objective seeding and execution
progress recording. Seeding guards and repository call ordering are unchanged,
and logging uses the original application target. Progress credit, outcome
creation, evidence formatting/file caps and repository updates have separate
modules. The host supplies borrowed execution facts instead of exposing CLI
argument types to the shared crate.

Fourteen static comparisons cover the original guards, formulas, repository
sequence, metadata and unchanged root tests. Lexical graphs cover 24 shared
and 13 application files with no missing/unreferenced modules. New
modules have at most 42 code lines. Only existing anyhow/tracing dependency
edges were added. No OKRs were read or written by runtime execution. No Cargo,
rustc, Python or tests ran; compilation remains unverified. Evidence resides
in artifacts/consolidation-forage-progress/. The full workspace goal is active.

## Forage quality workflow and change snapshots

Forage-core owns Git change snapshots and ordered quality-check orchestration,
while the application supplies its existing policy-gated Bash runner. Both
original command strings and their order are retained, as are the Rust-file
skip guard, result flags, empty-output messages and 200/300-character summary
limits. One check module owns formatting and outcome handling for both specs.
The host execution wrapper still takes snapshots before and after execution,
then invokes quality checks at the same point. No dependency changed.

Eleven static comparisons cover both specialized check branches, command
specifications, Git behavior, output records, unchanged authorization adapter
and original forage tests. Lexical graphs cover 29 core and 13
application files without missing/unreferenced modules. New files have at most
37 code lines. Standalone formatting, file-limit and whitespace checks were
used. No quality commands, Cargo, rustc, Python or tests were executed.
Compilation and tests remain unverified. Evidence is in
artifacts/consolidation-forage-verification/. The full workspace goal is active.

## Shared MCP contracts and result conversion

MCP-core owns wire records and all their constructors, protocol-version
constants, decoded message classification and local tool-result conversion.
Each record has a focused module. Application types and transport-message
paths re-export the shared owners; transport operations are unchanged. Tool
images retain their MIME field spelling, base64 handling and error behavior.

Sixty-four static comparisons cover records, enum variants, serde attributes,
constructors, constants, message classification, result conversion, unchanged
transport code and image tests, and lockfile package preservation. Lexical
graphs cover 53 core and 35 application files without missing or
unreferenced modules. New Rust files have at most 47 code lines. Formatting,
file limits and staged whitespace checks were used. No Cargo, rustc, Python,
MCP transport or tests ran; compilation and tests remain unverified. Evidence
is retained in artifacts/consolidation-mcp-contracts/. The whole goal is active.

## Shared MCP transports

The separate mcp-transport crate owns the transport trait, inert transport,
standard-I/O channels/threads, HTTP/SSE transport and subprocess channels/tasks.
Reader, writer, stderr, construction and protocol operations have separate
modules. It uses mcp-core wire types and has no application dependency.
The application preserves its transport import path through re-exports.

Channel capacities, spawn order, framing, errors and close behavior remain
unchanged. All moved log calls retain their original targets, including the
separate subprocess stderr target. SSE reception remains the existing
placeholder; this extraction does not change that behavior.

Twenty-four static comparisons cover every transport implementation and task
body, constructor sequencing, fields, and lockfile package preservation.
Lexical graphs cover 17 transport and 35 application files without
missing/unreferenced modules. New files have at most 34 code lines. Formatting,
file limits and whitespace checks were used. No transport, network request,
subprocess, Cargo, rustc, Python or Rust test was executed. Compilation and
tests remain unverified. Evidence: artifacts/consolidation-mcp-transport/.
The full workspace consolidation goal remains active.

## MCP request correlation ownership

MCP-transport::rpc owns a connection's request-ID counter, pending response
channels, request timeouts/errors, notifications and response delivery. The
application retains receive-loop dispatch, server registration, initialization
identity and permission checks. Its transport accessor borrows the same Arc
allocation without cloning; constructor state and per-client counter scope
are unchanged. Timeout and cancellation cleanup behavior was not altered.

Ten static checks compare original request/notification/response operations,
full client adaptation, subprocess approval helper, state initialization and
transport borrowing. Lexical graphs cover 21 transport files and 35
application files with no missing/unreferenced modules. Tokio's existing time
feature is now explicit. New files have at most 29 code lines. Formatting,
file-limit and whitespace checks passed; no connection, request, subprocess,
Cargo, rustc, Python or tests ran. Compilation and tests remain unverified.
Evidence: artifacts/consolidation-mcp-rpc/. The workspace goal remains active.

## Shared MCP remote protocol operations

RpcConnection now owns request construction and response decoding for
initialization, tool discovery/invocation, resources and prompts. The application
still supplies its package version, updates cached metadata, sends initialized
notifications, logs connection/tool discovery and refreshes tools in the original
order. Remote tool calls remain gated before approval metadata is stripped and
the shared request is invoked. No authorization moved out of the host.

Twelve static comparisons cover request bodies/decoding and the complete client
adaptation, including initialization completion, registry behavior and tool
policy ordering. Lexical graphs cover 26 transport and 35 application
files with no missing/unreferenced modules. New modules have at most 24 code
lines. No dependencies changed. Formatting, source limits and staged whitespace
checks were used. No requests, transports, Cargo, rustc, Python or tests ran;
compilation and tests remain unverified. Evidence resides in
artifacts/consolidation-mcp-operations/. The workspace goal remains active.

## MCP registry and tool routing

MCP-core::registry owns generic client storage, capability metadata, cached
tool lookup, fallback search, refresh, dispatch and disconnect sequencing.
The application registry wraps it for subprocess authorization and connection
startup. RegistryClient delegates tool calls to the existing policy-aware
application client, preserving authorization before remote requests.

Twenty static comparisons cover all registry methods, field shapes, the
unchanged application client methods, connection order and factory delegation.
Lock order and logging targets are preserved. Lexical graphs cover 64 shared
core and 37 application files without missing/unreferenced modules.
New files have at most 32 code lines. Only existing Tokio/tracing dependencies
were added. Formatting, file limits and whitespace checks were used. No
registry, subprocess, request, Cargo, rustc, Python or tests ran. Compilation
and tests remain unverified. Evidence: artifacts/consolidation-mcp-registry/.
The full workspace consolidation goal remains active.

## Shared MCP server catalog

MCP-core::server_catalog owns registered tool/resource/prompt handlers and
metadata. Registration, metadata lookup, protocol listings, direct tool lookup
and resource reads retain original locking, clone and handler-call order. The
application server forwards catalog calls and borrows the same tool-map lock
for its bus-aware execution path. Bus events, default-tool setup, request
dispatch, initialization identity and authorization remain in the application.

Fifty-one static comparisons cover moved methods, handler aliases, forwarding
arguments, empty map construction and unchanged host behavior. Lexical graphs
cover 75 core and 37 application files without missing/unreferenced
modules. New files have at most 35 code lines. No dependencies changed.
Formatting, file-limit and staged whitespace checks were used. No handler,
registry, transport, Cargo, rustc, Python or test executed. Compilation and
tests remain unverified. Evidence: artifacts/consolidation-mcp-catalog/.
The full workspace goal remains active.

## MCP server protocol responses

MCP-core::server_protocol now owns initialization response construction,
built-in prompt metadata and rendering, and tool-result wire normalization.
Separate modules hold initialization, prompt dispatch, each prompt template,
and tool-result conversion. Application server methods retain handler execution,
read-lock lifetime, bus request/response publication and approval integration.
The capability callback is evaluated only after initialization parameters parse.

All 39 static comparisons passed: exact template strings, error and default
behavior, response conversion, host forwarding and unaffected server methods.
Lexical module scans cover 81 core and 37 application files with no missing or
orphaned modules; the four-crate local dependency closure has no detected cycle.
All six new files have at most 40 code lines. The changed-source line ratchet
and diff whitespace check passed. Standalone rustfmt was used. No Cargo, rustc,
Python, application runtime, Rust tests or doctests ran; compilation and runtime
behavior remain unverified. Evidence: artifacts/consolidation-mcp-server-protocol/.
The whole-workspace goal remains active.

## MCP bus adapter module ownership

Application server/bus now owns bus-aware tool and resource registration.
Event query, bridge status and Ralph status each have a registration module
and an execution handler module; recent-event, PRD and progress resources
each have their own module. The parent server retains bridge attachment and
delegates registration. These are application adapters because their live bus
bridge dependency belongs to the application, not to shared MCP wire logic.

All 39 static comparisons passed, covering complete reconstructed registration
bodies, schemas, handler captures, reading and output behavior, logging,
registration order and unaffected server methods. The verifier accounts for
standalone rustfmt removing redundant braces from one filter closure.
Lexical scans cover 47 application MCP files and 81 shared-core files without
missing or orphaned modules. All ten new modules are at most 41 code lines.
The changed-source file-limit ratchet and diff whitespace checks passed.
No Cargo, rustc, Python, bus handlers, application runtime or Rust tests ran;
compilation and runtime behavior remain unverified. Evidence is retained in
artifacts/consolidation-mcp-bus-modules/. The workspace goal remains active.

## MCP server host and fallback module cohesion

The application server root now holds only state and module declarations
(34 code lines). Construction, attachment, tool/resource/prompt catalog
facades, transport loop, setup selection, application registry adaptation,
request dispatch, shared protocol forwarding and tool invocation have distinct
modules. Every server file, including bus adapters, is at most 47 code lines.
Fallback registration has one module per schema plus separate read-file,
write-file and directory-list handlers. Existing command/search handlers
remain at their application policy boundaries.

All 39 static comparisons passed. They account for every original method,
public signature, state field, relocated path and log target; reconstruct all
six fallback registrations and complete handler bodies; and check registration
order and file-size limits. Internal methods use parent-visible access to
preserve calls across their new sibling modules. Lexical scans found all 68
application MCP files and 81 shared-core files reachable without missing
modules. The changed-source line ratchet and whitespace checks passed.
Standalone rustfmt was used. No Cargo, rustc, Python, filesystem tool handler,
application runtime, Rust test or doctest ran. Compilation and runtime remain
unverified. Evidence: artifacts/consolidation-mcp-server-modules/. The broader
workspace objective remains active; client and bridge ownership still need audit.

## HTTP bus bridge ownership

The new codetether-bus-http crate owns authenticated SSE ingestion, bounded
event retention, bridge status, worker/workspace endpoint discovery and local
workspace matching. Its 21 modules separate transport, reconnect policy, line
parsing, buffer mutation/query, resolver operations and matching rules. The
application MCP bus_bridge path re-exports the existing public API. This adapter
depends on shared bus envelopes without coupling HTTP discovery to MCP wire types.

All 30 static comparisons passed: original method and function bodies, public
signatures, state layouts and serialization, constants, original test source,
and the one extracted active-worker filter. HTTP bearer-token use, response
validation, reconnect timing, SSE line framing, eviction and event order remain
unchanged. Internal field/method visibility allows sibling implementation modules.
Old log targets are explicit. Lexical scans found all 21 new crate and 68 MCP
application files reachable. The five-crate local dependency closure has no
detected cycle. New files have at most 44 code lines.

The workspace manifest and lockfile were edited for the new local crate.
The lockfile adds only its package entry and the application dependency, using
the already locked reqwest 0.13.4. No dependency resolution ran. File-limit and
diff whitespace checks passed; standalone rustfmt formatted moved Rust. No Cargo,
rustc, Python, network request, credential lookup, background task, Rust test or
doctest ran. Compilation and runtime remain unverified. Evidence is retained in
artifacts/consolidation-bus-http/. The whole-workspace goal remains active.

## Remote MCP client session ownership

MCP-transport::client::RemoteClient owns the correlated connection, server
metadata, name and tool cache, initialization, incoming message handling,
resource/prompt forwarding and transport closure. Ten focused modules separate
these responsibilities. The application McpClient retains registry identity,
subprocess authorization, outgoing tool policy and package-version selection.
Its root and five implementation modules now form a small host adapter.

All 38 static comparisons passed, covering moved methods and lock operations,
constructor state, explicit initialization version, log targets, complete host
forwarding, registry methods and policy-before-execution order. Sampling remains
the existing unsupported response; notification refresh behavior is preserved.
All new modules and the client root are at most 45 code lines. Lexical scans
cover all 36 transport and 73 application MCP files with no missing or orphaned
modules; the five-crate local dependency closure has no detected cycle.
File-limit and whitespace checks passed. Standalone rustfmt formatted the Rust.
No Cargo, rustc, Python, subprocess, peer connection, Rust test or doctest ran;
compilation and runtime remain unverified. Evidence is retained in
artifacts/consolidation-mcp-remote-client/. The full workspace goal remains active.

## Native search backend ownership

Codetether-search::native owns filename glob matching, content-query regex
construction, path collection and matching-line rendering. The seven new
modules include the two pre-existing regression tests. MCP keeps borrowed-JSON
compatibility entrypoints and wire-response construction. The native backend
has no MCP dependency and can be reused independently of MCP registration.

All 10 static comparisons passed: complete original search/matcher/traversal
function bodies, unchanged output and error behavior, compatibility forwarding,
and the two tests with only their call paths changed. New modules are at most
30 code lines. Lexical scans cover all 18 search-crate and 71 application MCP
files with no missing or orphaned modules and no cycle in the four-crate local
dependency closure. The manifest and lockfile add already-locked glob, regex,
walkdir and the tempfile dev dependency to the search crate; no dependency
resolution ran. File-limit and whitespace checks passed. Standalone rustfmt
was used. No Cargo, rustc, Python, native search execution or Rust tests ran;
compilation and runtime remain unverified. Evidence is retained in
artifacts/consolidation-native-search/. The whole-workspace goal remains active.

## Worker HTTP service ownership

Codetether-worker-http owns WorkerServerState and the HTTP service: health and
readiness probes, worker status, task CloudEvent ingress, bus publish and SSE
subscription. Its 18 modules separate state construction, task notification,
heartbeat and identity updates, routing, payloads and endpoint behavior. The
application worker_server API remains compatible and only adapts CLI hostname
and port to the shared serve function. Shared bus and worker-core types remain
the same underlying types through the existing application re-exports.

All 31 static comparisons passed, including state methods, state/wire layouts,
route setup, response codes, task-channel readiness, CloudEvent header parsing,
SSE filtering, keepalive and lag records, and explicit original logging targets.
Every new file is at most 46 code lines. Lexical scans found all 18 modules
reachable and no missing files or dependency cycle in the six-crate local
closure. The lockfile adds only the local worker-http package and its application
reference; dependency resolution was not run. Changed-source line limits and
whitespace checks passed; standalone rustfmt formatted the extraction.

No Cargo, rustc, Python, listener, HTTP request, task delivery, Rust test or
doctest ran. Compilation and runtime behavior remain unverified. Evidence is
retained in artifacts/consolidation-worker-http/. The complete workspace
consolidation goal remains active.

## Forage execution evidence and rubric loading

Forage-core::execution owns the execution outcome record, sorted Git-snapshot
comparison and quality-result formatting. Application execution preserves the
snapshot-before-engine, snapshot-after-engine and policy-gated quality call
order. The shared helper accepts BTreeSet snapshots, matching the existing Git
snapshot API. Forage-core::rubric_load owns optional file reading, JSON/list
normalization, first-seen case-insensitive deduplication and alignment clamping.
The CLI wrapper passes explicit goals, file path, required flag and threshold.

All 32 static comparisons passed, including complete unchanged application
functions and test bodies, loader logic, evidence construction, ordering and
error text. All five new files are at most 50 code lines. Lexical scans found
all 34 shared-core and 13 application files reachable, without missing modules
or a cycle in the three-crate local dependency closure. Manifest/lock changes
add already-locked serde_json and Tokio filesystem support; no dependency
resolution ran. File-limit and whitespace checks passed.

Standalone rustfmt was used. No Cargo, rustc, Python, execution engine, quality
command, rubric file read, Rust test or doctest ran; compilation and runtime
remain unverified. Evidence: artifacts/consolidation-forage-evidence/. The
whole-workspace goal remains active, including the forage application loop.

## Forage application adapter modules

Eight focused application modules now own S3 task health, progress translation,
engine dispatch and evidence collection, swarm execution, swarm configuration,
audit initialization/publication, bus observation and opportunity assembly.
Their application service dependencies remain in src; the existing shared
forage-core owns domain selection, evidence and rubric logic. The root retains
the main loop and Go engine, which still need their own cohesion work.

All 27 static comparisons passed, including each complete relocated function
and the unchanged loop, Go engine and test-module bodies. Each relocated
function has one implementation and preserves its signature except parent-only
visibility. Original tracing targets are explicit. The eight modules are at
most 42 code lines. Lexical scans cover all 21 application and 34 core files
without missing modules; the three-crate dependency closure is unchanged.
File-limit and whitespace checks passed. Standalone rustfmt was used.
No Cargo, rustc, Python, engine, S3 task, audit initialization, bus drain, Rust
test or doctest was executed. Compilation and runtime behavior remain
unverified. Evidence: artifacts/consolidation-forage-adapters/. The broader
workspace goal remains active.

## Forage Go-engine adapter cohesion

The application go_exec modules separate provider resolution, objective/run
preparation, Ralph invocation and result rendering. ProviderRegistry and
OkrRepository are returned to named executor bindings so they remain owned
through execution. The default model, approval construction, timeout bounds,
iteration/concurrency arguments, failure status and output text are preserved.

All 25 static comparisons passed, including helper bodies, assembled execution
flow, named owner bindings, the registry provider return type and unchanged
root functions/test module. The verifier accounts for standalone rustfmt
collapsing a single-expression success arm. All four new files are at most 40
code lines. Lexical scans cover 25 application and 34 shared-core files with
no missing modules; the three-crate dependency closure remains acyclic.
File-limit and whitespace checks passed. No manifests changed.

No Cargo, rustc, Python, provider resolution, Vault access, Ralph execution,
Rust test or doctest ran; compilation and runtime behavior remain unverified.
Evidence: artifacts/consolidation-forage-go/. The main forage loop and its
remaining cohesion work are still part of the active workspace objective.

## Forage lifecycle reporting cohesion

The application events module now owns startup, scan, selection, console/JSON
presentation, cycle audit, execution start/success/failure, sleep and shutdown
reporting. Its RunEvents adapter borrows existing arguments, rubric and bus
handle. The loop retains original counters, selection, execution, progress
persistence, fail-fast decisions and sleep operations. No shared owner or
additional bus handle was introduced.

All 32 static comparisons passed, covering complete reporting blocks and the
loop with only equivalent method calls substituted, plus unchanged functions
and test source. The verifier accounts for cloning a borrowed selection slice
and omitting redundant references when forwarding borrowed strings/slices.
All 12 new files are at most 36 code lines. Lexical scans cover 37 application
and 34 core files without missing modules; dependency closure remains unchanged.
File-limit and whitespace checks passed. Standalone rustfmt was used.
No Cargo, rustc, Python, event emission, audit write, engine, Rust test or doctest
ran; compilation and runtime remain unverified. Evidence is retained in
artifacts/consolidation-forage-events/. Loop orchestration and the residual
test module remain pending within the active whole-workspace objective.

## Forage discovery and execution phases

The application loop now delegates work discovery, selected-item execution
and successful-evidence persistence to dedicated modules. Discovery retains
empty-queue seeding, interactive clarification, rebuilding and top-N selection.
Execution retains the execute flag, item order, counters and fail-fast behavior.
Success handling retains nonfatal progress-write errors before completion
reporting. Startup, cycle boundaries and sleeping remain in lifecycle assembly.

All 24 static comparisons passed, including full extracted blocks, their
ordered call sites and unchanged functions/test module. New files are at most
39 code lines. Lexical scans found all 40 application and 34 core modules
reachable without missing paths or dependency-cycle changes. File-limit and
whitespace checks passed; standalone rustfmt was used. No Cargo, rustc, Python,
queue scan, clarification, execution engine, progress write, Rust test or
doctest ran. Compilation and runtime remain unverified. Evidence:
artifacts/consolidation-forage-phases/. Lifecycle assembly and the residual
test module still need work under the active whole-workspace objective.

## Forage lifecycle assembly

The application startup module owns repository/rubric loading, seeding, global
bus setup and service handles. The runtime module borrows these owners and
separates initial counters, per-cycle work and loop control. Optional S3 startup
has its own adapter. No extra bus/owner clones or ownership transfers were
introduced; original owners stay alive around the awaited loop.

All 29 static comparisons passed, including startup ordering, the borrowed
context fields, per-cycle operations, S3 health timing, cycle bounds, sleeping,
normal shutdown and unchanged test module. All six new lifecycle files are at
most 36 code lines. Lexical scans found all 46 application and 34 core modules
reachable. The three-crate dependency closure is unchanged and acyclic.
File-limit and whitespace checks passed; standalone rustfmt was used.

No Cargo, rustc, Python, service startup, cycle execution, Rust test or doctest
ran. Compilation and runtime remain unverified. Evidence is retained in
artifacts/consolidation-forage-runtime/. Residual test-module cohesion and a
final forage ownership audit remain within the active whole-workspace goal.

## Forage test ownership and fixture consolidation

Eight existing scoring, prompt normalization, progress-ratio and seeding tests
now reside with forage-core. Eight application selection/configuration/progress
adapter tests remain in focused application modules. Repeated CLI argument
and opportunity construction is captured in two test fixtures. Runtime imports
now live with their using modules. The application forage root is 48 code lines;
every forage source file is within the 50-code-line limit.

All 34 static comparisons passed, including the original sixteen test bodies
with fixture calls expanded, fixture field values, and unchanged operational
code after import relocation. No assertions were removed and no test ran.
Lexical scans cover all 57 application and 43 core files without missing or
orphaned modules; the three-crate dependency closure remains acyclic. The
manifest adds existing tempfile as a dev dependency and Tokio test runtime
features; the lockfile only adds tempfile to forage-core. No resolution ran.
Changed-file limits and whitespace checks passed; standalone rustfmt was used.

No Cargo, rustc, Python, fixture execution, Rust tests or doctests ran;
compilation and runtime remain unverified. Evidence:
artifacts/consolidation-forage-tests/. Clarification/persistence ownership
review remains before the final forage audit; the full workspace goal is active.

## Forage clarified goals and ownership review

Forage-core::seed::persist_clarified_goals now owns objective construction and
repository persistence for interactively supplied goals. Application code
retains stdin/TTY behavior, quiet-mode handling and the success message after
a successful write. Objective titles, status, key-result values and write
ordering are unchanged by static source comparison. Existing clarification
tests are unchanged and unrun.

The ownership review classifies every remaining application forage file:
startup and borrowed cycle lifecycle; provider/CLI/swarm execution; guarded
shell and optional scoring adapters; bus/audit/terminal reporting; interactive
input; and application integration tests. Reusable selection, seeding,
progress persistence, evidence, quality workflow and rubric loading belong to
forage-core. All 57 application files remain within 50 code lines. The detailed
classification and source hashes are retained in the ownership evidence.

Six static checks passed. Lexical scans found all 57 application and 44 core
files reachable, without missing modules or a cycle in the three-crate local
dependency closure. File-limit and whitespace checks passed; no manifests
changed. Standalone rustfmt formatted the move. No Cargo, rustc, Python, goal
input, persistence operation, Rust test or doctest ran. Compilation and runtime
remain unverified. Evidence: artifacts/consolidation-forage-clarified/. This
finishes the forage ownership review, not the whole-workspace objective, which
remains active across the other pending source owners.

## Ralph parallel quality-repair sequencing

Ralph-core::quality_repair owns the evolving quality state, initial model-result
handling and bounded check/verify/repair loop. Its RepairHost interface delegates
checks, verification, repair execution and exhaustion reporting to application
adapters. The existing verification/repair methods are unchanged; the host
implementation forwards to them and to the existing policy-gated quality shell.

All 13 static comparisons passed, covering inclusive attempt bounds, callback
order, short-circuit exits, initial errors, disabled-quality verification, state
layout, application forwarding and unchanged effectful host methods. Five new
core modules and one application host file are at most 39 code lines. Lexical
scans found all 111 core and 101 application Ralph files reachable, without
missing modules or a cycle in the five-crate local dependency closure. No
manifest changes were required. File-limit and whitespace checks passed;
standalone rustfmt formatted the move.

No Cargo, rustc, Python, quality command, verification command, repair agent,
Rust test or doctest ran. Compilation and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-quality-repair/. The full workspace objective and
the remaining Ralph ownership audit stay active.

## Ralph progress record construction

Ralph-core::progress now owns completed and failed ProgressEntry construction
and the parallel story response-write/record adapter. Completed entries accept
a learning-extraction callback, preserving the existing sequential versus static
extractors and timestamp placement. The application retains sequential state
mutation, verification, event publication and completion decisions. The unused
application static-extractor facade was removed after caller inspection.

All 11 static comparisons passed: complete entry fields, learning extraction,
timestamps, parallel file-write placement and ignored errors, full original
parallel_result tracing target, and unchanged sequential flow. New modules are
at most 26 code lines. Lexical scans cover all 114 core and 101 application
files without missing modules or dependency-cycle changes. No dependencies
changed. File-limit and whitespace checks passed; standalone rustfmt was used.
No Cargo, rustc, Python, progress write, model call, Rust test or doctest ran.
Compilation and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-progress-records/. The broader goal and remaining
Ralph execution ownership audit remain active.

## Ralph quality evidence ownership

Ralph-core now owns verification outcome application, nonempty failure-reason
updates and repair-output accumulation alongside its retry state machine.
Application code still supplies the original verification future and success
log callback, preserving the existing policy-aware verifier and error text.
Repair execution and exhausted-retry reporting remain host operations.

All nine static comparisons passed: verification success/failure field updates,
diagnostic assignment order, log placement, repair formatting and host command
invocation paths. New files are at most 22 code lines. Lexical scans cover all
117 core and 101 application files without missing modules or dependency-cycle
changes. No manifest changes were required. File-limit and whitespace checks
passed; standalone rustfmt was used. No Cargo, rustc, Python, verification,
repair execution, Rust test or doctest ran; compilation and runtime remain
unverified. Evidence: artifacts/consolidation-ralph-quality-state/. Remaining
Ralph execution ownership and the whole-workspace objective stay active.

## Ralph scheduling ownership

Ralph-core::scheduling now owns the sequential iteration loop and dependency-
stage scheduling. SequentialHost supplies live state, iteration selection,
prompt generation, model invocation and response/error handling. ParallelHost
supplies stage planning, optional manager construction, limit checks and full
stage execution. The application stage callback keeps task handles and its
semaphore alive through collection and finalization, while the shared loop
retains the single manager across stages without additional cloning.

The top-level application run method moved into its own lifecycle module with
mode selection, final status, cleanup and finish order unchanged. All 18 static
comparisons passed, covering the two loops, callback forwarding, entire stage
execution block, application state layout and lifecycle. Eight new files are
at most 32 code lines. Lexical scans found all 122 core and 104 application files
reachable, without missing modules or dependency-cycle changes. No dependencies
changed. File-limit and whitespace checks passed; standalone rustfmt was used.
No Cargo, rustc, Python, scheduler run, model call, worktree operation, Rust test
or doctest ran. Compilation and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-scheduling/. The broader goal and remaining Ralph
host ownership audit stay active.

## Ralph background store update ownership

Ralph-core::state_store::updates owns optional-store checks, snapshot capture
and nonblocking initial-run, story-result, iteration, checkpoint and completion
updates. Application lifecycle methods select when each update is scheduled.
All captured values retain the original clone and timestamp order, and the
background error log uses the original persistence-module target.

All 14 static comparisons passed, including all five store blocks, their
application call positions, background task behavior and declared Tokio runtime
support. The seven new files are at most 27 code lines. Lexical scans found all
129 core and 104 application files reachable with no dependency-cycle changes.
The existing Tokio dependency now explicitly enables rt for shared spawning;
no packages or lockfile entries changed. File-limit and whitespace checks
passed; standalone rustfmt was used. No Cargo, rustc, Python, background task,
store write, Rust test or doctest ran; compilation and runtime remain unverified.
Evidence: artifacts/consolidation-ralph-store-updates/. The broader workspace
goal and remaining Ralph host ownership audit remain active.

## Ralph relay delegation policy

Ralph-core::delegation now owns stable role keys, role-to-skill/difficulty/tool
use mapping and candidate ranking. The application keeps ProviderRegistry
access and passes a lazy callback. Disabled delegation returns its original
defaults before candidate listing, and enabled selection retains the default
model. The selected-provider log keeps its original application target.

All 11 static comparisons passed, including role constants, mapping functions,
full ranking control flow and application forwarding. The existing session
delegation API re-exports the same context-policy types used directly by the
shared policy. Six new modules are at most 27 code lines. Lexical scans cover
135 core and 104 application files without missing modules; the six-crate
local dependency closure is acyclic. The manifest/lockfile add the existing
context-policy crate as a direct dependency; no resolution ran. File-limit and
whitespace checks passed; standalone rustfmt was used. No Cargo, rustc, Python,
provider lookup, ranking execution, Rust test or doctest ran. Compilation and
runtime remain unverified. Evidence: artifacts/consolidation-ralph-delegation/.
The complete workspace objective and remaining Ralph host review remain active.

## Ralph persisted-state initialization

Ralph-core::lifecycle::load_initial_state owns PRD loading, working-directory
resolution and pending-state construction with a resumed iteration count.
A callback preserves the original loaded-PRD log position. The application
constructor retains provider/model/config ownership, integration defaults and
UUID generation after successful state loading.

Five static comparisons passed, covering the complete loading block, path
branches, callback timing, owned PRD path and unchanged constructor tail. The
new file has 30 code lines. Lexical scans cover 136 core and 104 application
files without missing modules or dependency-cycle changes. No dependencies
changed. File-limit and whitespace checks passed; standalone rustfmt was used.
No Cargo, rustc, Python, PRD file read, Rust test or doctest ran. Compilation
and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-initial-state/. Remaining host ownership and the
full workspace goal remain active.

## Ralph parallel host module cohesion

The parallel host now groups its driver, stage planning, worktree preparation,
task context, quality callbacks, result collection and scheduler adapter under
ralph_loop::parallel. Thirty-one existing files moved beneath this owner. Only
merge callers outside the module access its result-context module; their two
paths were updated. Inherent method/type/field visibility retains the original
ralph_loop scope, and relocated log calls have explicit original module targets.

All 36 static comparisons passed, covering every relocated file, the two merge
imports, unchanged parent state and remaining declarations. All new module files
are at most 47 code lines. Lexical scans cover 105 application and 136 core files
without missing or orphaned modules; the six-crate dependency closure is
unchanged and acyclic. No manifests changed. File-limit and whitespace checks
passed; standalone rustfmt was used. No Cargo, rustc, Python, parallel task,
worktree operation, Rust test or doctest ran. Compilation and runtime remain
unverified. Evidence: artifacts/consolidation-ralph-parallel-modules/. Relay and
lifecycle layout still need review under the active whole-workspace objective.

## Ralph relay and sequential host cohesion

Twenty-two existing implementation files now sit under dedicated relay and
sequential modules. Relay owns team/session preparation, baton turns, event
forwarding, convergence and finishing. Sequential owns iteration selection,
prompting, response/error handling and its shared-scheduler adapter. Existing
inherent visibility remains restricted to the original ralph_loop scope, and
all relocated logging retains its original target.

All 25 static comparisons passed, covering every moved file and the parent
state/declarations. All grouped files are at most 47 code lines. Lexical scans
cover 107 application and 136 core files without missing or orphaned modules.
The six-crate dependency closure is unchanged and acyclic. No dependencies
changed. File-limit and whitespace checks passed; standalone rustfmt was used.
No Cargo, rustc, Python, relay session, sequential agent, Rust test or doctest
ran. Compilation and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-relay-sequential/. Lifecycle/merge layout and final
Ralph ownership review remain part of the active whole-workspace objective.

## Ralph lifecycle, model, workspace and reporting groups

Thirty-five remaining flat implementation files are grouped by application
responsibility: lifecycle/configuration/persistence, provider/tool execution,
workspace commits/merges/conflicts, and bus/event reporting. Existing method
visibility retains the original ralph_loop scope. Workspace merge imports
point explicitly to the sibling parallel result context. Original log targets
are explicit for moved macros. The Ralph loop root is now 47 code lines.

All 38 static comparisons passed, including each relocated file, root state,
public template API and declarations. Every moved/new module is at most 49
code lines. Lexical scans cover all 111 application and 136 core files without
missing or orphaned modules; the six-crate dependency closure is unchanged.
File-limit and whitespace checks passed. Standalone rustfmt was used. No
Cargo, rustc, Python, model call, merge, runtime operation, Rust test or doctest
ran; compilation and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-host-groups/. A final Ralph residual ownership
review remains within the broader active workspace objective.

## Ralph event projection ownership

Ralph-core::events now builds initial story presentation records and maps
shared SwarmEvent payloads into RalphEvent values. The application keeps
channel creation, forwarding and close handling. Unsupported swarm variants
still skip forwarding; supported variants retain their payload and story ID.
The core directly declares its existing shared swarm-contract dependency.

Seven static comparisons passed: initial records, forwarding signature,
nonblocking event sends, payload mapping, unsupported variants, channel
capacity/clone order and the narrow lockfile addition. The two new modules
are at most 27 code lines. Lexical scans cover 138 core and 111 application
files without missing modules or a cycle in the seven-crate local dependency
closure. The lockfile only adds swarm-core to Ralph-core; no resolution ran.
File-limit and whitespace checks passed; standalone rustfmt was used.
No Cargo, rustc, Python, channel execution, Rust test or doctest ran; compilation
and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-event-mapping/. The final Ralph ownership review
and full workspace objective remain active.

## Ralph relay convergence and predecessor policy

Ralph-core::relay now owns relay progress, repeated-output convergence,
terminal-marker handling, handoff composition and predecessor selection.
The host still owns ordered sessions, event forwarding, turns and shutdown.
Unused application text wrappers were removed; shared presentation helpers
continue to provide the same normalization and handoff text.

Eight static comparisons passed, including state layout, complete observation
logic, predecessor indexing, unchanged round execution and shutdown. The three
new files are at most 25 code lines. Lexical scans found 141 core and 108
application files reachable without missing modules or dependency changes.
File-limit and whitespace checks passed; standalone rustfmt was used. No
Cargo, rustc, Python, relay session, Rust test or doctest ran. Compilation and
runtime remain unverified. Evidence: artifacts/consolidation-ralph-relay-policy/.
Remaining host ownership review and the full workspace goal remain active.

## Ralph bus learning and result protocol

Ralph-core::bus owns learning collection and story-result publication through
a caller-provided shared bus. The application retains its optional-bus guard
and passes existing Arc references. Collector identity, drain order, learning
format, handoff conditions and learning/handoff/progress publication order are
unchanged. No extra bus clones were introduced.

Seven static comparisons passed, including complete helper bodies, guarded
application forwarding and the Arc receiver required by AgentBus::handle.
Three new modules are at most 43 code lines. Lexical scans cover 144 core and
108 application files without missing modules or a cycle in the nine-crate
local dependency closure. The manifest/lockfile add the existing bus-core
dependency only; no resolution ran. File-limit and whitespace checks passed.
Standalone rustfmt was used. No Cargo, rustc, Python, bus publication or drain,
Rust test or doctest ran. Compilation and runtime remain unverified. Evidence:
artifacts/consolidation-ralph-bus/. Remaining Ralph host ownership review and
the full workspace goal stay active.

## Ralph outcome policy and source ownership review

Shared outcome helpers now own conflict success-marker interpretation and
parallel-story fallback failure text. Shared scheduling supplies owned PRD
stages. The application retains logging, event publication, worktree retention
and stage execution. Static comparisons preserve the original predicates,
explicit-reason precedence, stage order and surrounding host operations.

The source ownership review accounts for all 108 remaining application files:
compatibility exports; lifecycle/dependency assembly; provider and tool calls;
relay sessions; parallel/sequential host callbacks; policy-gated verification
and Git operations; live reporting; and runtime integration tests. All are at
most 49 code lines. The per-file classification and hashes are retained in the
ownership evidence. Shared policy and persistence responsibilities are in
ralph-core rather than duplicated in these adapters.

Eight static checks passed. Lexical scans cover all 148 core and 108 application
files without missing modules or cycles in the nine-crate local dependency
closure. No dependencies changed. File-limit and whitespace checks passed;
standalone rustfmt was used. No Cargo, rustc, Python, conflict handling, stage
execution, Rust test or doctest ran; compilation and runtime remain unverified.
Evidence: artifacts/consolidation-ralph-outcome-policy/. This completes Ralph
source-ownership review, not the full goal: tools, TUI, session, A2A, CLI, HTTP
server, mux and other pending owners still require workspace consolidation.


### A2A HTTP client extraction

`codetether-a2a-http` owns HTTP discovery and JSON-RPC client operations in six
modules covering construction, discovery, messages, tasks, and transport.
`src/a2a/client.rs` retains the original public path through a compatibility
re-export. Wire types remain owned by `codetether-a2a-core`.

Static token comparison confirms all seven method implementations and the
client fields are unchanged. All six modules are reachable, their local
dependency closure is acyclic, and each source file has at most 50 code lines.
The source line-budget and whitespace checks passed. Evidence is retained in
`artifacts/consolidation-a2a-http/`. Compilation, tests, and runtime behavior
remain unverified: Cargo, rustc, and runtime checks were not run.
The broader consolidation goal remains active.


### Shared A2A task-routing scope

`codetether-a2a-worker-core::task_scope` owns task-intake targeting policy.
Envelope field lookup and ordered agent/worker/workspace checks have separate
modules; the existing application module re-exports `check_task_scope`.
Metadata precedence, the first-rejection order, global/empty-workspace behavior,
and rejection messages are preserved. The existing 11 test functions moved
unchanged into three focused test modules. No dependencies were added.

Static comparison expands the extracted lookup expression and verifies the
original policy tokens, metadata lookup, and every moved test function. The
worker-core module graph reaches all 36 source files without missing modules;
all six new files are below 50 code lines. File-budget and whitespace checks
passed. Evidence is in `artifacts/consolidation-a2a-scope/`. Tests and compilation
were not run under the Cargo/rustc restriction. The overall goal remains active.


### Shared resumable A2A stream and QUIC transport

`codetether-a2a-stream` owns event IDs, frame parsing, persisted cursors,
resume headers, resync payloads, bounded staging, reconnect policy, transport
probes, and QUIC clients/servers/readers/writers. Existing application module
paths re-export these modules. TCP observations remain in telemetry and socket
defaults remain in provider-core; neither implementation was duplicated.
The new crate depends only on telemetry among workspace crates. Its FIPS
feature forwards the existing rustls/QUIC flags and is enabled by the
application FIPS feature. Worker transport selection remains in the host.

Embedded regression cases were separated into focused test modules, preserving
all 23 test functions. Source-token comparisons cover all 24 relocated files,
allowing only the two owner-import updates and test-module declarations. All
33 crate files and three application compatibility files are reachable; the
local dependency graph is acyclic. Every new source file stays below 50 code
lines. Evidence is retained in `artifacts/consolidation-a2a-stream/`.
Compilation, tests, doctests, and transport behavior remain unverified under
the Cargo/rustc restriction. The broader consolidation remains active.


### Shared A2A peer discovery

`codetether-a2a-discovery` owns mDNS advertisement and browsing, resolved peer
records, interface bounds, hostname normalization, address ranking, and peer
sighting/expiry state. These responsibilities are organized into `mdns`,
`address`, and `liveness` modules. It has no workspace-crate dependencies.
The application retains bus/route expiry orchestration and forwards its
package version through the existing five-argument announcement API so the
TXT record still identifies the application version. Original log targets
are explicit in relocated code. Existing public paths remain available.

Static comparison covers 17 relocated source files and 13 unchanged test
functions. All 19 crate files are reachable, and each stays below 50 code
lines. The only lockfile additions are the local crate and its application
dependency. Evidence is retained in `artifacts/consolidation-a2a-discovery/`.
Compilation, tests, doctests, and live discovery were not run. The wider
consolidation goal remains active.
