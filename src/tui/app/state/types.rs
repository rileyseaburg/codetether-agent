//! Core TUI state containers.

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::{Arc, Mutex, atomic::AtomicBool};
use std::time::Instant;

use ratatui::text::Line;
use tokio::sync::{Notify, mpsc::UnboundedReceiver};

use crate::session::{ImageAttachment, SessionSummary};
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::{audit_view::AuditViewState, bus_log::BusLogState};
use crate::tui::{chat::message::ChatMessage, help::HelpScrollState};
use crate::tui::{ralph_view::RalphViewState, swarm_view::SwarmViewState};
use crate::tui::{symbol_search::SymbolSearchState, worker_bridge::IncomingTask};

use super::{agent_profile::SpawnedAgent, chat_latency, context_health, git_state, model_picker};

type ModelRefreshRx = Option<UnboundedReceiver<model_picker::ModelRefreshEvent>>;
type ChatSyncRx = Option<UnboundedReceiver<crate::tui::chat::sync::ChatSyncUiEvent>>;
type ShellRx = Option<UnboundedReceiver<crate::tui::app::input::shell_bg::ShellEvent>>;
type PendingPaste = crate::tui::app::input::pasted_text::PendingTextPaste;
type WatchdogNotice = Option<crate::tui::app::watchdog::WatchdogNotification>;
type PendingOkr = Option<crate::tui::app::okr_gate::PendingOkrApproval>;
type SmartRetry = Option<crate::tui::app::smart_switch::PendingSmartSwitchRetry>;
type TurnCancel = Option<Arc<Notify>>;
type VoiceStopFlag = Option<Arc<AtomicBool>>;
type VoiceTextSlot = Option<Arc<Mutex<Option<String>>>>;

/// Root container for the interactive TUI.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::app::state::App;
///
/// let app = App::default();
/// assert!(app.state.needs_redraw);
/// ```
#[derive(Default)]
pub struct App {
    pub state: AppState,
}

/// Mutable state backing all TUI views, input handling, and background tasks.
///
/// # Examples
///
/// ```
/// use codetether_agent::tui::app::state::AppState;
///
/// let state = AppState::default();
/// assert!(state.chat_auto_follow);
/// ```
#[rustfmt::skip]
pub struct AppState {
    pub view_mode: ViewMode, pub input_mode: InputMode, pub messages: Vec<ChatMessage>, pub input: String, pub input_cursor: usize, pub input_scroll: usize,
    pub chat_scroll: usize, pub chat_last_max_scroll: usize, pub chat_auto_follow: bool, pub tool_preview_scroll: usize, pub tool_preview_last_max_scroll: usize,
    pub protocol_selected: usize, pub protocol_scroll: usize, pub status: String, pub processing: bool, pub session_id: Option<String>, pub sessions: Vec<SessionSummary>, pub selected_session: usize, pub session_filter: String, pub cwd_display: String,
    pub bus_log: BusLogState, pub swarm: SwarmViewState, pub audit: AuditViewState, pub git: git_state::GitViewState, pub ralph: RalphViewState, pub audit_loop: crate::tui::audit_loop_view::audit_loop_state::AuditLoopState, pub symbol_search: SymbolSearchState,
    pub slash_suggestions: Vec<String>, pub selected_slash_suggestion: usize, pub command_history: Vec<String>, pub history_index: Option<usize>,
    pub worker_id: Option<String>, pub worker_name: Option<String>, pub a2a_connected: bool, pub peer_endpoint_ready: bool, pub recent_tasks: Vec<String>, pub worker_bridge_registered_agents: HashSet<String>, pub active_tasks: crate::tui::app::bus::active_tasks::ActiveTasks, pub tool_calls: crate::tui::app::bus::tool_calls::ToolCallTracker, pub bus_cursor: u64, pub worker_bridge_processing_state: Option<bool>, pub worker_task_queue: VecDeque<IncomingTask>, pub active_remote_task: Option<IncomingTask>,
    pub help_scroll: HelpScrollState, pub show_help: bool, pub available_models: Vec<String>, pub selected_model_index: usize, pub model_picker_active: bool, pub model_filter: String, pub model_refresh_in_flight: bool, pub model_refresh_rx: ModelRefreshRx, pub model_picker_target_model: Option<String>,
    pub streaming_text: String, pub processing_started_at: Option<Instant>, pub current_request_first_token_ms: Option<u64>, pub current_request_last_token_ms: Option<u64>, pub last_request_first_token_ms: Option<u64>, pub last_request_last_token_ms: Option<u64>, pub last_completion_model: Option<String>, pub last_completion_latency_ms: Option<u64>, pub last_completion_prompt_tokens: Option<usize>, pub last_completion_output_tokens: Option<usize>,
    pub context_used: Option<usize>, pub context_budget: Option<usize>, pub context_health: context_health::ContextHealthState, pub last_tool_name: Option<String>, pub last_tool_latency_ms: Option<u64>, pub last_tool_success: Option<bool>, pub pending_tool_name: Option<String>, pub pending_tool_started_at: Option<Instant>, pub chat_latency: chat_latency::ChatLatencyStats,
    pub pending_images: Vec<ImageAttachment>, pub pending_text_pastes: Vec<PendingPaste>, pub auto_apply_edits: bool, pub allow_network: bool, pub slash_autocomplete: bool, pub use_worktree: bool, pub selected_settings_index: usize, pub current_turn_cancel: TurnCancel, pub last_key_at: Option<Instant>,
    pub mcp_registry: Arc<crate::tui::app::mcp::TuiMcpRegistry>, pub spawned_agents: HashMap<String, SpawnedAgent>, pub active_spawned_agent: Option<String>, pub streaming_agent_texts: HashMap<String, String>, pub cached_message_lines: Vec<Line<'static>>, pub cached_messages_len: usize, pub cached_max_width: usize, pub cached_streaming_snapshot: Option<String>, pub cached_processing: bool, pub cached_tool_preview_scroll: usize, pub cached_frozen_len: usize,
    pub watchdog_notification: WatchdogNotice, pub main_watchdog_root_prompt: Option<String>, pub main_last_event_at: Option<Instant>, pub main_watchdog_restart_count: u32, pub main_inflight_prompt: Option<String>, pub okr_repository: Option<Arc<crate::okr::OkrRepository>>, pub pending_okr_approval: PendingOkr, pub pending_smart_switch_retry: SmartRetry, pub smart_switch_retry_count: u32, pub smart_switch_attempted_models: Vec<String>,
    pub chat_sync_rx: ChatSyncRx, pub chat_sync_status: Option<String>, pub chat_sync_last_success: Option<String>, pub chat_sync_last_error: Option<String>, pub chat_sync_uploaded_bytes: u64, pub chat_sync_uploaded_batches: u64,
    pub autochat: crate::tui::app::autochat::state::AutochatState, pub file_picker: crate::tui::app::file_picker::FilePickerState, pub workspace: crate::tui::models::WorkspaceSnapshot, pub goal_prompt: Option<crate::tui::app::goal_prompt::GoalPromptState>, pub chat_layout_mode: crate::tui::ui::webview::layout_mode::ChatLayoutMode,
    pub recording_stop_flag: VoiceStopFlag, pub pending_voice_text: VoiceTextSlot, pub saved_chat_scroll: usize, pub saved_chat_auto_follow: bool, pub saved_tool_preview_scroll: usize, pub streaming_start: Option<Instant>, pub streaming_chars: usize, pub forage: crate::tui::forage_run::ForageState, pub needs_redraw: bool, pub shell_rx: ShellRx, pub shell_running: bool,
    /// Active in-TUI editor buffer, present only while in `ViewMode::Editor`.
    pub editor: Option<crate::tui::ui::editor::FileBuffer>, pub editor_scroll: usize,
}
