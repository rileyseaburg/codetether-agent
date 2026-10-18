//! Core TUI state containers.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, atomic::AtomicBool};
use std::time::Instant;

use tokio::sync::{Notify, mpsc::UnboundedReceiver};

use crate::session::ImageAttachment;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::symbol_search::SymbolSearchState;
use crate::tui::{audit_view::AuditViewState, bus_log::BusLogState};
use crate::tui::{chat::message::ChatMessage, help::HelpScrollState};
use crate::tui::{ralph_view::RalphViewState, swarm_view::SwarmViewState};

use super::{agent_profile::SpawnedAgent, context_health, git_state};

type ChatSyncRx = Option<UnboundedReceiver<crate::tui::chat::sync::ChatSyncUiEvent>>;
type ShellRx = Option<UnboundedReceiver<crate::tui::app::input::shell_bg::ShellEvent>>;
type PendingPaste = crate::tui::app::input::pasted_text::PendingTextPaste;
type WatchdogNotice = Option<crate::tui::app::watchdog::WatchdogNotification>;
type PendingOkr = Option<crate::tui::app::okr_gate::PendingOkrApproval>;
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
/// assert!(state.chat_viewport.chat_auto_follow);
/// ```
#[rustfmt::skip]
pub struct AppState {
    pub view_mode: ViewMode, pub input_mode: InputMode, pub messages: Vec<ChatMessage>, pub input: String, pub input_cursor: usize, pub input_scroll: usize,
    pub chat_viewport: codetether_chat_layout::ChatScrollState, pub(crate) history_page: super::history_page::HistoryPageState, pub approval_preview_scroll: u16, pub tool_viewport: codetether_tool_panel::ToolPreviewScrollState,
    pub protocol_selected: usize, pub protocol_scroll: usize, pub status: String, pub processing: bool, pub approval_waiting: bool, pub session_id: Option<String>, pub session_picker: codetether_session_picker::SessionPickerState, pub cwd_display: String,
    pub bus_log: BusLogState, pub swarm: SwarmViewState, pub audit: AuditViewState, pub git: git_state::GitViewState, pub ralph: RalphViewState, pub audit_loop: crate::tui::audit_loop_view::audit_loop_state::AuditLoopState, pub symbol_search: SymbolSearchState,
    pub completion: codetether_slash_command::CompletionState, pub history: codetether_composer_view::CommandHistory,
    pub worker: codetether_tui_worker::WorkerState, pub peer_endpoint_ready: bool, pub active_tasks: crate::tui::app::bus::active_tasks::ActiveTasks, pub tool_calls: crate::tui::app::bus::tool_calls::ToolCallTracker, pub bus_cursor: u64, pub live_inbound: Option<crate::a2a::live_inbox::Inbound>,
    pub help_scroll: HelpScrollState, pub show_help: bool, pub model_picker: codetether_model_picker::ModelPickerState,
    pub streaming: codetether_chat_model::StreamingState, pub request_timing: codetether_telemetry::RequestTimingState, pub last_completion_model: Option<String>, pub last_completion_latency_ms: Option<u64>, pub last_completion_prompt_tokens: Option<usize>, pub last_completion_output_tokens: Option<usize>,
    pub context_used: Option<usize>, pub context_budget: Option<usize>, pub context_health: context_health::ContextHealthState, pub tool_timing: codetether_telemetry::ToolTimingState,
    pub pending_images: Vec<ImageAttachment>, pub pending_text_pastes: Vec<PendingPaste>, pub auto_apply_edits: bool, pub allow_network: bool, pub slash_autocomplete: bool, pub use_worktree: bool, pub selected_settings_index: usize, pub current_turn_cancel: TurnCancel, pub last_key_at: Option<Instant>,
    pub mcp_registry: Arc<crate::tui::app::mcp::TuiMcpRegistry>, pub spawned_agents: HashMap<String, SpawnedAgent>, pub active_spawned_agent: Option<String>, pub subagent_detail_mode: bool, pub subagent_detail_scroll: u16, pub streaming_agent_texts: HashMap<String, String>, pub message_cache: codetether_message_cache::RenderedLineCache,
    pub watchdog_notification: WatchdogNotice, pub main_last_event_at: Option<Instant>, pub main_watchdog_restart_count: u32, pub main_inflight_prompt: Option<String>, pub okr_repository: Option<Arc<crate::okr::OkrRepository>>, pub pending_okr_approval: PendingOkr, pub retry_state: codetether_model_failover::RetryState,
    pub stream_recovery: codetether_stream_recovery::StreamRecoveryState,
    pub chat_sync_rx: ChatSyncRx, pub chat_sync_status: Option<String>, pub chat_sync_last_success: Option<String>, pub chat_sync_last_error: Option<String>, pub chat_sync_uploaded_bytes: u64, pub chat_sync_uploaded_batches: u64,
    pub autochat: crate::tui::app::autochat::state::AutochatState, pub file_picker: crate::tui::app::file_picker::FilePickerState, pub workspace: crate::tui::models::WorkspaceSnapshot, pub goal_prompt: Option<crate::tui::app::goal_prompt::GoalPromptState>, pub(crate) answer_review_yes: bool, pub fuzzy_find: Option<crate::tui::app::fuzzy_find::FuzzyFindState>, pub chat_layout_mode: crate::tui::ui::webview::layout_mode::ChatLayoutMode,
    pub recording_stop_flag: VoiceStopFlag, pub pending_voice_text: VoiceTextSlot, pub saved_chat_scroll: usize, pub saved_chat_auto_follow: bool, pub saved_tool_preview_scroll: usize, pub forage: crate::tui::forage_run::ForageState, pub needs_redraw: bool, pub shell_rx: ShellRx, pub shell_running: bool,
    /// Active in-TUI editor buffer, present only while in `ViewMode::Editor`.
    pub editor: Option<crate::tui::ui::editor::FileBuffer>, pub(crate) goal_editor: Option<crate::tui::app::input::goal_answer::editor::Draft>, pub(crate) approval_edit: Option<crate::tui::app::state::approval_queue::edit_session::ApprovalEditSession>, pub editor_scroll: usize, pub editor_hscroll: usize, pub chat_hit: crate::tui::ui::chat_view::hit::ChatHit,
    /// Editor LSP state: shared manager and transient hover/JSDoc popup.
    pub editor_lsp: crate::tui::ui::editor::lsp_state::EditorLspState,
    pub spawn_form: Option<crate::tui::app::spawn_form::SpawnFormState>, pub(crate) interlude: Option<crate::tui::app::state::interlude::InterludeState>,
}
