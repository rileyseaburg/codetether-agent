# CodeTether TUI 100x Agent Audit

Date: 2026-05-12
Worktree: `.codetether-worktrees/audit-tui-100x`
Branch: `audit/tui-100x`

## Scope

This audit follows the default `codetether tui` path, because that is the
primary user surface: CLI dispatch, TUI startup, session/provider startup,
prompt submission, worktree isolation, A2A background behavior, chat rendering,
help/discoverability, and the SRP/50-line-file ratchet.

No `cargo build` or `cargo check` was run, per repository instructions.
`./check_file_limits.sh` was run and passed because this audit did not change
`src/**/*.rs`.

## Executive Summary

The product has a large amount of real capability, but the default TUI path is
currently risky and too implicit. The biggest 100x improvements are not new
models or more commands. They are making the TUI trustworthy, observable, and
goal-oriented by default.

The highest priority issue is data loss risk in default worktree mode:
successful TUI edits can be made in an isolated worktree, then the non-PR path
merges only the branch and cleans up the worktree without first committing the
worktree's uncommitted edits.

The second highest priority issue is exposure risk: the TUI starts an A2A peer
on `0.0.0.0` by default, while the A2A router does not enforce inbound auth
before running `Session::prompt`.

## Critical Findings

### 1. Default Worktree Mode Can Drop Successful Edits

Evidence:

- `src/tui/app/state/default_impl.rs:88` defaults `use_worktree` to `true`.
- `src/session/types.rs:118-206` persists `use_worktree` with a default of
  `true`.
- `src/tui/app/input/chat_spawn.rs:38-44` creates a worktree whenever
  `use_worktree` is enabled.
- `src/tui/app/input/worktree_result.rs:47-53` merges locally for the normal
  non-PR success path, then cleans up the worktree.
- `src/tui/app/input/pr.rs:21-23` stages and commits worktree changes only on
  the PR path.
- `src/worktree.rs:298-341` merges the worktree branch. It does not commit
  uncommitted files inside the worktree before merging.

Impact:

The normal TUI user says "fix this file", the agent edits files in the
worktree, the session completes, then the TUI attempts to merge a branch that
may have no commits and removes the worktree. The visible chat can look
successful while the filesystem result is missing.

100x fix:

- Before any successful worktree publish or merge, stage and commit dirty
  worktree contents.
- If there is no diff, skip merge and tell the user "no file changes".
- If commit, merge, PR creation, or cleanup fails, keep the worktree and show
  the exact path in chat.
- Add an integration test for a TUI worktree run that writes a file and verifies
  the parent checkout receives the change.
- Move worktree outcome into a visible "Changes" panel: files changed, branch,
  PR URL or merge commit, retained worktree path on failure.

### 2. TUI Opens an Unauthenticated A2A Peer on the LAN by Default

Evidence:

- `src/cli/mod.rs:241-288` enables A2A by default and defaults the bind host to
  `0.0.0.0`.
- `src/main.rs:557-587` passes those defaults into TUI startup.
- `src/a2a/server.rs:51-57` creates routes without auth middleware.
- `src/a2a/server.rs:127-129` advertises no security scheme in the agent card.
- `src/a2a/server.rs:320-326` creates a fresh session and runs
  `session.prompt(&prompt)` for inbound blocking `message/send`.

Impact:

Any peer that can reach the bound port can send prompts to the agent process.
Because this process is inside the user's workspace, this is a high-trust
surface exposed as a default behavior.

100x fix:

- Default TUI A2A to loopback-only, or make A2A opt-in for interactive users.
- Require bearer auth for inbound A2A prompt execution.
- Show an explicit first-run consent prompt if LAN peer mode is enabled.
- Add a TUI status chip with the bind address, auth state, and discovered peer
  count.
- Add rate limits and an "approve inbound task" gate for mutating requests.

### 3. Startup Still Blocks on Provider and Bridge Initialization

Evidence:

- `src/tui/app/run.rs:173-174` draws "Loading providers and workspace..."
  early.
- `src/tui/app/run.rs:176-203` then waits on `tokio::join!` for provider
  loading, worker bridge startup, session scanning, config, and workspace scan.
- The session scan has a timeout at `src/tui/app/run.rs:185-190`, but provider
  registry loading via `ProviderRegistry::from_vault()` has no TUI-level budget.
- `src/provider/init_vault.rs:29-93` performs Vault/provider initialization.

Impact:

The TUI can show an initial frame, then remain non-interactive while provider or
worker setup stalls. For the primary product surface, startup should become
usable before integrations finish.

100x fix:

- Enter the event loop immediately with an empty provider state.
- Load providers, worker bridge, workspace snapshot, and session resume through
  independent startup channels.
- Put provider readiness in the model picker and status bar.
- Add short visible budgets: "Vault timed out after 2s; using env providers",
  with retry from `/model`.

### 4. Worktree Merge Recovery Is Too Aggressive

Evidence:

- `src/worktree.rs:245-269` can abort merges, reset index entries, and run
  `git checkout -- .` during pre-merge cleanup.
- `src/worktree.rs:272-292` stashes dirty parent checkout changes.
- `src/worktree.rs:308-330` retries conflicts with `-X theirs`.
- `src/tui/app/input/merge.rs:27-30` falls back from PR creation failure to a
  local merge.

Impact:

This is too much hidden mutation for the default TUI path. Users need to trust
that CodeTether will preserve their work, not silently prefer an agent branch
or mutate the parent checkout after a PR failure.

100x fix:

- Treat failed PR creation as "branch retained, PR not created", not "merge
  locally".
- Never auto-resolve conflicts with `-X theirs` in interactive TUI mode.
- Require an explicit user action for conflict resolution or local merge.
- Replace destructive cleanup with a written recovery plan.

### 5. TUI State and Command Routing Violate SRP at Product Scale

Evidence:

- `src/tui/app/state/mod.rs:71-226` holds a single `AppState` with chat,
  session picker, worker bridge, model picker, metrics, files, voice, A2A,
  autochat, watchdog, settings, and rendering cache state.
- `src/tui/app/commands.rs` has 1114 non-comment code lines.
- Static analysis found 80 TUI files over 50 non-comment code lines, totaling
  13,816 code lines in oversized files.
- Top TUI offenders include `commands.rs`, `ralph_view.rs`, `swarm_view.rs`,
  `bus_log.rs`, `message_formatter.rs`, and `help.rs`.

Impact:

This makes small TUI changes risky and slows the product loop. The 50-line
ratchet protects new code, but the primary user surface remains dominated by
legacy monoliths.

100x fix:

- Split `AppState` into cohesive state structs: `ChatState`, `ComposerState`,
  `SessionPickerState`, `AgentMeshState`, `ProviderState`, `WorktreeState`,
  `RuntimeStatusState`, and `SettingsState`.
- Replace `commands.rs` with a declarative command registry:
  name, aliases, args schema, help text, handler, view permissions.
- Generate slash autocomplete, `/keys`, and help from the same registry.
- Refactor one oversized TUI file per PR as a ratchet, starting with
  `commands.rs` and `worktree` outcome handling.

### 6. Help, Autocomplete, and Actual Key Behavior Drift

Evidence:

- `src/tui/app/state/slash_commands.rs:4-49` omits handled commands such as
  `/go`, `/goal`, `/undo`, `/fork`, and `/import-codex`.
- `src/tui/help.rs:324-325` documents mouse wheel scrolling.
- `src/tui/app/run.rs:105-112` intentionally does not enable mouse capture, so
  normal TUI mouse events are not available in typical terminals.
- `src/tui/app/event_handlers/keybinds.rs:57-59` only treats `?` as help when
  not in chat.

Impact:

Users discover features through stale or partial surfaces. A "100x" TUI cannot
hide core workflows behind inconsistent command lists.

100x fix:

- Single command/key registry drives help, hints, autocomplete, and tests.
- Add snapshot tests that assert every handled command appears in help and
  autocomplete.
- Decide whether mouse support or native selection wins, then make docs and
  behavior match.

### 7. Remote TUI Worker Tasks Are Queued and Displayed, Not Clearly Executed

Evidence:

- `src/tui/worker_bridge.rs:1-8` says the bridge receives incoming tasks.
- `src/tui/app/background.rs:49-66` queues incoming worker tasks.
- `src/tui/app/background.rs:69-84` dequeues and displays the next task when
  idle.
- `src/tui/app/inbox.rs:12-40` contains code to dispatch the next task as a
  prompt, but the active event loop in `src/tui/app/event_loop/select_loop.rs`
  does not call it.
- `src/tui/app/session_result.rs` emits bus replies, but the active result path
  in `src/tui/app/background.rs:102-150` does not use it.

Impact:

The code contains competing paths for remote-task handling. That is dangerous
for an A2A-native product because external agents may believe the TUI accepted
work that it only displayed.

100x fix:

- Choose one behavior: manual inbox approval or automatic execution.
- If manual, show an Inbox panel with Accept/Reject and never dequeue silently.
- If automatic, wire `inbox::trigger_next` into the active tick path and emit
  bus replies through the active result path.
- Add tests for queued task -> executed task -> reply event.

## Product-Level 100x Improvements

1. Make the TUI a changes cockpit, not just chat.
   Show pending files, diffs, tool timeline, validation status, context health,
   and worktree/PR state in stable panels.

2. Make "goal + acceptance criteria" first-class.
   `/goal` exists, but the first screen should make the current objective,
   constraints, and done criteria visible and editable.

3. Make mutation trust explicit.
   Every edit path should have a visible lifecycle: proposed, applied,
   validated, committed, merged/PR-created, or retained for recovery.

4. Make startup progressive.
   A user should be able to type immediately while providers, model lists,
   A2A, session scan, index, and LSP warm independently.

5. Make command discovery generated.
   One registry should power slash hints, help, key docs, command parsing, and
   command tests.

6. Make background work legible.
   A2A tasks, swarm agents, Ralph, autochat, and model retries need one shared
   activity model with status, owner, cancel/retry, and final artifact links.

## Immediate Patch Order

1. Fix default worktree result handling so successful edits are committed before
   merge and preserved on failure.
2. Lock down TUI A2A defaults: loopback or auth-required, plus visible status.
3. Convert startup initialization to background state updates instead of a
   blocking `join!`.
4. Replace command/help/autocomplete lists with a command registry.
5. Split `commands.rs` and `AppState` along real product boundaries.

