//! Initial values for the application-local TUI state.
//!
//! Component construction stays explicit at the declaration site; this module
//! owns the existing view selection, flags, counters, buffers and status text.
/// Implement Default for the local TUI state using existing component factories.
///
/// Factories are evaluated in the original state-field order.
/// # Usage
/// Supply the local state type followed by named constructor paths for bus,
/// swarm, audit, git, Ralph, audit-loop, symbols, help, model picker, MCP,
/// autochat, workspace, layout and forage components.
#[macro_export]
macro_rules! impl_app_state_default {
    ($state:ty; bus = $bus:path, swarm = $swarm:path, audit = $audit:path, git = $git:path, ralph = $ralph:path, audit_loop = $audit_loop:path, symbols = $symbols:path, help = $help:path, models = $models:path, mcp = $mcp:path, autochat = $autochat:path, workspace = $workspace:path, layout = $layout:path, forage = $forage:path) => {
impl ::core::default::Default for $state {
    #[rustfmt::skip]
    fn default() -> Self {
        Self {
            view: ::core::default::Default::default(),
            messages: ::std::vec![],
            composer: ::core::default::Default::default(),
            chat_viewport: ::core::default::Default::default(),
            history_page: ::core::default::Default::default(), approval_preview_scroll: 0, tool_viewport: ::core::default::Default::default(),
            protocol_registry: ::core::default::Default::default(),
            status: "Ready — type a message and press Enter. Ctrl+C/Ctrl+Q quits.".to_string(),
            processing: false, approval_waiting: false,
            session_id: None,
            session_picker: ::core::default::Default::default(),
            cwd_display: ::std::string::String::new(),
            bus_log: $bus(),
            swarm: $swarm(),
            audit: $audit(),
            git: $git(),
            ralph: $ralph(),
            audit_loop: $audit_loop(),
            symbol_search: $symbols(),
            worker: ::core::default::Default::default(),
            peer_endpoint_ready: false,
            active_tasks: ::core::default::Default::default(), tool_calls: ::core::default::Default::default(), bus_cursor: 0,
            live_inbound: None,
            help: $help(),
            model_picker: $models(),
            streaming: ::core::default::Default::default(),
            request_timing: ::core::default::Default::default(),
            last_completion: ::core::default::Default::default(),
            context_health: ::core::default::Default::default(),
            tool_timing: ::core::default::Default::default(),
            pending_images: ::std::vec::Vec::new(),
            pending_text_pastes: ::std::vec::Vec::new(),
            current_turn_cancel: None,
            preferences: ::core::default::Default::default(),
            settings_panel: ::core::default::Default::default(),
            mcp_registry: ::std::sync::Arc::new($mcp()),
            agent_tree: ::core::default::Default::default(),
            message_cache: ::core::default::Default::default(),
            watchdog: ::core::default::Default::default(),
            okr_repository: None,
            pending_okr_approval: None,
            retry_state: ::core::default::Default::default(),
            stream_recovery: ::core::default::Default::default(),
            chat_sync: ::core::default::Default::default(),
            autochat: $autochat(),
            file_picker: ::core::default::Default::default(),
            fuzzy_find: None,
            workspace: $workspace(), goal_prompt: None, answer_review_yes: false,
            chat_layout_mode: $layout(),
            last_key_at: None,
            recording: ::core::default::Default::default(),
            forage: $forage(),
            needs_redraw: true,
            shell: ::core::default::Default::default(),
            editor: ::core::default::Default::default(), goal_editor: None, approval_edit: None, chat_hit: ::core::default::Default::default(), editor_lsp: ::core::default::Default::default(),
            spawn_form: None, interlude: None,
        }
    }
}

    };
}
