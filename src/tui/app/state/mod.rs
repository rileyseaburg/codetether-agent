//! TUI application state — the central `AppState` struct and its methods.
//!
//! Methods are split into focused submodules by concern:
//!
//! - `agent_profile` — spawned agent types and profile lookup
//! - `profile_defs` — named profile constants
//! - `slash_commands` — `/command` constant table
//! - `input_cursor` — cursor movement and text editing
//! - `slash_suggest` — slash autocomplete suggestion methods
//! - `scroll` — chat scroll sentinel scheme and tool-preview scroll
//! - `session_nav` — session list filtering and selection
//! - `worker_bridge` — A2A worker connection state
//! - `history` — command history ↑/↓ navigation
//! - `model_picker` — async model refresh from providers
//! - `model_picker_nav` — synchronous model picker navigation
//! - `timing` — request latency tracking
//! - `steering` — queued steering messages
//! - `settings_nav` — settings selection and view-mode switching
//! - `message_cache` — render-line cache for performance

#![allow(dead_code)]

pub mod agent_profile;
pub mod default_impl;
pub mod history;
pub mod input_cursor;
pub mod input_edit;
pub mod message_cache;
pub mod model_picker;
pub mod model_picker_nav;
pub mod profile_defs;
pub mod scroll;
pub mod session_nav;
pub mod settings_nav;
pub mod slash_commands;
pub mod slash_suggest;
pub mod steering;
pub mod timing;
pub mod worker_bridge;

#[cfg(test)]
mod tests;

// Re-exports so external `use crate::tui::app::state::X` still works.
pub use agent_profile::{SpawnedAgent, agent_profile};
pub use profile_defs::AgentProfile;
pub use slash_commands::SLASH_COMMANDS;

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;

use crate::session::{ImageAttachment, Session, SessionSummary};
use crate::tui::bus_log::BusLogState;
use crate::tui::chat::message::ChatMessage;
use crate::tui::help::HelpScrollState;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::RalphViewState;
use crate::tui::swarm_view::SwarmViewState;
use crate::tui::symbol_search::SymbolSearchState;
use crate::tui::worker_bridge::IncomingTask;

pub use crate::session::SessionEvent;

#[derive(Default)]
pub struct App {
    pub state: AppState,
}

pub struct AppState {
    pub view_mode: ViewMode,
    pub input_mode: InputMode,
    pub messages: Vec<ChatMessage>,
    pub input: String,
    pub input_cursor: usize,
    pub input_scroll: usize,
    pub chat_scroll: usize,
    pub chat_last_max_scroll: usize,
    pub tool_preview_scroll: usize,
    pub tool_preview_last_max_scroll: usize,
    pub status: String,
    pub processing: bool,
    pub session_id: Option<String>,
    pub sessions: Vec<SessionSummary>,
    pub selected_session: usize,
    pub session_filter: String,
    pub cwd_display: String,
    pub bus_log: BusLogState,
    pub swarm: SwarmViewState,
    pub ralph: RalphViewState,
    pub symbol_search: SymbolSearchState,
    pub slash_suggestions: Vec<String>,
    pub selected_slash_suggestion: usize,
    pub command_history: Vec<String>,
    pub history_index: Option<usize>,
    pub worker_id: Option<String>,
    pub worker_name: Option<String>,
    pub a2a_connected: bool,
    pub recent_tasks: Vec<String>,
    pub worker_bridge_registered_agents: HashSet<String>,
    pub worker_bridge_processing_state: Option<bool>,
    pub worker_task_queue: VecDeque<IncomingTask>,
    pub help_scroll: HelpScrollState,
    pub show_help: bool,
    pub available_models: Vec<String>,
    pub selected_model_index: usize,
    pub model_picker_active: bool,
    pub model_filter: String,
    pub streaming_text: String,
    pub processing_started_at: Option<Instant>,
    pub current_request_first_token_ms: Option<u64>,
    pub current_request_last_token_ms: Option<u64>,
    pub last_request_first_token_ms: Option<u64>,
    pub last_request_last_token_ms: Option<u64>,
    pub last_completion_model: Option<String>,
    pub last_completion_latency_ms: Option<u64>,
    pub last_completion_prompt_tokens: Option<usize>,
    pub last_completion_output_tokens: Option<usize>,
    pub last_tool_name: Option<String>,
    pub last_tool_latency_ms: Option<u64>,
    pub last_tool_success: Option<bool>,
    pub pending_images: Vec<ImageAttachment>,
    pub queued_steering: Vec<String>,
    pub auto_apply_edits: bool,
    pub allow_network: bool,
    pub slash_autocomplete: bool,
    pub use_worktree: bool,
    pub selected_settings_index: usize,
    pub mcp_registry: Arc<crate::tui::app::mcp::TuiMcpRegistry>,
    pub spawned_agents: HashMap<String, SpawnedAgent>,
    pub active_spawned_agent: Option<String>,
    pub streaming_agent_texts: HashMap<String, String>,
    pub cached_message_lines: Vec<ratatui::text::Line<'static>>,
    pub cached_messages_len: usize,
    pub cached_max_width: usize,
    pub cached_streaming_snapshot: Option<String>,
    pub cached_processing: bool,
    pub cached_frozen_len: usize,
    pub watchdog_notification: Option<super::watchdog::WatchdogNotification>,
    pub main_watchdog_root_prompt: Option<String>,
    pub main_last_event_at: Option<Instant>,
    pub main_watchdog_restart_count: u32,
    pub main_inflight_prompt: Option<String>,
    pub okr_repository: Option<Arc<crate::okr::OkrRepository>>,
    pub pending_okr_approval: Option<super::okr_gate::PendingOkrApproval>,
    pub pending_smart_switch_retry: Option<crate::tui::app::smart_switch::PendingSmartSwitchRetry>,
    pub smart_switch_retry_count: u32,
    pub smart_switch_attempted_models: Vec<String>,
    pub chat_sync_rx:
        Option<tokio::sync::mpsc::UnboundedReceiver<crate::tui::chat::sync::ChatSyncUiEvent>>,
    pub chat_sync_status: Option<String>,
    pub chat_sync_last_success: Option<String>,
    pub chat_sync_last_error: Option<String>,
    pub chat_sync_uploaded_bytes: u64,
    pub chat_sync_uploaded_batches: u64,
    pub autochat: super::autochat::state::AutochatState,
    pub file_picker_dir: std::path::PathBuf,
    pub file_picker_entries: Vec<crate::tui::app::file_picker::FilePickerEntry>,
    pub file_picker_selected: usize,
    pub file_picker_filter: String,
    pub file_picker_active: bool,
    pub workspace: crate::tui::models::WorkspaceSnapshot,
    pub chat_layout_mode: crate::tui::ui::webview::layout_mode::ChatLayoutMode,
}
