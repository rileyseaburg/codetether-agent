//! Core TUI state containers.

use std::sync::Arc;
use std::time::Instant;

use tokio::sync::Notify;

use crate::session::ImageAttachment;
use crate::tui::chat::message::ChatMessage;
use crate::tui::models::ViewMode;
use crate::tui::symbol_search::SymbolSearchState;
use crate::tui::{audit_view::AuditViewState, bus_log::BusLogState};
use crate::tui::{ralph_view::RalphViewState, swarm_view::SwarmViewState};

use super::{context_health, git_state};

type PendingPaste = crate::tui::app::input::pasted_text::PendingTextPaste;
type PendingOkr = Option<crate::tui::app::okr_gate::PendingOkrApproval>;
type TurnCancel = Option<Arc<Notify>>;

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
    pub view_mode: ViewMode, pub messages: Vec<ChatMessage>, pub composer: codetether_composer_view::ComposerState,
    pub chat_viewport: codetether_chat_layout::ChatScrollState, pub(crate) history_page: super::history_page::HistoryPageState, pub approval_preview_scroll: u16, pub tool_viewport: codetether_tool_panel::ToolPreviewScrollState,
    pub protocol_registry: codetether_protocol_view::RegistryState, pub status: String, pub processing: bool, pub approval_waiting: bool, pub session_id: Option<String>, pub session_picker: codetether_session_picker::SessionPickerState, pub cwd_display: String,
    pub bus_log: BusLogState, pub swarm: SwarmViewState, pub audit: AuditViewState, pub git: git_state::GitViewState, pub ralph: RalphViewState, pub audit_loop: crate::tui::audit_loop_view::audit_loop_state::AuditLoopState, pub symbol_search: SymbolSearchState,
    pub worker: codetether_tui_worker::WorkerState, pub peer_endpoint_ready: bool, pub active_tasks: crate::tui::app::bus::active_tasks::ActiveTasks, pub tool_calls: crate::tui::app::bus::tool_calls::ToolCallTracker, pub bus_cursor: u64, pub live_inbound: Option<crate::a2a::live_inbox::Inbound>,
    pub help: codetether_help_view::HelpState, pub model_picker: codetether_model_picker::ModelPickerState,
    pub streaming: codetether_chat_model::StreamingState, pub request_timing: codetether_telemetry::RequestTimingState, pub last_completion: codetether_telemetry::CompletionSummary,
    pub context_health: context_health::ContextHealthState, pub tool_timing: codetether_telemetry::ToolTimingState,
    pub pending_images: Vec<ImageAttachment>, pub pending_text_pastes: Vec<PendingPaste>, pub preferences: codetether_settings_policy::persistence::Preferences, pub settings_panel: codetether_settings_policy::panel::PanelState, pub current_turn_cancel: TurnCancel, pub last_key_at: Option<Instant>,
    pub mcp_registry: Arc<crate::tui::app::mcp::TuiMcpRegistry>, pub agent_tree: codetether_agent_tree::AgentTreeState<crate::session::Session>, pub message_cache: codetether_message_cache::RenderedLineCache,
    pub watchdog: codetether_watchdog::WatchdogState, pub okr_repository: Option<Arc<crate::okr::OkrRepository>>, pub pending_okr_approval: PendingOkr, pub retry_state: codetether_model_failover::RetryState,
    pub stream_recovery: codetether_stream_recovery::StreamRecoveryState,
    pub chat_sync: codetether_chat_sync::ChatSyncState,
    pub autochat: crate::tui::app::autochat::state::AutochatState, pub file_picker: crate::tui::app::file_picker::FilePickerState, pub workspace: crate::tui::models::WorkspaceSnapshot, pub goal_prompt: Option<crate::tui::app::goal_prompt::GoalPromptState>, pub(crate) answer_review_yes: bool, pub fuzzy_find: Option<crate::tui::app::fuzzy_find::FuzzyFindState>, pub chat_layout_mode: crate::tui::ui::webview::layout_mode::ChatLayoutMode,
    pub recording: codetether_voice_capture::RecordingState, pub saved_scroll: codetether_tui_navigation::view_state::SavedScroll, pub forage: crate::tui::forage_run::ForageState, pub needs_redraw: bool, pub shell: codetether_chat_shell::State,
    /// Active editor document and its viewport offsets.
    pub editor: codetether_editor::EditorState, pub(crate) goal_editor: Option<crate::tui::app::input::goal_answer::editor::Draft>, pub(crate) approval_edit: Option<crate::tui::app::state::approval_queue::edit_session::ApprovalEditSession>, pub chat_hit: crate::tui::ui::chat_view::hit::ChatHit,
    /// Editor LSP state: shared manager and transient hover/JSDoc popup.
    pub editor_lsp: crate::tui::ui::editor::lsp_state::EditorLspState,
    pub spawn_form: Option<crate::tui::app::spawn_form::SpawnFormState>, pub(crate) interlude: Option<crate::tui::app::state::interlude::InterludeState>,
}
