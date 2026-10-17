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
| tool | 1237 | 45630 | Shared contracts/results/adapters extracted; concrete tools and registry composition remain |
| tui | 1208 | 42457 | Pending cohesion and dependency audit |
| session | 952 | 31933 | Partial extraction; inspect remaining implementation |
| provider | 141 | 3096 | Shared contracts/HTTP/images/context limits, Codex OAuth/lifecycle/request, message and response protocol/HTTP and WebSocket transport, and Google/Bedrock auth, transcript conversion, body policy, native requests, responses, streaming, and Bedrock execution/discovery/refresh and OpenAI/DeepSeek/Z.AI/Anthropic/Moonshot/Google/StepFun/OpenRouter/Vertex Anthropic/Vertex GLM/GLM-5/Copilot implementations extracted; other adapters/registry pending |
| a2a | 356 | 13447 | Pending cohesion and dependency audit |
| cli | 198 | 10322 | Pending cohesion and dependency audit |
| swarm | 289 | 10287 | Pending cohesion and dependency audit |
| server | 213 | 8264 | Pending cohesion and dependency audit |
| mux | 252 | 7209 | Pending cohesion and dependency audit |
| cognition | 201 | 6795 | Pending cohesion and dependency audit |
| mcp | 36 | 3340 | Pending cohesion and dependency audit |
| ralph | 57 | 2366 | Shared services extracted; lifecycle, sequential scheduling, conflict and relay coordinator/turn adapters modularized; parallel scheduling and direct model-call adapters remain pending |
| bus | 38 | 2783 | Pending cohesion and dependency audit |
| lsp | 39 | 1200 | Protocol core and response decoding extracted; client orchestration remains pending |
| rlm | 16 | 185 | Ownership audited: shared execution extracted; only compatibility APIs, local-router setup and provider/session-bus adapters remain |
| config | 1 | 1 | Configuration crate compatibility facade |
| forage | 13 | 1918 | Pending cohesion and dependency audit |
| runtime_policy | 56 | 1774 | Pending cohesion and dependency audit |
| main | 1 | 1590 | Entrypoint/adapter; enforce thin boundary |
| telemetry | 5 | 48 | Core extracted; thin application identity/startup adapters |
| indexer | 8 | 146 | Core extracted; CLI and provider-resolution adapters retained |
| agent | 33 | 662 | Catalog/contracts/prompts extracted; execution, tool policy, session and swarm integration remain |
| platform | 1 | 1 | Native services extracted; Linux application and GNU/MSVC Windows platform type checks passed |
| approval | 15 | 469 | Ledger extracted; live/session adapters retained |
| autochat | 6 | 875 | Pending cohesion and dependency audit |
| vectordb | 13 | 409 | Core extracted; provider/backend adapters retained |
| benchmark | 12 | 303 | Ownership audited: shared services and suite scheduling extracted; remaining modules translate Ralph/provider/telemetry state and retain runtime ownership |
| crash | 1 | 10 | Shared crash service extracted; adapter supplies application version and memory-guard setup |
| github_pr | 10 | 120 | Shared PR policy/context/HTTP extracted; CLI body input, A2A credential acquisition and output adapters retained |
| moltbook | 1 | 1 | Client, wire types, HTTP helpers, credentials and content helpers extracted; compatibility facade retained |
| review | 11 | 200 | Review contracts, prompt construction, and parsing moved to approval-core; runtime state, runner, and tool-policy adapters retained |
| actor | 14 | 352 | Audit reachability before moving |
| worker_server | 1 | 295 | Pending cohesion and dependency audit |
| search | 9 | 92 | Shared planning/execution pipeline and contracts extracted; model registry resolution and concrete tool dispatch retained as host adapters |
| image_clipboard | 2 | 33 | Portable image operations extracted; native clipboard access remains an application adapter |
| voice | 9 | 229 | Pending cohesion and dependency audit |
| worktree_cli | 6 | 144 | Entrypoint/adapter; enforce thin boundary |
| cleanup_cli | 4 | 137 | Entrypoint/adapter; enforce thin boundary |
| session_router_tmp | 1 | 79 | Audit reachability before moving |
| lib | 1 | 56 | Entrypoint/adapter; enforce thin boundary |
| cloudevents | 1 | 44 | Pending cohesion and dependency audit |
| memory | 2 | 39 | Pending cohesion and dependency audit |
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
| codetether-a2a-worker-core | 27 | yes | none |
| codetether-agent-core | 36 | yes | none |
| codetether-approval-core | 28 | yes | none |
| codetether-audit | 1 | yes | none |
| codetether-benchmark | 42 | yes | none |
| codetether-browser | 102 | yes | none |
| codetether-companion-capture | 6 | yes | none |
| codetether-companion-core | 12 | yes | none |
| codetether-companion-protocol | 8 | yes | none |
| codetether-companion-relay | 65 | yes | none |
| codetether-crash | 40 | yes | none |
| codetether-config | 71 | yes | none |
| codetether-distill | 15 | yes | none |
| codetether-event-stream | 32 | yes | none |
| codetether-github-pr | 35 | yes | none |
| codetether-goal-core | 71 | yes | none |
| codetether-indexer | 101 | yes | none |
| codetether-k8s | 80 | yes | none |
| codetether-knowledge-graph | 11 | yes | none |
| codetether-lsp-core | 55 | yes | none |
| codetether-marketplace | 4 | yes | none |
| codetether-mesh | 3 | yes | none |
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
| codetether-ralph-core | 95 | yes | none |
| codetether-rlm | 448 | yes | none |
| codetether-search | 11 | yes | none |
| codetether-secrets | 81 | yes | none |
| codetether-session-runtime | 20 | yes | none |
| codetether-telemetry | 62 | yes | none |
| codetether-theme | 13 | yes | none |
| codetether-tls | 4 | yes | none |
| codetether-tool-core | 31 | yes | none |
| codetether-tetherscript-values | 4 | yes | none |
| codetether-provider-tetherscript | 11 | yes | none |
| codetether-gemini-web-protocol | 74 | yes | none |
| codetether-gemini-web-transport | 13 | yes | none |
| codetether-provider-gemini-web | 35 | yes | none |
| codetether-provider-metrics | 16 | yes | none |
| codetether-provider-bonsai | 80 | yes | none |
| codetether-moltbook | 36 | yes | none |
| codetether-model-catalog | 30 | yes | none |
| codetether-util | 3 | yes | none |
| codetether-vectordb | 14 | yes | none |
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
