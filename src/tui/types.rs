//! TUI data type definitions

use super::constants::*;
use super::*;

enum MessageType {
    Text(String),
    Image {
        url: String,
        mime_type: Option<String>,
    },
    ToolCall {
        name: String,
        arguments_preview: String,
        arguments_len: usize,
        truncated: bool,
    },
    ToolResult {
        name: String,
        output_preview: String,
        output_len: usize,
        truncated: bool,
        success: bool,
        duration_ms: Option<u64>,
    },
    File {
        path: String,
        mime_type: Option<String>,
    },
    Thinking(String),
}

/// View mode for the TUI
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ViewMode {
    Chat,
    Swarm,
    Ralph,
    BusLog,
    Protocol,
    SessionPicker,
    ModelPicker,
    AgentPicker,
    FilePicker,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ChatLayoutMode {
    Classic,
    Webview,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum WorkspaceEntryKind {
    Directory,
    File,
}

#[derive(Debug, Clone)]
struct WorkspaceEntry {
    name: String,
    kind: WorkspaceEntryKind,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum FilePickerEntryKind {
    Parent,
    Directory,
    File,
}

#[derive(Debug, Clone)]
struct FilePickerEntry {
    name: String,
    path: PathBuf,
    kind: FilePickerEntryKind,
    size_bytes: Option<u64>,
}

#[derive(Debug, Clone, Default)]
struct WorkspaceSnapshot {
    root_display: String,
    git_branch: Option<String>,
    git_dirty_files: usize,
    entries: Vec<WorkspaceEntry>,
    captured_at: String,
}

/// Application state
struct App {
    input: String,
    cursor_position: usize,
    messages: Vec<ChatMessage>,
    current_agent: String,
    scroll: usize,
    show_help: bool,
    command_history: Vec<String>,
    history_index: Option<usize>,
    session: Option<Session>,
    is_processing: bool,
    processing_message: Option<String>,
    current_tool: Option<String>,
    current_tool_started_at: Option<Instant>,
    /// Tracks when processing started for elapsed timer display
    processing_started_at: Option<Instant>,
    /// Partial streaming text being assembled (shown with typing indicator)
    streaming_text: Option<String>,
    /// Partial streaming text per spawned agent (shown live in chat)
    streaming_agent_texts: HashMap<String, String>,
    /// Total tool calls in this session for inspector
    tool_call_count: usize,
    response_rx: Option<mpsc::Receiver<SessionEvent>>,
    /// Cached provider registry to avoid reloading from Vault on every message
    provider_registry: Option<std::sync::Arc<crate::provider::ProviderRegistry>>,
    /// Working directory for workspace-scoped session filtering
    workspace_dir: PathBuf,
    // Swarm mode state
    view_mode: ViewMode,
    chat_layout: ChatLayoutMode,
    show_inspector: bool,
    workspace: WorkspaceSnapshot,
    swarm_state: SwarmViewState,
    swarm_rx: Option<mpsc::Receiver<SwarmEvent>>,
    // Ralph mode state
    ralph_state: RalphViewState,
    ralph_rx: Option<mpsc::Receiver<RalphEvent>>,
    // Bus protocol log state
    bus_log_state: BusLogState,
    bus_log_rx: Option<mpsc::Receiver<crate::bus::BusEnvelope>>,
    bus: Option<std::sync::Arc<crate::bus::AgentBus>>,
    // Session picker state
    session_picker_list: Vec<SessionSummary>,
    session_picker_selected: usize,
    session_picker_filter: String,
    session_picker_confirm_delete: bool,
    session_picker_offset: usize, // Pagination offset
    // Model picker state
    model_picker_list: Vec<(String, String, String)>, // (display label, provider/model value, human name)
    model_picker_selected: usize,
    model_picker_filter: String,
    // Agent picker state
    agent_picker_selected: usize,
    agent_picker_filter: String,
    // File picker state
    file_picker_dir: PathBuf,
    file_picker_entries: Vec<FilePickerEntry>,
    file_picker_selected: usize,
    file_picker_filter: String,
    file_picker_preview_title: String,
    file_picker_preview_lines: Vec<String>,
    // Protocol registry view state
    protocol_selected: usize,
    protocol_scroll: usize,
    active_model: Option<String>,
    // Spawned sub-agents state
    active_spawned_agent: Option<String>,
    spawned_agents: HashMap<String, SpawnedAgent>,
    agent_response_rxs: Vec<(String, mpsc::Receiver<SessionEvent>)>,
    agent_tool_started_at: HashMap<String, Instant>,
    autochat_rx: Option<mpsc::Receiver<AutochatUiEvent>>,
    autochat_running: bool,
    autochat_started_at: Option<Instant>,
    autochat_status: Option<String>,
    chat_archive_path: Option<PathBuf>,
    archived_message_count: usize,
    chat_sync_rx: Option<mpsc::Receiver<ChatSyncUiEvent>>,
    chat_sync_status: Option<String>,
    chat_sync_last_success: Option<String>,
    chat_sync_last_error: Option<String>,
    chat_sync_uploaded_bytes: u64,
    chat_sync_uploaded_batches: usize,
    secure_environment: bool,
    // OKR approval gate state
    pending_okr_approval: Option<PendingOkrApproval>,
    okr_repository: Option<std::sync::Arc<OkrRepository>>,
    // Cached max scroll for key handlers
    last_max_scroll: usize,
    /// Cached rendered message lines (rebuilt only when messages/state changes)
    cached_message_lines: Vec<ratatui::text::Line<'static>>,
    /// Number of messages when cache was last built
    cached_messages_len: usize,
    /// Width when cache was last built
    cached_max_width: usize,
    /// Streaming text snapshot when cache was last built
    cached_streaming_snapshot: Option<String>,
    /// Whether processing state when cache was last built
    cached_processing: bool,
    /// Whether autochat was running when cache was last built
    cached_autochat_running: bool,
    /// Pending image attachments (base64 data URLs) to send with the next message
    pending_images: Vec<PendingImage>,
    /// Last user prompt currently in flight for the main chat request.
    main_inflight_prompt: Option<String>,
    /// Original user prompt for watchdog recovery retries.
    main_watchdog_root_prompt: Option<String>,
    /// Last time we received a main-session event while processing.
    main_last_event_at: Option<Instant>,
    /// Number of watchdog restart attempts for current main in-flight request.
    main_watchdog_restart_count: u8,
    worker_bridge: Option<TuiWorkerBridge>,
    worker_bridge_registered_agents: HashSet<String>,
    worker_bridge_processing_state: Option<bool>,
    worker_task_queue: VecDeque<crate::tui::worker_bridge::IncomingTask>,
    worker_autorun_enabled: bool,
    smart_switch_retry_count: u8,
    smart_switch_attempted_models: HashSet<String>,
    pending_smart_switch_retry: Option<PendingSmartSwitchRetry>,
}

/// A pending image attachment from clipboard paste
#[derive(Debug, Clone)]
struct PendingImage {
    /// Base64-encoded data URL (e.g., "data:image/png;base64,...")
    data_url: String,
    /// Image dimensions for display
    width: usize,
    height: usize,
    /// Size in bytes (before base64 encoding)
    size_bytes: usize,
}

#[derive(Debug, Clone)]
struct PendingSmartSwitchRetry {
    prompt: String,
    target_model: String,
}

#[allow(dead_code)]
#[derive(Clone)]
struct ChatMessage {
    role: String,
    content: String,
    timestamp: String,
    message_type: MessageType,
    /// Per-step usage metadata (set on assistant messages)
    usage_meta: Option<UsageMeta>,
    /// Name of the spawned agent that produced this message (None = main chat)
    agent_name: Option<String>,
}

/// A spawned sub-agent with its own independent LLM session.
#[allow(dead_code)]
struct SpawnedAgent {
    /// User-facing name (e.g. "planner", "coder")
    name: String,
    /// System instructions for this agent
    instructions: String,
    /// Independent conversation session
    session: Session,
    /// Whether this agent is currently processing a message
    is_processing: bool,
}

#[derive(Debug, Clone, Copy)]
struct AgentProfile {
    codename: &'static str,
    profile: &'static str,
    personality: &'static str,
    collaboration_style: &'static str,
    signature_move: &'static str,
}

/// Token usage + cost + latency for one LLM round-trip
#[derive(Debug, Clone)]
struct UsageMeta {
    prompt_tokens: usize,
    completion_tokens: usize,
    duration_ms: u64,
    cost_usd: Option<f64>,
}

enum AutochatUiEvent {
    Progress(String),
    SystemMessage(String),
    AgentEvent {
        agent_name: String,
        event: Box<SessionEvent>,
    },
    Completed {
        summary: String,
        okr_id: Option<String>,
        okr_run_id: Option<String>,
        relay_id: Option<String>,
    },
}

#[derive(Debug, Clone)]
struct ChatSyncConfig {
    endpoint: String,
    fallback_endpoint: Option<String>,
    access_key: String,
    secret_key: String,
    bucket: String,
    prefix: String,
    interval_secs: u64,
    ignore_cert_check: bool,
}

enum ChatSyncUiEvent {
    Status(String),
    BatchUploaded {
        bytes: u64,
        records: usize,
        object_key: String,
    },
    Error(String),
}

/// Persistent checkpoint for an in-flight autochat relay.
///
/// Saved after each successful agent turn so that a crashed TUI can resume
