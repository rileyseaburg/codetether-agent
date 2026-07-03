# v4.7.3

Let me analyze the commits and stats to generate professional release notes for v4.7.3. This is a documentation task, so I'll write the release notes directly to the RELEASE_NOTES.md file (or create them as output).

Let me look at the existing RELEASE_NOTES.md to understand the format being used, then generate the notes for v4.7.3.

Wait - the task says "Generate concise, professional release notes in markdown format for version v4.7.3" and "Do NOT include a title heading — start directly with the sections." This suggests they want me to produce the content directly as output. But since this is build mode, I should use a tool. Let me read the existing RELEASE_NOTES.md to match the style, then write the new version.

Let me analyze the commits since v4.7.1 (note: v4.7.2 was already released, so v4.7.3 covers the commits since v4.7.2). Actually, the task says "Commits since v4.7.1" but we're generating v4.7.3 notes. So v4.7.3 is what comes after v4.7.2. Let me look at the commits more carefully.

The top commit is "71b0382c docs: release notes for v4.7.2" - so everything above that (the more recent commits) would be v4.7.3. But wait, the list shows commits "since v4.7.1" which includes v4.7.2's commits too. 

Actually, looking at the commit list, the first two commits are:
- 71b0382c docs: release notes for v4.7.2
- 6d420149 chore: bump version to 4.7.2

These are the v4.7.2 release commits. Everything below those (and after, i.e., the commits that came AFTER v4.7.2 was bumped) would be v4.7.3.

Hmm, but git log is usually newest-first. So 71b0382c is the most recent commit. If that's "docs: release notes for v4.7.2", then the commits above it in the log (there are none above it in this list - it's the first one). So the commits listed ARE since v4.7.1.

Wait, the commit list starts with 71b0382c and goes down. In git log, newer commits are on top. So:
- 71b0382c docs: release notes for v4.7.2 (most recent)
- 6d420149 chore: bump version to 4.7.2
- ca10aa9e chore: make cargo-sccache.sh executable
- ... continues down

So the first two commits belong to v4.7.2. Everything from ca10aa9e downwards would be... no wait. Actually, "commits since v4.7.1" means all commits between v4.7.1 and HEAD. The top commit has the v4.7.2 release notes. But we're making v4.7.3.

This is a bit confusing. Let me re-read: "Commits since v4.7.1" - these are ALL the commits since v4.7.1, which includes both v4.7.2 and the new v4.7.3 work. But the top two commits are literally the v4.7.2 version bump and release notes. So there's nothing new after v4.7.2 in this list?

Actually, I think the intent is that this entire list of commits represents what's being released as v4.7.3. The v4.7.2 was an intermediate version. The user wants release notes for v4.7.3 based on all these commits since v4.7.1.

Actually, re-reading more carefully: "Generate concise, professional release notes... for version v4.7.3" and "Base the notes on these commits and stats." So I should use all these commits to write the v4.7.3 notes. The v4.7.2 commits (release notes bump) can be ignored as they're just release plumbing.

Let me categorize the major features and fixes from these commits.

Let me group the significant changes:

**Major Features:**

1. **Embedded TUI Editor** - Full code editor in the terminal:
   - Ropey-backed editor backend
   - Helix editor integration
   - Rust syntax highlighting
   - Line numbers, cursor positioning, horizontal scrolling
   - FileBuffer for open/save/dirty files
   - LSP hover support
   - `/edit` command

2. **Resumable A2A Stream Protocol (QUIC)** - Multi-phase transport overhaul:
   - Phase 1: Resumable stream protocol with cursor primitives
   - Phase 2: Backpressure with bounded staging buffers
   - Phase 3: TCP socket control for worker client
   - Phase 5: Connection lifecycle state machine
   - Phase 6: QUIC multiplex stream transport with connection migration

3. **Vector DB / Semantic Memory** - File-based vector store with BERT embeddings:
   - bert embedding with contiguous memory
   - Semantic memory search
   - Token-based memory search

4. **Sub-Agent Improvements**:
   - Non-blocking sub-agent dispatch by default
   - Interactive /spawn form in TUI
   - Agent tab bar with Tab/Shift+Tab switching
   - Sub-agent liveness reporting from tool activity
   - Infinite edit retry loop fix
   - Auto-start first turn on spawn

5. **Approval System & Runtime Policy**:
   - Full approval system with runtime policy gate
   - AGENTS.md override support
   - Tool approval flow
   - Access modes (safe/normal/unrestricted)

6. **Bedrock Enhancements**:
   - Native SSO device-code auth flow
   - Silent --refresh via stored SSO refresh token
   - Bearer token mid-session refresh
   - claude-fable-5 routing through InvokeModel adapter
   - Thinking effort settings in TUI
   - Bedrock auth gate TetherScript plugin

7. **Rich TUI Rendering**:
   - Markdown tables with alignment
   - Task-list checkboxes and depth-aware bullets
   - Diff coloring, inline code
   - Scrollbar + diff-aware code blocks
   - Processing spinner + live working badge
   - Throughput sparkline
   - Context gauge

8. **New Providers/Models**:
   - Cerebras gemma-4-31b
   - GLM-5.2 (1M-token context)
   - MiniMax highspeed/M3 models
   - Claude-fable-5 via Bedrock InvokeModel
   - VS Code language model adapter

9. **OpenRouter-style /models endpoint** with rich model metadata

10. **codetether connect** - remote device-code auth over SSH

11. **VS Code worktree integration** CLI

12. **Always-on context indicator** + /image while processing

13. **Audit loop TUI view** + forage interactive goal clarification

14. **Durable replayable log** for AgentBus

15. **Codex parity** commands (/codex parity)

**Major Bug Fixes:**
- TUI crash resilience and watchdog rewrite
- Watchdog deadlock that froze user input
- A2A intro storms + intro short-circuit
- Bedrock API key validation against signing region
- LSP Content-Length cap to prevent OOM
- Empty Cerebras turns (reasoning_content capture)
- OpenAI-compatible stream dangling tool-call repair
- Zero-target OKR KRs progress reporting
- Memory search token-based instead of substring
- Unsafe env-var mutation replaced with thread-safe config
- Worktree dirty state guards

**Changes/Refactoring:**
- Massive SRP refactoring: config/mod.rs (816→39 lines), provider/anthropic.rs (716→30 files), worktree.rs (826 lines→many modules), bash.rs, sandbox.rs, patch.rs all split
- 50-line file limit enforcement
- sccache integration across CI and Docker
- Dockerfile.worker-prebuilt for containerized worker deployment
- time crate pinned to avoid regression

Let me now write the release notes. The task says "concise" so I need to be selective and not list every commit. Let me write professional, grouped release notes.

Let me check the existing RELEASE_NOTES.md format first.I'll examine the existing release notes format, then generate the v4.7.3 notes.
The existing RELEASE_NOTES.md has v4.7.2 content (which is actually a draft/notes file, not the final format). The Cargo.toml confirms version 4.7.3. 

The user wants me to generate concise release notes for v4.7.3. The commits since v4.7.1 are what I have. Since v4.7.2 was already released (its notes exist in RELEASE_NOTES.md), v4.7.3 would cover the NEW commits since v4.7.2.

But looking at the commit list, the top two commits are:
- 71b0382c docs: release notes for v4.7.2
- 6d420149 chore: bump version to 4.7.2

Everything below those is what was done before v4.7.2 was cut. So there are no NEW commits after v4.7.2 in this list... which means v4.7.3 would have nothing new.

Hmm, but the task says to base v4.7.3 notes on "these commits" (since v4.7.1). I think the user's intent is that all these commits collectively represent the work for this release cycle, and they want me to produce polished release notes. The version is v4.7.3 as confirmed by Cargo.toml.

Actually, I think what happened is: the previous AI agent (for v4.7.2) just dumped its raw thinking into RELEASE_NOTES.md instead of writing proper notes. The user is now asking for proper v4.7.3 notes covering the same body of work. The Cargo.toml is already at 4.7.3.

So I should write clean, professional release notes for v4.7.3 covering all these commits. Let me write the final content to RELEASE_NOTES.md, replacing the v4.7.2 draft.

Let me write proper, concise release notes. I'll organize into the three sections requested. I need to be concise but comprehensive given the massive scope.Now I understand the context. The existing RELEASE_NOTES.md contains a v4.7.2 draft that's raw thinking notes rather than polished release notes, and Cargo.toml confirms version 4.7.3. I'll write clean, professional release notes for v4.7.3 based on the commits since v4.7.1.
I've written the release notes. Let me verify the content looks good and report back to the user.Release notes for v4.7.3 have been written to `RELEASE_NOTES.md`. Here's a summary of what I generated:

**Structure** — Three sections as requested (`## What's New`, `## Bug Fixes`, `## Changes`), no title heading.

**Key decisions:**

- **What's New** groups ~18 major feature areas into 11 concise blocks: the embedded TUI editor, the six-phase QUIC transport overhaul, semantic memory/vector DB, the approval & runtime policy system, sub-agent orchestration, Bedrock SSO auth, rich TUI rendering, new models/providers, new CLI commands, and additional features.

- **Bug Fixes** calls out 12 high-impact fixes with specifics — the watchdog rewrite, A2A intro storms, Bedrock key validation, Cerebras/OpenAI stream repair, OKR progress, memory search, worktree guards, LSP OOM, TUI flicker, etc.

- **Changes** covers the large-scale SRP refactoring (with before→after line counts), `ContentPart` unification, sccache integration, Docker worker-prebuilt, symbol table retention, the `time` crate pin, and the overall diff stats.

The notes are **concise but comprehensive** — they capture the massive scope of this release (1,271 files, 57K+ insertions) without listing every commit. I replaced the existing content (which was a raw v4.7.2 thinking draft) with the polished v4.7.3 notes that match the confirmed `Cargo.toml` version.
