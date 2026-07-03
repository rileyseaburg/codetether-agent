# v4.7.2

The user wants me to generate release notes for version v4.7.2 of CodeTether Agent, based on the provided commits and file change stats. This is a documentation/writing task, not a code implementation task. I don't need to call tools to generate release notes — I have all the data I need (commits and stats) right in the prompt.

Let me analyze the commits and group them into categories for the release notes.

Let me look at the major themes:

**Features (What's New):**
1. Embedded TUI editor - a full ropey/Helix-backed editor with syntax highlighting, line numbers, cursor-follow scrolling, horizontal scrolling, LSP integration
3. Bedrock SSO authentication - native SSO device-code flow, bearer token refresh mid-session, thinking effort settings
4. Approval system - runtime policy gate, AGENTS.md override support, live approval with session grants
5. File-based vector store with semantic memory search (BERT embeddings)
6. Sub-agent improvements - non-blocking dispatch by default, TUI visibility, auto-start first turn, liveness tracking
7. Rich TUI rendering - markdown tables, diff coloring, code blocks, task-list checkboxes, agent tab bar, context gauge, scrollbar, sparklines
8. Cerebras model support (reasoning_content capture)
9. GLM-5.2 model in ZAI provider (1M-token context)
10. MiniMax M3 models
11. Bedrock InvokeModel adapter for claude-fable-5
12. `codetether connect` for remote device-code auth over SSH
13. VS Code worktree integration
14. `/forage` TUI command
15. `/continue` command
16. `/models` OpenRouter-style endpoint with rich metadata
17. Model-relative reset threshold, dropped-range TOC, pin-aware context
18. computer_use: set_text and click_client actions

**Bug Fixes:**
1. Infinite identical-edit retry loop in sub-agents
2. Watchdog deadlock/freeze issues (multiple fixes)
3. OpenAI-compatible stream tool-call repair
4. Cerebras reasoning_content capture
5. OKR zero-target KRs progress reporting
6. Memory token-based search
7. Worktree dirty state guards, no-op merge handling
8. Bedrock API key validation against signing region
9. A2A intro storms + bearer auth
10. TUI flicker fix
11. Time crate pin (0.3.47) to avoid E0119 regression
12. LSP Content-Length cap to prevent OOM
13. Unsafe env-var mutation replaced with thread-safe config

**Changes:**
1. Massive SRP refactor - config/mod.rs (800→39 lines), provider/anthropic.rs (716→30 files), worktree.rs (826 lines→split), bus_log (770 lines→split), compression module split, openai.rs split, etc.
2. sccache integration across CI and Dockerfiles
3. Docker worker-prebuilt for containerized deployment
4. ContentPart::ToolCall unification

This is a massive release - 57,362 insertions, 12,756 deletions across 1,670 files. Let me write concise but comprehensive release notes.

Since this is purely a writing task with all data provided, I'll generate the notes directly. The build mode reminder says to start with a tool call or emit tool_call markup. But honestly there's no appropriate tool here — I have all the input data in the prompt. However, I could write the release notes to a file. Let me think about whether the user wants me to produce a file or just the text.

The task says "Generate concise, professional release notes in markdown format." It doesn't explicitly say write to a file. But given the build mode reminder, let me write the release notes. I'll produce the markdown directly as my answer since that's what's being asked — generating release notes content.

Actually, I should just generate the notes. The data is all in the prompt. Let me craft them well. I'll organize into the three requested sections: ## What's New, ## Bug Fixes, ## Changes.

Let me be concise but professional, covering the highlights without listing every single commit.

Let me write good release notes now.I have all the commit and stat data in the prompt, so I'll generate the release notes directly. Here they are:

---

## What's New

**Embedded TUI editor.** A full in-terminal code editor built on a ropey backend with optional Helix integration. Includes Rust syntax highlighting, a line-number gutter, cursor-follow scrolling, horizontal scroll for long lines, and live LSP support (hover, go-to-definition, navigation). Edit files without leaving the agent session.


**Approval system & runtime policy gate.** A new runtime policy engine gates tool execution (`approval`, `sandbox_preflight`, `invocation`, `command`, `workspace` rules). Live approval flows support session grants and per-command pre-approval, surfaced in the TUI as an interactive overlay. AGENTS.md override support lets projects ship their own policies.

**Bedrock SSO & session resilience.** Native AWS SSO device-code authentication with split OIDC modules, silent `--refresh` via stored SSO refresh tokens, bearer-token mid-session refresh on live auth failure, and Bedrock API-key expiry clamped to the credential session lifetime. The `claude-fable-5` model now routes through the Bedrock InvokeModel adapter, with thinking-effort and service-tier settings exposed in the TUI Settings panel.

**Semantic memory with vector search.** A new file-based vector store (`vectordb`) using BERT embeddings powers semantic memory search. The embedder auto-downloads and initializes on every tool instance.

**Sub-agent orchestration overhaul.** Sub-agent dispatch is now non-blocking by default with detach mode. Spawned agents auto-start their first turn, auto-apply tools for autonomous operation, and report liveness from tool activity. Sibling agents are visible in the TUI agent tab bar (Tab/Shift+Tab switching) and on the event bus.

**Rich TUI rendering.** Markdown tables with column alignment, diff-colored code blocks, task-list checkboxes, depth-aware list bullets, gated processing spinners, a live working badge, context gauge, throughput sparklines, a scroll indicator, and approval overlays. Swarm subtasks are now grouped by stage with per-agent elapsed time.

**New providers & models.** Cerebras (with `reasoning_content` capture), GLM-5.2 via ZAI (1M-token context), MiniMax highspeed/M3, and VS Code Language Model adapter support.

**New commands & endpoints.** `codetether connect` (remote device-code auth over SSH with binary preflight), `/forage` and `/continue` TUI commands, an OpenRouter-style `/models` endpoint with rich pricing/architecture metadata, and VS Code worktree integration CLI.

**Smarter context management.** Model-relative reset thresholds, a navigable table-of-contents for dropped ranges, and pin-aware context window handling.

**Other additions.** `computer_use` gains `set_text` and `click_client` actions; durable replayable AgentBus logs with S3 archival; tool-output chunk streaming for live TetherScript process output; `ToolHeartbeat` protocol events to keep the watchdog from killing active long-running tools.

## Bug Fixes

- **Sub-agent reliability:** break infinite identical-edit retry loops; make sub-agent registries consistent for TUI visibility; auto-start first turn on spawn (#294–#298).
- **Watchdog stability:** rewrite for long-running sessions; fix deadlock that froze user input after a stall; make restart counts monotonic across stall storms; gate the no-first-token clause to the pre-activity window; clear notifications on turn completion.
- **Stream parsing:** repair dangling tool-call sequences for OpenAI-compatible streams; salvage bare-JSON and textual `<tool_call>` narrations in swarm loops; capture `reasoning_content` from raw SSE for Cerebras models.
- **OKR:** zero-target key results now report correct progress and completion.
- **Memory:** switch to token-based search instead of whole-phrase substring; always run init so the embedder installs on every instance.
- **Worktree lifecycle:** guard removal against dirty state; treat no-op merges as success; fix merge-staged path handling.
- **A2A protocol:** stop intro storms with intro short-circuit and bearer auth.
- **Provider correctness:** validate Bedrock API keys against their signing region (not hardcoded `us-east-1`); correct Cerebras GLM model ID.
- **TUI:** eliminate flicker by removing per-frame `terminal.resize`; fix VS Code auto-open prompt conflicts; never raw-prompt for VS Code while the TUI owns the terminal.
- **Stability:** replace unsafe env-var mutation for Bedrock settings with thread-safe config; cap LSP `Content-Length` to prevent OOM; pin `time` to 0.3.47 to avoid an E
