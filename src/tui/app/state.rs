#![allow(dead_code)]

use std::collections::{HashMap, HashSet, VecDeque};
use std::sync::Arc;
use std::time::Instant;

use ratatui::text::Line;

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionSummary};
use crate::tui::app::text::normalize_slash_command;
use crate::tui::bus_log::BusLogState;
use crate::tui::chat::message::ChatMessage;
use crate::tui::help::HelpScrollState;
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::ralph_view::RalphViewState;
use crate::tui::swarm_view::SwarmViewState;
use crate::tui::symbol_search::SymbolSearchState;
use crate::tui::worker_bridge::IncomingTask;

pub use crate::session::SessionEvent;

const SLASH_COMMANDS: &[&str] = &[
    "/help",
    "/sessions",
    "/swarm",
    "/ralph",
    "/bus",
    "/protocol",
    "/file",
    "/autoapply",
    "/network",
    "/autocomplete",
    "/mcp",
    "/model",
    "/settings",
    "/lsp",
    "/rlm",
    "/latency",
    "/symbols",
    "/chat",
    "/new",
    "/keys",
    "/spawn",
    "/kill",
    "/agents",
    "/agent",
    // easy-mode aliases
    "/add",
    "/talk",
    "/say",
    "/list",
    "/ls",
    "/remove",
    "/rm",
    "/focus",
    "/home",
    "/main",
];

/// A spawned sub-agent with its own independent LLM session.
#[allow(dead_code)]
pub struct SpawnedAgent {
    /// User-facing name (e.g. "planner", "coder")
    pub name: String,
    /// System instructions for this agent
    pub instructions: String,
    /// Independent conversation session
    pub session: Session,
    /// Whether this agent is currently processing a message
    pub is_processing: bool,
}

#[derive(Debug, Clone, Copy)]
pub struct AgentProfile {
    pub codename: &'static str,
    pub profile: &'static str,
    pub personality: &'static str,
    pub collaboration_style: &'static str,
    pub signature_move: &'static str,
}

/// Map an agent name to its codename profile.
pub fn agent_profile(agent_name: &str) -> AgentProfile {
    let normalized = agent_name.to_ascii_lowercase();

    if normalized.contains("planner") {
        return AgentProfile {
            codename: "Strategist",
            profile: "Goal decomposition specialist",
            personality: "calm, methodical, and dependency-aware",
            collaboration_style: "opens with numbered plans and explicit priorities",
            signature_move: "turns vague goals into concrete execution ladders",
        };
    }

    if normalized.contains("research") {
        return AgentProfile {
            codename: "Archivist",
            profile: "Evidence and assumptions analyst",
            personality: "curious, skeptical, and detail-focused",
            collaboration_style: "validates claims and cites edge-case evidence",
            signature_move: "surfaces blind spots before implementation starts",
        };
    }

    if normalized.contains("coder") || normalized.contains("implement") {
        return AgentProfile {
            codename: "Forge",
            profile: "Implementation architect",
            personality: "pragmatic, direct, and execution-heavy",
            collaboration_style: "proposes concrete code-level actions quickly",
            signature_move: "translates plans into shippable implementation steps",
        };
    }

    if normalized.contains("review") {
        return AgentProfile {
            codename: "Sentinel",
            profile: "Quality and regression guardian",
            personality: "disciplined, assertive, and standards-driven",
            collaboration_style: "challenges weak reasoning and hardens quality",
            signature_move: "detects brittle assumptions and failure modes",
        };
    }

    if normalized.contains("tester") || normalized.contains("test") {
        return AgentProfile {
            codename: "Probe",
            profile: "Verification strategist",
            personality: "adversarial in a good way, systematic, and precise",
            collaboration_style: "designs checks around failure-first thinking",
            signature_move: "builds test matrices that catch hidden breakage",
        };
    }

    if normalized.contains("integrat") {
        return AgentProfile {
            codename: "Conductor",
            profile: "Cross-stream synthesis lead",
            personality: "balanced, diplomatic, and outcome-oriented",
            collaboration_style: "reconciles competing inputs into one plan",
            signature_move: "merges parallel work into coherent delivery",
        };
    }

    if normalized.contains("skeptic") || normalized.contains("risk") {
        return AgentProfile {
            codename: "Radar",
            profile: "Risk and threat analyst",
            personality: "blunt, anticipatory, and protective",
            collaboration_style: "flags downside scenarios and mitigation paths",
            signature_move: "turns uncertainty into explicit risk registers",
        };
    }

    if normalized.contains("summary") || normalized.contains("summarizer") {
        return AgentProfile {
            codename: "Beacon",
            profile: "Decision synthesis specialist",
            personality: "concise, clear, and action-first",
            collaboration_style: "compresses complexity into executable next steps",
            signature_move: "creates crisp briefings that unblock teams quickly",
        };
    }

    let fallback_profiles = [
        AgentProfile {
            codename: "Navigator",
            profile: "Generalist coordinator",
            personality: "adaptable and context-aware",
            collaboration_style: "balances speed with clarity",
            signature_move: "keeps team momentum aligned",
        },
        AgentProfile {
            codename: "Vector",
            profile: "Execution operator",
            personality: "focused and deadline-driven",
            collaboration_style: "prefers direct action and feedback loops",
            signature_move: "drives ambiguous tasks toward decisions",
        },
        AgentProfile {
            codename: "Signal",
            profile: "Communication specialist",
            personality: "clear, friendly, and structured",
            collaboration_style: "frames updates for quick handoffs",
            signature_move: "turns noisy context into clean status",
        },
        AgentProfile {
            codename: "Kernel",
            profile: "Core-systems thinker",
            personality: "analytical and stable",
            collaboration_style: "organizes work around constraints and invariants",
            signature_move: "locks down the critical path early",
        },
    ];

    let mut hash: u64 = 2_166_136_261;
    for byte in normalized.bytes() {
        hash = (hash ^ u64::from(byte)).wrapping_mul(16_777_619);
    }
    fallback_profiles[hash as usize % fallback_profiles.len()]
}

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
    pub auto_apply_edits: bool,
    pub allow_network: bool,
    pub slash_autocomplete: bool,
    pub selected_settings_index: usize,
    pub mcp_registry: Arc<crate::tui::app::mcp::TuiMcpRegistry>,
    // Spawned sub-agent state
    pub spawned_agents: HashMap<String, SpawnedAgent>,
    pub active_spawned_agent: Option<String>,
    pub streaming_agent_texts: HashMap<String, String>,
    // Message line cache for render performance
    pub cached_message_lines: Vec<ratatui::text::Line<'static>>,
    pub cached_messages_len: usize,
    pub cached_max_width: usize,
    pub cached_streaming_snapshot: Option<String>,
    pub cached_processing: bool,
    // Watchdog state for stuck request detection
    pub main_watchdog_root_prompt: Option<String>,
    pub main_last_event_at: Option<Instant>,
    pub main_watchdog_restart_count: u32,
    pub main_inflight_prompt: Option<String>,
    // OKR gate state for /go command
    pub okr_repository: Option<Arc<crate::okr::OkrRepository>>,
    pub pending_okr_approval: Option<super::okr_gate::PendingOkrApproval>,
    // Smart switch state
    pub pending_smart_switch_retry: Option<crate::tui::app::smart_switch::PendingSmartSwitchRetry>,
    pub smart_switch_retry_count: u32,
    pub smart_switch_attempted_models: Vec<String>,
    // Chat sync state
    pub chat_sync_rx: Option<tokio::sync::mpsc::UnboundedReceiver<crate::tui::chat::sync::ChatSyncUiEvent>>,
    pub chat_sync_status: Option<String>,
    pub chat_sync_last_success: Option<String>,
    pub chat_sync_last_error: Option<String>,
    pub chat_sync_uploaded_bytes: u64,
    pub chat_sync_uploaded_batches: u64,
    // File picker state
    pub file_picker_dir: std::path::PathBuf,
    pub file_picker_entries: Vec<crate::tui::app::file_picker::FilePickerEntry>,
    pub file_picker_selected: usize,
    pub file_picker_filter: String,
    pub file_picker_active: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            view_mode: ViewMode::Chat,
            input_mode: InputMode::Normal,
            messages: vec![],
            input: String::new(),
            input_cursor: 0,
            input_scroll: 0,
            chat_scroll: 0,
            chat_last_max_scroll: 0,
            tool_preview_scroll: 0,
            tool_preview_last_max_scroll: 0,
            status: "Ready — type a message and press Enter. Ctrl+C/Ctrl+Q quits.".to_string(),
            processing: false,
            session_id: None,
            sessions: vec![],
            selected_session: 0,
            session_filter: String::new(),
            cwd_display: String::new(),
            bus_log: BusLogState::new(),
            swarm: SwarmViewState::new(),
            ralph: RalphViewState::new(),
            symbol_search: SymbolSearchState::new(),
            slash_suggestions: vec![],
            selected_slash_suggestion: 0,
            command_history: Vec::new(),
            history_index: None,
            worker_id: None,
            worker_name: None,
            a2a_connected: false,
            recent_tasks: Vec::new(),
            worker_bridge_registered_agents: HashSet::new(),
            worker_bridge_processing_state: None,
            worker_task_queue: VecDeque::new(),
            help_scroll: HelpScrollState::default(),
            show_help: false,
            available_models: Vec::new(),
            selected_model_index: 0,
            model_picker_active: false,
            model_filter: String::new(),
            streaming_text: String::new(),
            processing_started_at: None,
            current_request_first_token_ms: None,
            current_request_last_token_ms: None,
            last_request_first_token_ms: None,
            last_request_last_token_ms: None,
            last_completion_model: None,
            last_completion_latency_ms: None,
            last_completion_prompt_tokens: None,
            last_completion_output_tokens: None,
            last_tool_name: None,
            last_tool_latency_ms: None,
            last_tool_success: None,
            pending_images: Vec::new(),
            auto_apply_edits: false,
            allow_network: false,
            slash_autocomplete: true,
            selected_settings_index: 0,
            mcp_registry: Arc::new(crate::tui::app::mcp::TuiMcpRegistry::new()),
            // Spawned sub-agents
            spawned_agents: HashMap::new(),
            active_spawned_agent: None,
            streaming_agent_texts: HashMap::new(),
            // Message line cache
            cached_message_lines: Vec::new(),
            cached_messages_len: 0,
            cached_max_width: 0,
            cached_streaming_snapshot: None,
            cached_processing: false,
            // Watchdog state
            main_watchdog_root_prompt: None,
            main_last_event_at: None,
            main_watchdog_restart_count: 0,
            main_inflight_prompt: None,
            // OKR gate state
            okr_repository: None,
            pending_okr_approval: None,
            pending_smart_switch_retry: None,
            smart_switch_retry_count: 0,
            smart_switch_attempted_models: Vec::new(),
            // Chat sync state
            chat_sync_rx: None,
            chat_sync_status: None,
            chat_sync_last_success: None,
            chat_sync_last_error: None,
            chat_sync_uploaded_bytes: 0,
            chat_sync_uploaded_batches: 0,
            // File picker state
            file_picker_dir: std::path::PathBuf::new(),
            file_picker_entries: Vec::new(),
            file_picker_selected: 0,
            file_picker_filter: String::new(),
            file_picker_active: false,
        }
    }
}

impl AppState {
    const SETTINGS_COUNT: usize = 3;

    /// Check whether the cached message lines are still valid for the given
    /// width. Returns `true` when the cache can be reused.
    pub(crate) fn is_message_cache_valid(&self, max_width: usize) -> bool {
        self.cached_messages_len == self.messages.len()
            && self.cached_max_width == max_width
            && self.cached_streaming_snapshot.as_deref() == Some(&self.streaming_text)
            && self.cached_processing == self.processing
    }
    /// Combined cache-check method: returns cached lines (draining ownership) when
    /// the cache is still valid for `max_width`, or `None` when a rebuild is
    /// required.
    pub fn get_or_build_message_lines(&mut self, max_width: usize) -> Option<Vec<Line<'static>>> {
        if self.is_message_cache_valid(max_width) && !self.cached_message_lines.is_empty() {
            Some(self.take_cached_message_lines())
        } else {
            None
        }
    }


    /// Take ownership of the cached lines, clearing the cache.
    pub(crate) fn take_cached_message_lines(&mut self) -> Vec<Line<'static>> {
        self.cached_message_lines.drain(..).collect()
    }

    /// Store rebuilt message lines in the cache.
    pub(crate) fn store_message_lines(&mut self, lines: Vec<Line<'static>>, max_width: usize) {
        self.cached_message_lines = lines;
        self.cached_messages_len = self.messages.len();
        self.cached_max_width = max_width;
        self.cached_streaming_snapshot = if self.processing {
            Some(self.streaming_text.clone())
        } else {
            None
        };
        self.cached_processing = self.processing;
    }

    fn input_char_count(&self) -> usize {
        self.input.chars().count()
    }

    fn char_to_byte_index(&self, char_index: usize) -> usize {
        if char_index == 0 {
            return 0;
        }
        self.input
            .char_indices()
            .nth(char_index)
            .map(|(idx, _)| idx)
            .unwrap_or_else(|| self.input.len())
    }

    fn char_at(&self, char_index: usize) -> Option<char> {
        self.input.chars().nth(char_index)
    }

    pub fn clamp_input_cursor(&mut self) {
        self.input_cursor = self.input_cursor.min(self.input_char_count());
    }

    #[allow(dead_code)]
    pub fn ensure_input_cursor_visible(&mut self, visible_width: usize) {
        if visible_width == 0 {
            self.input_scroll = self.input_cursor;
            return;
        }
        if self.input_cursor < self.input_scroll {
            self.input_scroll = self.input_cursor;
        } else if self.input_cursor >= self.input_scroll.saturating_add(visible_width) {
            self.input_scroll = self
                .input_cursor
                .saturating_sub(visible_width.saturating_sub(1));
        }
    }

    pub fn settings_select_prev(&mut self) {
        if self.selected_settings_index > 0 {
            self.selected_settings_index -= 1;
        }
    }

    pub fn settings_select_next(&mut self) {
        if self.selected_settings_index + 1 < Self::SETTINGS_COUNT {
            self.selected_settings_index += 1;
        }
    }

    pub fn move_cursor_left(&mut self) {
        if self.input_cursor > 0 {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_right(&mut self) {
        if self.input_cursor < self.input_char_count() {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_word_left(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor > 0
            && self
                .char_at(self.input_cursor - 1)
                .is_some_and(char::is_whitespace)
        {
            self.input_cursor -= 1;
        }
        while self.input_cursor > 0
            && self
                .char_at(self.input_cursor - 1)
                .is_some_and(|c| !c.is_whitespace())
        {
            self.input_cursor -= 1;
        }
    }

    pub fn move_cursor_word_right(&mut self) {
        self.clamp_input_cursor();
        while self.input_cursor < self.input_char_count()
            && self
                .char_at(self.input_cursor)
                .is_some_and(char::is_whitespace)
        {
            self.input_cursor += 1;
        }
        while self.input_cursor < self.input_char_count()
            && self
                .char_at(self.input_cursor)
                .is_some_and(|c| !c.is_whitespace())
        {
            self.input_cursor += 1;
        }
    }

    pub fn move_cursor_home(&mut self) {
        self.input_cursor = 0;
    }

    pub fn move_cursor_end(&mut self) {
        self.input_cursor = self.input_char_count();
    }

    pub fn insert_char(&mut self, c: char) {
        self.clamp_input_cursor();
        let byte_index = self.char_to_byte_index(self.input_cursor);
        self.input.insert(byte_index, c);
        self.input_cursor += 1;
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn insert_text(&mut self, text: &str) {
        if text.is_empty() {
            return;
        }

        self.clamp_input_cursor();
        let byte_index = self.char_to_byte_index(self.input_cursor);
        self.input.insert_str(byte_index, text);
        self.input_cursor += text.chars().count();
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn delete_backspace(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor == 0 {
            return;
        }
        let start = self.char_to_byte_index(self.input_cursor - 1);
        let end = self.char_to_byte_index(self.input_cursor);
        self.input.replace_range(start..end, "");
        self.input_cursor -= 1;
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn delete_forward(&mut self) {
        self.clamp_input_cursor();
        if self.input_cursor >= self.input_char_count() {
            return;
        }
        let start = self.char_to_byte_index(self.input_cursor);
        let end = self.char_to_byte_index(self.input_cursor + 1);
        self.input.replace_range(start..end, "");
        self.history_index = None;
        self.refresh_slash_suggestions();
    }

    pub fn scroll_up(&mut self, amount: usize) {
        if self.chat_scroll >= 1_000_000 {
            self.chat_scroll = self.chat_last_max_scroll.saturating_sub(amount);
        } else {
            self.chat_scroll = self.chat_scroll.saturating_sub(amount);
        }
    }

    pub fn scroll_down(&mut self, amount: usize) {
        if self.chat_scroll >= 1_000_000 {
            return;
        }

        let next = self.chat_scroll.saturating_add(amount);
        if next >= self.chat_last_max_scroll {
            self.scroll_to_bottom();
        } else {
            self.chat_scroll = next;
        }
    }

    pub fn scroll_to_bottom(&mut self) {
        // Sentinel value — clamped to actual content height at render time.
        self.chat_scroll = 1_000_000;
    }

    pub fn scroll_to_top(&mut self) {
        self.chat_scroll = 0;
    }

    pub fn set_chat_max_scroll(&mut self, max_scroll: usize) {
        self.chat_last_max_scroll = max_scroll;
        if max_scroll == 0 {
            self.chat_scroll = 0;
        } else if self.chat_scroll < 1_000_000 {
            self.chat_scroll = self.chat_scroll.min(max_scroll);
        }
    }

    pub fn scroll_tool_preview_up(&mut self, amount: usize) {
        self.tool_preview_scroll = self.tool_preview_scroll.saturating_sub(amount);
    }

    pub fn scroll_tool_preview_down(&mut self, amount: usize) {
        let next = self.tool_preview_scroll.saturating_add(amount);
        self.tool_preview_scroll = next.min(self.tool_preview_last_max_scroll);
    }

    pub fn reset_tool_preview_scroll(&mut self) {
        self.tool_preview_scroll = 0;
    }

    pub fn set_tool_preview_max_scroll(&mut self, max_scroll: usize) {
        self.tool_preview_last_max_scroll = max_scroll;
        self.tool_preview_scroll = self.tool_preview_scroll.min(max_scroll);
    }

    pub fn sessions_select_prev(&mut self) {
        if self.selected_session > 0 {
            self.selected_session -= 1;
        }
    }

    pub fn sessions_select_next(&mut self) {
        if self.selected_session + 1 < self.filtered_sessions().len() {
            self.selected_session += 1;
        }
    }

    pub fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
        if self.session_filter.is_empty() {
            self.sessions.iter().enumerate().collect()
        } else {
            let filter = self.session_filter.to_lowercase();
            self.sessions
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    s.title
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&filter)
                        || s.id.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    pub fn clear_session_filter(&mut self) {
        self.session_filter.clear();
        self.selected_session = 0;
    }

    pub fn session_filter_backspace(&mut self) {
        self.session_filter.pop();
        self.selected_session = 0;
    }

    pub fn session_filter_push(&mut self, c: char) {
        self.session_filter.push(c);
        self.selected_session = 0;
    }

    pub fn set_view_mode(&mut self, view_mode: ViewMode) {
        self.view_mode = view_mode;
    }

    pub fn set_worker_bridge(&mut self, worker_id: String, worker_name: String) {
        self.worker_id = Some(worker_id);
        self.worker_name = Some(worker_name);
        self.a2a_connected = true;
    }

    pub fn sync_worker_bridge_processing(&mut self, processing: bool) {
        self.worker_bridge_processing_state = Some(processing);
    }

    pub fn register_worker_agent(&mut self, name: String) {
        self.worker_bridge_registered_agents.insert(name);
    }

    pub fn push_recent_task(&mut self, task: String) {
        self.recent_tasks.push(task);
        if self.recent_tasks.len() > 20 {
            let drain_len = self.recent_tasks.len() - 20;
            self.recent_tasks.drain(0..drain_len);
        }
    }

    pub fn enqueue_worker_task(&mut self, task: IncomingTask) {
        self.worker_task_queue.push_back(task);
        while self.worker_task_queue.len() > 20 {
            self.worker_task_queue.pop_front();
        }
    }

    pub fn dequeue_worker_task(&mut self) -> Option<IncomingTask> {
        self.worker_task_queue.pop_front()
    }

    pub fn refresh_slash_suggestions(&mut self) {
        if !self.input.starts_with('/') {
            self.slash_suggestions.clear();
            self.selected_slash_suggestion = 0;
            return;
        }

        let raw_query = self.input.split_whitespace().next().unwrap_or("");
        let query = normalize_slash_command(raw_query).to_lowercase();
        self.slash_suggestions = SLASH_COMMANDS
            .iter()
            .filter(|cmd| cmd.starts_with(&query))
            .map(|cmd| (*cmd).to_string())
            .collect();

        if self.slash_suggestions.is_empty() && query == "/" {
            self.slash_suggestions = SLASH_COMMANDS
                .iter()
                .map(|cmd| (*cmd).to_string())
                .collect();
        }

        if self.selected_slash_suggestion >= self.slash_suggestions.len() {
            self.selected_slash_suggestion = self.slash_suggestions.len().saturating_sub(1);
        }
    }

    pub fn slash_suggestions_visible(&self) -> bool {
        self.view_mode == ViewMode::Chat
            && !self.processing
            && self.input.starts_with('/')
            && !self.input.trim().chars().any(char::is_whitespace)
            && !self.slash_suggestions.is_empty()
    }

    pub fn select_prev_slash_suggestion(&mut self) {
        if !self.slash_suggestions.is_empty() {
            self.selected_slash_suggestion = self.selected_slash_suggestion.saturating_sub(1);
        }
    }

    pub fn select_next_slash_suggestion(&mut self) {
        if !self.slash_suggestions.is_empty() {
            self.selected_slash_suggestion =
                (self.selected_slash_suggestion + 1).min(self.slash_suggestions.len() - 1);
        }
    }

    pub fn selected_slash_suggestion(&self) -> Option<&str> {
        self.slash_suggestions
            .get(self.selected_slash_suggestion)
            .map(String::as_str)
    }

    pub fn apply_selected_slash_suggestion(&mut self) -> bool {
        if let Some(selected) = self.selected_slash_suggestion() {
            self.input = selected.to_string();
            self.input_cursor = self.input_char_count();
            self.input_mode = InputMode::Command;
            self.refresh_slash_suggestions();
            return true;
        }
        false
    }

    pub fn push_history(&mut self, entry: String) {
        if !entry.trim().is_empty() {
            self.command_history.push(entry);
            self.history_index = None;
        }
    }

    pub fn history_prev(&mut self) -> bool {
        if self.command_history.is_empty() {
            return false;
        }
        let history_len = self.command_history.len();
        let new_index = match self.history_index {
            Some(index) if index > 0 => Some(index - 1),
            Some(index) => Some(index),
            None => Some(history_len - 1),
        };
        if let Some(index) = new_index {
            self.history_index = Some(index);
            self.input = self.command_history[index].clone();
            self.input_cursor = self.input_char_count();
            self.input_mode = if self.input.starts_with('/') {
                InputMode::Command
            } else {
                InputMode::Editing
            };
            self.refresh_slash_suggestions();
            return true;
        }
        false
    }

    pub fn history_next(&mut self) -> bool {
        if self.command_history.is_empty() {
            return false;
        }
        match self.history_index {
            Some(index) if index + 1 < self.command_history.len() => {
                let next = index + 1;
                self.history_index = Some(next);
                self.input = self.command_history[next].clone();
            }
            Some(_) => {
                self.history_index = None;
                self.input.clear();
            }
            None => return false,
        }
        self.input_cursor = self.input_char_count();
        self.input_mode = if self.input.is_empty() {
            InputMode::Normal
        } else if self.input.starts_with('/') {
            InputMode::Command
        } else {
            InputMode::Editing
        };
        self.refresh_slash_suggestions();
        true
    }

    pub fn filtered_models(&self) -> Vec<&str> {
        if self.model_filter.is_empty() {
            self.available_models.iter().map(String::as_str).collect()
        } else {
            let filter = self.model_filter.to_lowercase();
            self.available_models
                .iter()
                .map(String::as_str)
                .filter(|model| model.to_lowercase().contains(&filter))
                .collect()
        }
    }

    pub fn set_available_models(&mut self, models: Vec<String>) {
        self.available_models = models;
        if self.selected_model_index >= self.filtered_models().len() {
            self.selected_model_index = self.filtered_models().len().saturating_sub(1);
        }
    }

    pub fn open_model_picker(&mut self) {
        self.model_picker_active = true;
        self.model_filter.clear();
        self.selected_model_index = 0;
        self.status = "Model picker".to_string();
    }

    pub fn close_model_picker(&mut self) {
        self.model_picker_active = false;
        self.model_filter.clear();
        self.selected_model_index = 0;
    }

    pub fn model_select_prev(&mut self) {
        self.selected_model_index = self.selected_model_index.saturating_sub(1);
    }

    pub fn model_select_next(&mut self) {
        if self.selected_model_index + 1 < self.filtered_models().len() {
            self.selected_model_index += 1;
        }
    }

    pub fn model_filter_push(&mut self, c: char) {
        self.model_filter.push(c);
        self.selected_model_index = 0;
    }

    pub fn model_filter_backspace(&mut self) {
        self.model_filter.pop();
        self.selected_model_index = 0;
    }

    pub fn selected_model(&self) -> Option<&str> {
        self.filtered_models()
            .get(self.selected_model_index)
            .copied()
    }

    pub async fn refresh_available_models(
        &mut self,
        registry: Option<&Arc<ProviderRegistry>>,
    ) -> anyhow::Result<()> {
        let mut models = Vec::new();
        let Some(registry) = registry else {
            self.available_models.clear();
            return Ok(());
        };

        for provider_name in registry.list() {
            if let Some(provider) = registry.get(provider_name)
                && let Ok(provider_models) = provider.list_models().await
            {
                models.extend(provider_models.into_iter().map(|model| {
                    if model.id.contains('/') {
                        model.id
                    } else {
                        format!("{provider_name}/{}", model.id)
                    }
                }));
            }
        }

        models.sort();
        models.dedup();
        self.set_available_models(models);
        Ok(())
    }

    pub fn begin_request_timing(&mut self) {
        self.processing_started_at = Some(Instant::now());
        self.current_request_first_token_ms = None;
        self.current_request_last_token_ms = None;
    }

    pub fn current_request_elapsed_ms(&self) -> Option<u64> {
        self.processing_started_at
            .map(|started| started.elapsed().as_millis() as u64)
    }

    pub fn note_text_token(&mut self) {
        let Some(elapsed_ms) = self.current_request_elapsed_ms() else {
            return;
        };
        if self.current_request_first_token_ms.is_none() {
            self.current_request_first_token_ms = Some(elapsed_ms);
        }
        self.current_request_last_token_ms = Some(elapsed_ms);
    }

    pub fn complete_request_timing(&mut self) {
        self.last_request_first_token_ms = self.current_request_first_token_ms;
        self.last_request_last_token_ms = self.current_request_last_token_ms;
        self.clear_request_timing();
    }

    pub fn clear_request_timing(&mut self) {
        self.processing_started_at = None;
        self.current_request_first_token_ms = None;
        self.current_request_last_token_ms = None;
    }

    pub fn clear_input(&mut self) {
        self.input.clear();
        self.input_cursor = 0;
        self.input_scroll = 0;
        self.input_mode = InputMode::Normal;
        self.slash_suggestions.clear();
        self.selected_slash_suggestion = 0;
        self.history_index = None;
    }
}

#[cfg(test)]
mod tests {
    use super::AppState;
    use std::time::{Duration, Instant};

    #[test]
    fn utf8_cursor_moves_by_character_not_byte() {
        let mut state = AppState {
            input: "aé🙂z".to_string(),
            ..Default::default()
        };

        state.move_cursor_end();
        assert_eq!(state.input_cursor, 4);

        state.move_cursor_left();
        assert_eq!(state.input_cursor, 3);
        state.move_cursor_left();
        assert_eq!(state.input_cursor, 2);
        state.move_cursor_right();
        assert_eq!(state.input_cursor, 3);
    }

    #[test]
    fn utf8_backspace_removes_full_character() {
        let mut state = AppState {
            input: "aé🙂z".to_string(),
            ..Default::default()
        };

        state.move_cursor_end();
        state.move_cursor_left();
        state.delete_backspace();

        assert_eq!(state.input, "aéz");
        assert_eq!(state.input_cursor, 2);
    }

    #[test]
    fn utf8_delete_forward_removes_full_character() {
        let mut state = AppState {
            input: "aé🙂z".to_string(),
            input_cursor: 1,
            ..Default::default()
        };

        state.delete_forward();

        assert_eq!(state.input, "a🙂z");
        assert_eq!(state.input_cursor, 1);
    }

    #[test]
    fn utf8_insert_respects_character_cursor() {
        let mut state = AppState {
            input: "a🙂z".to_string(),
            input_cursor: 2,
            ..Default::default()
        };

        state.insert_char('é');

        assert_eq!(state.input, "a🙂éz");
        assert_eq!(state.input_cursor, 3);
    }

    #[test]
    fn model_filter_limits_visible_models() {
        let mut state = AppState {
            available_models: vec![
                "zai/glm-5".to_string(),
                "openai/gpt-4o".to_string(),
                "openrouter/qwen/qwen3-coder".to_string(),
            ],
            ..Default::default()
        };

        state.model_filter_push('g');
        state.model_filter_push('p');
        state.model_filter_push('t');

        assert_eq!(state.filtered_models(), vec!["openai/gpt-4o"]);
        assert_eq!(state.selected_model(), Some("openai/gpt-4o"));
    }

    #[test]
    fn scroll_up_from_follow_latest_enters_manual_scroll_mode() {
        let mut state = AppState::default();
        state.set_chat_max_scroll(25);
        state.scroll_to_bottom();

        state.scroll_up(1);

        assert_eq!(state.chat_scroll, 24);
    }

    #[test]
    fn scroll_down_returns_to_follow_latest_at_bottom() {
        let mut state = AppState::default();
        state.set_chat_max_scroll(25);
        state.chat_scroll = 24;

        state.scroll_down(1);

        assert_eq!(state.chat_scroll, 1_000_000);
    }

    #[test]
    fn tool_preview_scroll_clamps_to_rendered_max() {
        let mut state = AppState::default();
        state.set_tool_preview_max_scroll(4);

        state.scroll_tool_preview_down(10);
        assert_eq!(state.tool_preview_scroll, 4);

        state.scroll_tool_preview_up(2);
        assert_eq!(state.tool_preview_scroll, 2);

        state.set_tool_preview_max_scroll(1);
        assert_eq!(state.tool_preview_scroll, 1);
    }

    #[test]
    fn request_timing_captures_first_and_last_token() {
        let mut state = AppState::default();
        state.processing_started_at = Some(
            Instant::now()
                .checked_sub(Duration::from_millis(12))
                .unwrap(),
        );

        state.note_text_token();
        let first = state
            .current_request_first_token_ms
            .expect("first token should be recorded");
        assert_eq!(state.current_request_last_token_ms, Some(first));

        state.processing_started_at = Some(
            Instant::now()
                .checked_sub(Duration::from_millis(34))
                .unwrap(),
        );
        state.note_text_token();

        let last = state
            .current_request_last_token_ms
            .expect("last token should be recorded");
        assert!(last >= first);

        state.complete_request_timing();

        assert_eq!(state.last_request_first_token_ms, Some(first));
        assert_eq!(state.last_request_last_token_ms, Some(last));
        assert!(state.processing_started_at.is_none());
        assert!(state.current_request_first_token_ms.is_none());
        assert!(state.current_request_last_token_ms.is_none());
    }
}