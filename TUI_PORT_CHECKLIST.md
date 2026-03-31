# TUI Feature Port Checklist

> Gap analysis: `tui_old.rs` (11,964 lines) → modular `src/tui/` (59 files)  
> ~4,500 lines of functionality remain unported across 10 major systems.

---

## Tier 1 — Core Missing Systems

### Spawned Sub-Agents (~600 lines)

Foundation for autochat/relay. Old code: `tui_old.rs` L415–L500, L4584–L4960, L5369–L5650, L10750+.

- [ ] `SpawnedAgent` struct with independent LLM session, name, instructions, model
- [ ] `spawned_agents: HashMap<String, SpawnedAgent>` field on `AppState`
- [ ] `active_spawned_agent: Option<String>` for focus tracking
- [ ] `agent_response_rxs` — per-agent `mpsc` receiver channels
- [ ] `streaming_agent_texts: HashMap<String, String>` for streaming display
- [ ] `dispatch_to_agent_internal()` — route message to a spawned agent
- [ ] `send_to_agent()` — public send wrapper
- [ ] `handle_agent_response()` — process `SessionEvent` from spawned agent sessions
- [ ] `@mention` routing (`@agent_name message` dispatches to that agent)
- [ ] Auto-routing when `active_spawned_agent` is set
- [ ] Agent profiles system (Strategist, Archivist, Forge, Sentinel, Probe codenames)
- [ ] `/spawn <name> [instructions]` command
- [ ] `/kill <name>` command (with bus shutdown announcement)
- [ ] `/agents` command (list all spawned agents with status)
- [ ] `/agent <name>` command (focus chat, open picker, or send single message)
- [ ] `ViewMode::AgentPicker` variant
- [ ] `Ctrl+A` key binding → agent picker
- [ ] Event loop branch for per-agent response channels

### Autochat / Multi-Agent Relay (~2,000 lines)

Depends on spawned sub-agents. Old code: `tui_old.rs` L4373–L4470, L5800–L6690, L10750+.  
Constants already exist in `src/tui/constants.rs` (`AUTOCHAT_*`) but no implementation.

- [ ] `autochat_rx: Option<mpsc::UnboundedReceiver<AutochatUiEvent>>` field
- [ ] `autochat_running: bool`, `autochat_started_at`, `autochat_status` fields
- [ ] `AutochatUiEvent` enum (or import from existing module)
- [ ] `handle_autochat_event()` handler
- [ ] `start_autochat_execution()` — launch relay with model rotation
- [ ] `resume_autochat_relay()` — resume from checkpoint
- [ ] `RelayCheckpoint` struct for crash recovery (serialize/deserialize)
- [ ] `/autochat [count] <task>` command
- [ ] `/autochat-local [count] <task>` command (forced local CUDA)
- [ ] `/resume [session_id]` command
- [ ] Event loop branch for `autochat_rx`

### OKR Approval Gate (~200 lines)

Depends on autochat. Old code: `tui_old.rs` L1280–L1360.

- [ ] `PendingOkrApproval` struct with `Okr`, `OkrRun`, task, model
- [ ] `pending_okr_approval: Option<PendingOkrApproval>` field
- [ ] `okr_repository: Option<Arc<OkrRepository>>` field
- [ ] `propose()` async method — draft OKR with model
- [ ] Interactive A/D key approval flow in event handlers
- [ ] `/go <task>` command (OKR-gated relay)
- [ ] Integration with autochat start flow

### Smart Model Switching / Watchdog (~300 lines)

Old code: `tui_old.rs` L10800–L11000, scattered through event handlers.  
Constants + tests already exist (`SMART_SWITCH_MAX_RETRIES`, `SMART_SWITCH_PROVIDER_PRIORITY`).

- [ ] `smart_switch_retry_count`, `smart_switch_attempted_models` fields
- [ ] `PendingSmartSwitchRetry` struct
- [ ] `is_retryable_provider_error()` — classify errors by retryability
- [ ] `smart_switch_preferred_models()` — per-provider fallback model list
- [ ] `maybe_schedule_smart_switch_retry()` — trigger on session error
- [ ] `apply_pending_smart_switch_retry()` — execute the retry
- [ ] Watchdog timer fields: `main_watchdog_root_prompt`, `main_last_event_at`, `main_watchdog_restart_count`
- [ ] Watchdog timer in event loop (detect stuck requests, auto-restart)
- [ ] Hook into `session_events.rs` Error handler to trigger smart switch

---

## Tier 2 — Significant UI Features

### File Picker (~200 lines)

Old code: `tui_old.rs` L245–L280, L8248, render fn ~L9600.  
Constants exist (`FILE_PICKER_MAX_ENTRIES`, `FILE_PICKER_PREVIEW_*`). Current `/file` just does raw path attachment.

- [ ] `FilePickerEntry` struct with `name`, `kind` (Parent/Directory/File)
- [ ] `file_picker_dir`, `file_picker_entries`, `file_picker_selected`, `file_picker_filter` fields
- [ ] `open_file_picker()` — populate entries from filesystem
- [ ] `attach_file_to_input()` — insert path into input buffer
- [ ] `ViewMode::FilePicker` variant
- [ ] `Ctrl+O` key binding → file picker
- [ ] Render function with directory listing, filter, preview
- [ ] Navigation: `↑↓` select, `Enter` open/attach, `Esc` cancel, typing filters

### /undo Command (~50 lines)

Old code: `tui_old.rs` L5212–L5270.

- [ ] Walk messages backwards, remove last user turn + all assistant/tool responses
- [ ] Also remove from `session.messages` and call `session.save()`
- [ ] `/undo` slash command registration
- [ ] "Nothing to undo" guard

### Chat Sync / MinIO Persistence (~250 lines)

Old code: `tui_old.rs` L460–L500, L1073–L1130, L6690–L6760.  
New code has a 9-line stub in `src/tui/chat/sync.rs`.

- [ ] `ChatSyncConfig` struct (endpoint, credentials, bucket)
- [ ] `ChatSyncBatch` struct
- [ ] `sync_chat_archive_batch()` — MinIO upload logic
- [ ] `run_chat_sync_worker()` — background worker with batch upload
- [ ] `ChatSyncUiEvent` variants (Status, BatchUploaded, Error)
- [ ] `handle_chat_sync_event()` handler
- [ ] `to_archive_record()` for message serialization
- [ ] `chat_sync_rx` field + event loop branch
- [ ] Status fields: `chat_sync_status`, `chat_sync_last_success`, `chat_sync_last_error`, `chat_sync_uploaded_*`

### Protocol Registry View (~200 lines)

Old code: `tui_old.rs` L9450–L9600 (render), L8266 (Ctrl+P), L8630–L8674 (ui dispatch).  
New `/bus` shows bus log but not protocol card registry.

- [ ] `render_protocol_registry()` — master/detail layout for registered AgentCards
- [ ] `protocol_selected`, `protocol_scroll` state fields
- [ ] `ViewMode::Protocol` variant
- [ ] `Ctrl+P` key binding → protocol registry
- [ ] `/protocol` command switches to Protocol view (currently aliased to `/bus`)

---

## Tier 3 — Layout & Polish

### Webview Layout (~800 lines)

Old code: `tui_old.rs` L9352–L9450 (render_webview_chat), plus helper fns.  
Alternative three-pane layout with sidebar, chat center, optional inspector.

- [ ] `ChatLayoutMode` enum (Classic, Webview)
- [ ] `chat_layout`, `show_inspector` fields
- [ ] `render_webview_chat()` — top-level webview renderer
- [ ] `render_webview_header()` — header bar with model/status
- [ ] `render_webview_sidebar()` — session list sidebar
- [ ] `render_webview_chat_center()` — main chat area (uses message cache)
- [ ] `render_webview_inspector()` — inspector pane (tool calls, context)
- [ ] `render_webview_input()` — input area
- [ ] `/webview` command
- [ ] `/classic` command
- [ ] `/inspector` command — toggle inspector pane
- [ ] `Ctrl+B` key binding → toggle layout
- [ ] `F3` key binding → toggle inspector (currently mapped to Ralph view)

### Message Line Caching (~40 lines)

Old code: `tui_old.rs` L6750–L6800.  
New code rebuilds message lines every frame — performance cost on long chats.

- [ ] `cached_message_lines: Vec<Line<'static>>` field
- [ ] `cached_messages_len`, `cached_max_width` fields
- [ ] `cached_streaming_snapshot`, `cached_processing`, `cached_autochat_running`
- [ ] `get_or_build_message_lines()` — only rebuild when content changes

### Easy-Mode Command Aliases (~80 lines)

Old code: `tui_old.rs` L10750–L10850.  
User-friendly aliases mapping to advanced commands.

- [ ] `normalize_easy_command()` function
- [ ] `/add <name>` → `/spawn <name>`
- [ ] `/talk <name> <msg>` → `/agent <name> <msg>`
- [ ] `/list` → `/agents`
- [ ] `/remove <name>` → `/kill <name>`
- [ ] `/focus <name>` → `/agent <name>`
- [ ] `/home` → `/agent main` (exit focused agent)
- [ ] `/go <task>` → OKR-gated autochat with model rotation
- [ ] `is_easy_go_command()`, `next_go_model()` for model rotation

---

## Missing Key Bindings

| Binding | Function | Old Location | Status |
|---------|----------|-------------|--------|
| `Ctrl+A` | Agent picker | L8253 | ❌ Missing |
| `Ctrl+O` | File picker | L8248 | ❌ Missing |
| `Ctrl+B` | Toggle webview layout | L8220 | ❌ Missing |
| `Ctrl+L` | Bus protocol log | L8258 | ❌ Missing |
| `Ctrl+P` | Protocol registry | L8266 | ❌ Missing |
| `Ctrl+S` | Toggle swarm view | L8122 | ❌ Missing |
| `F3` | Toggle inspector pane | L8220 | ⚠️ Remapped to Ralph |

## Missing Event Loop Branches

| Branch | Purpose | Old Location |
|--------|---------|-------------|
| `autochat_rx` | Autochat UI events | `run_app()` select! |
| Per-agent `response_rx` | Spawned agent events | `run_app()` select! |
| `chat_sync_rx` | Chat sync status | `run_app()` select! |
| Watchdog timer | Stuck request detection | `run_app()` select! |

## Missing ViewMode Variants

| Variant | Purpose | Status |
|---------|---------|--------|
| `Protocol` | Protocol AgentCard registry | ❌ Missing |
| `AgentPicker` | Spawned agent picker | ❌ Missing |
| `FilePicker` | Interactive file browser | ❌ Missing |

---

## Porting Order (Recommended)

1. **Spawned Sub-Agents** — foundational; autochat, OKR gate, and easy-mode all depend on it
2. **Smart Switch / Watchdog** — standalone; improves reliability immediately
3. **File Picker** — standalone UI feature
4. **/undo** — small, standalone
5. **Autochat / Relay** — depends on #1
6. **OKR Approval Gate** — depends on #5
7. **Easy-Mode Aliases** — depends on #1 and #5
8. **Chat Sync / MinIO** — standalone background system
9. **Protocol Registry View** — standalone UI view
10. **Message Line Caching** — performance optimization
11. **Webview Layout** — largest chunk, lowest priority (alternative layout)
