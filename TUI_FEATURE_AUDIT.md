# TUI Feature Audit

## Scope
This audit reviews the current TUI implementation under `src/tui/` and classifies features as:
- **Implemented**: actively wired and usable
- **Partial**: visible in UI or commands, but incomplete, or missing key interactions
- **Unwired / Dead-path**: code exists but is not part of the active flow

## Executive Summary
The TUI has a strong active core around chat, sessions, model selection, settings, symbol search, bus log visibility, file/image attachment, clipboard support, and monitor-style swarm/Ralph views. However, there are also several obvious gaps:

1. **Protocol view is declared but not rendered**.
2. **LSP and RLM views are informational shells, not true feature panels**.
3. **`/go` approval flow appears drafted but is not actually dispatched from slash command handling**.
4. **`/agent` is advertised but has no handler in active command dispatch**.
5. **`feature_extracted/` contains parked duplicate/incomplete implementations that are not source-of-truth**.
6. **`/autochat` is now live at MVP level, but not yet integrated with the fuller parked relay path**.

---

## 1. Core TUI Runtime

### Status: Implemented

### Evidence
- `src/tui/app/run.rs`
  - Initializes terminal, secrets, provider registry, worker bridge, session resume, workspace snapshot.
- `src/tui/app/event_loop/mod.rs`
  - Main draw/event loop is active.
- `src/tui/app/event_loop/select_loop.rs`
  - Multiplexes terminal events, session events, results, watchdog, autochat tick.

### Notes
This is a real, active runtime with session resume, background tasks, watchdog hooks, and event coalescing.

---

## 2. Chat View / Main Conversation UI

### Status: Implemented

### Evidence
- `src/tui/ui/main.rs`
  - `ViewMode::Chat => render_chat_or_webview(...)`
- `src/tui/ui/chat_view/mod.rs`
  - Chat rendering decomposed into many focused submodules.
- `src/tui/app/input/chat_submit.rs`
  - Enter in chat submits prompts or slash commands.
- `src/tui/app/message_text.rs`, `session_events.rs`
  - Active message/session synchronization.

### Notes
This is the main production path and appears to be the most complete feature in the TUI.

---

## 3. Webview Chat Layout Toggle

### Status: Implemented

### Evidence
- `src/tui/app/event_handlers/keyboard.rs`
  - `Ctrl+B` toggles classic/webview layout.
- `src/tui/ui/webview/mod.rs`
  - Active renderer for webview layout.
- `src/tui/app/commands.rs`
  - `/webview` and `/classic` switch layout mode.

### Notes
This is not a browser embed; it is an alternate ratatui layout. But it is active and wired.

---

## 4. Session Picker / Session Resume

### Status: Implemented

### Evidence
- `src/tui/app/run.rs`
  - Loads last workspace session with bounded resume window.
- `src/tui/app/session_sync.rs`
  - `refresh_sessions`, `return_to_chat`.
- `src/tui/ui/sessions.rs`
  - Session picker rendering.
- `src/tui/app/input/enter.rs`
  - Enter dispatch for `ViewMode::Sessions`.
- `src/tui/app/navigation.rs`
  - Up/down navigation and Esc behavior for sessions.

### Notes
Usable and fully part of active flow.

---

## 5. Model Picker

### Status: Implemented

### Evidence
- `src/tui/model_picker.rs`
  - Renders live model list and selection.
- `src/tui/app/commands.rs`
  - `/model`
- `src/tui/app/navigation.rs`
  - Up/down navigation for model picker.
- `src/tui/app/input/enter.rs`
  - Enter applies model.

### Notes
This is a real active feature.

---

## 6. Settings Panel

### Status: Implemented

### Evidence
- `src/tui/settings.rs`
  - Renders settings panel.
- `src/tui/app/commands.rs`
  - `/settings`
- `src/tui/app/event_handlers/mode_keys.rs`
  - `a`, `n`, `Tab` toggles settings in-panel.
- `src/tui/app/input/enter.rs`
  - Enter toggles selected setting.

### Notes
Works as a real settings surface for auto-apply, network, autocomplete, and worktree isolation.

---

## 7. File Attachment / File Picker / Image Attachment

### Status: Implemented

### Evidence
- `src/tui/app/file_picker.rs`
  - Directory scan, filter, image detection, rendering, enter behavior.
- `src/tui/app/event_handlers/keyboard.rs`
  - `Ctrl+O` opens file picker.
- `src/tui/app/input/char_input.rs`
  - File picker filter typing.
- `src/tui/app/input/backspace.rs`
  - File picker filter backspace.
- `src/tui/app/file_share.rs`
  - Attach file contents into composer.
- `src/tui/app/commands.rs`
  - `/file`, `/image`

### Notes
This is live and reasonably complete.

---

## 8. Clipboard Paste / Clipboard Image / Copy Latest Reply

### Status: Implemented

### Evidence
- `src/tui/app/event_handlers/keyboard.rs`
  - `Ctrl+V` and `Ctrl+Y`
- `src/tui/app/event_handlers/clipboard.rs`
  - text-first clipboard paste, image attach alternate path.
- `src/tui/app/event_handlers/copy_reply.rs`
  - copies latest assistant response.

### Notes
This is actively wired and useful.

---

## 9. Slash Command Autocomplete / Composer Editing / History

### Status: Implemented

### Evidence
- `src/tui/app/text.rs`
  - command normalization and alias handling.
- `src/tui/app/event_handlers/keybinds.rs`
  - Tab accepts slash suggestion.
- `src/tui/app/navigation.rs`
  - `Ctrl+Up/Down` command history in chat.
- `src/tui/app/input/chat_submit.rs`
  - slash command dispatch.

### Notes
Core editing/composer UX is active.

---

## 10. Symbol Search

### Status: Implemented, but shallow integration

### Evidence
- `src/tui/symbol_search.rs`
  - popup UI and state.
- `src/tui/app/symbols.rs`
  - calls `LspTool::workspaceSymbol`.
- `src/tui/app/event_handlers/keyboard.rs`
  - `Ctrl+T` opens symbol search.
- `src/tui/app/commands.rs`
  - `/symbols`
- `src/tui/app/input/char_input.rs`
  - typing refreshes search.

### Notes
The popup is active, but result parsing is simplistic and Enter only closes the popup after setting a status string. There is no actual jump-to-definition/navigation action yet.

Classification rationale: mostly implemented, but deeper symbol navigation is still partial.

---

## 11. Bus Log / Protocol Bus Visibility

### Status: Implemented

### Evidence
- `src/tui/bus_log.rs`
  - active bus log rendering and detailed envelope formatting.
- `src/tui/ui/main.rs`
  - `ViewMode::Bus => render_bus_view(...)`
- `src/tui/app/commands.rs`
  - `/bus`, `/protocol` currently route to `ViewMode::Bus`
- `src/tui/app/navigation.rs`
  - selection/detail/filter controls.
- `src/tui/app/input/enter.rs`
  - Enter opens detail / applies filter behavior.

### Notes
This is the real active "protocol visibility" feature today.

---

## 12. Swarm Monitor View

### Status: Implemented as monitor UI

### Evidence
- `src/tui/swarm_view.rs`
  - complete view state and rendering behavior.
- `src/tui/ui/main.rs`
  - `ViewMode::Swarm => render_swarm_view(...)`
- `src/tui/app/commands.rs`
  - `/swarm`
- `src/tui/app/navigation.rs`
  - list/detail navigation.

### Notes
This appears to be a real monitor UI. Whether every upstream producer emits rich live data is outside this audit, but the TUI side is implemented.

---

## 13. Ralph Monitor View

### Status: Implemented as monitor UI

### Evidence
- `src/tui/ralph_view.rs`
  - complete state/event model and renderer.
- `src/tui/ui/main.rs`
  - `ViewMode::Ralph => render_ralph_view(...)`
- `src/tui/app/commands.rs`
  - `/ralph`
- `src/tui/app/navigation.rs`
  - selection/detail navigation.

### Notes
As with swarm, the TUI rendering and navigation are active.

---

## 14. Latency Inspector

### Status: Implemented

### Evidence
- `src/tui/latency.rs`
  - active renderer with timing, provider metrics, and tool latency summaries.
- `src/tui/ui/main.rs`
  - `ViewMode::Latency => render_latency(...)`
- `src/tui/app/commands.rs`
  - `/latency`

### Notes
This is a real active inspector.

---

## 15. Inspector View

### Status: Implemented

### Evidence
- `src/tui/ui/inspector/mod.rs`
  - active entrypoint.
- `src/tui/ui/main.rs`
  - `ViewMode::Inspector => render_inspector_view(...)`
- `src/tui/app/commands.rs`
  - `/inspector`

### Notes
Inspector view is active. Detailed completeness of each subpanel was not exhaustively audited, but the routing/rendering is real.

---

## 16. MCP Command Surface

### Status: Implemented

### Evidence
- `src/tui/app/commands.rs`
  - `/mcp connect`, `/mcp servers`, `/mcp tools`, `/mcp call`
- Uses `app.state.mcp_registry` actively.

### Notes
This is CLI-in-TUI command functionality rather than a dedicated visual panel, but it is clearly implemented.

---

## 17. Spawned Sub-Agent Management

### Status: Partial

### Evidence
- `src/tui/app/commands.rs`
  - `/spawn`, `/kill`, `/agents` are implemented.
- `src/tui/app/state/mod.rs`
  - active spawned-agent state exists.

### Missing / Problematic
- `/agent` is advertised in multiple places but no active handler exists:
  - listed in help/status hints
  - present in slash command tables
  - normalized by aliases like `/talk`, `/say`, `/focus`
  - **no actual `/agent` command handler in `handle_slash_command`**

### Impact
Users are told they can focus/message a spawned sub-agent, but the active slash dispatcher does not implement it.

---

## 18. `/autochat`

### Status: Partial (improved from earlier fake behavior)

### Evidence
- `src/tui/app/commands.rs`
  - `/autochat` is dispatched.
- `src/tui/app/autochat/handler.rs`
  - active event handling.
- `src/tui/app/event_loop/autochat.rs`
  - active background drain.
- `src/tui/app/autochat/worker.rs`
  - now executes a real provider-backed background task.

### Remaining Gap
- The fuller relay/OKR-aware logic still appears parked in `src/tui/feature_extracted/autochat_relay.rs`.
- Current live implementation is functional but not yet the full intended multi-agent relay path.

### Impact
No longer a fake completion path, but still not feature-complete relative to the parked design.

---

## 19. `/go` OKR approval flow

### Status: Partial / likely unwired

### Evidence
- `src/tui/app/commands.rs`
  - contains `handle_go_command(...)`
- `src/tui/app/okr_gate.rs`
  - substantial OKR drafting logic exists.
- `src/tui/app/event_handlers/okr.rs`
  - approval/deny handlers exist.

### Missing / Problematic
- In active slash dispatch (`handle_slash_command`), there is **no visible path calling `handle_go_command`** in the audited regions.
- `chat_submit.rs` sends slash commands only through `handle_slash_command`.
- Help and hints mention `/go`, but active dispatch evidence is incomplete.

### Impact
This looks like a drafted feature with major parts implemented, but active end-to-end wiring is suspect.

---

## 20. Protocol View (`ViewMode::Protocol`)

### Status: Declared but blank in active UI

### Evidence
- `src/tui/models/view_mode_registry.rs`
  - includes `ViewMode::Protocol`
- `src/tui/models/view_mode_display.rs`
  - display name + shortcut hint (`Ctrl+P`)
- `src/tui/ui/main.rs`
  - `ViewMode::Protocol => {}`
- `src/tui/app/input/enter.rs`
  - Enter in Protocol mode does nothing.
- `src/tui/protocol_registry.rs`
  - a renderer exists, but is not wired into top-level dispatcher.

### Impact
This is the clearest example of a user-visible but incomplete feature.

---

## 21. LSP View

### Status: Partial / informational shell

### Evidence
- `src/tui/lsp.rs`
  - text explicitly says:
    - "Planned next step: connect symbol search and diagnostics navigation."
    - "For now, use this view as a dedicated workspace/introspection panel."
- `src/tui/ui/main.rs`
  - renderer is wired.
- `src/tui/app/commands.rs`
  - `/lsp`

### Impact
The view exists, but it is not a full LSP workspace workflow panel yet.

---

## 22. RLM View

### Status: Partial / informational shell

### Evidence
- `src/tui/rlm.rs`
  - text says it is the "future home" for large-codebase analysis.
- `src/tui/ui/main.rs`
  - renderer is wired.
- `src/tui/app/commands.rs`
  - `/rlm`

### Impact
This is an informational shell, not a complete feature panel.

---

## 23. Help Overlay

### Status: Implemented, but contains stale claims

### Evidence
- `src/tui/help.rs`
  - real help overlay rendering.

### Problems found
Help currently advertises or implies some capabilities that do not cleanly match the active code:
- `/protocol` behaves as a bus-log alias, not a dedicated protocol registry view.
- `/agent` is documented but not actively handled.
- `Ctrl+P` is listed as Protocol shortcut in display metadata, but active keybinding implementation for Protocol view was not found in the audited handlers.

### Impact
The help system is real but partially inaccurate.

---

## 24. Parked `feature_extracted/` modules

### Status: Unwired / dead-path / ambiguous source of truth

### Evidence
- `src/tui/feature_extracted/mod.rs`
  - documents modules as partially integrated and pending stabilization.
- `src/tui/feature_extracted/watchdog_and_pickers.rs`
  - explicitly says it is not wired into the compilation graph.
- `src/tui/feature_extracted/webview.rs`
  - contains duplicate protocol rendering logic.
- `src/tui/feature_extracted/autochat_relay.rs`
  - contains much more substantial relay logic than the active `app/autochat/worker.rs` path.

### Impact
This directory creates uncertainty about what code is canonical and what is abandoned/refactor staging.

---

## 25. Command Inventory Status

### Clearly implemented slash commands
- `/help`
- `/sessions`
- `/import-codex`
- `/swarm`
- `/ralph`
- `/bus`
- `/protocol` (alias to bus, not dedicated protocol view)
- `/model`
- `/settings`
- `/lsp`
- `/rlm`
- `/latency`
- `/inspector`
- `/chat`
- `/webview`
- `/classic`
- `/symbols`
- `/new`
- `/undo`
- `/file`
- `/image`
- `/autoapply`
- `/network`
- `/autocomplete`
- `/steer`
- `/mcp ...`
- `/spawn`
- `/kill`
- `/agents`
- `/autochat`

### Drafted / suspicious / not confirmed end-to-end
- `/go`
- `/team`
- `/agent`

---

## Highest-Priority Gaps

### P0
1. **Wire Protocol view renderer**
   - `ViewMode::Protocol` currently renders nothing.
2. **Implement `/agent` or stop advertising it**
   - Current help/aliases imply support that active dispatch does not provide.
3. **Resolve `/go` wiring**
   - Confirm and fix active slash dispatch to the OKR approval flow.

### P1
4. **Upgrade `/autochat` from MVP provider request to full relay flow**
   - Reconcile with parked `feature_extracted/autochat_relay.rs` logic.
5. **Turn LSP view from informational shell into an actual interactive panel**
6. **Turn RLM view from informational shell into an actual interactive panel**

### P2
7. **Audit and shrink/remove `feature_extracted/` dead paths**
8. **Correct help/shortcut documentation to match actual behavior**

---

## Recommended Issue Mapping
- **#57** `/autochat` real relay wiring
- **#58** Protocol view rendering and navigation
- **#59** resolve parked `feature_extracted` dead/duplicate paths
- New follow-up issue recommended: **Implement `/agent` active handler or remove it from UX**
- New follow-up issue recommended: **Wire `/go` end-to-end from slash command to OKR approval execution**
- New follow-up issue recommended: **Replace LSP/RLM informational shells with real interactive functionality**

---

## Bottom Line
The TUI is **not mostly unfinished**. Its **core workflow is real and substantial**. But several user-facing features are either:
- placeholders (`LSP`, `RLM`),
- declared but blank (`Protocol`),
- partially integrated (`/autochat`), or
- advertised without active handler evidence (`/agent`, likely `/go`).

So the correct characterization is:

> **The TUI core is implemented, but there are several unfinished or misleading edge features that should be cleaned up next.**
