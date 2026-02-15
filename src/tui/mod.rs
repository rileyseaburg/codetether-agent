//! Terminal User Interface
//!
//! Interactive TUI using Ratatui

pub mod bus_log;
pub mod message_formatter;
pub mod ralph_view;
pub mod swarm_view;
pub mod theme;
pub mod theme_utils;
pub mod token_display;

/// Sentinel value meaning "scroll to bottom"
const SCROLL_BOTTOM: usize = 1_000_000;

// Tool-call / tool-result rendering can carry very large JSON payloads (e.g. patches, file blobs).
// If we pretty-print + split/collect that payload on every frame, the TUI can appear to “stop
// rendering” after a few tool calls due to render-time CPU churn.
const TOOL_ARGS_PRETTY_JSON_MAX_BYTES: usize = 16_000;
const TOOL_ARGS_PREVIEW_MAX_LINES: usize = 10;
const TOOL_ARGS_PREVIEW_MAX_BYTES: usize = 6_000;
const TOOL_OUTPUT_PREVIEW_MAX_LINES: usize = 5;
const TOOL_OUTPUT_PREVIEW_MAX_BYTES: usize = 4_000;
const AUTOCHAT_MAX_AGENTS: usize = 8;
const AUTOCHAT_DEFAULT_AGENTS: usize = 3;
const AUTOCHAT_MAX_ROUNDS: usize = 3;
const AUTOCHAT_MAX_DYNAMIC_SPAWNS: usize = 3;
const AUTOCHAT_SPAWN_CHECK_MIN_CHARS: usize = 800;
const AUTOCHAT_RLM_THRESHOLD_CHARS: usize = 6_000;
const AUTOCHAT_QUICK_DEMO_TASK: &str = "Self-organize into the right specialties for this task, then relay one concrete implementation plan with clear next handoffs.";
const GO_SWAP_MODEL_GLM: &str = "zai/glm-5";
const GO_SWAP_MODEL_MINIMAX: &str = "minimax-credits/MiniMax-M2.5-highspeed";
const CHAT_SYNC_DEFAULT_INTERVAL_SECS: u64 = 15;
const CHAT_SYNC_MAX_INTERVAL_SECS: u64 = 300;
const CHAT_SYNC_MAX_BATCH_BYTES: usize = 512 * 1024;
const CHAT_SYNC_DEFAULT_BUCKET: &str = "codetether-chat-archive";
const CHAT_SYNC_DEFAULT_PREFIX: &str = "sessions";
const AGENT_AVATARS: [&str; 12] = [
    "[o_o]", "[^_^]", "[>_<]", "[._.]", "[+_+]", "[~_~]", "[x_x]", "[0_0]", "[*_*]", "[=_=]",
    "[T_T]", "[u_u]",
];

use crate::bus::relay::{ProtocolRelayRuntime, RelayAgentProfile};
use crate::config::Config;
use crate::okr::{
    ApprovalDecision, KeyResult, KrOutcome, KrOutcomeType, Okr, OkrRepository, OkrRun, OkrRunStatus,
};
use crate::provider::{ContentPart, Role};
use crate::ralph::{RalphConfig, RalphLoop};
use crate::rlm::RlmExecutor;
use crate::session::{Session, SessionEvent, SessionSummary, list_sessions_with_opencode_paged};
use crate::swarm::{DecompositionStrategy, SwarmConfig, SwarmExecutor};
use crate::tui::bus_log::{BusLogState, render_bus_log};
use crate::tui::message_formatter::MessageFormatter;
use crate::tui::ralph_view::{RalphEvent, RalphViewState, render_ralph_view};
use crate::tui::swarm_view::{SwarmEvent, SwarmViewState, render_swarm_view};
use crate::tui::theme::Theme;
use crate::tui::token_display::TokenDisplay;
use anyhow::Result;
use base64::Engine;
use crossterm::{
    event::{
        DisableBracketedPaste, EnableBracketedPaste, Event, EventStream, KeyCode, KeyEventKind,
        KeyModifiers,
    },
    execute,
    terminal::{EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode, enable_raw_mode},
};
use futures::StreamExt;
use minio::s3::builders::ObjectContent;
use minio::s3::creds::StaticProvider;
use minio::s3::http::BaseUrl;
use minio::s3::types::S3Api;
use minio::s3::{Client as MinioClient, ClientBuilder as MinioClientBuilder};
use ratatui::{
    Frame, Terminal,
    backend::CrosstermBackend,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{
        Block, Borders, Clear, Paragraph, Scrollbar, ScrollbarOrientation, ScrollbarState, Wrap,
    },
};
use serde::{Deserialize, Serialize, de::DeserializeOwned};
use std::collections::HashMap;
use std::io::{self, Read, Seek, SeekFrom, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{Duration, Instant};
use tokio::sync::mpsc;
use uuid::Uuid;

/// Safely parse a UUID string with explicit skip/warn behavior.
/// Returns None and logs a warning if the string is not a valid UUID,
/// preventing silent NIL UUID fallback that could corrupt data linkage.
fn parse_uuid_guarded(s: &str, context: &str) -> Option<Uuid> {
    match s.parse::<Uuid>() {
        Ok(uuid) => Some(uuid),
        Err(e) => {
            tracing::warn!(
                context,
                uuid_str = %s,
                error = %e,
                "Invalid UUID string - skipping operation to prevent NIL UUID corruption"
            );
            None
        }
    }
}

/// Run the TUI
pub async fn run(project: Option<PathBuf>) -> Result<()> {
    // Change to project directory if specified
    if let Some(dir) = project {
        std::env::set_current_dir(&dir)?;
    }

    // Setup terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableBracketedPaste)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Run the app
    let result = run_app(&mut terminal).await;

    // Restore terminal
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableBracketedPaste
    )?;
    terminal.show_cursor()?;

    result
}

/// Message type for chat display
#[derive(Debug, Clone)]
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
}

#[allow(dead_code)]
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
        event: SessionEvent,
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
/// the relay from exactly where it left off.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct RelayCheckpoint {
    /// Original user task
    task: String,
    /// Model reference used for all agents
    model_ref: String,
    /// Ordered list of agent names in relay order
    ordered_agents: Vec<String>,
    /// Session IDs for each agent (agent name → session UUID)
    agent_session_ids: HashMap<String, String>,
    /// Agent profiles: (name, system instructions, capabilities)
    agent_profiles: Vec<(String, String, Vec<String>)>,
    /// Current round (1-based)
    round: usize,
    /// Current agent index within the round
    idx: usize,
    /// The baton text to pass to the next agent
    baton: String,
    /// Total turns completed so far
    turns: usize,
    /// Convergence hit count
    convergence_hits: usize,
    /// Dynamic spawn count
    dynamic_spawn_count: usize,
    /// RLM handoff count
    rlm_handoff_count: usize,
    /// Workspace directory
    workspace_dir: PathBuf,
    /// When the relay was started
    started_at: String,
    /// OKR ID this relay is associated with (if any)
    #[serde(default)]
    okr_id: Option<String>,
    /// OKR run ID this relay is associated with (if any)
    #[serde(default)]
    okr_run_id: Option<String>,
    /// Key result progress cursor: map of kr_id -> current value
    #[serde(default)]
    kr_progress: HashMap<String, f64>,
}

impl RelayCheckpoint {
    fn checkpoint_path() -> Option<PathBuf> {
        crate::config::Config::data_dir().map(|d| d.join("relay_checkpoint.json"))
    }

    async fn save(&self) -> Result<()> {
        if let Some(path) = Self::checkpoint_path() {
            if let Some(parent) = path.parent() {
                tokio::fs::create_dir_all(parent).await?;
            }
            let content = serde_json::to_string_pretty(self)?;
            tokio::fs::write(&path, content).await?;
            tracing::debug!("Relay checkpoint saved");
        }
        Ok(())
    }

    async fn load() -> Option<Self> {
        let path = Self::checkpoint_path()?;
        let content = tokio::fs::read_to_string(&path).await.ok()?;
        serde_json::from_str(&content).ok()
    }

    async fn delete() {
        if let Some(path) = Self::checkpoint_path() {
            let _ = tokio::fs::remove_file(&path).await;
            tracing::debug!("Relay checkpoint deleted");
        }
    }
}

/// Estimate USD cost from model name and token counts.
/// Uses approximate per-million-token pricing for well-known models.
fn estimate_cost(model: &str, prompt_tokens: usize, completion_tokens: usize) -> Option<f64> {
    let normalized_model = model.to_ascii_lowercase();

    // (input $/M, output $/M)
    let (input_rate, output_rate) = match normalized_model.as_str() {
        // Anthropic - Claude (Bedrock Opus 4.6 pricing: $5/$25)
        m if m.contains("claude-opus") => (5.0, 25.0),
        m if m.contains("claude-sonnet") => (3.0, 15.0),
        m if m.contains("claude-haiku") => (0.25, 1.25),
        // OpenAI
        m if m.contains("gpt-4o-mini") => (0.15, 0.6),
        m if m.contains("gpt-4o") => (2.5, 10.0),
        m if m.contains("o3") => (10.0, 40.0),
        m if m.contains("o4-mini") => (1.10, 4.40),
        // Google
        m if m.contains("gemini-2.5-pro") => (1.25, 10.0),
        m if m.contains("gemini-2.5-flash") => (0.15, 0.6),
        m if m.contains("gemini-2.0-flash") => (0.10, 0.40),
        // Bedrock third-party
        m if m.contains("kimi-k2") => (0.35, 1.40),
        m if m.contains("deepseek") => (0.80, 2.0),
        m if m.contains("llama") => (0.50, 1.50),
        // MiniMax
        // Highspeed: $0.6/M input, $2.4/M output
        // Regular: $0.3/M input, $1.2/M output
        m if m.contains("minimax") && m.contains("highspeed") => (0.60, 2.40),
        m if m.contains("minimax") && m.contains("m2") => (0.30, 1.20),
        // Amazon Nova
        m if m.contains("nova-pro") => (0.80, 3.20),
        m if m.contains("nova-lite") => (0.06, 0.24),
        m if m.contains("nova-micro") => (0.035, 0.14),
        // Z.AI GLM
        m if m.contains("glm-5") => (2.0, 8.0),
        m if m.contains("glm-4.7-flash") => (0.0, 0.0),
        m if m.contains("glm-4.7") => (0.50, 2.0),
        m if m.contains("glm-4") => (0.35, 1.40),
        _ => return None,
    };
    let cost =
        (prompt_tokens as f64 * input_rate + completion_tokens as f64 * output_rate) / 1_000_000.0;
    Some(cost)
}

fn current_spinner_frame() -> &'static str {
    const SPINNER: [&str; 10] = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
    let idx = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis()
        / 100) as usize
        % SPINNER.len();
    SPINNER[idx]
}

fn format_duration_ms(duration_ms: u64) -> String {
    if duration_ms >= 60_000 {
        format!(
            "{}m{:02}s",
            duration_ms / 60_000,
            (duration_ms % 60_000) / 1000
        )
    } else if duration_ms >= 1000 {
        format!("{:.1}s", duration_ms as f64 / 1000.0)
    } else {
        format!("{duration_ms}ms")
    }
}

fn format_bytes(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    if bytes < 1024 {
        format!("{bytes}B")
    } else if (bytes as f64) < MB {
        format!("{:.1}KB", bytes as f64 / KB)
    } else if (bytes as f64) < GB {
        format!("{:.1}MB", bytes as f64 / MB)
    } else {
        format!("{:.2}GB", bytes as f64 / GB)
    }
}

fn env_non_empty(name: &str) -> Option<String> {
    std::env::var(name)
        .ok()
        .map(|value| value.trim().to_string())
        .filter(|value| !value.is_empty())
}

fn env_bool(name: &str) -> Option<bool> {
    let value = env_non_empty(name)?;
    let value = value.to_ascii_lowercase();
    match value.as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn parse_bool_str(value: &str) -> Option<bool> {
    match value.trim().to_ascii_lowercase().as_str() {
        "1" | "true" | "yes" | "on" => Some(true),
        "0" | "false" | "no" | "off" => Some(false),
        _ => None,
    }
}

fn is_placeholder_secret(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "replace-me" | "changeme" | "change-me" | "your-token" | "your-key"
    )
}

fn env_non_placeholder(name: &str) -> Option<String> {
    env_non_empty(name).filter(|value| !is_placeholder_secret(value))
}

fn vault_extra_string(secrets: &crate::secrets::ProviderSecrets, keys: &[&str]) -> Option<String> {
    keys.iter().find_map(|key| {
        secrets
            .extra
            .get(*key)
            .and_then(|value| value.as_str())
            .map(|value| value.trim().to_string())
            .filter(|value| !value.is_empty())
    })
}

fn vault_extra_bool(secrets: &crate::secrets::ProviderSecrets, keys: &[&str]) -> Option<bool> {
    keys.iter().find_map(|key| {
        secrets.extra.get(*key).and_then(|value| match value {
            serde_json::Value::Bool(flag) => Some(*flag),
            serde_json::Value::String(text) => parse_bool_str(text),
            _ => None,
        })
    })
}

fn is_secure_environment_from_values(
    secure_environment: Option<bool>,
    secure_env: Option<bool>,
    environment_name: Option<&str>,
) -> bool {
    if let Some(value) = secure_environment {
        return value;
    }

    if let Some(value) = secure_env {
        return value;
    }

    environment_name.is_some_and(|value| {
        matches!(
            value.trim().to_ascii_lowercase().as_str(),
            "secure" | "production" | "prod"
        )
    })
}

fn is_secure_environment() -> bool {
    is_secure_environment_from_values(
        env_bool("CODETETHER_SECURE_ENVIRONMENT"),
        env_bool("CODETETHER_SECURE_ENV"),
        env_non_empty("CODETETHER_ENV").as_deref(),
    )
}

fn normalize_minio_endpoint(endpoint: &str) -> String {
    let mut normalized = endpoint.trim().trim_end_matches('/').to_string();
    if let Some(stripped) = normalized.strip_suffix("/login") {
        normalized = stripped.trim_end_matches('/').to_string();
    }
    if !normalized.starts_with("http://") && !normalized.starts_with("https://") {
        normalized = format!("http://{normalized}");
    }
    normalized
}

fn minio_fallback_endpoint(endpoint: &str) -> Option<String> {
    if endpoint.contains(":9001") {
        Some(endpoint.replacen(":9001", ":9000", 1))
    } else {
        None
    }
}

fn sanitize_s3_key_segment(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    for ch in value.chars() {
        if ch.is_ascii_alphanumeric() || matches!(ch, '-' | '_' | '.') {
            out.push(ch);
        } else {
            out.push('-');
        }
    }
    let cleaned = out.trim_matches('-').to_string();
    if cleaned.is_empty() {
        "unknown".to_string()
    } else {
        cleaned
    }
}

async fn parse_chat_sync_config(
    require_chat_sync: bool,
) -> std::result::Result<Option<ChatSyncConfig>, String> {
    let enabled = env_bool("CODETETHER_CHAT_SYNC_ENABLED").unwrap_or(false);
    if !enabled {
        tracing::info!(
            secure_required = require_chat_sync,
            "Remote chat sync disabled (CODETETHER_CHAT_SYNC_ENABLED is false or unset)"
        );
        if require_chat_sync {
            return Err(
                "CODETETHER_CHAT_SYNC_ENABLED must be true in secure environment".to_string(),
            );
        }
        return Ok(None);
    }

    let vault_secrets = crate::secrets::get_provider_secrets("chat-sync-minio").await;

    let endpoint_raw = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_ENDPOINT")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                secrets
                    .base_url
                    .clone()
                    .filter(|value| !value.trim().is_empty())
                    .or_else(|| vault_extra_string(secrets, &["endpoint", "minio_endpoint"]))
            })
        })
        .ok_or_else(|| {
            "CODETETHER_CHAT_SYNC_MINIO_ENDPOINT is required when chat sync is enabled (env or Vault: codetether/providers/chat-sync-minio.base_url)".to_string()
        })?;
    let endpoint = normalize_minio_endpoint(&endpoint_raw);
    let fallback_endpoint = minio_fallback_endpoint(&endpoint);

    let access_key = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                vault_extra_string(
                    secrets,
                    &[
                        "access_key",
                        "minio_access_key",
                        "username",
                        "key",
                        "api_key",
                    ],
                )
                .or_else(|| {
                    secrets
                        .api_key
                        .as_ref()
                        .map(|value| value.trim().to_string())
                        .filter(|value| !value.is_empty())
                })
            })
        })
        .ok_or_else(|| {
            "CODETETHER_CHAT_SYNC_MINIO_ACCESS_KEY is required when chat sync is enabled (env or Vault: codetether/providers/chat-sync-minio.access_key/api_key)".to_string()
        })?;
    let secret_key = env_non_placeholder("CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                vault_extra_string(
                    secrets,
                    &[
                        "secret_key",
                        "minio_secret_key",
                        "password",
                        "secret",
                        "api_secret",
                    ],
                )
            })
        })
        .ok_or_else(|| {
            "CODETETHER_CHAT_SYNC_MINIO_SECRET_KEY is required when chat sync is enabled (env or Vault: codetether/providers/chat-sync-minio.secret_key)".to_string()
        })?;

    let bucket = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_BUCKET")
        .or_else(|| {
            vault_secrets
                .as_ref()
                .and_then(|secrets| vault_extra_string(secrets, &["bucket", "minio_bucket"]))
        })
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_BUCKET.to_string());
    let prefix = env_non_empty("CODETETHER_CHAT_SYNC_MINIO_PREFIX")
        .or_else(|| {
            vault_secrets
                .as_ref()
                .and_then(|secrets| vault_extra_string(secrets, &["prefix", "minio_prefix"]))
        })
        .unwrap_or_else(|| CHAT_SYNC_DEFAULT_PREFIX.to_string())
        .trim_matches('/')
        .to_string();

    let interval_secs = env_non_empty("CODETETHER_CHAT_SYNC_INTERVAL_SECS")
        .and_then(|value| value.parse::<u64>().ok())
        .unwrap_or(CHAT_SYNC_DEFAULT_INTERVAL_SECS)
        .clamp(1, CHAT_SYNC_MAX_INTERVAL_SECS);

    let ignore_cert_check = env_bool("CODETETHER_CHAT_SYNC_MINIO_INSECURE_SKIP_TLS_VERIFY")
        .or_else(|| {
            vault_secrets.as_ref().and_then(|secrets| {
                vault_extra_bool(
                    secrets,
                    &[
                        "insecure_skip_tls_verify",
                        "ignore_cert_check",
                        "skip_tls_verify",
                    ],
                )
            })
        })
        .unwrap_or(false);

    Ok(Some(ChatSyncConfig {
        endpoint,
        fallback_endpoint,
        access_key,
        secret_key,
        bucket,
        prefix,
        interval_secs,
        ignore_cert_check,
    }))
}

fn chat_sync_checkpoint_path(archive_path: &Path, config: &ChatSyncConfig) -> PathBuf {
    let endpoint_tag = sanitize_s3_key_segment(&config.endpoint.replace("://", "-"));
    let bucket_tag = sanitize_s3_key_segment(&config.bucket);
    archive_path.with_file_name(format!(
        "chat_events.minio-sync.{endpoint_tag}.{bucket_tag}.offset"
    ))
}

fn load_chat_sync_offset(checkpoint_path: &Path) -> u64 {
    std::fs::read_to_string(checkpoint_path)
        .ok()
        .and_then(|value| value.trim().parse::<u64>().ok())
        .unwrap_or(0)
}

fn store_chat_sync_offset(checkpoint_path: &Path, offset: u64) {
    if let Some(parent) = checkpoint_path.parent()
        && let Err(err) = std::fs::create_dir_all(parent)
    {
        tracing::warn!(error = %err, path = %parent.display(), "Failed to create chat sync checkpoint directory");
        return;
    }

    if let Err(err) = std::fs::write(checkpoint_path, offset.to_string()) {
        tracing::warn!(error = %err, path = %checkpoint_path.display(), "Failed to persist chat sync checkpoint");
    }
}

fn build_minio_client(endpoint: &str, config: &ChatSyncConfig) -> Result<MinioClient> {
    let base_url: BaseUrl = endpoint.parse()?;
    let provider = StaticProvider::new(&config.access_key, &config.secret_key, None);
    let client = MinioClientBuilder::new(base_url)
        .provider(Some(Box::new(provider)))
        .ignore_cert_check(Some(config.ignore_cert_check))
        .build()?;
    Ok(client)
}

async fn ensure_minio_bucket(client: &MinioClient, bucket: &str) -> Result<()> {
    let exists = client.bucket_exists(bucket).send().await?;
    if !exists.exists {
        match client.create_bucket(bucket).send().await {
            Ok(_) => {}
            Err(err) => {
                let error_text = err.to_string();
                if !error_text.contains("BucketAlreadyOwnedByYou")
                    && !error_text.contains("BucketAlreadyExists")
                {
                    return Err(anyhow::anyhow!(error_text));
                }
            }
        }
    }
    Ok(())
}

fn read_chat_archive_batch(archive_path: &Path, offset: u64) -> Result<(Vec<u8>, u64, usize)> {
    let metadata = std::fs::metadata(archive_path)?;
    let file_len = metadata.len();
    if offset >= file_len {
        return Ok((Vec::new(), offset, 0));
    }

    let mut file = std::fs::File::open(archive_path)?;
    file.seek(SeekFrom::Start(offset))?;

    let target_bytes = (file_len - offset).min(CHAT_SYNC_MAX_BATCH_BYTES as u64) as usize;
    let mut buffer = vec![0_u8; target_bytes];
    let read = file.read(&mut buffer)?;
    buffer.truncate(read);

    if read == 0 {
        return Ok((Vec::new(), offset, 0));
    }

    // Try to end batches on a newline when there is still more data pending.
    if offset + (read as u64) < file_len {
        if let Some(last_newline) = buffer.iter().rposition(|byte| *byte == b'\n') {
            buffer.truncate(last_newline + 1);
        }

        if buffer.is_empty() {
            let mut rolling = Vec::new();
            let mut temp = [0_u8; 4096];
            loop {
                let n = file.read(&mut temp)?;
                if n == 0 {
                    break;
                }
                rolling.extend_from_slice(&temp[..n]);
                if let Some(pos) = rolling.iter().position(|byte| *byte == b'\n') {
                    rolling.truncate(pos + 1);
                    break;
                }
                if rolling.len() >= CHAT_SYNC_MAX_BATCH_BYTES {
                    break;
                }
            }
            buffer = rolling;
        }
    }

    let next_offset = offset + buffer.len() as u64;
    let records = buffer.iter().filter(|byte| **byte == b'\n').count();
    Ok((buffer, next_offset, records))
}

#[derive(Debug)]
struct ChatSyncBatch {
    bytes: u64,
    records: usize,
    object_key: String,
    next_offset: u64,
}

async fn sync_chat_archive_batch(
    client: &MinioClient,
    archive_path: &Path,
    config: &ChatSyncConfig,
    host_tag: &str,
    offset: u64,
) -> Result<Option<ChatSyncBatch>> {
    if !archive_path.exists() {
        return Ok(None);
    }

    ensure_minio_bucket(client, &config.bucket).await?;

    let (payload, next_offset, records) = read_chat_archive_batch(archive_path, offset)?;
    if payload.is_empty() {
        return Ok(None);
    }

    let timestamp = chrono::Utc::now().format("%Y%m%dT%H%M%SZ");
    let key_prefix = config.prefix.trim_matches('/');
    let object_key = if key_prefix.is_empty() {
        format!("{host_tag}/{timestamp}-chat-events-{offset:020}-{next_offset:020}.jsonl")
    } else {
        format!(
            "{key_prefix}/{host_tag}/{timestamp}-chat-events-{offset:020}-{next_offset:020}.jsonl"
        )
    };

    let bytes = payload.len() as u64;
    let content = ObjectContent::from(payload);
    client
        .put_object_content(&config.bucket, &object_key, content)
        .send()
        .await?;

    Ok(Some(ChatSyncBatch {
        bytes,
        records,
        object_key,
        next_offset,
    }))
}

async fn run_chat_sync_worker(
    tx: mpsc::Sender<ChatSyncUiEvent>,
    archive_path: PathBuf,
    config: ChatSyncConfig,
) {
    let checkpoint_path = chat_sync_checkpoint_path(&archive_path, &config);
    let mut offset = load_chat_sync_offset(&checkpoint_path);
    let host_tag = sanitize_s3_key_segment(
        &std::env::var("HOSTNAME").unwrap_or_else(|_| "localhost".to_string()),
    );

    let fallback_label = config
        .fallback_endpoint
        .as_ref()
        .map(|fallback| format!(" (fallback: {fallback})"))
        .unwrap_or_default();

    let _ = tx
        .send(ChatSyncUiEvent::Status(format!(
            "Archive sync enabled → {} / {} every {}s{}",
            config.endpoint, config.bucket, config.interval_secs, fallback_label
        )))
        .await;

    tracing::info!(
        endpoint = %config.endpoint,
        bucket = %config.bucket,
        prefix = %config.prefix,
        interval_secs = config.interval_secs,
        checkpoint = %checkpoint_path.display(),
        archive = %archive_path.display(),
        "Chat sync worker started"
    );

    let mut interval = tokio::time::interval(Duration::from_secs(config.interval_secs));
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

    loop {
        interval.tick().await;

        let primary_client = match build_minio_client(&config.endpoint, &config) {
            Ok(client) => client,
            Err(err) => {
                let _ = tx
                    .send(ChatSyncUiEvent::Error(format!(
                        "Chat sync client init failed for {}: {err}",
                        config.endpoint
                    )))
                    .await;
                continue;
            }
        };

        let outcome = match sync_chat_archive_batch(
            &primary_client,
            &archive_path,
            &config,
            &host_tag,
            offset,
        )
        .await
        {
            Ok(result) => Ok(result),
            Err(primary_err) => {
                if let Some(fallback_endpoint) = &config.fallback_endpoint {
                    let fallback_client = build_minio_client(fallback_endpoint, &config);
                    match fallback_client {
                        Ok(client) => {
                            let _ = tx
                                .send(ChatSyncUiEvent::Status(format!(
                                    "Primary endpoint failed; retrying with fallback {}",
                                    fallback_endpoint
                                )))
                                .await;
                            sync_chat_archive_batch(
                                &client,
                                &archive_path,
                                &config,
                                &host_tag,
                                offset,
                            )
                            .await
                        }
                        Err(err) => Err(anyhow::anyhow!(
                            "Primary sync error: {primary_err}; fallback init failed: {err}"
                        )),
                    }
                } else {
                    Err(primary_err)
                }
            }
        };

        match outcome {
            Ok(Some(batch)) => {
                offset = batch.next_offset;
                store_chat_sync_offset(&checkpoint_path, offset);
                tracing::info!(
                    bytes = batch.bytes,
                    records = batch.records,
                    object_key = %batch.object_key,
                    next_offset = batch.next_offset,
                    "Chat sync uploaded batch"
                );
                let _ = tx
                    .send(ChatSyncUiEvent::BatchUploaded {
                        bytes: batch.bytes,
                        records: batch.records,
                        object_key: batch.object_key,
                    })
                    .await;
            }
            Ok(None) => {}
            Err(err) => {
                tracing::warn!(error = %err, "Chat sync batch upload failed");
                let _ = tx
                    .send(ChatSyncUiEvent::Error(format!("Chat sync failed: {err}")))
                    .await;
            }
        }
    }
}

#[derive(Debug, Serialize)]
struct ChatArchiveRecord {
    recorded_at: String,
    workspace: String,
    session_id: Option<String>,
    role: String,
    agent_name: Option<String>,
    message_type: String,
    content: String,
    tool_name: Option<String>,
    tool_success: Option<bool>,
    tool_duration_ms: Option<u64>,
}

impl ChatMessage {
    fn new(role: impl Into<String>, content: impl Into<String>) -> Self {
        let content = content.into();
        Self {
            role: role.into(),
            timestamp: chrono::Local::now().format("%H:%M").to_string(),
            message_type: MessageType::Text(content.clone()),
            content,
            usage_meta: None,
            agent_name: None,
        }
    }

    fn with_message_type(mut self, message_type: MessageType) -> Self {
        self.message_type = message_type;
        self
    }

    fn with_usage_meta(mut self, meta: UsageMeta) -> Self {
        self.usage_meta = Some(meta);
        self
    }

    fn with_agent_name(mut self, name: impl Into<String>) -> Self {
        self.agent_name = Some(name.into());
        self
    }
}

/// Pending OKR approval gate state for /go commands
struct PendingOkrApproval {
    /// The OKR being proposed
    okr: Okr,
    /// The OKR run being proposed
    run: OkrRun,
    /// Original task that triggered the OKR
    task: String,
    /// Agent count for the relay
    agent_count: usize,
    /// Model to use
    model: String,
}

impl PendingOkrApproval {
    /// Create a new pending approval from a task
    fn new(task: String, agent_count: usize, model: String) -> Self {
        let okr_id = Uuid::new_v4();

        // Create OKR with default key results based on task
        let mut okr = Okr::new(
            format!("Relay: {}", truncate_with_ellipsis(&task, 60)),
            format!("Execute relay task: {}", task),
        );
        okr.id = okr_id;

        // Add default key results for relay execution
        let kr1 = KeyResult::new(okr_id, "Relay completes all rounds", 100.0, "%");
        let kr2 = KeyResult::new(okr_id, "Team produces actionable handoff", 1.0, "count");
        let kr3 = KeyResult::new(okr_id, "No critical errors", 0.0, "count");

        okr.add_key_result(kr1);
        okr.add_key_result(kr2);
        okr.add_key_result(kr3);

        // Create the run
        let mut run = OkrRun::new(
            okr_id,
            format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
        );
        let _ = run.submit_for_approval();

        Self {
            okr,
            run,
            task,
            agent_count,
            model,
        }
    }

    /// Get the approval prompt text
    fn approval_prompt(&self) -> String {
        let krs: Vec<String> = self
            .okr
            .key_results
            .iter()
            .map(|kr| format!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit))
            .collect();

        format!(
            "⚠️  /go OKR Draft\n\n\
            Task: {}\n\
            Agents: {} | Model: {}\n\n\
            Objective: {}\n\n\
            Key Results:\n{}\n\n\
            Press [A] to approve or [D] to deny",
            truncate_with_ellipsis(&self.task, 100),
            self.agent_count,
            self.model,
            self.okr.title,
            krs.join("\n")
        )
    }
}

impl WorkspaceSnapshot {
    fn capture(root: &Path, max_entries: usize) -> Self {
        let mut entries: Vec<WorkspaceEntry> = Vec::new();

        if let Ok(read_dir) = std::fs::read_dir(root) {
            for entry in read_dir.flatten() {
                let file_name = entry.file_name().to_string_lossy().to_string();
                if should_skip_workspace_entry(&file_name) {
                    continue;
                }

                let kind = match entry.file_type() {
                    Ok(ft) if ft.is_dir() => WorkspaceEntryKind::Directory,
                    _ => WorkspaceEntryKind::File,
                };

                entries.push(WorkspaceEntry {
                    name: file_name,
                    kind,
                });
            }
        }

        entries.sort_by(|a, b| match (a.kind, b.kind) {
            (WorkspaceEntryKind::Directory, WorkspaceEntryKind::File) => std::cmp::Ordering::Less,
            (WorkspaceEntryKind::File, WorkspaceEntryKind::Directory) => {
                std::cmp::Ordering::Greater
            }
            _ => a
                .name
                .to_ascii_lowercase()
                .cmp(&b.name.to_ascii_lowercase()),
        });
        entries.truncate(max_entries);

        Self {
            root_display: root.to_string_lossy().to_string(),
            git_branch: detect_git_branch(root),
            git_dirty_files: detect_git_dirty_files(root),
            entries,
            captured_at: chrono::Local::now().format("%H:%M:%S").to_string(),
        }
    }
}

fn should_skip_workspace_entry(name: &str) -> bool {
    matches!(
        name,
        ".git" | "node_modules" | "target" | ".next" | "__pycache__" | ".venv"
    )
}

fn detect_git_branch(root: &Path) -> Option<String> {
    let output = Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["rev-parse", "--abbrev-ref", "HEAD"])
        .output()
        .ok()?;

    if !output.status.success() {
        return None;
    }

    let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
    if branch.is_empty() {
        None
    } else {
        Some(branch)
    }
}

fn detect_git_dirty_files(root: &Path) -> usize {
    let output = match Command::new("git")
        .arg("-C")
        .arg(root)
        .args(["status", "--porcelain"])
        .output()
    {
        Ok(out) => out,
        Err(_) => return 0,
    };

    if !output.status.success() {
        return 0;
    }

    String::from_utf8_lossy(&output.stdout)
        .lines()
        .filter(|line| !line.trim().is_empty())
        .count()
}

fn resolve_provider_for_model_autochat(
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
    model_ref: &str,
) -> Option<(std::sync::Arc<dyn crate::provider::Provider>, String)> {
    let (provider_name, model_name) = crate::provider::parse_model_string(model_ref);
    if let Some(provider_name) = provider_name
        && let Some(provider) = registry.get(provider_name)
    {
        return Some((provider, model_name.to_string()));
    }

    let fallbacks = [
        "zai",
        "openai",
        "github-copilot",
        "anthropic",
        "openrouter",
        "novita",
        "moonshotai",
        "google",
    ];

    for provider_name in fallbacks {
        if let Some(provider) = registry.get(provider_name) {
            return Some((provider, model_ref.to_string()));
        }
    }

    registry
        .list()
        .first()
        .copied()
        .and_then(|name| registry.get(name))
        .map(|provider| (provider, model_ref.to_string()))
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedRelayProfile {
    #[serde(default)]
    name: String,
    #[serde(default)]
    specialty: String,
    #[serde(default)]
    mission: String,
    #[serde(default)]
    capabilities: Vec<String>,
}

#[derive(Debug, Clone, Deserialize)]
struct PlannedRelayResponse {
    #[serde(default)]
    profiles: Vec<PlannedRelayProfile>,
}

#[derive(Debug, Clone, Deserialize)]
struct RelaySpawnDecision {
    #[serde(default)]
    spawn: bool,
    #[serde(default)]
    reason: String,
    #[serde(default)]
    profile: Option<PlannedRelayProfile>,
}

fn slugify_label(value: &str) -> String {
    let mut out = String::with_capacity(value.len());
    let mut last_dash = false;

    for ch in value.chars() {
        let ch = ch.to_ascii_lowercase();
        if ch.is_ascii_alphanumeric() {
            out.push(ch);
            last_dash = false;
        } else if !last_dash {
            out.push('-');
            last_dash = true;
        }
    }

    out.trim_matches('-').to_string()
}

fn sanitize_relay_agent_name(value: &str) -> String {
    let raw = slugify_label(value);
    let base = if raw.is_empty() {
        "auto-specialist".to_string()
    } else if raw.starts_with("auto-") {
        raw
    } else {
        format!("auto-{raw}")
    };

    truncate_with_ellipsis(&base, 48)
        .trim_end_matches("...")
        .to_string()
}

fn unique_relay_agent_name(base: &str, existing: &[String]) -> String {
    if !existing.iter().any(|name| name == base) {
        return base.to_string();
    }

    let mut suffix = 2usize;
    loop {
        let candidate = format!("{base}-{suffix}");
        if !existing.iter().any(|name| name == &candidate) {
            return candidate;
        }
        suffix += 1;
    }
}

fn relay_instruction_from_plan(name: &str, specialty: &str, mission: &str) -> String {
    format!(
        "You are @{name}.\n\
         Specialty: {specialty}.\n\
         Mission: {mission}\n\n\
         This is a protocol-first relay conversation. Treat incoming handoffs as authoritative context.\n\
         Keep responses concise, concrete, and useful for the next specialist.\n\
         Include one clear recommendation for what the next agent should do.\n\
         If the task is too large for the current team, explicitly call out missing specialties and handoff boundaries.",
    )
}

fn build_runtime_profile_from_plan(
    profile: PlannedRelayProfile,
    existing: &[String],
) -> Option<(String, String, Vec<String>)> {
    let specialty = if profile.specialty.trim().is_empty() {
        "generalist".to_string()
    } else {
        profile.specialty.trim().to_string()
    };

    let mission = if profile.mission.trim().is_empty() {
        "Advance the relay with concrete next actions and clear handoffs.".to_string()
    } else {
        profile.mission.trim().to_string()
    };

    let base_name = if profile.name.trim().is_empty() {
        format!("auto-{}", slugify_label(&specialty))
    } else {
        profile.name.trim().to_string()
    };

    let sanitized = sanitize_relay_agent_name(&base_name);
    let name = unique_relay_agent_name(&sanitized, existing);
    if name.trim().is_empty() {
        return None;
    }

    let mut capabilities: Vec<String> = Vec::new();
    let specialty_cap = slugify_label(&specialty);
    if !specialty_cap.is_empty() {
        capabilities.push(specialty_cap);
    }

    for capability in profile.capabilities {
        let normalized = slugify_label(&capability);
        if !normalized.is_empty() && !capabilities.contains(&normalized) {
            capabilities.push(normalized);
        }
    }

    for required in ["relay", "context-handoff", "rlm-aware", "autochat"] {
        if !capabilities.iter().any(|capability| capability == required) {
            capabilities.push(required.to_string());
        }
    }

    let instructions = relay_instruction_from_plan(&name, &specialty, &mission);
    Some((name, instructions, capabilities))
}

fn extract_json_payload<T: DeserializeOwned>(text: &str) -> Option<T> {
    let trimmed = text.trim();
    if let Ok(value) = serde_json::from_str::<T>(trimmed) {
        return Some(value);
    }

    if let (Some(start), Some(end)) = (trimmed.find('{'), trimmed.rfind('}'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    if let (Some(start), Some(end)) = (trimmed.find('['), trimmed.rfind(']'))
        && start < end
        && let Ok(value) = serde_json::from_str::<T>(&trimmed[start..=end])
    {
        return Some(value);
    }

    None
}

async fn plan_relay_profiles_with_registry(
    task: &str,
    model_ref: &str,
    requested_agents: usize,
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> Option<Vec<(String, String, Vec<String>)>> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry, model_ref)?;
    let requested_agents = requested_agents.clamp(2, AUTOCHAT_MAX_AGENTS);

    let request = crate::provider::CompletionRequest {
        model: model_name,
        messages: vec![
            crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text {
                    text: "You are a relay-team architect. Return ONLY valid JSON.".to_string(),
                }],
            },
            crate::provider::Message {
                role: crate::provider::Role::User,
                content: vec![crate::provider::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nDesign a task-specific relay team.\n\
                         Respond with JSON object only:\n\
                         {{\n  \"profiles\": [\n    {{\"name\":\"auto-...\",\"specialty\":\"...\",\"mission\":\"...\",\"capabilities\":[\"...\"]}}\n  ]\n}}\n\
                         Requirements:\n\
                         - Return {} profiles\n\
                         - Names must be short kebab-case\n\
                         - Capabilities must be concise skill tags\n\
                         - Missions should be concrete and handoff-friendly",
                        requested_agents
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(1200),
        stop: Vec::new(),
    };

    let response = provider.complete(request).await.ok()?;
    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text }
            | crate::provider::ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let planned = extract_json_payload::<PlannedRelayResponse>(&text)?;
    let mut existing = Vec::<String>::new();
    let mut runtime = Vec::<(String, String, Vec<String>)>::new();

    for profile in planned.profiles.into_iter().take(AUTOCHAT_MAX_AGENTS) {
        if let Some((name, instructions, capabilities)) =
            build_runtime_profile_from_plan(profile, &existing)
        {
            existing.push(name.clone());
            runtime.push((name, instructions, capabilities));
        }
    }

    if runtime.len() >= 2 {
        Some(runtime)
    } else {
        None
    }
}

async fn decide_dynamic_spawn_with_registry(
    task: &str,
    model_ref: &str,
    latest_output: &str,
    round: usize,
    ordered_agents: &[String],
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> Option<(String, String, Vec<String>, String)> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry, model_ref)?;
    let team = ordered_agents
        .iter()
        .map(|name| format!("@{name}"))
        .collect::<Vec<_>>()
        .join(", ");
    let output_excerpt = truncate_with_ellipsis(latest_output, 2200);

    let request = crate::provider::CompletionRequest {
        model: model_name,
        messages: vec![
            crate::provider::Message {
                role: crate::provider::Role::System,
                content: vec![crate::provider::ContentPart::Text {
                    text: "You are a relay scaling controller. Return ONLY valid JSON.".to_string(),
                }],
            },
            crate::provider::Message {
                role: crate::provider::Role::User,
                content: vec![crate::provider::ContentPart::Text {
                    text: format!(
                        "Task:\n{task}\n\nRound: {round}\nCurrent team: {team}\n\
                         Latest handoff excerpt:\n{output_excerpt}\n\n\
                         Decide whether the team needs one additional specialist right now.\n\
                         Respond with JSON object only:\n\
                         {{\n  \"spawn\": true|false,\n  \"reason\": \"...\",\n  \"profile\": {{\"name\":\"auto-...\",\"specialty\":\"...\",\"mission\":\"...\",\"capabilities\":[\"...\"]}}\n}}\n\
                         If spawn=false, profile may be null or omitted."
                    ),
                }],
            },
        ],
        tools: Vec::new(),
        temperature: Some(1.0),
        top_p: Some(0.9),
        max_tokens: Some(420),
        stop: Vec::new(),
    };

    let response = provider.complete(request).await.ok()?;
    let text = response
        .message
        .content
        .iter()
        .filter_map(|part| match part {
            crate::provider::ContentPart::Text { text }
            | crate::provider::ContentPart::Thinking { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n");

    let decision = extract_json_payload::<RelaySpawnDecision>(&text)?;
    if !decision.spawn {
        return None;
    }

    let profile = decision.profile?;
    let (name, instructions, capabilities) =
        build_runtime_profile_from_plan(profile, ordered_agents)?;
    let reason = if decision.reason.trim().is_empty() {
        "Model requested additional specialist for task scope.".to_string()
    } else {
        decision.reason.trim().to_string()
    };

    Some((name, instructions, capabilities, reason))
}

async fn prepare_autochat_handoff_with_registry(
    task: &str,
    from_agent: &str,
    output: &str,
    model_ref: &str,
    registry: &std::sync::Arc<crate::provider::ProviderRegistry>,
) -> (String, bool) {
    let mut used_rlm = false;
    let mut relay_payload = output.to_string();

    if output.len() > AUTOCHAT_RLM_THRESHOLD_CHARS {
        relay_payload = truncate_with_ellipsis(output, 3_500);

        if let Some((provider, model_name)) =
            resolve_provider_for_model_autochat(registry, model_ref)
        {
            let mut executor =
                RlmExecutor::new(output.to_string(), provider, model_name).with_max_iterations(2);

            let query = "Summarize this agent output for the next specialist in a relay. Keep:\n\
                         1) key conclusions, 2) unresolved risks, 3) exact next action.\n\
                         Keep it concise and actionable.";
            match executor.analyze(query).await {
                Ok(result) => {
                    relay_payload = result.answer;
                    used_rlm = true;
                }
                Err(err) => {
                    tracing::warn!(error = %err, "RLM handoff summarization failed; using truncation fallback");
                }
            }
        }
    }

    (
        format!(
            "Relay task:\n{task}\n\nIncoming handoff from @{from_agent}:\n{relay_payload}\n\n\
             Continue the work from this handoff. Keep your response focused and provide one concrete next-step instruction for the next agent."
        ),
        used_rlm,
    )
}

/// Ralph worker for TUI `/go` approval flow.
///
/// Loads a provider, generates a PRD, runs the Ralph loop, and reports
/// progress back to the TUI via the `AutochatUiEvent` channel.
async fn run_go_ralph_worker(
    tx: mpsc::Sender<AutochatUiEvent>,
    mut okr: crate::okr::Okr,
    mut run: crate::okr::OkrRun,
    task: String,
    model: String,
    bus: Option<std::sync::Arc<crate::bus::AgentBus>>,
    max_concurrent_stories: usize,
) {
    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Loading providers from Vault…".to_string(),
        ))
        .await;

    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(r) => std::sync::Arc::new(r),
        Err(err) => {
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: format!("❌ Failed to load providers: {err}"),
                    okr_id: Some(okr.id.to_string()),
                    okr_run_id: Some(run.id.to_string()),
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let (provider, resolved_model) = match resolve_provider_for_model_autochat(&registry, &model) {
        Some(pair) => pair,
        None => {
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: format!("❌ No provider available for model '{model}'"),
                    okr_id: Some(okr.id.to_string()),
                    okr_run_id: Some(run.id.to_string()),
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Generating PRD from task and key results…".to_string(),
        ))
        .await;

    let okr_id_str = okr.id.to_string();
    let run_id_str = run.id.to_string();

    match crate::cli::go_ralph::execute_go_ralph(
        &task,
        &mut okr,
        &mut run,
        provider,
        &resolved_model,
        10,
        bus,
        max_concurrent_stories,
        Some(registry.clone()),
    )
    .await
    {
        Ok(result) => {
            // Persist final run state
            if let Ok(repo) = crate::okr::OkrRepository::from_config().await {
                let _ = repo.update_run(run).await;
            }

            let summary = crate::cli::go_ralph::format_go_ralph_result(&result, &task);
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary,
                    okr_id: Some(okr_id_str),
                    okr_run_id: Some(run_id_str),
                    relay_id: None,
                })
                .await;
        }
        Err(err) => {
            // Mark run as failed
            run.status = crate::okr::OkrRunStatus::Failed;
            if let Ok(repo) = crate::okr::OkrRepository::from_config().await {
                let _ = repo.update_run(run).await;
            }

            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: format!("❌ Ralph execution failed: {err}"),
                    okr_id: Some(okr_id_str),
                    okr_run_id: Some(run_id_str),
                    relay_id: None,
                })
                .await;
        }
    }
}

async fn run_autochat_worker(
    tx: mpsc::Sender<AutochatUiEvent>,
    bus: std::sync::Arc<crate::bus::AgentBus>,
    fallback_profiles: Vec<(String, String, Vec<String>)>,
    task: String,
    model_ref: String,
    okr_id: Option<Uuid>,
    okr_run_id: Option<Uuid>,
) {
    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Loading providers from Vault…".to_string(),
        ))
        .await;

    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(registry) => std::sync::Arc::new(registry),
        Err(err) => {
            let _ = tx
                .send(AutochatUiEvent::SystemMessage(format!(
                    "Failed to load providers for /autochat: {err}"
                )))
                .await;
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: "Autochat aborted: provider registry unavailable.".to_string(),
                    okr_id: None,
                    okr_run_id: None,
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let relay = ProtocolRelayRuntime::new(bus.clone());
    let requested_agents = fallback_profiles.len().clamp(2, AUTOCHAT_MAX_AGENTS);

    let planned_profiles = match plan_relay_profiles_with_registry(
        &task,
        &model_ref,
        requested_agents,
        &registry,
    )
    .await
    {
        Some(planned) => {
            let _ = tx
                .send(AutochatUiEvent::Progress(format!(
                    "Model self-organized relay team ({} agents)…",
                    planned.len()
                )))
                .await;
            planned
        }
        None => {
            let _ = tx
                    .send(AutochatUiEvent::SystemMessage(
                        "Dynamic team planning unavailable; using fallback self-organizing relay profiles."
                            .to_string(),
                    ))
                    .await;
            fallback_profiles
        }
    };

    let mut relay_profiles = Vec::with_capacity(planned_profiles.len());
    let mut ordered_agents = Vec::with_capacity(planned_profiles.len());
    let mut sessions: HashMap<String, Session> = HashMap::new();
    let mut setup_errors: Vec<String> = Vec::new();
    let mut checkpoint_profiles: Vec<(String, String, Vec<String>)> = Vec::new();
    let mut kr_progress: HashMap<String, f64> = HashMap::new();

    // Convert Uuid to String for checkpoint storage
    let okr_id_str = okr_id.map(|id| id.to_string());
    let okr_run_id_str = okr_run_id.map(|id| id.to_string());

    // Load KR targets if OKR is associated
    let kr_targets: HashMap<String, f64> =
        if let (Some(okr_id_val), Some(_run_id)) = (&okr_id_str, &okr_run_id_str) {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Ok(okr_uuid) = okr_id_val.parse::<Uuid>() {
                    if let Ok(Some(okr)) = repo.get_okr(okr_uuid).await {
                        okr.key_results
                            .iter()
                            .map(|kr| (kr.id.to_string(), kr.target_value))
                            .collect()
                    } else {
                        HashMap::new()
                    }
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        };

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Initializing relay agent sessions…".to_string(),
        ))
        .await;

    for (name, instructions, capabilities) in planned_profiles {
        match Session::new().await {
            Ok(mut session) => {
                session.metadata.model = Some(model_ref.clone());
                session.agent = name.clone();
                session.bus = Some(bus.clone());
                session.add_message(crate::provider::Message {
                    role: Role::System,
                    content: vec![ContentPart::Text {
                        text: instructions.clone(),
                    }],
                });

                relay_profiles.push(RelayAgentProfile {
                    name: name.clone(),
                    capabilities: capabilities.clone(),
                });
                checkpoint_profiles.push((name.clone(), instructions, capabilities));
                ordered_agents.push(name.clone());
                sessions.insert(name, session);
            }
            Err(err) => {
                setup_errors.push(format!(
                    "Failed creating relay agent session @{name}: {err}"
                ));
            }
        }
    }

    if !setup_errors.is_empty() {
        let _ = tx
            .send(AutochatUiEvent::SystemMessage(format!(
                "Relay setup warnings:\n{}",
                setup_errors.join("\n")
            )))
            .await;
    }

    if ordered_agents.len() < 2 {
        let _ = tx
            .send(AutochatUiEvent::SystemMessage(
                "Autochat needs at least 2 agents to relay.".to_string(),
            ))
            .await;
        let _ = tx
            .send(AutochatUiEvent::Completed {
                summary: "Autochat aborted: insufficient relay participants.".to_string(),
                okr_id: None,
                okr_run_id: None,
                relay_id: None,
            })
            .await;
        return;
    }

    relay.register_agents(&relay_profiles);

    let _ = tx
        .send(AutochatUiEvent::Progress(format!(
            "Relay {} registered {} agents. Starting handoffs…",
            relay.relay_id(),
            ordered_agents.len()
        )))
        .await;

    let roster_profiles = relay_profiles
        .iter()
        .map(|profile| {
            let capability_summary = if profile.capabilities.is_empty() {
                "skills: dynamic-specialist".to_string()
            } else {
                format!("skills: {}", profile.capabilities.join(", "))
            };

            format!(
                "• {} — {}",
                format_agent_identity(&profile.name),
                capability_summary
            )
        })
        .collect::<Vec<_>>()
        .join("\n");
    let _ = tx
        .send(AutochatUiEvent::SystemMessage(format!(
            "Relay {id} started • model: {model_ref}\n\nTeam personalities:\n{roster_profiles}",
            id = relay.relay_id()
        )))
        .await;

    let mut baton = format!(
        "Task:\n{task}\n\nStart by proposing an execution strategy and one immediate next step."
    );
    let mut previous_normalized: Option<String> = None;
    let mut convergence_hits = 0usize;
    let mut turns = 0usize;
    let mut rlm_handoff_count = 0usize;
    let mut dynamic_spawn_count = 0usize;
    let mut status = "max_rounds_reached";
    let mut failure_note: Option<String> = None;

    'relay_loop: for round in 1..=AUTOCHAT_MAX_ROUNDS {
        let mut idx = 0usize;
        while idx < ordered_agents.len() {
            let to = ordered_agents[idx].clone();
            let from = if idx == 0 {
                if round == 1 {
                    "user".to_string()
                } else {
                    ordered_agents[ordered_agents.len() - 1].clone()
                }
            } else {
                ordered_agents[idx - 1].clone()
            };

            turns += 1;
            relay.send_handoff(&from, &to, &baton);
            let _ = tx
                .send(AutochatUiEvent::Progress(format!(
                    "Round {round}/{AUTOCHAT_MAX_ROUNDS} • @{from} → @{to}"
                )))
                .await;

            let Some(mut session) = sessions.remove(&to) else {
                status = "agent_error";
                failure_note = Some(format!("Relay agent @{to} session was unavailable."));
                break 'relay_loop;
            };

            let (event_tx, mut event_rx) = mpsc::channel(256);
            let registry_for_prompt = registry.clone();
            let baton_for_prompt = baton.clone();

            let join = tokio::spawn(async move {
                let result = session
                    .prompt_with_events(&baton_for_prompt, event_tx, registry_for_prompt)
                    .await;
                (session, result)
            });

            while !join.is_finished() {
                while let Ok(event) = event_rx.try_recv() {
                    if !matches!(event, SessionEvent::SessionSync(_)) {
                        let _ = tx
                            .send(AutochatUiEvent::AgentEvent {
                                agent_name: to.clone(),
                                event,
                            })
                            .await;
                    }
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }

            let (updated_session, result) = match join.await {
                Ok(value) => value,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} task join error: {err}"));
                    break 'relay_loop;
                }
            };

            while let Ok(event) = event_rx.try_recv() {
                if !matches!(event, SessionEvent::SessionSync(_)) {
                    let _ = tx
                        .send(AutochatUiEvent::AgentEvent {
                            agent_name: to.clone(),
                            event,
                        })
                        .await;
                }
            }

            sessions.insert(to.clone(), updated_session);

            let output = match result {
                Ok(response) => response.text,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                    let _ = tx
                        .send(AutochatUiEvent::SystemMessage(format!(
                            "Relay agent @{to} failed: {err}"
                        )))
                        .await;
                    break 'relay_loop;
                }
            };

            let normalized = normalize_for_convergence(&output);
            if previous_normalized.as_deref() == Some(normalized.as_str()) {
                convergence_hits += 1;
            } else {
                convergence_hits = 0;
            }
            previous_normalized = Some(normalized);

            let (next_handoff, used_rlm) =
                prepare_autochat_handoff_with_registry(&task, &to, &output, &model_ref, &registry)
                    .await;
            if used_rlm {
                rlm_handoff_count += 1;
            }

            baton = next_handoff;

            // Update KR progress after each turn
            if !kr_targets.is_empty() {
                let max_turns = ordered_agents.len() * AUTOCHAT_MAX_ROUNDS;
                let progress_ratio = (turns as f64 / max_turns as f64).min(1.0);

                for (kr_id, target) in &kr_targets {
                    let current = progress_ratio * target;
                    let existing = kr_progress.get(kr_id).copied().unwrap_or(0.0);
                    // Only update if progress increased (idempotent)
                    if current > existing {
                        kr_progress.insert(kr_id.clone(), current);
                    }
                }

                // Persist mid-run for real-time visibility (best-effort)
                if let Some(ref run_id_str) = okr_run_id_str {
                    if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                        if let Some(run_uuid) =
                            parse_uuid_guarded(run_id_str, "relay_mid_run_persist")
                        {
                            if let Ok(Some(mut run)) = repo.get_run(run_uuid).await {
                                if run.is_resumable() {
                                    run.iterations = turns as u32;
                                    for (kr_id, value) in &kr_progress {
                                        run.update_kr_progress(kr_id, *value);
                                    }
                                    run.status = crate::okr::OkrRunStatus::Running;
                                    let _ = repo.update_run(run).await;
                                }
                            }
                        }
                    }
                }
            }
            let can_attempt_spawn = dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
                && ordered_agents.len() < AUTOCHAT_MAX_AGENTS
                && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

            if can_attempt_spawn
                && let Some((name, instructions, capabilities, reason)) =
                    decide_dynamic_spawn_with_registry(
                        &task,
                        &model_ref,
                        &output,
                        round,
                        &ordered_agents,
                        &registry,
                    )
                    .await
            {
                match Session::new().await {
                    Ok(mut spawned_session) => {
                        spawned_session.metadata.model = Some(model_ref.clone());
                        spawned_session.agent = name.clone();
                        spawned_session.bus = Some(bus.clone());
                        spawned_session.add_message(crate::provider::Message {
                            role: Role::System,
                            content: vec![ContentPart::Text {
                                text: instructions.clone(),
                            }],
                        });

                        relay.register_agents(&[RelayAgentProfile {
                            name: name.clone(),
                            capabilities: capabilities.clone(),
                        }]);

                        ordered_agents.insert(idx + 1, name.clone());
                        checkpoint_profiles.push((name.clone(), instructions, capabilities));
                        sessions.insert(name.clone(), spawned_session);
                        dynamic_spawn_count += 1;

                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn: {} joined relay after @{to}.\nReason: {reason}",
                                format_agent_identity(&name)
                            )))
                            .await;
                    }
                    Err(err) => {
                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn requested but failed to create @{name}: {err}"
                            )))
                            .await;
                    }
                }
            }

            if convergence_hits >= 2 {
                status = "converged";
                break 'relay_loop;
            }

            // Save relay checkpoint so a crash can resume from here
            {
                let agent_session_ids: HashMap<String, String> = sessions
                    .iter()
                    .map(|(name, s)| (name.clone(), s.id.clone()))
                    .collect();
                let next_idx = idx + 1;
                let (ck_round, ck_idx) = if next_idx >= ordered_agents.len() {
                    (round + 1, 0)
                } else {
                    (round, next_idx)
                };
                let checkpoint = RelayCheckpoint {
                    task: task.clone(),
                    model_ref: model_ref.clone(),
                    ordered_agents: ordered_agents.clone(),
                    agent_session_ids,
                    agent_profiles: checkpoint_profiles.clone(),
                    round: ck_round,
                    idx: ck_idx,
                    baton: baton.clone(),
                    turns,
                    convergence_hits,
                    dynamic_spawn_count,
                    rlm_handoff_count,
                    workspace_dir: std::env::current_dir().unwrap_or_default(),
                    started_at: chrono::Utc::now().to_rfc3339(),
                    okr_id: okr_id_str.clone(),
                    okr_run_id: okr_run_id_str.clone(),
                    kr_progress: kr_progress.clone(),
                };
                if let Err(err) = checkpoint.save().await {
                    tracing::warn!("Failed to save relay checkpoint: {err}");
                }
            }

            idx += 1;
        }
    }

    relay.shutdown_agents(&ordered_agents);

    // Relay completed normally — delete the checkpoint
    RelayCheckpoint::delete().await;

    // Update OKR run with progress if associated
    if let Some(ref run_id_str) = okr_run_id_str {
        if let Some(repo) = crate::okr::persistence::OkrRepository::from_config()
            .await
            .ok()
        {
            if let Some(run_uuid) = parse_uuid_guarded(run_id_str, "relay_completion_persist") {
                if let Ok(Some(mut run)) = repo.get_run(run_uuid).await {
                    // Update KR progress from checkpoint
                    for (kr_id, value) in &kr_progress {
                        run.update_kr_progress(kr_id, *value);
                    }

                    // Create outcomes per KR with progress (link to actual KR IDs)
                    let relay_id = relay.relay_id().to_string();
                    let base_evidence = vec![
                        format!("relay:{}", relay_id),
                        format!("turns:{}", turns),
                        format!("agents:{}", ordered_agents.len()),
                        format!("status:{}", status),
                        format!("rlm_handoffs:{}", rlm_handoff_count),
                        format!("dynamic_spawns:{}", dynamic_spawn_count),
                    ];

                    // Set outcome type based on status
                    let outcome_type = if status == "converged" {
                        KrOutcomeType::FeatureDelivered
                    } else if status == "max_rounds" {
                        KrOutcomeType::Evidence
                    } else {
                        KrOutcomeType::Evidence
                    };

                    // Create one outcome per KR, linked to the actual KR ID
                    for (kr_id_str, value) in &kr_progress {
                        // Parse KR ID with guardrail to prevent NIL UUID linkage
                        if let Some(kr_uuid) =
                            parse_uuid_guarded(kr_id_str, "relay_outcome_kr_link")
                        {
                            let kr_description = format!(
                                "Relay outcome for KR {}: {} agents, {} turns, status={}",
                                kr_id_str,
                                ordered_agents.len(),
                                turns,
                                status
                            );
                            run.outcomes.push({
                                let mut outcome =
                                    KrOutcome::new(kr_uuid, kr_description).with_value(*value);
                                outcome.run_id = Some(run.id);
                                outcome.outcome_type = outcome_type;
                                outcome.evidence = base_evidence.clone();
                                outcome.source = "autochat relay".to_string();
                                outcome
                            });
                        }
                    }

                    // Mark complete or update status based on execution result
                    if status == "converged" {
                        run.complete();
                    } else if status == "agent_error" {
                        run.status = crate::okr::OkrRunStatus::Failed;
                    } else {
                        run.status = crate::okr::OkrRunStatus::Completed;
                    }
                    // Clear checkpoint ID at completion - checkpoint lifecycle complete
                    run.relay_checkpoint_id = None;
                    let _ = repo.update_run(run).await;
                }
            }
        }
    }

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Finalizing relay summary…".to_string(),
        ))
        .await;

    let mut summary = format!(
        "Autochat complete ({status}) — relay {} with {} agents over {} turns.",
        relay.relay_id(),
        ordered_agents.len(),
        turns,
    );
    if let Some(note) = failure_note {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if rlm_handoff_count > 0 {
        summary.push_str(&format!("\n\nRLM compressed handoffs: {rlm_handoff_count}"));
    }
    if dynamic_spawn_count > 0 {
        summary.push_str(&format!("\nDynamic relay spawns: {dynamic_spawn_count}"));
    }
    summary.push_str(&format!(
        "\n\nFinal relay handoff:\n{}",
        truncate_with_ellipsis(&baton, 4_000)
    ));
    summary.push_str(&format!(
        "\n\nCleanup: deregistered relay agents and disposed {} autochat worker session(s).",
        sessions.len()
    ));

    let relay_id = relay.relay_id().to_string();
    let okr_id_for_completion = okr_id_str.clone();
    let okr_run_id_for_completion = okr_run_id_str.clone();
    let _ = tx
        .send(AutochatUiEvent::Completed {
            summary,
            okr_id: okr_id_for_completion,
            okr_run_id: okr_run_id_for_completion,
            relay_id: Some(relay_id),
        })
        .await;
}

/// Resume an autochat relay from a persisted checkpoint.
///
/// Reloads agent sessions from disk, reconstructs the relay, and continues
/// from the exact round/index where the previous run was interrupted.
async fn resume_autochat_worker(
    tx: mpsc::Sender<AutochatUiEvent>,
    bus: std::sync::Arc<crate::bus::AgentBus>,
    checkpoint: RelayCheckpoint,
) {
    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Resuming relay — loading providers…".to_string(),
        ))
        .await;

    let registry = match crate::provider::ProviderRegistry::from_vault().await {
        Ok(registry) => std::sync::Arc::new(registry),
        Err(err) => {
            let _ = tx
                .send(AutochatUiEvent::SystemMessage(format!(
                    "Failed to load providers for relay resume: {err}"
                )))
                .await;
            let _ = tx
                .send(AutochatUiEvent::Completed {
                    summary: "Relay resume aborted: provider registry unavailable.".to_string(),
                    okr_id: checkpoint.okr_id.clone(),
                    okr_run_id: checkpoint.okr_run_id.clone(),
                    relay_id: None,
                })
                .await;
            return;
        }
    };

    let relay = ProtocolRelayRuntime::new(bus.clone());
    let task = checkpoint.task;
    let model_ref = checkpoint.model_ref;
    let mut ordered_agents = checkpoint.ordered_agents;
    let mut checkpoint_profiles = checkpoint.agent_profiles;
    let mut baton = checkpoint.baton;
    let mut turns = checkpoint.turns;
    let mut convergence_hits = checkpoint.convergence_hits;
    let mut rlm_handoff_count = checkpoint.rlm_handoff_count;
    let mut dynamic_spawn_count = checkpoint.dynamic_spawn_count;
    let start_round = checkpoint.round;
    let start_idx = checkpoint.idx;
    let okr_run_id_str = checkpoint.okr_run_id.clone();
    let mut kr_progress = checkpoint.kr_progress.clone();

    // Load KR targets if OKR is associated
    let kr_targets: HashMap<String, f64> =
        if let (Some(okr_id_val), Some(_run_id)) = (&checkpoint.okr_id, &checkpoint.okr_run_id) {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Ok(okr_uuid) = okr_id_val.parse::<uuid::Uuid>() {
                    if let Ok(Some(okr)) = repo.get_okr(okr_uuid).await {
                        okr.key_results
                            .iter()
                            .map(|kr| (kr.id.to_string(), kr.target_value))
                            .collect()
                    } else {
                        HashMap::new()
                    }
                } else {
                    HashMap::new()
                }
            } else {
                HashMap::new()
            }
        } else {
            HashMap::new()
        };

    // Persist KR progress immediately after resuming from checkpoint
    if !kr_progress.is_empty() {
        if let Some(ref run_id_str) = okr_run_id_str {
            if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                if let Some(run_uuid) = parse_uuid_guarded(run_id_str, "resume_mid_run_persist") {
                    if let Ok(Some(mut run)) = repo.get_run(run_uuid).await {
                        if run.is_resumable() {
                            run.iterations = turns as u32;
                            for (kr_id, value) in &kr_progress {
                                run.update_kr_progress(kr_id, *value);
                            }
                            run.status = crate::okr::OkrRunStatus::Running;
                            let _ = repo.update_run(run).await;
                        }
                    }
                }
            }
        }
    }

    // Reload agent sessions from disk
    let mut sessions: HashMap<String, Session> = HashMap::new();
    let mut load_errors: Vec<String> = Vec::new();

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Reloading agent sessions from disk…".to_string(),
        ))
        .await;

    for (agent_name, session_id) in &checkpoint.agent_session_ids {
        match Session::load(session_id).await {
            Ok(session) => {
                sessions.insert(agent_name.clone(), session);
            }
            Err(err) => {
                load_errors.push(format!(
                    "Failed to reload @{agent_name} ({session_id}): {err}"
                ));
            }
        }
    }

    if !load_errors.is_empty() {
        let _ = tx
            .send(AutochatUiEvent::SystemMessage(format!(
                "Session reload warnings:\n{}",
                load_errors.join("\n")
            )))
            .await;
    }

    // Re-register agents with the relay
    let relay_profiles: Vec<RelayAgentProfile> = checkpoint_profiles
        .iter()
        .map(|(name, _, capabilities)| RelayAgentProfile {
            name: name.clone(),
            capabilities: capabilities.clone(),
        })
        .collect();
    relay.register_agents(&relay_profiles);

    let _ = tx
        .send(AutochatUiEvent::SystemMessage(format!(
            "Resuming relay from round {start_round}, agent index {start_idx}\n\
             Task: {}\n\
             Agents: {}\n\
             Turns completed so far: {turns}",
            truncate_with_ellipsis(&task, 120),
            ordered_agents.join(", ")
        )))
        .await;

    let mut previous_normalized: Option<String> = None;
    let mut status = "max_rounds_reached";
    let mut failure_note: Option<String> = None;

    'relay_loop: for round in start_round..=AUTOCHAT_MAX_ROUNDS {
        let first_idx = if round == start_round { start_idx } else { 0 };
        let mut idx = first_idx;
        while idx < ordered_agents.len() {
            let to = ordered_agents[idx].clone();
            let from = if idx == 0 {
                if round == 1 {
                    "user".to_string()
                } else {
                    ordered_agents[ordered_agents.len() - 1].clone()
                }
            } else {
                ordered_agents[idx - 1].clone()
            };

            turns += 1;
            relay.send_handoff(&from, &to, &baton);
            let _ = tx
                .send(AutochatUiEvent::Progress(format!(
                    "Round {round}/{AUTOCHAT_MAX_ROUNDS} • @{from} → @{to} (resumed)"
                )))
                .await;

            let Some(mut session) = sessions.remove(&to) else {
                status = "agent_error";
                failure_note = Some(format!("Relay agent @{to} session was unavailable."));
                break 'relay_loop;
            };

            let (event_tx, mut event_rx) = mpsc::channel(256);
            let registry_for_prompt = registry.clone();
            let baton_for_prompt = baton.clone();

            let join = tokio::spawn(async move {
                let result = session
                    .prompt_with_events(&baton_for_prompt, event_tx, registry_for_prompt)
                    .await;
                (session, result)
            });

            while !join.is_finished() {
                while let Ok(event) = event_rx.try_recv() {
                    if !matches!(event, SessionEvent::SessionSync(_)) {
                        let _ = tx
                            .send(AutochatUiEvent::AgentEvent {
                                agent_name: to.clone(),
                                event,
                            })
                            .await;
                    }
                }
                tokio::time::sleep(Duration::from_millis(20)).await;
            }

            let (updated_session, result) = match join.await {
                Ok(value) => value,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} task join error: {err}"));
                    break 'relay_loop;
                }
            };

            while let Ok(event) = event_rx.try_recv() {
                if !matches!(event, SessionEvent::SessionSync(_)) {
                    let _ = tx
                        .send(AutochatUiEvent::AgentEvent {
                            agent_name: to.clone(),
                            event,
                        })
                        .await;
                }
            }

            sessions.insert(to.clone(), updated_session);

            let output = match result {
                Ok(response) => response.text,
                Err(err) => {
                    status = "agent_error";
                    failure_note = Some(format!("Relay agent @{to} failed: {err}"));
                    let _ = tx
                        .send(AutochatUiEvent::SystemMessage(format!(
                            "Relay agent @{to} failed: {err}"
                        )))
                        .await;
                    break 'relay_loop;
                }
            };

            let normalized = normalize_for_convergence(&output);
            if previous_normalized.as_deref() == Some(normalized.as_str()) {
                convergence_hits += 1;
            } else {
                convergence_hits = 0;
            }
            previous_normalized = Some(normalized);

            let (next_handoff, used_rlm) =
                prepare_autochat_handoff_with_registry(&task, &to, &output, &model_ref, &registry)
                    .await;
            if used_rlm {
                rlm_handoff_count += 1;
            }

            baton = next_handoff;

            // Update KR progress after each turn
            if !kr_targets.is_empty() {
                let max_turns = ordered_agents.len() * AUTOCHAT_MAX_ROUNDS;
                let progress_ratio = (turns as f64 / max_turns as f64).min(1.0);

                for (kr_id, target) in &kr_targets {
                    let current = progress_ratio * target;
                    let existing = kr_progress.get(kr_id).copied().unwrap_or(0.0);
                    // Only update if progress increased (idempotent)
                    if current > existing {
                        kr_progress.insert(kr_id.clone(), current);
                    }
                }

                // Persist mid-run for real-time visibility (best-effort)
                if let Some(ref run_id_str) = okr_run_id_str {
                    if let Ok(repo) = crate::okr::persistence::OkrRepository::from_config().await {
                        if let Some(run_uuid) =
                            parse_uuid_guarded(run_id_str, "resumed_relay_mid_run_persist")
                        {
                            if let Ok(Some(mut run)) = repo.get_run(run_uuid).await {
                                if run.is_resumable() {
                                    run.iterations = turns as u32;
                                    for (kr_id, value) in &kr_progress {
                                        run.update_kr_progress(kr_id, *value);
                                    }
                                    run.status = crate::okr::OkrRunStatus::Running;
                                    let _ = repo.update_run(run).await;
                                }
                            }
                        }
                    }
                }
            }

            let can_attempt_spawn = dynamic_spawn_count < AUTOCHAT_MAX_DYNAMIC_SPAWNS
                && ordered_agents.len() < AUTOCHAT_MAX_AGENTS
                && output.len() >= AUTOCHAT_SPAWN_CHECK_MIN_CHARS;

            if can_attempt_spawn
                && let Some((name, instructions, capabilities, reason)) =
                    decide_dynamic_spawn_with_registry(
                        &task,
                        &model_ref,
                        &output,
                        round,
                        &ordered_agents,
                        &registry,
                    )
                    .await
            {
                match Session::new().await {
                    Ok(mut spawned_session) => {
                        spawned_session.metadata.model = Some(model_ref.clone());
                        spawned_session.agent = name.clone();
                        spawned_session.bus = Some(bus.clone());
                        spawned_session.add_message(crate::provider::Message {
                            role: Role::System,
                            content: vec![ContentPart::Text {
                                text: instructions.clone(),
                            }],
                        });

                        relay.register_agents(&[RelayAgentProfile {
                            name: name.clone(),
                            capabilities: capabilities.clone(),
                        }]);

                        ordered_agents.insert(idx + 1, name.clone());
                        checkpoint_profiles.push((name.clone(), instructions, capabilities));
                        sessions.insert(name.clone(), spawned_session);
                        dynamic_spawn_count += 1;

                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn: {} joined relay after @{to}.\nReason: {reason}",
                                format_agent_identity(&name)
                            )))
                            .await;
                    }
                    Err(err) => {
                        let _ = tx
                            .send(AutochatUiEvent::SystemMessage(format!(
                                "Dynamic spawn requested but failed to create @{name}: {err}"
                            )))
                            .await;
                    }
                }
            }

            if convergence_hits >= 2 {
                status = "converged";
                break 'relay_loop;
            }

            // Save relay checkpoint
            {
                let agent_session_ids: HashMap<String, String> = sessions
                    .iter()
                    .map(|(name, s)| (name.clone(), s.id.clone()))
                    .collect();
                let next_idx = idx + 1;
                let (ck_round, ck_idx) = if next_idx >= ordered_agents.len() {
                    (round + 1, 0)
                } else {
                    (round, next_idx)
                };
                let ck = RelayCheckpoint {
                    task: task.clone(),
                    model_ref: model_ref.clone(),
                    ordered_agents: ordered_agents.clone(),
                    agent_session_ids,
                    agent_profiles: checkpoint_profiles.clone(),
                    round: ck_round,
                    idx: ck_idx,
                    baton: baton.clone(),
                    turns,
                    convergence_hits,
                    dynamic_spawn_count,
                    rlm_handoff_count,
                    workspace_dir: std::env::current_dir().unwrap_or_default(),
                    started_at: chrono::Utc::now().to_rfc3339(),
                    okr_id: checkpoint.okr_id.clone(),
                    okr_run_id: checkpoint.okr_run_id.clone(),
                    kr_progress: kr_progress.clone(),
                };
                if let Err(err) = ck.save().await {
                    tracing::warn!("Failed to save relay checkpoint: {err}");
                }
            }

            idx += 1;
        }
    }

    relay.shutdown_agents(&ordered_agents);

    // Relay completed normally — delete the checkpoint
    RelayCheckpoint::delete().await;

    // Update OKR run with progress if associated
    if let Some(ref run_id_str) = okr_run_id_str {
        if let Some(repo) = crate::okr::persistence::OkrRepository::from_config()
            .await
            .ok()
        {
            if let Some(run_uuid) =
                parse_uuid_guarded(run_id_str, "resumed_relay_completion_persist")
            {
                if let Ok(Some(mut run)) = repo.get_run(run_uuid).await {
                    // Update KR progress from checkpoint
                    for (kr_id, value) in &kr_progress {
                        run.update_kr_progress(kr_id, *value);
                    }

                    // Create outcomes per KR with progress (link to actual KR IDs)
                    let base_evidence = vec![
                        format!("turns:{}", turns),
                        format!("agents:{}", ordered_agents.len()),
                        format!("status:{}", status),
                        "resumed:true".to_string(),
                    ];

                    let outcome_type = if status == "converged" {
                        KrOutcomeType::FeatureDelivered
                    } else {
                        KrOutcomeType::Evidence
                    };

                    // Create one outcome per KR, linked to the actual KR ID
                    for (kr_id_str, value) in &kr_progress {
                        // Parse KR ID with guardrail to prevent NIL UUID linkage
                        if let Some(kr_uuid) =
                            parse_uuid_guarded(kr_id_str, "resumed_relay_outcome_kr_link")
                        {
                            let kr_description = format!(
                                "Resumed relay outcome for KR {}: {} agents, {} turns, status={}",
                                kr_id_str,
                                ordered_agents.len(),
                                turns,
                                status
                            );
                            run.outcomes.push({
                                let mut outcome =
                                    KrOutcome::new(kr_uuid, kr_description).with_value(*value);
                                outcome.run_id = Some(run.id);
                                outcome.outcome_type = outcome_type;
                                outcome.evidence = base_evidence.clone();
                                outcome.source = "autochat relay (resumed)".to_string();
                                outcome
                            });
                        }
                    }

                    // Mark complete or update status based on execution result
                    if status == "converged" {
                        run.complete();
                    } else if status == "agent_error" {
                        run.status = crate::okr::OkrRunStatus::Failed;
                    } else {
                        run.status = crate::okr::OkrRunStatus::Completed;
                    }
                    // Clear checkpoint ID at completion - checkpoint lifecycle complete
                    run.relay_checkpoint_id = None;
                    let _ = repo.update_run(run).await;
                }
            }
        }
    }

    let _ = tx
        .send(AutochatUiEvent::Progress(
            "Finalizing resumed relay summary…".to_string(),
        ))
        .await;

    let mut summary = format!(
        "Resumed relay complete ({status}) — {} agents over {} turns.",
        ordered_agents.len(),
        turns,
    );
    if let Some(note) = failure_note {
        summary.push_str(&format!("\n\nFailure detail: {note}"));
    }
    if rlm_handoff_count > 0 {
        summary.push_str(&format!("\n\nRLM compressed handoffs: {rlm_handoff_count}"));
    }
    if dynamic_spawn_count > 0 {
        summary.push_str(&format!("\nDynamic relay spawns: {dynamic_spawn_count}"));
    }
    summary.push_str(&format!(
        "\n\nFinal relay handoff:\n{}",
        truncate_with_ellipsis(&baton, 4_000)
    ));

    let _ = tx
        .send(AutochatUiEvent::Completed {
            summary,
            okr_id: checkpoint.okr_id.clone(),
            okr_run_id: checkpoint.okr_run_id.clone(),
            relay_id: Some(relay.relay_id().to_string()),
        })
        .await;
}

impl App {
    fn new() -> Self {
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        let chat_archive_path = directories::ProjectDirs::from("com", "codetether", "codetether")
            .map(|dirs| dirs.data_local_dir().join("chat_events.jsonl"));

        Self {
            input: String::new(),
            cursor_position: 0,
            messages: vec![
                ChatMessage::new("system", "Welcome to CodeTether Agent! Press ? for help."),
                ChatMessage::new(
                    "assistant",
                    "Quick start (easy mode):\n• Type a message to chat with the AI\n• /go <task> - OKR-gated relay (requires approval, tracks outcomes)\n• /autochat <task> - tactical relay (fast path, no OKR)\n• /add <name> - create a helper teammate\n• /talk <name> <message> - message a teammate\n• /list - show teammates\n• /remove <name> - remove teammate\n• /home - return to main chat\n• /help - open help\n\nPower user mode: /spawn, /agent, /swarm, /ralph, /protocol",
                ),
            ],
            current_agent: "build".to_string(),
            scroll: 0,
            show_help: false,
            command_history: Vec::new(),
            history_index: None,
            session: None,
            is_processing: false,
            processing_message: None,
            current_tool: None,
            current_tool_started_at: None,
            processing_started_at: None,
            streaming_text: None,
            streaming_agent_texts: HashMap::new(),
            tool_call_count: 0,
            response_rx: None,
            provider_registry: None,
            workspace_dir: workspace_root.clone(),
            view_mode: ViewMode::Chat,
            chat_layout: ChatLayoutMode::Webview,
            show_inspector: true,
            workspace: WorkspaceSnapshot::capture(&workspace_root, 18),
            swarm_state: SwarmViewState::new(),
            swarm_rx: None,
            ralph_state: RalphViewState::new(),
            ralph_rx: None,
            bus_log_state: BusLogState::new(),
            bus_log_rx: None,
            bus: None,
            session_picker_list: Vec::new(),
            session_picker_selected: 0,
            session_picker_filter: String::new(),
            session_picker_confirm_delete: false,
            session_picker_offset: 0,
            model_picker_list: Vec::new(),
            model_picker_selected: 0,
            model_picker_filter: String::new(),
            agent_picker_selected: 0,
            agent_picker_filter: String::new(),
            protocol_selected: 0,
            protocol_scroll: 0,
            active_model: None,
            active_spawned_agent: None,
            spawned_agents: HashMap::new(),
            agent_response_rxs: Vec::new(),
            agent_tool_started_at: HashMap::new(),
            autochat_rx: None,
            autochat_running: false,
            autochat_started_at: None,
            autochat_status: None,
            chat_archive_path,
            archived_message_count: 0,
            chat_sync_rx: None,
            chat_sync_status: None,
            chat_sync_last_success: None,
            chat_sync_last_error: None,
            chat_sync_uploaded_bytes: 0,
            chat_sync_uploaded_batches: 0,
            secure_environment: false,
            pending_okr_approval: None,
            okr_repository: None,
            last_max_scroll: 0,
        }
    }

    fn refresh_workspace(&mut self) {
        let workspace_root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        self.workspace = WorkspaceSnapshot::capture(&workspace_root, 18);
    }

    fn update_cached_sessions(&mut self, sessions: Vec<SessionSummary>) {
        // Default to 100 sessions, configurable via CODETETHER_SESSION_PICKER_LIMIT env var
        let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        self.session_picker_list = sessions.into_iter().take(limit).collect();
        if self.session_picker_selected >= self.session_picker_list.len() {
            self.session_picker_selected = self.session_picker_list.len().saturating_sub(1);
        }
    }

    fn is_agent_protocol_registered(&self, agent_name: &str) -> bool {
        self.bus
            .as_ref()
            .is_some_and(|bus| bus.registry.get(agent_name).is_some())
    }

    fn protocol_registered_count(&self) -> usize {
        self.bus.as_ref().map_or(0, |bus| bus.registry.len())
    }

    fn protocol_cards(&self) -> Vec<crate::a2a::types::AgentCard> {
        let Some(bus) = &self.bus else {
            return Vec::new();
        };

        let mut ids = bus.registry.agent_ids();
        ids.sort_by_key(|id| id.to_lowercase());

        ids.into_iter()
            .filter_map(|id| bus.registry.get(&id))
            .collect()
    }

    fn open_protocol_view(&mut self) {
        self.protocol_selected = 0;
        self.protocol_scroll = 0;
        self.view_mode = ViewMode::Protocol;
    }

    fn unique_spawned_name(&self, base: &str) -> String {
        if !self.spawned_agents.contains_key(base) {
            return base.to_string();
        }

        let mut suffix = 2usize;
        loop {
            let candidate = format!("{base}-{suffix}");
            if !self.spawned_agents.contains_key(&candidate) {
                return candidate;
            }
            suffix += 1;
        }
    }

    fn build_autochat_profiles(&self, count: usize) -> Vec<(String, String, Vec<String>)> {
        let mut profiles = Vec::with_capacity(count);
        for idx in 0..count {
            let base = format!("auto-agent-{}", idx + 1);
            let name = self.unique_spawned_name(&base);
            let instructions = format!(
                "You are @{name}.\n\
                 Role policy: self-organize from task context and current handoff instead of assuming a fixed persona.\n\
                 Mission: advance the relay with concrete, high-signal next actions and clear ownership boundaries.\n\n\
                 This is a protocol-first relay conversation. Treat the incoming handoff as the authoritative context.\n\
                 Keep your response concise, concrete, and useful for the next specialist.\n\
                 Include one clear recommendation for what the next agent should do.\n\
                 If the task scope is too large, explicitly call out missing specialties and handoff boundaries.",
            );
            let capabilities = vec![
                "generalist".to_string(),
                "self-organizing".to_string(),
                "relay".to_string(),
                "context-handoff".to_string(),
                "rlm-aware".to_string(),
                "autochat".to_string(),
            ];

            profiles.push((name, instructions, capabilities));
        }

        profiles
    }
    async fn start_autochat_execution(
        &mut self,
        agent_count: usize,
        task: String,
        config: &Config,
        okr_id: Option<Uuid>,
        okr_run_id: Option<Uuid>,
    ) {
        if !(2..=AUTOCHAT_MAX_AGENTS).contains(&agent_count) {
            self.messages.push(ChatMessage::new(
                "system",
                format!(
                    "Usage: /autochat <count> <task>\ncount must be between 2 and {AUTOCHAT_MAX_AGENTS}."
                ),
            ));
            return;
        }

        if self.autochat_running {
            self.messages.push(ChatMessage::new(
                "system",
                "Autochat relay already running. Wait for it to finish before starting another.",
            ));
            return;
        }

        let status_msg = if okr_id.is_some() {
            format!(
                "Preparing OKR-gated relay with {agent_count} agents…\nTask: {}\n(Approval-granted execution)",
                truncate_with_ellipsis(&task, 160)
            )
        } else {
            format!(
                "Preparing relay with {agent_count} agents…\nTask: {}\n(Compact mode: live agent streaming here, detailed relay envelopes in /buslog)",
                truncate_with_ellipsis(&task, 180)
            )
        };

        self.messages.push(ChatMessage::new(
            "user",
            format!("/autochat {agent_count} {task}"),
        ));
        self.messages.push(ChatMessage::new("system", status_msg));
        self.scroll = SCROLL_BOTTOM;

        let Some(bus) = self.bus.clone() else {
            self.messages.push(ChatMessage::new(
                "system",
                "Protocol bus unavailable; cannot start /autochat relay.",
            ));
            return;
        };

        let model_ref = self
            .active_model
            .clone()
            .or_else(|| config.default_model.clone())
            .unwrap_or_else(|| "zai/glm-5".to_string());

        let profiles = self.build_autochat_profiles(agent_count);
        if profiles.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No relay profiles could be created.",
            ));
            return;
        }

        let (tx, rx) = mpsc::channel(512);
        self.autochat_rx = Some(rx);
        self.autochat_running = true;
        self.autochat_started_at = Some(Instant::now());
        self.autochat_status = Some("Preparing relay…".to_string());
        self.active_spawned_agent = None;

        tokio::spawn(async move {
            run_autochat_worker(tx, bus, profiles, task, model_ref, okr_id, okr_run_id).await;
        });
    }

    /// Resume an interrupted autochat relay from a checkpoint.
    async fn resume_autochat_relay(&mut self, checkpoint: RelayCheckpoint) {
        if self.autochat_running {
            self.messages.push(ChatMessage::new(
                "system",
                "Autochat relay already running. Wait for it to finish before resuming.",
            ));
            return;
        }

        let Some(bus) = self.bus.clone() else {
            self.messages.push(ChatMessage::new(
                "system",
                "Protocol bus unavailable; cannot resume relay.",
            ));
            return;
        };

        self.messages.push(ChatMessage::new(
            "system",
            format!(
                "Resuming interrupted relay…\nTask: {}\nAgents: {}\nResuming from round {}, agent index {}\nTurns completed: {}",
                truncate_with_ellipsis(&checkpoint.task, 120),
                checkpoint.ordered_agents.join(" → "),
                checkpoint.round,
                checkpoint.idx,
                checkpoint.turns,
            ),
        ));
        self.scroll = SCROLL_BOTTOM;

        let (tx, rx) = mpsc::channel(512);
        self.autochat_rx = Some(rx);
        self.autochat_running = true;
        self.autochat_started_at = Some(Instant::now());
        self.autochat_status = Some("Resuming relay…".to_string());
        self.active_spawned_agent = None;

        tokio::spawn(async move {
            resume_autochat_worker(tx, bus, checkpoint).await;
        });
    }

    async fn submit_message(&mut self, config: &Config) {
        if self.input.is_empty() {
            return;
        }

        let mut message = std::mem::take(&mut self.input);
        let easy_go_requested = is_easy_go_command(&message);
        self.cursor_position = 0;

        // Check for pending OKR approval gate response FIRST
        // This must be before any command normalization
        if let Some(pending) = self.pending_okr_approval.take() {
            let response = message.trim().to_lowercase();
            let approved = matches!(
                response.as_str(),
                "a" | "approve" | "y" | "yes" | "A" | "Approve" | "Y" | "Yes"
            );
            let denied = matches!(
                response.as_str(),
                "d" | "deny" | "n" | "no" | "D" | "Deny" | "N" | "No"
            );

            if approved {
                // User approved - save OKR and run, then execute
                let okr_id = pending.okr.id;
                let run_id = pending.run.id;
                let task = pending.task.clone();
                let agent_count = pending.agent_count;
                let _model = pending.model.clone();

                // Update run status to approved
                let mut approved_run = pending.run;
                approved_run.record_decision(ApprovalDecision::approve(
                    approved_run.id,
                    "User approved via TUI",
                ));

                // Save to repository if available
                if let Some(ref repo) = self.okr_repository {
                    let repo = std::sync::Arc::clone(repo);
                    let okr_to_save = pending.okr;
                    let run_to_save = approved_run;
                    tokio::spawn(async move {
                        if let Err(e) = repo.create_okr(okr_to_save).await {
                            tracing::error!(error = %e, "Failed to save approved OKR");
                        }
                        if let Err(e) = repo.create_run(run_to_save).await {
                            tracing::error!(error = %e, "Failed to save approved OKR run");
                        }
                    });
                }

                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "✅ OKR approved. Starting OKR-gated relay (ID: {})...",
                        okr_id
                    ),
                ));
                self.scroll = SCROLL_BOTTOM;

                // Start execution with OKR IDs
                self.start_autochat_execution(
                    agent_count,
                    task,
                    config,
                    Some(okr_id),
                    Some(run_id),
                )
                .await;
                return;
            } else if denied {
                // User denied - record decision and cancel
                let mut denied_run = pending.run;
                denied_run
                    .record_decision(ApprovalDecision::deny(denied_run.id, "User denied via TUI"));
                self.messages.push(ChatMessage::new(
                    "system",
                    "❌ OKR denied. Relay cancelled.",
                ));
                self.scroll = SCROLL_BOTTOM;
                return;
            } else {
                // Invalid response - re-prompt
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Invalid response. {}\n\nPress [A] to approve or [D] to deny.",
                        pending.approval_prompt()
                    ),
                ));
                self.scroll = SCROLL_BOTTOM;
                // Put the pending approval back
                self.pending_okr_approval = Some(pending);
                // Put the input back so user can try again
                self.input = message;
                return;
            }
        }

        // Save to command history
        if !message.trim().is_empty() {
            self.command_history.push(message.clone());
            self.history_index = None;
        }

        // Easy-mode slash aliases (/go, /add, /talk, /list, ...)
        message = normalize_easy_command(&message);

        if message.trim() == "/help" {
            self.show_help = true;
            return;
        }

        // Backward-compatible /agent command aliases
        if message.trim().starts_with("/agent") {
            let rest = message.trim().strip_prefix("/agent").unwrap_or("").trim();

            if rest.is_empty() {
                self.open_agent_picker();
                return;
            }

            if rest == "pick" || rest == "picker" || rest == "select" {
                self.open_agent_picker();
                return;
            }

            if rest == "main" || rest == "off" {
                if let Some(target) = self.active_spawned_agent.take() {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Exited focused sub-agent chat (@{target})."),
                    ));
                } else {
                    self.messages
                        .push(ChatMessage::new("system", "Already in main chat mode."));
                }
                return;
            }

            if rest == "build" || rest == "plan" {
                self.current_agent = rest.to_string();
                self.active_spawned_agent = None;
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Switched main agent to '{rest}'. (Tab also works.)"),
                ));
                return;
            }

            if rest == "list" || rest == "ls" {
                message = "/agents".to_string();
            } else if let Some(args) = rest
                .strip_prefix("spawn ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                message = format!("/spawn {args}");
            } else if let Some(name) = rest
                .strip_prefix("kill ")
                .map(str::trim)
                .filter(|s| !s.is_empty())
            {
                message = format!("/kill {name}");
            } else if !rest.contains(' ') {
                let target = rest.trim_start_matches('@');
                if self.spawned_agents.contains_key(target) {
                    self.active_spawned_agent = Some(target.to_string());
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Focused chat on @{target}. Type messages directly; use /agent main to exit focus."
                        ),
                    ));
                } else {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "No agent named @{target}. Use /agents to list, or /spawn <name> <instructions> to create one."
                        ),
                    ));
                }
                return;
            } else if let Some((name, content)) = rest.split_once(' ') {
                let target = name.trim().trim_start_matches('@');
                let content = content.trim();
                if target.is_empty() || content.is_empty() {
                    self.messages
                        .push(ChatMessage::new("system", "Usage: /agent <name> <message>"));
                    return;
                }
                message = format!("@{target} {content}");
            } else {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Unknown /agent usage. Try /agent, /agent <name>, /agent <name> <message>, or /agent list.",
                ));
                return;
            }
        }

        // Check for /autochat command
        if let Some(rest) = command_with_optional_args(&message, "/autochat") {
            let Some((count, task)) = parse_autochat_args(rest) else {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Usage: /autochat [count] <task>\nEasy mode: /go <task>\nExamples:\n  /autochat implement protocol-first relay with tests\n  /autochat 4 implement protocol-first relay with tests\ncount range: 2-{} (default: {})",
                        AUTOCHAT_MAX_AGENTS,
                        AUTOCHAT_DEFAULT_AGENTS,
                    ),
                ));
                return;
            };

            if easy_go_requested {
                let current_model = self
                    .active_model
                    .as_deref()
                    .or(config.default_model.as_deref());
                let next_model = next_go_model(current_model);
                self.active_model = Some(next_model.clone());
                if let Some(session) = self.session.as_mut() {
                    session.metadata.model = Some(next_model.clone());
                }

                // Initialize OKR repository if not already done
                if self.okr_repository.is_none() {
                    if let Ok(repo) = OkrRepository::from_config().await {
                        self.okr_repository = Some(std::sync::Arc::new(repo));
                    }
                }

                // Create pending OKR approval gate
                // For /go, default to max concurrency unless user specified a count
                let go_count = if rest.trim().starts_with(|c: char| c.is_ascii_digit()) {
                    count
                } else {
                    AUTOCHAT_MAX_AGENTS
                };
                let pending =
                    PendingOkrApproval::new(task.to_string(), go_count, next_model.clone());

                self.messages
                    .push(ChatMessage::new("system", pending.approval_prompt()));
                self.scroll = SCROLL_BOTTOM;

                // Store pending approval and wait for user input
                self.pending_okr_approval = Some(pending);
                return;
            }

            self.start_autochat_execution(count, task.to_string(), config, None, None)
                .await;
            return;
        }

        // Check for /swarm command
        if let Some(task) = command_with_optional_args(&message, "/swarm") {
            if task.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Usage: /swarm <task description>",
                ));
                return;
            }
            self.start_swarm_execution(task.to_string(), config).await;
            return;
        }

        // Check for /ralph command
        if message.trim().starts_with("/ralph") {
            let prd_path = message
                .trim()
                .strip_prefix("/ralph")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .unwrap_or("prd.json")
                .to_string();
            self.start_ralph_execution(prd_path, config).await;
            return;
        }

        if message.trim() == "/webview" {
            self.chat_layout = ChatLayoutMode::Webview;
            self.messages.push(ChatMessage::new(
                "system",
                "Switched to webview layout. Use /classic to return to single-pane chat.",
            ));
            return;
        }

        if message.trim() == "/classic" {
            self.chat_layout = ChatLayoutMode::Classic;
            self.messages.push(ChatMessage::new(
                "system",
                "Switched to classic layout. Use /webview for dashboard-style panes.",
            ));
            return;
        }

        if message.trim() == "/inspector" {
            self.show_inspector = !self.show_inspector;
            let state = if self.show_inspector {
                "enabled"
            } else {
                "disabled"
            };
            self.messages.push(ChatMessage::new(
                "system",
                format!("Inspector pane {}. Press F3 to toggle quickly.", state),
            ));
            return;
        }

        if message.trim() == "/refresh" {
            self.refresh_workspace();
            let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);
            // Reset offset on refresh
            self.session_picker_offset = 0;
            match list_sessions_with_opencode_paged(&self.workspace_dir, limit, 0).await {
                Ok(sessions) => self.update_cached_sessions(sessions),
                Err(err) => self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Workspace refreshed, but failed to refresh sessions: {}",
                        err
                    ),
                )),
            }
            self.messages.push(ChatMessage::new(
                "system",
                "Workspace and session cache refreshed.",
            ));
            return;
        }

        if message.trim() == "/archive" {
            let details = if let Some(path) = &self.chat_archive_path {
                format!(
                    "Chat archive: {}\nCaptured records in this run: {}\n{}",
                    path.display(),
                    self.archived_message_count,
                    self.chat_sync_summary(),
                )
            } else {
                format!(
                    "Chat archive path unavailable in this environment.\n{}",
                    self.chat_sync_summary()
                )
            };
            self.messages.push(ChatMessage::new("system", details));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        // Check for /view command to toggle views
        if message.trim() == "/view" {
            self.view_mode = match self.view_mode {
                ViewMode::Chat
                | ViewMode::SessionPicker
                | ViewMode::ModelPicker
                | ViewMode::AgentPicker
                | ViewMode::BusLog
                | ViewMode::Protocol => ViewMode::Swarm,
                ViewMode::Swarm | ViewMode::Ralph => ViewMode::Chat,
            };
            return;
        }

        // Check for /buslog command to open protocol bus log
        if message.trim() == "/buslog" || message.trim() == "/bus" {
            self.view_mode = ViewMode::BusLog;
            return;
        }

        // Check for /protocol command to inspect registered AgentCards
        if message.trim() == "/protocol" || message.trim() == "/registry" {
            self.open_protocol_view();
            return;
        }

        // Check for /spawn command - create a named sub-agent
        if let Some(rest) = command_with_optional_args(&message, "/spawn") {
            let default_instructions = |agent_name: &str| {
                let profile = agent_profile(agent_name);
                format!(
                    "You are @{agent_name}, codename {codename}.\n\
                     Profile: {profile_line}.\n\
                     Personality: {personality}.\n\
                     Collaboration style: {style}.\n\
                     Signature move: {signature}.\n\
                     Be a helpful teammate: explain in simple words, short steps, and a friendly tone.",
                    codename = profile.codename,
                    profile_line = profile.profile,
                    personality = profile.personality,
                    style = profile.collaboration_style,
                    signature = profile.signature_move,
                )
            };

            let (name, instructions, used_default_instructions) = if rest.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "Usage: /spawn <name> [instructions]\nEasy mode: /add <name>\nExample: /spawn planner You are a planning agent. Break tasks into steps.",
                ));
                return;
            } else {
                let mut parts = rest.splitn(2, char::is_whitespace);
                let name = parts.next().unwrap_or("").trim();
                if name.is_empty() {
                    self.messages.push(ChatMessage::new(
                        "system",
                        "Usage: /spawn <name> [instructions]\nEasy mode: /add <name>",
                    ));
                    return;
                }

                let instructions = parts.next().map(str::trim).filter(|s| !s.is_empty());
                match instructions {
                    Some(custom) => (name.to_string(), custom.to_string(), false),
                    None => (name.to_string(), default_instructions(name), true),
                }
            };

            if self.spawned_agents.contains_key(&name) {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Agent @{name} already exists. Use /kill {name} first."),
                ));
                return;
            }

            match Session::new().await {
                Ok(mut session) => {
                    // Use the same model as the main chat
                    session.metadata.model = self
                        .active_model
                        .clone()
                        .or_else(|| config.default_model.clone());
                    session.agent = name.clone();
                    if let Some(ref b) = self.bus {
                        session.bus = Some(b.clone());
                    }

                    // Add system message with the agent's instructions
                    session.add_message(crate::provider::Message {
                        role: Role::System,
                        content: vec![ContentPart::Text {
                            text: format!(
                                "You are @{name}, a specialized sub-agent. {instructions}\n\n\
                                 When you receive a message from another agent (prefixed with their name), \
                                 respond helpfully. Keep responses concise and focused on your specialty."
                            ),
                        }],
                    });

                    // Announce on bus
                    let mut protocol_registered = false;
                    if let Some(ref bus) = self.bus {
                        let handle = bus.handle(&name);
                        handle.announce_ready(vec!["sub-agent".to_string(), name.clone()]);
                        protocol_registered = bus.registry.get(&name).is_some();
                    }

                    let agent = SpawnedAgent {
                        name: name.clone(),
                        instructions: instructions.clone(),
                        session,
                        is_processing: false,
                    };
                    self.spawned_agents.insert(name.clone(), agent);
                    self.active_spawned_agent = Some(name.clone());

                    let protocol_line = if protocol_registered {
                        format!("Protocol registration: ✅ bus://local/{name}")
                    } else {
                        "Protocol registration: ⚠ unavailable (bus not connected)".to_string()
                    };
                    let profile_summary = format_agent_profile_summary(&name);

                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Spawned agent {}\nProfile: {}\nInstructions: {instructions}\nFocused chat on @{name}. Type directly, or use @{name} <message>.\n{protocol_line}{}",
                            format_agent_identity(&name),
                            profile_summary,
                            if used_default_instructions {
                                "\nTip: I used friendly default instructions. You can customize with /add <name> <instructions>."
                            } else {
                                ""
                            }
                        ),
                    ));
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to spawn agent: {e}"),
                    ));
                }
            }
            return;
        }

        // Check for /agents command - list spawned agents
        if message.trim() == "/agents" {
            if self.spawned_agents.is_empty() {
                self.messages.push(ChatMessage::new(
                    "system",
                    "No agents spawned. Use /spawn <name> <instructions> to create one.",
                ));
            } else {
                let mut lines = vec![format!(
                    "Active agents: {} (protocol registered: {})",
                    self.spawned_agents.len(),
                    self.protocol_registered_count()
                )];

                let mut agents = self.spawned_agents.iter().collect::<Vec<_>>();
                agents.sort_by(|(a, _), (b, _)| a.to_lowercase().cmp(&b.to_lowercase()));

                for (name, agent) in agents {
                    let status = if agent.is_processing {
                        "⚡ working"
                    } else {
                        "● idle"
                    };
                    let protocol_status = if self.is_agent_protocol_registered(name) {
                        "🔗 protocol"
                    } else {
                        "⚠ protocol-pending"
                    };
                    let focused = if self.active_spawned_agent.as_deref() == Some(name.as_str()) {
                        " [focused]"
                    } else {
                        ""
                    };
                    let profile_summary = format_agent_profile_summary(name);
                    lines.push(format!(
                        "  {} @{name} [{status}] {protocol_status}{focused} — {} | {}",
                        agent_avatar(name),
                        profile_summary,
                        agent.instructions
                    ));
                }
                self.messages
                    .push(ChatMessage::new("system", lines.join("\n")));
                self.messages.push(ChatMessage::new(
                    "system",
                    "Tip: use /agent to open the picker, /agent <name> to focus, or Ctrl+A.",
                ));
            }
            return;
        }

        // Check for /kill command - remove a spawned agent
        if let Some(name) = command_with_optional_args(&message, "/kill") {
            if name.is_empty() {
                self.messages
                    .push(ChatMessage::new("system", "Usage: /kill <name>"));
                return;
            }

            let name = name.to_string();
            if self.spawned_agents.remove(&name).is_some() {
                // Remove its response channels
                self.agent_response_rxs.retain(|(n, _)| n != &name);
                self.streaming_agent_texts.remove(&name);
                if self.active_spawned_agent.as_deref() == Some(name.as_str()) {
                    self.active_spawned_agent = None;
                }
                // Announce shutdown on bus
                if let Some(ref bus) = self.bus {
                    let handle = bus.handle(&name);
                    handle.announce_shutdown();
                }
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Agent @{name} removed."),
                ));
            } else {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("No agent named @{name}. Use /agents to list."),
                ));
            }
            return;
        }

        // Check for @mention - route message to a specific spawned agent
        if message.trim().starts_with('@') {
            let trimmed = message.trim();
            let (target, content) = match trimmed.split_once(' ') {
                Some((mention, rest)) => (
                    mention.strip_prefix('@').unwrap_or(mention).to_string(),
                    rest.to_string(),
                ),
                None => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Usage: @agent_name your message\nAvailable: {}",
                            if self.spawned_agents.is_empty() {
                                "none (use /spawn first)".to_string()
                            } else {
                                self.spawned_agents
                                    .keys()
                                    .map(|n| format!("@{n}"))
                                    .collect::<Vec<_>>()
                                    .join(", ")
                            }
                        ),
                    ));
                    return;
                }
            };

            if !self.spawned_agents.contains_key(&target) {
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "No agent named @{target}. Available: {}",
                        if self.spawned_agents.is_empty() {
                            "none (use /spawn first)".to_string()
                        } else {
                            self.spawned_agents
                                .keys()
                                .map(|n| format!("@{n}"))
                                .collect::<Vec<_>>()
                                .join(", ")
                        }
                    ),
                ));
                return;
            }

            // Show the user's @mention message in chat
            self.messages
                .push(ChatMessage::new("user", format!("@{target} {content}")));
            self.scroll = SCROLL_BOTTOM;

            // Send the message over the bus
            if let Some(ref bus) = self.bus {
                let handle = bus.handle("user");
                handle.send_to_agent(
                    &target,
                    vec![crate::a2a::types::Part::Text {
                        text: content.clone(),
                    }],
                );
            }

            // Send the message to the target agent's session
            self.send_to_agent(&target, &content, config).await;
            return;
        }

        // If a spawned agent is focused, route plain messages there automatically.
        if !message.trim().starts_with('/')
            && let Some(target) = self.active_spawned_agent.clone()
        {
            if !self.spawned_agents.contains_key(&target) {
                self.active_spawned_agent = None;
                self.messages.push(ChatMessage::new(
                    "system",
                    format!(
                        "Focused agent @{target} is no longer available. Use /agents or /spawn to continue."
                    ),
                ));
                return;
            }

            let content = message.trim().to_string();
            if content.is_empty() {
                return;
            }

            self.messages
                .push(ChatMessage::new("user", format!("@{target} {content}")));
            self.scroll = SCROLL_BOTTOM;

            if let Some(ref bus) = self.bus {
                let handle = bus.handle("user");
                handle.send_to_agent(
                    &target,
                    vec![crate::a2a::types::Part::Text {
                        text: content.clone(),
                    }],
                );
            }

            self.send_to_agent(&target, &content, config).await;
            return;
        }

        // Check for /sessions command - open session picker
        if message.trim() == "/sessions" {
            let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                .ok()
                .and_then(|v| v.parse().ok())
                .unwrap_or(100);
            // Reset offset when opening session picker
            self.session_picker_offset = 0;
            match list_sessions_with_opencode_paged(&self.workspace_dir, limit, 0).await {
                Ok(sessions) => {
                    if sessions.is_empty() {
                        self.messages
                            .push(ChatMessage::new("system", "No saved sessions found."));
                    } else {
                        self.update_cached_sessions(sessions);
                        self.session_picker_selected = 0;
                        self.view_mode = ViewMode::SessionPicker;
                    }
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to list sessions: {}", e),
                    ));
                }
            }
            return;
        }

        // Check for /resume command to load a session or resume an interrupted relay
        if message.trim() == "/resume" || message.trim().starts_with("/resume ") {
            let session_id = message
                .trim()
                .strip_prefix("/resume")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());

            // If no specific session ID, check for an interrupted relay checkpoint first
            if session_id.is_none() {
                if let Some(checkpoint) = RelayCheckpoint::load().await {
                    self.messages.push(ChatMessage::new("user", "/resume"));
                    self.resume_autochat_relay(checkpoint).await;
                    return;
                }
            }

            let loaded = if let Some(id) = session_id {
                if let Some(oc_id) = id.strip_prefix("opencode_") {
                    if let Some(storage) = crate::opencode::OpenCodeStorage::new() {
                        Session::from_opencode(oc_id, &storage).await
                    } else {
                        Err(anyhow::anyhow!("OpenCode storage not available"))
                    }
                } else {
                    Session::load(id).await
                }
            } else {
                match Session::last_for_directory(Some(&self.workspace_dir)).await {
                    Ok(s) => Ok(s),
                    Err(_) => Session::last_opencode_for_directory(&self.workspace_dir).await,
                }
            };

            match loaded {
                Ok(session) => {
                    // Convert session messages to chat messages
                    self.messages.clear();
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!(
                            "Resumed session: {}\nCreated: {}\n{} messages loaded",
                            session.title.as_deref().unwrap_or("(untitled)"),
                            session.created_at.format("%Y-%m-%d %H:%M"),
                            session.messages.len()
                        ),
                    ));

                    for msg in &session.messages {
                        let role_str = match msg.role {
                            Role::System => "system",
                            Role::User => "user",
                            Role::Assistant => "assistant",
                            Role::Tool => "tool",
                        };

                        // Process each content part separately
                        for part in &msg.content {
                            match part {
                                ContentPart::Text { text } => {
                                    if !text.is_empty() {
                                        self.messages
                                            .push(ChatMessage::new(role_str, text.clone()));
                                    }
                                }
                                ContentPart::Image { url, mime_type } => {
                                    self.messages.push(
                                        ChatMessage::new(role_str, "").with_message_type(
                                            MessageType::Image {
                                                url: url.clone(),
                                                mime_type: mime_type.clone(),
                                            },
                                        ),
                                    );
                                }
                                ContentPart::ToolCall {
                                    name, arguments, ..
                                } => {
                                    let (preview, truncated) = build_tool_arguments_preview(
                                        name,
                                        arguments,
                                        TOOL_ARGS_PREVIEW_MAX_LINES,
                                        TOOL_ARGS_PREVIEW_MAX_BYTES,
                                    );
                                    self.messages.push(
                                        ChatMessage::new(role_str, format!("🔧 {name}"))
                                            .with_message_type(MessageType::ToolCall {
                                                name: name.clone(),
                                                arguments_preview: preview,
                                                arguments_len: arguments.len(),
                                                truncated,
                                            }),
                                    );
                                }
                                ContentPart::ToolResult { content, .. } => {
                                    let truncated = truncate_with_ellipsis(content, 500);
                                    let (preview, preview_truncated) = build_text_preview(
                                        content,
                                        TOOL_OUTPUT_PREVIEW_MAX_LINES,
                                        TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                                    );
                                    self.messages.push(
                                        ChatMessage::new(
                                            role_str,
                                            format!("✅ Result\n{truncated}"),
                                        )
                                        .with_message_type(MessageType::ToolResult {
                                            name: "tool".to_string(),
                                            output_preview: preview,
                                            output_len: content.len(),
                                            truncated: preview_truncated,
                                            success: true,
                                            duration_ms: None,
                                        }),
                                    );
                                }
                                ContentPart::File { path, mime_type } => {
                                    self.messages.push(
                                        ChatMessage::new(role_str, format!("📎 {}", path))
                                            .with_message_type(MessageType::File {
                                                path: path.clone(),
                                                mime_type: mime_type.clone(),
                                            }),
                                    );
                                }
                                ContentPart::Thinking { text } => {
                                    if !text.is_empty() {
                                        self.messages.push(
                                            ChatMessage::new(role_str, text.clone())
                                                .with_message_type(MessageType::Thinking(
                                                    text.clone(),
                                                )),
                                        );
                                    }
                                }
                            }
                        }
                    }

                    self.current_agent = session.agent.clone();
                    self.session = Some(session);
                    self.scroll = SCROLL_BOTTOM;
                }
                Err(e) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Failed to load session: {}", e),
                    ));
                }
            }
            return;
        }

        // Check for /model command - open model picker
        if message.trim() == "/model" || message.trim().starts_with("/model ") {
            let direct_model = message
                .trim()
                .strip_prefix("/model")
                .map(|s| s.trim())
                .filter(|s| !s.is_empty());

            if let Some(model_str) = direct_model {
                // Direct set: /model provider/model-name
                self.active_model = Some(model_str.to_string());
                if let Some(session) = self.session.as_mut() {
                    session.metadata.model = Some(model_str.to_string());
                }
                self.messages.push(ChatMessage::new(
                    "system",
                    format!("Model set to: {}", model_str),
                ));
            } else {
                // Open model picker
                self.open_model_picker(config).await;
            }
            return;
        }

        // Check for /undo command - remove last user turn and response
        if message.trim() == "/undo" {
            // Remove from TUI messages: walk backwards and remove everything
            // until we've removed the last "user" message (inclusive)
            let mut found_user = false;
            while let Some(msg) = self.messages.last() {
                if msg.role == "user" {
                    if found_user {
                        break; // hit the previous user turn, stop
                    }
                    found_user = true;
                }
                // Skip system messages that aren't part of the turn
                if msg.role == "system" && !found_user {
                    break;
                }
                self.messages.pop();
            }

            if !found_user {
                self.messages
                    .push(ChatMessage::new("system", "Nothing to undo."));
                return;
            }

            // Remove from session: walk backwards removing the last user message
            // and all assistant/tool messages after it
            if let Some(session) = self.session.as_mut() {
                let mut found_session_user = false;
                while let Some(msg) = session.messages.last() {
                    if msg.role == crate::provider::Role::User {
                        if found_session_user {
                            break;
                        }
                        found_session_user = true;
                    }
                    if msg.role == crate::provider::Role::System && !found_session_user {
                        break;
                    }
                    session.messages.pop();
                }
                if let Err(e) = session.save().await {
                    tracing::warn!(error = %e, "Failed to save session after undo");
                }
            }

            self.messages.push(ChatMessage::new(
                "system",
                "Undid last message and response.",
            ));
            self.scroll = SCROLL_BOTTOM;
            return;
        }

        // Check for /new command to start a fresh session
        if message.trim() == "/new" {
            self.session = None;
            self.messages.clear();
            self.messages.push(ChatMessage::new(
                "system",
                "Started a new session. Previous session was saved.",
            ));
            return;
        }

        // Add user message
        self.messages
            .push(ChatMessage::new("user", message.clone()));

        // Auto-scroll to bottom when user sends a message
        self.scroll = SCROLL_BOTTOM;

        let current_agent = self.current_agent.clone();
        let model = self
            .active_model
            .clone()
            .or_else(|| {
                config
                    .agents
                    .get(&current_agent)
                    .and_then(|agent| agent.model.clone())
            })
            .or_else(|| config.default_model.clone())
            .or_else(|| Some("zai/glm-5".to_string()));

        // Initialize session if needed
        if self.session.is_none() {
            match Session::new().await {
                Ok(mut session) => {
                    if let Some(ref b) = self.bus {
                        session.bus = Some(b.clone());
                    }
                    self.session = Some(session);
                }
                Err(err) => {
                    tracing::error!(error = %err, "Failed to create session");
                    self.messages
                        .push(ChatMessage::new("assistant", format!("Error: {err}")));
                    return;
                }
            }
        }

        let session = match self.session.as_mut() {
            Some(session) => session,
            None => {
                self.messages.push(ChatMessage::new(
                    "assistant",
                    "Error: session not initialized",
                ));
                return;
            }
        };

        if let Some(model) = model {
            session.metadata.model = Some(model);
        }

        session.agent = current_agent;

        // Set processing state
        self.is_processing = true;
        self.processing_message = Some("Thinking...".to_string());
        self.current_tool = None;
        self.current_tool_started_at = None;
        self.processing_started_at = Some(Instant::now());
        self.streaming_text = None;

        // Load provider registry once and cache it
        if self.provider_registry.is_none() {
            match crate::provider::ProviderRegistry::from_vault().await {
                Ok(registry) => {
                    self.provider_registry = Some(std::sync::Arc::new(registry));
                }
                Err(err) => {
                    tracing::error!(error = %err, "Failed to load provider registry");
                    self.messages.push(ChatMessage::new(
                        "assistant",
                        format!("Error loading providers: {err}"),
                    ));
                    self.is_processing = false;
                    return;
                }
            }
        }
        let registry = self.provider_registry.clone().unwrap();

        // Create channel for async communication
        let (tx, rx) = mpsc::channel(100);
        self.response_rx = Some(rx);

        // Clone session for async processing
        let session_clone = session.clone();
        let message_clone = message.clone();

        // Spawn async task to process the message with event streaming
        tokio::spawn(async move {
            let mut session = session_clone;
            if let Err(err) = session
                .prompt_with_events(&message_clone, tx.clone(), registry)
                .await
            {
                tracing::error!(error = %err, "Agent processing failed");
                let _ = tx.send(SessionEvent::Error(format!("Error: {err}"))).await;
                let _ = tx.send(SessionEvent::Done).await;
            }
        });
    }

    fn handle_response(&mut self, event: SessionEvent) {
        // Auto-scroll to bottom when new content arrives
        self.scroll = SCROLL_BOTTOM;

        match event {
            SessionEvent::Thinking => {
                self.processing_message = Some("Thinking...".to_string());
                self.current_tool = None;
                self.current_tool_started_at = None;
                if self.processing_started_at.is_none() {
                    self.processing_started_at = Some(Instant::now());
                }
            }
            SessionEvent::ToolCallStart { name, arguments } => {
                // Flush any streaming text before showing tool call
                if let Some(text) = self.streaming_text.take() {
                    if !text.is_empty() {
                        self.messages.push(ChatMessage::new("assistant", text));
                    }
                }
                self.processing_message = Some(format!("Running {}...", name));
                self.current_tool = Some(name.clone());
                self.current_tool_started_at = Some(Instant::now());
                self.tool_call_count += 1;

                let (preview, truncated) = build_tool_arguments_preview(
                    &name,
                    &arguments,
                    TOOL_ARGS_PREVIEW_MAX_LINES,
                    TOOL_ARGS_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new("tool", format!("🔧 {}", name)).with_message_type(
                        MessageType::ToolCall {
                            name,
                            arguments_preview: preview,
                            arguments_len: arguments.len(),
                            truncated,
                        },
                    ),
                );
            }
            SessionEvent::ToolCallComplete {
                name,
                output,
                success,
            } => {
                let icon = if success { "✓" } else { "✗" };
                let duration_ms = self
                    .current_tool_started_at
                    .take()
                    .map(|started| started.elapsed().as_millis() as u64);

                let (preview, truncated) = build_text_preview(
                    &output,
                    TOOL_OUTPUT_PREVIEW_MAX_LINES,
                    TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new("tool", format!("{} {}", icon, name)).with_message_type(
                        MessageType::ToolResult {
                            name,
                            output_preview: preview,
                            output_len: output.len(),
                            truncated,
                            success,
                            duration_ms,
                        },
                    ),
                );
                self.current_tool = None;
                self.processing_message = Some("Thinking...".to_string());
            }
            SessionEvent::TextChunk(text) => {
                // Show streaming text as it arrives (before TextComplete finalizes)
                self.streaming_text = Some(text);
            }
            SessionEvent::ThinkingComplete(text) => {
                if !text.is_empty() {
                    self.messages.push(
                        ChatMessage::new("assistant", &text)
                            .with_message_type(MessageType::Thinking(text)),
                    );
                }
            }
            SessionEvent::TextComplete(text) => {
                // Clear streaming preview and add the final message
                self.streaming_text = None;
                if !text.is_empty() {
                    self.messages.push(ChatMessage::new("assistant", text));
                }
            }
            SessionEvent::UsageReport {
                prompt_tokens,
                completion_tokens,
                duration_ms,
                model,
            } => {
                let cost_usd = estimate_cost(&model, prompt_tokens, completion_tokens);
                let meta = UsageMeta {
                    prompt_tokens,
                    completion_tokens,
                    duration_ms,
                    cost_usd,
                };
                // Attach to the most recent assistant message
                if let Some(msg) = self
                    .messages
                    .iter_mut()
                    .rev()
                    .find(|m| m.role == "assistant")
                {
                    msg.usage_meta = Some(meta);
                }
            }
            SessionEvent::SessionSync(session) => {
                // Sync the updated session (with full conversation history) back
                // so subsequent messages include prior context.
                self.session = Some(session);
            }
            SessionEvent::Error(err) => {
                self.current_tool_started_at = None;
                self.messages
                    .push(ChatMessage::new("assistant", format!("Error: {}", err)));
            }
            SessionEvent::Done => {
                self.is_processing = false;
                self.processing_message = None;
                self.current_tool = None;
                self.current_tool_started_at = None;
                self.processing_started_at = None;
                self.streaming_text = None;
                self.response_rx = None;
            }
        }
    }

    /// Send a message to a specific spawned agent
    async fn send_to_agent(&mut self, agent_name: &str, message: &str, _config: &Config) {
        // Load provider registry if needed
        if self.provider_registry.is_none() {
            match crate::provider::ProviderRegistry::from_vault().await {
                Ok(registry) => {
                    self.provider_registry = Some(std::sync::Arc::new(registry));
                }
                Err(err) => {
                    self.messages.push(ChatMessage::new(
                        "system",
                        format!("Error loading providers: {err}"),
                    ));
                    return;
                }
            }
        }
        let registry = self.provider_registry.clone().unwrap();

        let agent = match self.spawned_agents.get_mut(agent_name) {
            Some(a) => a,
            None => return,
        };

        agent.is_processing = true;
        self.streaming_agent_texts.remove(agent_name);
        let session_clone = agent.session.clone();
        let msg_clone = message.to_string();
        let agent_name_owned = agent_name.to_string();
        let bus_arc = self.bus.clone();

        let (tx, rx) = mpsc::channel(100);
        self.agent_response_rxs.push((agent_name.to_string(), rx));

        tokio::spawn(async move {
            let mut session = session_clone;
            if let Err(err) = session
                .prompt_with_events(&msg_clone, tx.clone(), registry)
                .await
            {
                tracing::error!(agent = %agent_name_owned, error = %err, "Spawned agent failed");
                let _ = tx.send(SessionEvent::Error(format!("Error: {err}"))).await;
                let _ = tx.send(SessionEvent::Done).await;
            }

            // Send the agent's response over the bus
            if let Some(ref bus) = bus_arc {
                let handle = bus.handle(&agent_name_owned);
                handle.send(
                    format!("agent.{agent_name_owned}.events"),
                    crate::bus::BusMessage::AgentMessage {
                        from: agent_name_owned.clone(),
                        to: "user".to_string(),
                        parts: vec![crate::a2a::types::Part::Text {
                            text: "(response complete)".to_string(),
                        }],
                    },
                );
            }
        });
    }

    /// Handle an event from a spawned agent
    fn handle_agent_response(&mut self, agent_name: &str, event: SessionEvent) {
        self.scroll = SCROLL_BOTTOM;

        match event {
            SessionEvent::Thinking => {
                // Show thinking indicator for this agent
                if let Some(agent) = self.spawned_agents.get_mut(agent_name) {
                    agent.is_processing = true;
                }
            }
            SessionEvent::ToolCallStart { name, arguments } => {
                self.streaming_agent_texts.remove(agent_name);
                self.agent_tool_started_at
                    .insert(agent_name.to_string(), Instant::now());
                let (preview, truncated) = build_tool_arguments_preview(
                    &name,
                    &arguments,
                    TOOL_ARGS_PREVIEW_MAX_LINES,
                    TOOL_ARGS_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new(
                        "tool",
                        format!("🔧 {} → {name}", format_agent_identity(agent_name)),
                    )
                    .with_message_type(MessageType::ToolCall {
                        name,
                        arguments_preview: preview,
                        arguments_len: arguments.len(),
                        truncated,
                    })
                    .with_agent_name(agent_name),
                );
            }
            SessionEvent::ToolCallComplete {
                name,
                output,
                success,
            } => {
                self.streaming_agent_texts.remove(agent_name);
                let icon = if success { "✓" } else { "✗" };
                let duration_ms = self
                    .agent_tool_started_at
                    .remove(agent_name)
                    .map(|started| started.elapsed().as_millis() as u64);
                let (preview, truncated) = build_text_preview(
                    &output,
                    TOOL_OUTPUT_PREVIEW_MAX_LINES,
                    TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                );
                self.messages.push(
                    ChatMessage::new(
                        "tool",
                        format!("{icon} {} → {name}", format_agent_identity(agent_name)),
                    )
                    .with_message_type(MessageType::ToolResult {
                        name,
                        output_preview: preview,
                        output_len: output.len(),
                        truncated,
                        success,
                        duration_ms,
                    })
                    .with_agent_name(agent_name),
                );
            }
            SessionEvent::TextChunk(text) => {
                if text.is_empty() {
                    self.streaming_agent_texts.remove(agent_name);
                } else {
                    self.streaming_agent_texts
                        .insert(agent_name.to_string(), text);
                }
            }
            SessionEvent::ThinkingComplete(text) => {
                self.streaming_agent_texts.remove(agent_name);
                if !text.is_empty() {
                    self.messages.push(
                        ChatMessage::new("assistant", &text)
                            .with_message_type(MessageType::Thinking(text))
                            .with_agent_name(agent_name),
                    );
                }
            }
            SessionEvent::TextComplete(text) => {
                self.streaming_agent_texts.remove(agent_name);
                if !text.is_empty() {
                    self.messages
                        .push(ChatMessage::new("assistant", &text).with_agent_name(agent_name));
                }
            }
            SessionEvent::UsageReport {
                prompt_tokens,
                completion_tokens,
                duration_ms,
                model,
            } => {
                let cost_usd = estimate_cost(&model, prompt_tokens, completion_tokens);
                let meta = UsageMeta {
                    prompt_tokens,
                    completion_tokens,
                    duration_ms,
                    cost_usd,
                };
                if let Some(msg) =
                    self.messages.iter_mut().rev().find(|m| {
                        m.role == "assistant" && m.agent_name.as_deref() == Some(agent_name)
                    })
                {
                    msg.usage_meta = Some(meta);
                }
            }
            SessionEvent::SessionSync(session) => {
                if let Some(agent) = self.spawned_agents.get_mut(agent_name) {
                    agent.session = session;
                }
            }
            SessionEvent::Error(err) => {
                self.streaming_agent_texts.remove(agent_name);
                self.agent_tool_started_at.remove(agent_name);
                self.messages.push(
                    ChatMessage::new("assistant", format!("Error: {err}"))
                        .with_agent_name(agent_name),
                );
            }
            SessionEvent::Done => {
                self.streaming_agent_texts.remove(agent_name);
                self.agent_tool_started_at.remove(agent_name);
                if let Some(agent) = self.spawned_agents.get_mut(agent_name) {
                    agent.is_processing = false;
                }
            }
        }
    }

    fn handle_autochat_event(&mut self, event: AutochatUiEvent) -> bool {
        match event {
            AutochatUiEvent::Progress(status) => {
                self.autochat_status = Some(status);
                false
            }
            AutochatUiEvent::SystemMessage(text) => {
                self.autochat_status = Some(
                    text.lines()
                        .next()
                        .unwrap_or("Relay update")
                        .trim()
                        .to_string(),
                );
                self.messages.push(ChatMessage::new("system", text));
                self.scroll = SCROLL_BOTTOM;
                false
            }
            AutochatUiEvent::AgentEvent { agent_name, event } => {
                self.autochat_status = Some(format!("Streaming from @{agent_name}…"));
                self.handle_agent_response(&agent_name, event);
                false
            }
            AutochatUiEvent::Completed {
                summary,
                okr_id,
                okr_run_id,
                relay_id,
            } => {
                self.autochat_status = Some("Completed".to_string());

                // Add OKR correlation info to the completion message if present
                let mut full_summary = summary.clone();
                if let (Some(okr_id), Some(okr_run_id)) = (&okr_id, &okr_run_id) {
                    full_summary.push_str(&format!(
                        "\n\n📊 OKR Tracking: okr_id={} run_id={}",
                        &okr_id[..8.min(okr_id.len())],
                        &okr_run_id[..8.min(okr_run_id.len())]
                    ));
                }
                if let Some(rid) = &relay_id {
                    full_summary.push_str(&format!("\n🔗 Relay: {}", rid));
                }

                self.messages
                    .push(ChatMessage::new("assistant", full_summary));
                self.scroll = SCROLL_BOTTOM;
                true
            }
        }
    }

    /// Handle a swarm event
    fn handle_swarm_event(&mut self, event: SwarmEvent) {
        self.swarm_state.handle_event(event.clone());

        // When swarm completes, switch back to chat view with summary
        if let SwarmEvent::Complete { success, ref stats } = event {
            self.view_mode = ViewMode::Chat;
            let summary = if success {
                format!(
                    "Swarm completed successfully.\n\
                     Subtasks: {} completed, {} failed\n\
                     Total tool calls: {}\n\
                     Time: {:.1}s (speedup: {:.1}x)",
                    stats.subagents_completed,
                    stats.subagents_failed,
                    stats.total_tool_calls,
                    stats.execution_time_ms as f64 / 1000.0,
                    stats.speedup_factor
                )
            } else {
                format!(
                    "Swarm completed with failures.\n\
                     Subtasks: {} completed, {} failed\n\
                     Check the subtask results for details.",
                    stats.subagents_completed, stats.subagents_failed
                )
            };
            self.messages.push(ChatMessage::new("system", &summary));
            self.swarm_rx = None;
        }

        if let SwarmEvent::Error(ref err) = event {
            self.messages
                .push(ChatMessage::new("system", &format!("Swarm error: {}", err)));
        }
    }

    /// Handle a Ralph event
    fn handle_ralph_event(&mut self, event: RalphEvent) {
        self.ralph_state.handle_event(event.clone());

        // When Ralph completes, switch back to chat view with summary
        if let RalphEvent::Complete {
            ref status,
            passed,
            total,
        } = event
        {
            self.view_mode = ViewMode::Chat;
            let summary = format!(
                "Ralph loop finished: {}\n\
                 Stories: {}/{} passed",
                status, passed, total
            );
            self.messages.push(ChatMessage::new("system", &summary));
            self.ralph_rx = None;
        }

        if let RalphEvent::Error(ref err) = event {
            self.messages
                .push(ChatMessage::new("system", &format!("Ralph error: {}", err)));
        }
    }

    /// Start Ralph execution for a PRD
    async fn start_ralph_execution(&mut self, prd_path: String, config: &Config) {
        // Add user message
        self.messages
            .push(ChatMessage::new("user", format!("/ralph {}", prd_path)));

        // Get model from config
        let model = self
            .active_model
            .clone()
            .or_else(|| config.default_model.clone())
            .or_else(|| Some("zai/glm-5".to_string()));

        let model = match model {
            Some(m) => m,
            None => {
                self.messages.push(ChatMessage::new(
                    "system",
                    "No model configured. Use /model to select one first.",
                ));
                return;
            }
        };

        // Check PRD exists
        let prd_file = std::path::PathBuf::from(&prd_path);
        if !prd_file.exists() {
            self.messages.push(ChatMessage::new(
                "system",
                format!("PRD file not found: {}", prd_path),
            ));
            return;
        }

        // Create channel for ralph events
        let (tx, rx) = mpsc::channel(200);
        self.ralph_rx = Some(rx);

        // Switch to Ralph view
        self.view_mode = ViewMode::Ralph;
        self.ralph_state = RalphViewState::new();

        // Build Ralph config
        let ralph_config = RalphConfig {
            prd_path: prd_path.clone(),
            max_iterations: 10,
            progress_path: "progress.txt".to_string(),
            quality_checks_enabled: true,
            auto_commit: true,
            model: Some(model.clone()),
            use_rlm: true,
            parallel_enabled: true,
            max_concurrent_stories: 100,
            worktree_enabled: true,
            story_timeout_secs: 300,
            conflict_timeout_secs: 120,
            relay_enabled: true,
            relay_max_agents: 8,
            relay_max_rounds: 3,
            max_steps_per_story: 30,
        };

        // Parse provider/model from the model string
        let (provider_name, model_name) = if let Some(pos) = model.find('/') {
            (model[..pos].to_string(), model[pos + 1..].to_string())
        } else {
            (model.clone(), model.clone())
        };

        let prd_path_clone = prd_path.clone();
        let tx_clone = tx.clone();

        // Spawn Ralph execution
        tokio::spawn(async move {
            // Get provider from registry
            let provider = match crate::provider::ProviderRegistry::from_vault().await {
                Ok(registry) => match registry.get(&provider_name) {
                    Some(p) => p,
                    None => {
                        let _ = tx_clone
                            .send(RalphEvent::Error(format!(
                                "Provider '{}' not found",
                                provider_name
                            )))
                            .await;
                        return;
                    }
                },
                Err(e) => {
                    let _ = tx_clone
                        .send(RalphEvent::Error(format!(
                            "Failed to load providers: {}",
                            e
                        )))
                        .await;
                    return;
                }
            };

            let prd_path_buf = std::path::PathBuf::from(&prd_path_clone);
            match RalphLoop::new(prd_path_buf, provider, model_name, ralph_config).await {
                Ok(ralph) => {
                    let mut ralph = ralph.with_event_tx(tx_clone.clone());
                    match ralph.run().await {
                        Ok(_state) => {
                            // Complete event already emitted by run()
                        }
                        Err(e) => {
                            let _ = tx_clone.send(RalphEvent::Error(e.to_string())).await;
                        }
                    }
                }
                Err(e) => {
                    let _ = tx_clone
                        .send(RalphEvent::Error(format!(
                            "Failed to initialize Ralph: {}",
                            e
                        )))
                        .await;
                }
            }
        });

        self.messages.push(ChatMessage::new(
            "system",
            format!("Starting Ralph loop with PRD: {}", prd_path),
        ));
    }

    /// Start swarm execution for a task
    async fn start_swarm_execution(&mut self, task: String, config: &Config) {
        // Add user message
        self.messages
            .push(ChatMessage::new("user", format!("/swarm {}", task)));

        // Get model from config
        let model = config
            .default_model
            .clone()
            .or_else(|| Some("zai/glm-5".to_string()));

        // Configure swarm
        let swarm_config = SwarmConfig {
            model,
            max_subagents: 10,
            max_steps_per_subagent: 50,
            worktree_enabled: true,
            worktree_auto_merge: true,
            working_dir: Some(
                std::env::current_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|_| ".".to_string()),
            ),
            ..Default::default()
        };

        // Create channel for swarm events
        let (tx, rx) = mpsc::channel(100);
        self.swarm_rx = Some(rx);

        // Switch to swarm view
        self.view_mode = ViewMode::Swarm;
        self.swarm_state = SwarmViewState::new();

        // Send initial event
        let _ = tx
            .send(SwarmEvent::Started {
                task: task.clone(),
                total_subtasks: 0,
            })
            .await;

        // Spawn swarm execution — executor emits all events via event_tx
        let task_clone = task;
        let bus_arc = self.bus.clone();
        tokio::spawn(async move {
            // Create executor with event channel — it handles decomposition + execution
            let mut executor = SwarmExecutor::new(swarm_config).with_event_tx(tx.clone());
            if let Some(bus) = bus_arc {
                executor = executor.with_bus(bus);
            }
            let result = executor
                .execute(&task_clone, DecompositionStrategy::Automatic)
                .await;

            match result {
                Ok(swarm_result) => {
                    let _ = tx
                        .send(SwarmEvent::Complete {
                            success: swarm_result.success,
                            stats: swarm_result.stats,
                        })
                        .await;
                }
                Err(e) => {
                    let _ = tx.send(SwarmEvent::Error(e.to_string())).await;
                }
            }
        });
    }

    /// Populate and open the model picker overlay
    async fn open_model_picker(&mut self, config: &Config) {
        let mut models: Vec<(String, String, String)> = Vec::new();

        // Try to build provider registry and list models
        match crate::provider::ProviderRegistry::from_vault().await {
            Ok(registry) => {
                for provider_name in registry.list() {
                    if let Some(provider) = registry.get(provider_name) {
                        match provider.list_models().await {
                            Ok(model_list) => {
                                for m in model_list {
                                    let label = format!("{}/{}", provider_name, m.id);
                                    let value = format!("{}/{}", provider_name, m.id);
                                    let name = m.name.clone();
                                    models.push((label, value, name));
                                }
                            }
                            Err(e) => {
                                tracing::warn!(
                                    "Failed to list models for {}: {}",
                                    provider_name,
                                    e
                                );
                            }
                        }
                    }
                }
            }
            Err(e) => {
                tracing::warn!("Failed to load provider registry: {}", e);
            }
        }

        // Fallback: also try from config
        if models.is_empty() {
            if let Ok(registry) = crate::provider::ProviderRegistry::from_config(config).await {
                for provider_name in registry.list() {
                    if let Some(provider) = registry.get(provider_name) {
                        if let Ok(model_list) = provider.list_models().await {
                            for m in model_list {
                                let label = format!("{}/{}", provider_name, m.id);
                                let value = format!("{}/{}", provider_name, m.id);
                                let name = m.name.clone();
                                models.push((label, value, name));
                            }
                        }
                    }
                }
            }
        }

        if models.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No models found. Check provider configuration (Vault or config).",
            ));
        } else {
            // Sort models by provider then name
            models.sort_by(|a, b| a.0.cmp(&b.0));
            self.model_picker_list = models;
            self.model_picker_selected = 0;
            self.model_picker_filter.clear();
            self.view_mode = ViewMode::ModelPicker;
        }
    }

    /// Get filtered session list for the session picker
    fn filtered_sessions(&self) -> Vec<(usize, &SessionSummary)> {
        if self.session_picker_filter.is_empty() {
            self.session_picker_list.iter().enumerate().collect()
        } else {
            let filter = self.session_picker_filter.to_lowercase();
            self.session_picker_list
                .iter()
                .enumerate()
                .filter(|(_, s)| {
                    s.title
                        .as_deref()
                        .unwrap_or("")
                        .to_lowercase()
                        .contains(&filter)
                        || s.agent.to_lowercase().contains(&filter)
                        || s.id.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    /// Get filtered model list
    fn filtered_models(&self) -> Vec<(usize, &(String, String, String))> {
        if self.model_picker_filter.is_empty() {
            self.model_picker_list.iter().enumerate().collect()
        } else {
            let filter = self.model_picker_filter.to_lowercase();
            self.model_picker_list
                .iter()
                .enumerate()
                .filter(|(_, (label, _, name))| {
                    label.to_lowercase().contains(&filter) || name.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    /// Get filtered spawned agents list (sorted by name)
    fn filtered_spawned_agents(&self) -> Vec<(String, String, bool, bool)> {
        let mut agents: Vec<(String, String, bool, bool)> = self
            .spawned_agents
            .iter()
            .map(|(name, agent)| {
                let protocol_registered = self.is_agent_protocol_registered(name);
                (
                    name.clone(),
                    agent.instructions.clone(),
                    agent.is_processing,
                    protocol_registered,
                )
            })
            .collect();

        agents.sort_by(|a, b| a.0.to_lowercase().cmp(&b.0.to_lowercase()));

        if self.agent_picker_filter.is_empty() {
            agents
        } else {
            let filter = self.agent_picker_filter.to_lowercase();
            agents
                .into_iter()
                .filter(|(name, instructions, _, _)| {
                    name.to_lowercase().contains(&filter)
                        || instructions.to_lowercase().contains(&filter)
                })
                .collect()
        }
    }

    /// Open picker for choosing a spawned sub-agent to focus
    fn open_agent_picker(&mut self) {
        if self.spawned_agents.is_empty() {
            self.messages.push(ChatMessage::new(
                "system",
                "No agents spawned yet. Use /spawn <name> <instructions> first.",
            ));
            return;
        }

        self.agent_picker_filter.clear();
        let filtered = self.filtered_spawned_agents();
        self.agent_picker_selected = if let Some(active) = &self.active_spawned_agent {
            filtered
                .iter()
                .position(|(name, _, _, _)| name == active)
                .unwrap_or(0)
        } else {
            0
        };
        self.view_mode = ViewMode::AgentPicker;
    }

    fn navigate_history(&mut self, direction: isize) {
        if self.command_history.is_empty() {
            return;
        }

        let history_len = self.command_history.len();
        let new_index = match self.history_index {
            Some(current) => {
                let new = current as isize + direction;
                if new < 0 {
                    None
                } else if new >= history_len as isize {
                    Some(history_len - 1)
                } else {
                    Some(new as usize)
                }
            }
            None => {
                if direction > 0 {
                    Some(0)
                } else {
                    Some(history_len.saturating_sub(1))
                }
            }
        };

        self.history_index = new_index;
        if let Some(index) = new_index {
            self.input = self.command_history[index].clone();
            self.cursor_position = self.input.len();
        } else {
            self.input.clear();
            self.cursor_position = 0;
        }
    }

    fn search_history(&mut self) {
        // Enhanced search: find commands matching current input prefix
        if self.command_history.is_empty() {
            return;
        }

        let search_term = self.input.trim().to_lowercase();

        if search_term.is_empty() {
            // Empty search - show most recent
            if !self.command_history.is_empty() {
                self.input = self.command_history.last().unwrap().clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(self.command_history.len() - 1);
            }
            return;
        }

        // Find the most recent command that starts with the search term
        for (index, cmd) in self.command_history.iter().enumerate().rev() {
            if cmd.to_lowercase().starts_with(&search_term) {
                self.input = cmd.clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(index);
                return;
            }
        }

        // If no prefix match, search for contains
        for (index, cmd) in self.command_history.iter().enumerate().rev() {
            if cmd.to_lowercase().contains(&search_term) {
                self.input = cmd.clone();
                self.cursor_position = self.input.len();
                self.history_index = Some(index);
                return;
            }
        }
    }

    fn autochat_status_label(&self) -> Option<String> {
        if !self.autochat_running {
            return None;
        }

        let elapsed = self
            .autochat_started_at
            .map(|started| {
                let elapsed = started.elapsed();
                if elapsed.as_secs() >= 60 {
                    format!("{}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
                } else {
                    format!("{:.1}s", elapsed.as_secs_f64())
                }
            })
            .unwrap_or_else(|| "0.0s".to_string());

        let phase = self
            .autochat_status
            .as_deref()
            .unwrap_or("Relay is running…")
            .to_string();

        Some(format!(
            "{} Autochat {elapsed} • {phase}",
            current_spinner_frame()
        ))
    }

    fn chat_sync_summary(&self) -> String {
        if self.chat_sync_rx.is_none() && self.chat_sync_status.is_none() {
            if self.secure_environment {
                return "Remote sync: REQUIRED in secure environment (not running)".to_string();
            }
            return "Remote sync: disabled (set CODETETHER_CHAT_SYNC_ENABLED=true)".to_string();
        }

        let status = self
            .chat_sync_status
            .as_deref()
            .unwrap_or("Remote sync active")
            .to_string();
        let last_success = self
            .chat_sync_last_success
            .as_deref()
            .unwrap_or("never")
            .to_string();
        let last_error = self
            .chat_sync_last_error
            .as_deref()
            .unwrap_or("none")
            .to_string();

        format!(
            "Remote sync: {status}\nUploaded batches: {} ({})\nLast success: {last_success}\nLast error: {last_error}",
            self.chat_sync_uploaded_batches,
            format_bytes(self.chat_sync_uploaded_bytes)
        )
    }

    fn handle_chat_sync_event(&mut self, event: ChatSyncUiEvent) {
        match event {
            ChatSyncUiEvent::Status(status) => {
                self.chat_sync_status = Some(status);
            }
            ChatSyncUiEvent::BatchUploaded {
                bytes,
                records,
                object_key,
            } => {
                self.chat_sync_uploaded_bytes = self.chat_sync_uploaded_bytes.saturating_add(bytes);
                self.chat_sync_uploaded_batches = self.chat_sync_uploaded_batches.saturating_add(1);
                let when = chrono::Local::now().format("%H:%M:%S").to_string();
                self.chat_sync_last_success = Some(format!(
                    "{} • {} records • {} • {}",
                    when,
                    records,
                    format_bytes(bytes),
                    object_key
                ));
                self.chat_sync_last_error = None;
                self.chat_sync_status =
                    Some(format!("Synced {} ({})", records, format_bytes(bytes)));
            }
            ChatSyncUiEvent::Error(error) => {
                self.chat_sync_last_error = Some(error.clone());
                self.chat_sync_status = Some("Sync error (will retry)".to_string());
            }
        }
    }

    fn to_archive_record(
        message: &ChatMessage,
        workspace: &str,
        session_id: Option<String>,
    ) -> ChatArchiveRecord {
        let (message_type, tool_name, tool_success, tool_duration_ms) = match &message.message_type
        {
            MessageType::Text(_) => ("text".to_string(), None, None, None),
            MessageType::Image { .. } => ("image".to_string(), None, None, None),
            MessageType::ToolCall { name, .. } => {
                ("tool_call".to_string(), Some(name.clone()), None, None)
            }
            MessageType::ToolResult {
                name,
                success,
                duration_ms,
                ..
            } => (
                "tool_result".to_string(),
                Some(name.clone()),
                Some(*success),
                *duration_ms,
            ),
            MessageType::File { .. } => ("file".to_string(), None, None, None),
            MessageType::Thinking(_) => ("thinking".to_string(), None, None, None),
        };

        ChatArchiveRecord {
            recorded_at: chrono::Utc::now().to_rfc3339(),
            workspace: workspace.to_string(),
            session_id,
            role: message.role.clone(),
            agent_name: message.agent_name.clone(),
            message_type,
            content: message.content.clone(),
            tool_name,
            tool_success,
            tool_duration_ms,
        }
    }

    fn flush_chat_archive(&mut self) {
        let Some(path) = self.chat_archive_path.clone() else {
            self.archived_message_count = self.messages.len();
            return;
        };

        if self.archived_message_count >= self.messages.len() {
            return;
        }

        let workspace = self.workspace_dir.to_string_lossy().to_string();
        let session_id = self.session.as_ref().map(|session| session.id.clone());
        let records: Vec<ChatArchiveRecord> = self.messages[self.archived_message_count..]
            .iter()
            .map(|message| Self::to_archive_record(message, &workspace, session_id.clone()))
            .collect();

        if let Some(parent) = path.parent()
            && let Err(err) = std::fs::create_dir_all(parent)
        {
            tracing::warn!(error = %err, path = %parent.display(), "Failed to create chat archive directory");
            return;
        }

        let mut file = match std::fs::OpenOptions::new()
            .create(true)
            .append(true)
            .open(&path)
        {
            Ok(file) => file,
            Err(err) => {
                tracing::warn!(error = %err, path = %path.display(), "Failed to open chat archive file");
                return;
            }
        };

        for record in records {
            if let Err(err) = serde_json::to_writer(&mut file, &record) {
                tracing::warn!(error = %err, path = %path.display(), "Failed to serialize chat archive record");
                return;
            }
            if let Err(err) = writeln!(&mut file) {
                tracing::warn!(error = %err, path = %path.display(), "Failed to write chat archive newline");
                return;
            }
        }

        self.archived_message_count = self.messages.len();
    }
}

async fn run_app(terminal: &mut Terminal<CrosstermBackend<io::Stdout>>) -> Result<()> {
    let mut app = App::new();
    // Use paginated session loading - default 100, configurable via CODETETHER_SESSION_PICKER_LIMIT
    let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
        .ok()
        .and_then(|v| v.parse().ok())
        .unwrap_or(100);
    if let Ok(sessions) = list_sessions_with_opencode_paged(&app.workspace_dir, limit, 0).await {
        app.update_cached_sessions(sessions);
    }

    // Create agent bus and subscribe the TUI as an observer
    let bus = std::sync::Arc::new(crate::bus::AgentBus::new());
    let mut bus_handle = bus.handle("tui-observer");
    let (bus_tx, bus_rx) = mpsc::channel::<crate::bus::BusEnvelope>(512);
    app.bus_log_rx = Some(bus_rx);
    app.bus = Some(bus.clone());

    // Auto-start S3 sink if MinIO is configured (set MINIO_ENDPOINT to enable)
    crate::bus::s3_sink::spawn_bus_s3_sink(bus.clone());

    // Spawn a forwarder task: bus broadcast → mpsc channel for the TUI event loop
    tokio::spawn(async move {
        loop {
            match bus_handle.recv().await {
                Some(env) => {
                    if bus_tx.send(env).await.is_err() {
                        break; // TUI closed
                    }
                }
                None => break, // bus closed
            }
        }
    });

    // Load configuration and theme
    let mut config = Config::load().await?;
    let mut theme = crate::tui::theme_utils::validate_theme(&config.load_theme());

    let secure_environment = is_secure_environment();
    app.secure_environment = secure_environment;

    match parse_chat_sync_config(secure_environment).await {
        Ok(Some(sync_config)) => {
            if let Some(archive_path) = app.chat_archive_path.clone() {
                let (chat_sync_tx, chat_sync_rx) = mpsc::channel::<ChatSyncUiEvent>(64);
                app.chat_sync_rx = Some(chat_sync_rx);
                app.chat_sync_status = Some("Starting remote archive sync worker…".to_string());
                tokio::spawn(async move {
                    run_chat_sync_worker(chat_sync_tx, archive_path, sync_config).await;
                });
            } else {
                let message = "Remote chat sync is enabled, but local archive path is unavailable.";
                if secure_environment {
                    return Err(anyhow::anyhow!(
                        "{message} Secure environment requires remote chat sync."
                    ));
                }
                app.messages.push(ChatMessage::new("system", message));
            }
        }
        Ok(None) => {}
        Err(err) => {
            if secure_environment {
                return Err(anyhow::anyhow!(
                    "Secure environment requires remote chat sync: {err}"
                ));
            }
            app.messages.push(ChatMessage::new(
                "system",
                format!("Remote chat sync disabled due to configuration error: {err}"),
            ));
        }
    }

    // Track last config modification time for hot-reloading
    let _config_paths = vec![
        std::path::PathBuf::from("./codetether.toml"),
        std::path::PathBuf::from("./.codetether/config.toml"),
    ];

    let _global_config_path = directories::ProjectDirs::from("com", "codetether", "codetether")
        .map(|dirs| dirs.config_dir().join("config.toml"));

    let mut last_check = Instant::now();
    let mut event_stream = EventStream::new();

    // Background session refresh — fires every 5s, sends results via channel
    let (session_tx, mut session_rx) = mpsc::channel::<Vec<crate::session::SessionSummary>>(1);
    {
        let workspace_dir = app.workspace_dir.clone();
        let session_limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                if let Ok(sessions) =
                    list_sessions_with_opencode_paged(&workspace_dir, session_limit, 0).await
                {
                    if session_tx.send(sessions).await.is_err() {
                        break; // TUI closed
                    }
                }
            }
        });
    }

    // Check for an interrupted relay checkpoint and notify the user
    if let Some(checkpoint) = RelayCheckpoint::load().await {
        app.messages.push(ChatMessage::new(
            "system",
            format!(
                "Interrupted relay detected!\nTask: {}\nAgents: {}\nCompleted {} turns, was at round {}, index {}\n\nType /resume to continue the relay from where it left off.",
                truncate_with_ellipsis(&checkpoint.task, 120),
                checkpoint.ordered_agents.join(" → "),
                checkpoint.turns,
                checkpoint.round,
                checkpoint.idx,
            ),
        ));
    }

    loop {
        // --- Periodic background work (non-blocking) ---

        // Receive session list updates from background task
        if let Ok(sessions) = session_rx.try_recv() {
            app.update_cached_sessions(sessions);
        }

        // Check for theme changes if hot-reload is enabled
        if config.ui.hot_reload && last_check.elapsed() > Duration::from_secs(2) {
            if let Ok(new_config) = Config::load().await {
                if new_config.ui.theme != config.ui.theme
                    || new_config.ui.custom_theme != config.ui.custom_theme
                {
                    theme = crate::tui::theme_utils::validate_theme(&new_config.load_theme());
                    config = new_config;
                }
            }
            last_check = Instant::now();
        }

        terminal.draw(|f| ui(f, &mut app, &theme))?;

        // Update max_scroll estimate for scroll key handlers
        // This needs to roughly match what ui() calculates
        let terminal_height = terminal.size()?.height.saturating_sub(6) as usize;
        let estimated_lines = app.messages.len() * 4; // rough estimate
        app.last_max_scroll = estimated_lines.saturating_sub(terminal_height);

        // Drain all pending async responses
        if let Some(mut rx) = app.response_rx.take() {
            while let Ok(response) = rx.try_recv() {
                app.handle_response(response);
            }
            app.response_rx = Some(rx);
        }

        // Drain all pending swarm events
        if let Some(mut rx) = app.swarm_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_swarm_event(event);
            }
            app.swarm_rx = Some(rx);
        }

        // Drain all pending ralph events
        if let Some(mut rx) = app.ralph_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_ralph_event(event);
            }
            app.ralph_rx = Some(rx);
        }

        // Drain all pending bus log events
        if let Some(mut rx) = app.bus_log_rx.take() {
            while let Ok(env) = rx.try_recv() {
                app.bus_log_state.ingest(&env);
            }
            app.bus_log_rx = Some(rx);
        }

        // Drain all pending spawned-agent responses
        {
            let mut i = 0;
            while i < app.agent_response_rxs.len() {
                let mut done = false;
                while let Ok(event) = app.agent_response_rxs[i].1.try_recv() {
                    if matches!(event, SessionEvent::Done) {
                        done = true;
                    }
                    let name = app.agent_response_rxs[i].0.clone();
                    app.handle_agent_response(&name, event);
                }
                if done {
                    app.agent_response_rxs.swap_remove(i);
                } else {
                    i += 1;
                }
            }
        }

        // Drain all pending background autochat events
        if let Some(mut rx) = app.autochat_rx.take() {
            let mut completed = false;
            while let Ok(event) = rx.try_recv() {
                if app.handle_autochat_event(event) {
                    completed = true;
                }
            }

            if completed || rx.is_closed() {
                if !completed && app.autochat_running {
                    app.messages.push(ChatMessage::new(
                        "system",
                        "Autochat relay worker stopped unexpectedly.",
                    ));
                    app.scroll = SCROLL_BOTTOM;
                }
                app.autochat_running = false;
                app.autochat_started_at = None;
                app.autochat_status = None;
                app.autochat_rx = None;
            } else {
                app.autochat_rx = Some(rx);
            }
        }

        // Drain all pending background chat sync events
        if let Some(mut rx) = app.chat_sync_rx.take() {
            while let Ok(event) = rx.try_recv() {
                app.handle_chat_sync_event(event);
            }

            if rx.is_closed() {
                app.chat_sync_status = Some("Remote archive sync worker stopped.".to_string());
                app.chat_sync_rx = None;
                if app.secure_environment {
                    return Err(anyhow::anyhow!(
                        "Remote archive sync worker stopped in secure environment"
                    ));
                }
            } else {
                app.chat_sync_rx = Some(rx);
            }
        }

        // Persist any newly appended chat messages for durable post-hoc analysis.
        app.flush_chat_archive();

        // Wait for terminal events asynchronously (no blocking!)
        let ev = tokio::select! {
            maybe_event = event_stream.next() => {
                match maybe_event {
                    Some(Ok(ev)) => ev,
                    Some(Err(_)) => continue,
                    None => return Ok(()), // stream ended
                }
            }
            // Tick at 50ms to keep rendering responsive during streaming
            _ = tokio::time::sleep(Duration::from_millis(50)) => continue,
        };

        // Handle bracketed paste: insert entire clipboard text at cursor without submitting
        if let Event::Paste(text) = &ev {
            // Ensure cursor is at a valid char boundary before inserting
            let mut pos = app.cursor_position;
            while pos > 0 && !app.input.is_char_boundary(pos) {
                pos -= 1;
            }
            app.cursor_position = pos;

            for c in text.chars() {
                if c == '\n' || c == '\r' {
                    // Replace newlines with spaces to keep paste as single message
                    app.input.insert(app.cursor_position, ' ');
                } else {
                    app.input.insert(app.cursor_position, c);
                }
                app.cursor_position += c.len_utf8();
            }
            continue;
        }

        if let Event::Key(key) = ev {
            // Only handle key press events (not release or repeat-release).
            // Crossterm 0.29+ emits Press, Repeat, and Release events;
            // processing all of them causes double character entry.
            if !matches!(key.kind, KeyEventKind::Press | KeyEventKind::Repeat) {
                continue;
            }

            // Help overlay
            if app.show_help {
                if matches!(key.code, KeyCode::Esc | KeyCode::Char('?')) {
                    app.show_help = false;
                }
                continue;
            }

            // Model picker overlay
            if app.view_mode == ViewMode::ModelPicker {
                match key.code {
                    KeyCode::Esc => {
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        if app.model_picker_selected > 0 {
                            app.model_picker_selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        let filtered = app.filtered_models();
                        if app.model_picker_selected < filtered.len().saturating_sub(1) {
                            app.model_picker_selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        let filtered = app.filtered_models();
                        if let Some((_, (label, value, _name))) =
                            filtered.get(app.model_picker_selected)
                        {
                            let label = label.clone();
                            let value = value.clone();
                            app.active_model = Some(value.clone());
                            if let Some(session) = app.session.as_mut() {
                                session.metadata.model = Some(value.clone());
                            }
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!("Model set to: {}", label),
                            ));
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Backspace => {
                        app.model_picker_filter.pop();
                        app.model_picker_selected = 0;
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        app.model_picker_filter.push(c);
                        app.model_picker_selected = 0;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    _ => {}
                }
                continue;
            }

            // Session picker overlay - handle specially
            if app.view_mode == ViewMode::SessionPicker {
                match key.code {
                    KeyCode::Esc => {
                        if app.session_picker_confirm_delete {
                            app.session_picker_confirm_delete = false;
                        } else {
                            app.session_picker_filter.clear();
                            app.session_picker_offset = 0;
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.session_picker_selected > 0 {
                            app.session_picker_selected -= 1;
                        }
                        app.session_picker_confirm_delete = false;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let filtered_count = app.filtered_sessions().len();
                        if app.session_picker_selected < filtered_count.saturating_sub(1) {
                            app.session_picker_selected += 1;
                        }
                        app.session_picker_confirm_delete = false;
                    }
                    KeyCode::Char('d') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        if app.session_picker_confirm_delete {
                            // Second press: actually delete
                            let filtered = app.filtered_sessions();
                            if let Some((orig_idx, _)) = filtered.get(app.session_picker_selected) {
                                let session_id = app.session_picker_list[*orig_idx].id.clone();
                                let is_active = app
                                    .session
                                    .as_ref()
                                    .map(|s| s.id == session_id)
                                    .unwrap_or(false);
                                if !is_active {
                                    if let Err(e) = Session::delete(&session_id).await {
                                        app.messages.push(ChatMessage::new(
                                            "system",
                                            format!("Failed to delete session: {}", e),
                                        ));
                                    } else {
                                        app.session_picker_list.retain(|s| s.id != session_id);
                                        if app.session_picker_selected
                                            >= app.session_picker_list.len()
                                        {
                                            app.session_picker_selected =
                                                app.session_picker_list.len().saturating_sub(1);
                                        }
                                    }
                                }
                            }
                            app.session_picker_confirm_delete = false;
                        } else {
                            // First press: ask for confirmation
                            let filtered = app.filtered_sessions();
                            if let Some((orig_idx, _)) = filtered.get(app.session_picker_selected) {
                                let is_active = app
                                    .session
                                    .as_ref()
                                    .map(|s| s.id == app.session_picker_list[*orig_idx].id)
                                    .unwrap_or(false);
                                if !is_active {
                                    app.session_picker_confirm_delete = true;
                                }
                            }
                        }
                    }
                    KeyCode::Backspace => {
                        app.session_picker_filter.pop();
                        app.session_picker_selected = 0;
                        app.session_picker_confirm_delete = false;
                    }
                    // Pagination: 'n' = next page, 'p' = previous page
                    KeyCode::Char('n') => {
                        let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                            .ok()
                            .and_then(|v| v.parse().ok())
                            .unwrap_or(100);
                        let new_offset = app.session_picker_offset + limit;
                        app.session_picker_offset = new_offset;
                        match list_sessions_with_opencode_paged(
                            &app.workspace_dir,
                            limit,
                            new_offset,
                        )
                        .await
                        {
                            Ok(sessions) => {
                                app.update_cached_sessions(sessions);
                                app.session_picker_selected = 0;
                            }
                            Err(e) => {
                                app.messages.push(ChatMessage::new(
                                    "system",
                                    format!("Failed to load more sessions: {}", e),
                                ));
                            }
                        }
                    }
                    KeyCode::Char('p') => {
                        if app.session_picker_offset > 0 {
                            let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
                                .ok()
                                .and_then(|v| v.parse().ok())
                                .unwrap_or(100);
                            let new_offset = app.session_picker_offset.saturating_sub(limit);
                            app.session_picker_offset = new_offset;
                            match list_sessions_with_opencode_paged(
                                &app.workspace_dir,
                                limit,
                                new_offset,
                            )
                            .await
                            {
                                Ok(sessions) => {
                                    app.update_cached_sessions(sessions);
                                    app.session_picker_selected = 0;
                                }
                                Err(e) => {
                                    app.messages.push(ChatMessage::new(
                                        "system",
                                        format!("Failed to load previous sessions: {}", e),
                                    ));
                                }
                            }
                        }
                    }
                    KeyCode::Char('/') => {
                        // Focus filter (no-op, just signals we're in filter mode)
                    }
                    KeyCode::Enter => {
                        app.session_picker_confirm_delete = false;
                        let filtered = app.filtered_sessions();
                        let session_id = filtered
                            .get(app.session_picker_selected)
                            .map(|(orig_idx, _)| app.session_picker_list[*orig_idx].id.clone());
                        if let Some(session_id) = session_id {
                            let load_result =
                                if let Some(oc_id) = session_id.strip_prefix("opencode_") {
                                    if let Some(storage) = crate::opencode::OpenCodeStorage::new() {
                                        Session::from_opencode(oc_id, &storage).await
                                    } else {
                                        Err(anyhow::anyhow!("OpenCode storage not available"))
                                    }
                                } else {
                                    Session::load(&session_id).await
                                };
                            match load_result {
                                Ok(session) => {
                                    app.messages.clear();
                                    app.messages.push(ChatMessage::new(
                                        "system",
                                        format!(
                                            "Resumed session: {}\nCreated: {}\n{} messages loaded",
                                            session.title.as_deref().unwrap_or("(untitled)"),
                                            session.created_at.format("%Y-%m-%d %H:%M"),
                                            session.messages.len()
                                        ),
                                    ));

                                    for msg in &session.messages {
                                        let role_str = match msg.role {
                                            Role::System => "system",
                                            Role::User => "user",
                                            Role::Assistant => "assistant",
                                            Role::Tool => "tool",
                                        };

                                        // Process each content part separately
                                        // (consistent with /resume command)
                                        for part in &msg.content {
                                            match part {
                                                ContentPart::Text { text } => {
                                                    if !text.is_empty() {
                                                        app.messages.push(ChatMessage::new(
                                                            role_str,
                                                            text.clone(),
                                                        ));
                                                    }
                                                }
                                                ContentPart::Image { url, mime_type } => {
                                                    app.messages.push(
                                                        ChatMessage::new(role_str, "")
                                                            .with_message_type(
                                                                MessageType::Image {
                                                                    url: url.clone(),
                                                                    mime_type: mime_type.clone(),
                                                                },
                                                            ),
                                                    );
                                                }
                                                ContentPart::ToolCall {
                                                    name, arguments, ..
                                                } => {
                                                    let (preview, truncated) =
                                                        build_tool_arguments_preview(
                                                            name,
                                                            arguments,
                                                            TOOL_ARGS_PREVIEW_MAX_LINES,
                                                            TOOL_ARGS_PREVIEW_MAX_BYTES,
                                                        );
                                                    app.messages.push(
                                                        ChatMessage::new(
                                                            role_str,
                                                            format!("🔧 {name}"),
                                                        )
                                                        .with_message_type(MessageType::ToolCall {
                                                            name: name.clone(),
                                                            arguments_preview: preview,
                                                            arguments_len: arguments.len(),
                                                            truncated,
                                                        }),
                                                    );
                                                }
                                                ContentPart::ToolResult { content, .. } => {
                                                    let truncated =
                                                        truncate_with_ellipsis(content, 500);
                                                    let (preview, preview_truncated) =
                                                        build_text_preview(
                                                            content,
                                                            TOOL_OUTPUT_PREVIEW_MAX_LINES,
                                                            TOOL_OUTPUT_PREVIEW_MAX_BYTES,
                                                        );
                                                    app.messages.push(
                                                        ChatMessage::new(
                                                            role_str,
                                                            format!("✅ Result\n{truncated}"),
                                                        )
                                                        .with_message_type(
                                                            MessageType::ToolResult {
                                                                name: "tool".to_string(),
                                                                output_preview: preview,
                                                                output_len: content.len(),
                                                                truncated: preview_truncated,
                                                                success: true,
                                                                duration_ms: None,
                                                            },
                                                        ),
                                                    );
                                                }
                                                ContentPart::File { path, mime_type } => {
                                                    app.messages.push(
                                                        ChatMessage::new(
                                                            role_str,
                                                            format!("📎 {path}"),
                                                        )
                                                        .with_message_type(MessageType::File {
                                                            path: path.clone(),
                                                            mime_type: mime_type.clone(),
                                                        }),
                                                    );
                                                }
                                                ContentPart::Thinking { text } => {
                                                    if !text.is_empty() {
                                                        app.messages.push(
                                                            ChatMessage::new(
                                                                role_str,
                                                                text.clone(),
                                                            )
                                                            .with_message_type(
                                                                MessageType::Thinking(text.clone()),
                                                            ),
                                                        );
                                                    }
                                                }
                                            }
                                        }
                                    }

                                    app.current_agent = session.agent.clone();
                                    app.session = Some(session);
                                    app.scroll = SCROLL_BOTTOM;
                                    app.view_mode = ViewMode::Chat;
                                }
                                Err(e) => {
                                    app.messages.push(ChatMessage::new(
                                        "system",
                                        format!("Failed to load session: {}", e),
                                    ));
                                    app.view_mode = ViewMode::Chat;
                                }
                            }
                        }
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && c != 'j'
                            && c != 'k' =>
                    {
                        app.session_picker_filter.push(c);
                        app.session_picker_selected = 0;
                        app.session_picker_confirm_delete = false;
                    }
                    _ => {}
                }
                continue;
            }

            // Agent picker overlay
            if app.view_mode == ViewMode::AgentPicker {
                match key.code {
                    KeyCode::Esc => {
                        app.agent_picker_filter.clear();
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        if app.agent_picker_selected > 0 {
                            app.agent_picker_selected -= 1;
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j')
                        if !key.modifiers.contains(KeyModifiers::ALT) =>
                    {
                        let filtered = app.filtered_spawned_agents();
                        if app.agent_picker_selected < filtered.len().saturating_sub(1) {
                            app.agent_picker_selected += 1;
                        }
                    }
                    KeyCode::Enter => {
                        let filtered = app.filtered_spawned_agents();
                        if let Some((name, _, _, _)) = filtered.get(app.agent_picker_selected) {
                            app.active_spawned_agent = Some(name.clone());
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!(
                                    "Focused chat on @{name}. Type messages directly; use /agent main to exit focus."
                                ),
                            ));
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Backspace => {
                        app.agent_picker_filter.pop();
                        app.agent_picker_selected = 0;
                    }
                    KeyCode::Char('m') if !key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.active_spawned_agent = None;
                        app.messages
                            .push(ChatMessage::new("system", "Returned to main chat mode."));
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char(c)
                        if !key.modifiers.contains(KeyModifiers::CONTROL)
                            && !key.modifiers.contains(KeyModifiers::ALT)
                            && c != 'j'
                            && c != 'k'
                            && c != 'm' =>
                    {
                        app.agent_picker_filter.push(c);
                        app.agent_picker_selected = 0;
                    }
                    _ => {}
                }
                continue;
            }

            // Swarm view key handling
            if app.view_mode == ViewMode::Swarm {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if app.swarm_state.detail_mode {
                            app.swarm_state.exit_detail();
                        } else {
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.swarm_state.detail_mode {
                            // In detail mode, Up/Down switch between agents
                            app.swarm_state.exit_detail();
                            app.swarm_state.select_prev();
                            app.swarm_state.enter_detail();
                        } else {
                            app.swarm_state.select_prev();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.swarm_state.detail_mode {
                            app.swarm_state.exit_detail();
                            app.swarm_state.select_next();
                            app.swarm_state.enter_detail();
                        } else {
                            app.swarm_state.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if !app.swarm_state.detail_mode {
                            app.swarm_state.enter_detail();
                        }
                    }
                    KeyCode::PageDown => {
                        app.swarm_state.detail_scroll_down(10);
                    }
                    KeyCode::PageUp => {
                        app.swarm_state.detail_scroll_up(10);
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    KeyCode::F(2) => {
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        app.view_mode = ViewMode::Chat;
                    }
                    _ => {}
                }
                continue;
            }

            // Ralph view key handling
            if app.view_mode == ViewMode::Ralph {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if app.ralph_state.detail_mode {
                            app.ralph_state.exit_detail();
                        } else {
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.ralph_state.detail_mode {
                            app.ralph_state.exit_detail();
                            app.ralph_state.select_prev();
                            app.ralph_state.enter_detail();
                        } else {
                            app.ralph_state.select_prev();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.ralph_state.detail_mode {
                            app.ralph_state.exit_detail();
                            app.ralph_state.select_next();
                            app.ralph_state.enter_detail();
                        } else {
                            app.ralph_state.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if !app.ralph_state.detail_mode {
                            app.ralph_state.enter_detail();
                        }
                    }
                    KeyCode::PageDown => {
                        app.ralph_state.detail_scroll_down(10);
                    }
                    KeyCode::PageUp => {
                        app.ralph_state.detail_scroll_up(10);
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    KeyCode::F(2) | KeyCode::Char('s')
                        if key.modifiers.contains(KeyModifiers::CONTROL) =>
                    {
                        app.view_mode = ViewMode::Chat;
                    }
                    _ => {}
                }
                continue;
            }

            // Bus log view key handling
            if app.view_mode == ViewMode::BusLog {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        if app.bus_log_state.detail_mode {
                            app.bus_log_state.exit_detail();
                        } else {
                            app.view_mode = ViewMode::Chat;
                        }
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.bus_log_state.detail_mode {
                            app.bus_log_state.exit_detail();
                            app.bus_log_state.select_prev();
                            app.bus_log_state.enter_detail();
                        } else {
                            app.bus_log_state.select_prev();
                        }
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        if app.bus_log_state.detail_mode {
                            app.bus_log_state.exit_detail();
                            app.bus_log_state.select_next();
                            app.bus_log_state.enter_detail();
                        } else {
                            app.bus_log_state.select_next();
                        }
                    }
                    KeyCode::Enter => {
                        if !app.bus_log_state.detail_mode {
                            app.bus_log_state.enter_detail();
                        }
                    }
                    KeyCode::PageDown => {
                        app.bus_log_state.detail_scroll_down(10);
                    }
                    KeyCode::PageUp => {
                        app.bus_log_state.detail_scroll_up(10);
                    }
                    // Clear all entries
                    KeyCode::Char('c') => {
                        app.bus_log_state.entries.clear();
                        app.bus_log_state.selected_index = 0;
                    }
                    // Jump to bottom (re-enable auto-scroll)
                    KeyCode::Char('g') => {
                        let len = app.bus_log_state.filtered_entries().len();
                        if len > 0 {
                            app.bus_log_state.selected_index = len - 1;
                            app.bus_log_state.list_state.select(Some(len - 1));
                        }
                        app.bus_log_state.auto_scroll = true;
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    _ => {}
                }
                continue;
            }

            // Protocol registry view key handling
            if app.view_mode == ViewMode::Protocol {
                match key.code {
                    KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                        return Ok(());
                    }
                    KeyCode::Esc => {
                        app.view_mode = ViewMode::Chat;
                    }
                    KeyCode::Up | KeyCode::Char('k') => {
                        if app.protocol_selected > 0 {
                            app.protocol_selected -= 1;
                        }
                        app.protocol_scroll = 0;
                    }
                    KeyCode::Down | KeyCode::Char('j') => {
                        let len = app.protocol_cards().len();
                        if app.protocol_selected < len.saturating_sub(1) {
                            app.protocol_selected += 1;
                        }
                        app.protocol_scroll = 0;
                    }
                    KeyCode::PageDown => {
                        app.protocol_scroll = app.protocol_scroll.saturating_add(10);
                    }
                    KeyCode::PageUp => {
                        app.protocol_scroll = app.protocol_scroll.saturating_sub(10);
                    }
                    KeyCode::Char('g') => {
                        app.protocol_scroll = 0;
                    }
                    KeyCode::Char('?') => {
                        app.show_help = true;
                    }
                    _ => {}
                }
                continue;
            }

            match key.code {
                // Quit
                KeyCode::Char('c') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }
                KeyCode::Char('q') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    return Ok(());
                }

                // Help
                KeyCode::Char('?') => {
                    app.show_help = true;
                }

                // OKR approval gate: 'a' to approve, 'd' to deny
                KeyCode::Char('a')
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && app.pending_okr_approval.is_some() =>
                {
                    if let Some(pending) = app.pending_okr_approval.take() {
                        // Approve: save OKR and run, then execute via Ralph PRD loop
                        app.messages.push(ChatMessage::new(
                            "system",
                            "✅ OKR approved! Starting Ralph PRD execution...",
                        ));
                        app.scroll = SCROLL_BOTTOM;

                        let task = pending.task.clone();
                        let agent_count = pending.agent_count;
                        let config = config.clone();
                        let okr = pending.okr;
                        let mut run = pending.run;

                        // Resolve model for Ralph execution
                        let model = app
                            .active_model
                            .clone()
                            .or_else(|| config.default_model.clone())
                            .unwrap_or_else(|| GO_SWAP_MODEL_MINIMAX.to_string());

                        let bus = app.bus.clone();

                        // Save OKR to repository
                        let okr_id = okr.id;
                        let okr_run_id = run.id;
                        run.record_decision(crate::okr::ApprovalDecision::approve(
                            run.id,
                            "User approved via TUI go command",
                        ));
                        run.correlation_id = Some(format!("ralph-{}", Uuid::new_v4()));

                        let okr_for_save = okr.clone();
                        let run_for_save = run.clone();
                        tokio::spawn(async move {
                            if let Ok(repo) = OkrRepository::from_config().await {
                                let _ = repo.create_okr(okr_for_save).await;
                                let _ = repo.create_run(run_for_save).await;
                                tracing::info!(okr_id = %okr_id, okr_run_id = %okr_run_id, "OKR run approved and saved");
                            }
                        });

                        // Reuse autochat UI channel for Ralph progress reporting
                        let (tx, rx) = mpsc::channel(512);
                        app.autochat_rx = Some(rx);
                        app.autochat_running = true;
                        app.autochat_started_at = Some(Instant::now());
                        app.autochat_status = Some("Generating PRD from task…".to_string());

                        tokio::spawn(async move {
                            run_go_ralph_worker(tx, okr, run, task, model, bus, agent_count).await;
                        });

                        continue;
                    }
                }

                KeyCode::Char('d')
                    if !key.modifiers.contains(KeyModifiers::CONTROL)
                        && app.pending_okr_approval.is_some() =>
                {
                    if let Some(mut pending) = app.pending_okr_approval.take() {
                        // Deny: record decision and show denial message
                        pending
                            .run
                            .record_decision(crate::okr::ApprovalDecision::deny(
                                pending.run.id,
                                "User denied via TUI keypress",
                            ));
                        app.messages.push(ChatMessage::new(
                            "system",
                            "❌ OKR denied. Relay not started.\n\nUse /autochat for tactical execution without OKR tracking.",
                        ));
                        app.scroll = SCROLL_BOTTOM;
                        continue;
                    }
                }

                // Toggle view mode (F2 or Ctrl+S)
                KeyCode::F(2) => {
                    app.view_mode = match app.view_mode {
                        ViewMode::Chat
                        | ViewMode::SessionPicker
                        | ViewMode::ModelPicker
                        | ViewMode::AgentPicker
                        | ViewMode::Protocol
                        | ViewMode::BusLog => ViewMode::Swarm,
                        ViewMode::Swarm | ViewMode::Ralph => ViewMode::Chat,
                    };
                }
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.view_mode = match app.view_mode {
                        ViewMode::Chat
                        | ViewMode::SessionPicker
                        | ViewMode::ModelPicker
                        | ViewMode::AgentPicker
                        | ViewMode::Protocol
                        | ViewMode::BusLog => ViewMode::Swarm,
                        ViewMode::Swarm | ViewMode::Ralph => ViewMode::Chat,
                    };
                }

                // Toggle inspector pane in webview layout
                KeyCode::F(3) => {
                    app.show_inspector = !app.show_inspector;
                }

                // Copy latest assistant message to clipboard (Ctrl+Y)
                KeyCode::Char('y') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    let msg = app
                        .messages
                        .iter()
                        .rev()
                        .find(|m| m.role == "assistant" && !m.content.trim().is_empty())
                        .or_else(|| {
                            app.messages
                                .iter()
                                .rev()
                                .find(|m| !m.content.trim().is_empty())
                        });

                    let Some(msg) = msg else {
                        app.messages
                            .push(ChatMessage::new("system", "Nothing to copy yet."));
                        app.scroll = SCROLL_BOTTOM;
                        continue;
                    };

                    let text = message_clipboard_text(msg);
                    match copy_text_to_clipboard_best_effort(&text) {
                        Ok(method) => {
                            app.messages.push(ChatMessage::new(
                                "system",
                                format!("Copied latest reply ({method})."),
                            ));
                            app.scroll = SCROLL_BOTTOM;
                        }
                        Err(err) => {
                            tracing::warn!(error = %err, "Copy to clipboard failed");
                            app.messages.push(ChatMessage::new(
                                "system",
                                "Could not copy to clipboard in this environment.",
                            ));
                            app.scroll = SCROLL_BOTTOM;
                        }
                    }
                }

                // Toggle chat layout (Ctrl+B)
                KeyCode::Char('b') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.chat_layout = match app.chat_layout {
                        ChatLayoutMode::Classic => ChatLayoutMode::Webview,
                        ChatLayoutMode::Webview => ChatLayoutMode::Classic,
                    };
                }

                // Escape - return to chat from swarm/picker view
                KeyCode::Esc => {
                    if app.view_mode == ViewMode::Swarm
                        || app.view_mode == ViewMode::Ralph
                        || app.view_mode == ViewMode::BusLog
                        || app.view_mode == ViewMode::Protocol
                        || app.view_mode == ViewMode::SessionPicker
                        || app.view_mode == ViewMode::ModelPicker
                        || app.view_mode == ViewMode::AgentPicker
                    {
                        app.view_mode = ViewMode::Chat;
                    }
                }

                // Model picker (Ctrl+M)
                KeyCode::Char('m') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_model_picker(&config).await;
                }

                // Agent picker (Ctrl+A)
                KeyCode::Char('a') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_agent_picker();
                }

                // Bus protocol log (Ctrl+L)
                KeyCode::Char('l') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.view_mode = ViewMode::BusLog;
                }

                // Protocol registry view (Ctrl+P)
                KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.open_protocol_view();
                }

                // Switch agent
                KeyCode::Tab => {
                    app.current_agent = if app.current_agent == "build" {
                        "plan".to_string()
                    } else {
                        "build".to_string()
                    };
                }

                // Submit message
                KeyCode::Enter => {
                    app.submit_message(&config).await;
                }

                // Vim-style scrolling (Alt + j/k)
                KeyCode::Char('j') if key.modifiers.contains(KeyModifiers::ALT) => {
                    if app.scroll < SCROLL_BOTTOM {
                        app.scroll = app.scroll.saturating_add(1);
                    }
                }
                KeyCode::Char('k') if key.modifiers.contains(KeyModifiers::ALT) => {
                    if app.scroll >= SCROLL_BOTTOM {
                        app.scroll = app.last_max_scroll; // Leave auto-scroll mode
                    }
                    app.scroll = app.scroll.saturating_sub(1);
                }

                // Command history
                KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.search_history();
                }
                KeyCode::Up if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.navigate_history(-1);
                }
                KeyCode::Down if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.navigate_history(1);
                }

                // Additional Vim-style navigation (with modifiers to avoid conflicts)
                KeyCode::Char('g') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.scroll = 0; // Go to top
                }
                KeyCode::Char('G') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    // Go to bottom (auto-scroll)
                    app.scroll = SCROLL_BOTTOM;
                }

                // Enhanced scrolling (with Alt to avoid conflicts)
                KeyCode::Char('d') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Half page down
                    if app.scroll < SCROLL_BOTTOM {
                        app.scroll = app.scroll.saturating_add(5);
                    }
                }
                KeyCode::Char('u') if key.modifiers.contains(KeyModifiers::ALT) => {
                    // Half page up
                    if app.scroll >= SCROLL_BOTTOM {
                        app.scroll = app.last_max_scroll;
                    }
                    app.scroll = app.scroll.saturating_sub(5);
                }

                // Text input
                KeyCode::Char(c) => {
                    // Ensure cursor is at a valid char boundary
                    while app.cursor_position > 0
                        && !app.input.is_char_boundary(app.cursor_position)
                    {
                        app.cursor_position -= 1;
                    }
                    app.input.insert(app.cursor_position, c);
                    app.cursor_position += c.len_utf8();
                }
                KeyCode::Backspace => {
                    // Move back to previous char boundary
                    while app.cursor_position > 0
                        && !app.input.is_char_boundary(app.cursor_position)
                    {
                        app.cursor_position -= 1;
                    }
                    if app.cursor_position > 0 {
                        // Find start of previous char
                        let prev = app.input[..app.cursor_position].char_indices().rev().next();
                        if let Some((idx, ch)) = prev {
                            app.input.replace_range(idx..idx + ch.len_utf8(), "");
                            app.cursor_position = idx;
                        }
                    }
                }
                KeyCode::Delete => {
                    // Ensure cursor is at a valid char boundary
                    while app.cursor_position > 0
                        && !app.input.is_char_boundary(app.cursor_position)
                    {
                        app.cursor_position -= 1;
                    }
                    if app.cursor_position < app.input.len() {
                        let ch = app.input[app.cursor_position..].chars().next();
                        if let Some(ch) = ch {
                            app.input.replace_range(
                                app.cursor_position..app.cursor_position + ch.len_utf8(),
                                "",
                            );
                        }
                    }
                }
                KeyCode::Left => {
                    // Move left by one character (not byte)
                    let prev = app.input[..app.cursor_position].char_indices().rev().next();
                    if let Some((idx, _)) = prev {
                        app.cursor_position = idx;
                    }
                }
                KeyCode::Right => {
                    if app.cursor_position < app.input.len() {
                        let ch = app.input[app.cursor_position..].chars().next();
                        if let Some(ch) = ch {
                            app.cursor_position += ch.len_utf8();
                        }
                    }
                }
                KeyCode::Home => {
                    app.cursor_position = 0;
                }
                KeyCode::End => {
                    app.cursor_position = app.input.len();
                }

                // Scroll (normalize first to handle SCROLL_BOTTOM sentinel)
                KeyCode::Up => {
                    if app.scroll >= SCROLL_BOTTOM {
                        app.scroll = app.last_max_scroll; // Leave auto-scroll mode
                    }
                    app.scroll = app.scroll.saturating_sub(1);
                }
                KeyCode::Down => {
                    if app.scroll < SCROLL_BOTTOM {
                        app.scroll = app.scroll.saturating_add(1);
                    }
                }
                KeyCode::PageUp => {
                    if app.scroll >= SCROLL_BOTTOM {
                        app.scroll = app.last_max_scroll;
                    }
                    app.scroll = app.scroll.saturating_sub(10);
                }
                KeyCode::PageDown => {
                    if app.scroll < SCROLL_BOTTOM {
                        app.scroll = app.scroll.saturating_add(10);
                    }
                }

                _ => {}
            }
        }
    }
}

fn ui(f: &mut Frame, app: &mut App, theme: &Theme) {
    // Check view mode
    if app.view_mode == ViewMode::Swarm {
        // Render swarm view
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Swarm view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Swarm view
        render_swarm_view(f, &mut app.swarm_state, chunks[0]);

        // Input area (for returning to chat)
        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc, Ctrl+S, or /view to return to chat ")
            .border_style(Style::default().fg(Color::Cyan));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        // Status bar
        let status_line = if app.swarm_state.detail_mode {
            Line::from(vec![
                Span::styled(
                    " AGENT DETAIL ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back to list | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Prev/Next agent | "),
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
                Span::raw(": Scroll"),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    " SWARM MODE ",
                    Style::default().fg(Color::Black).bg(Color::Cyan),
                ),
                Span::raw(" | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Select | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": Detail | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back | "),
                Span::styled("Ctrl+S", Style::default().fg(Color::Yellow)),
                Span::raw(": Toggle view"),
            ])
        };
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Ralph view
    if app.view_mode == ViewMode::Ralph {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Ralph view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        render_ralph_view(f, &mut app.ralph_state, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc to return to chat ")
            .border_style(Style::default().fg(Color::Magenta));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        let status_line = if app.ralph_state.detail_mode {
            Line::from(vec![
                Span::styled(
                    " STORY DETAIL ",
                    Style::default().fg(Color::Black).bg(Color::Magenta),
                ),
                Span::raw(" | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back to list | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Prev/Next story | "),
                Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
                Span::raw(": Scroll"),
            ])
        } else {
            Line::from(vec![
                Span::styled(
                    " RALPH MODE ",
                    Style::default().fg(Color::Black).bg(Color::Magenta),
                ),
                Span::raw(" | "),
                Span::styled("↑↓", Style::default().fg(Color::Yellow)),
                Span::raw(": Select | "),
                Span::styled("Enter", Style::default().fg(Color::Yellow)),
                Span::raw(": Detail | "),
                Span::styled("Esc", Style::default().fg(Color::Yellow)),
                Span::raw(": Back"),
            ])
        };
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Bus protocol log view
    if app.view_mode == ViewMode::BusLog {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Bus log view
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        render_bus_log(f, &mut app.bus_log_state, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc to return to chat ")
            .border_style(Style::default().fg(Color::Green));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        let count_info = format!(
            " {}/{} ",
            app.bus_log_state.visible_count(),
            app.bus_log_state.total_count()
        );
        let status_line = Line::from(vec![
            Span::styled(
                " BUS LOG ",
                Style::default().fg(Color::Black).bg(Color::Green),
            ),
            Span::raw(&count_info),
            Span::raw("| "),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Select | "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": Detail | "),
            Span::styled("c", Style::default().fg(Color::Yellow)),
            Span::raw(": Clear | "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Back"),
        ]);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Protocol registry view
    if app.view_mode == ViewMode::Protocol {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Protocol details
                Constraint::Length(3), // Input
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        render_protocol_registry(f, app, theme, chunks[0]);

        let input_block = Block::default()
            .borders(Borders::ALL)
            .title(" Press Esc to return to chat ")
            .border_style(Style::default().fg(Color::Blue));

        let input = Paragraph::new(app.input.as_str())
            .block(input_block)
            .wrap(Wrap { trim: false });
        f.render_widget(input, chunks[1]);

        let cards = app.protocol_cards();
        let status_line = Line::from(vec![
            Span::styled(
                " PROTOCOL REGISTRY ",
                Style::default().fg(Color::Black).bg(Color::Blue),
            ),
            Span::raw(format!(" {} cards | ", cards.len())),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Select | "),
            Span::styled("PgUp/PgDn", Style::default().fg(Color::Yellow)),
            Span::raw(": Scroll detail | "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Back"),
        ]);
        let status = Paragraph::new(status_line);
        f.render_widget(status, chunks[2]);
        return;
    }

    // Model picker view
    if app.view_mode == ViewMode::ModelPicker {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);

        let filter_display = if app.model_picker_filter.is_empty() {
            "type to filter".to_string()
        } else {
            format!("filter: {}", app.model_picker_filter)
        };

        let picker_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Select Model (↑↓ navigate, Enter select, Esc cancel) [{}] ",
                filter_display
            ))
            .border_style(Style::default().fg(Color::Magenta));

        let filtered = app.filtered_models();
        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        if let Some(ref active) = app.active_model {
            list_lines.push(Line::styled(
                format!("  Current: {}", active),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::DIM),
            ));
            list_lines.push(Line::from(""));
        }

        if filtered.is_empty() {
            list_lines.push(Line::styled(
                "  No models match filter",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            let mut current_provider = String::new();
            for (display_idx, (_, (label, _, human_name))) in filtered.iter().enumerate() {
                let provider = label.split('/').next().unwrap_or("");
                if provider != current_provider {
                    if !current_provider.is_empty() {
                        list_lines.push(Line::from(""));
                    }
                    list_lines.push(Line::styled(
                        format!("  ─── {} ───", provider),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ));
                    current_provider = provider.to_string();
                }

                let is_selected = display_idx == app.model_picker_selected;
                let is_active = app.active_model.as_deref() == Some(label.as_str());
                let marker = if is_selected { "▶" } else { " " };
                let active_marker = if is_active { " ✓" } else { "" };
                let model_id = label.split('/').skip(1).collect::<Vec<_>>().join("/");
                // Show human name if different from ID
                let display = if human_name != &model_id && !human_name.is_empty() {
                    format!("{} ({})", human_name, model_id)
                } else {
                    model_id
                };

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                } else if is_active {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                list_lines.push(Line::styled(
                    format!("  {} {}{}", marker, display, active_marker),
                    style,
                ));
            }
        }

        let list = Paragraph::new(list_lines)
            .block(picker_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, area);
        return;
    }

    // Session picker view
    if app.view_mode == ViewMode::SessionPicker {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Min(1),    // Session list
                Constraint::Length(1), // Status bar
            ])
            .split(f.area());

        // Build title with filter display
        let filter_display = if app.session_picker_filter.is_empty() {
            String::new()
        } else {
            format!(" [filter: {}]", app.session_picker_filter)
        };

        let list_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Sessions (↑↓ navigate, Enter load, d delete, Esc cancel){} ",
                filter_display
            ))
            .border_style(Style::default().fg(Color::Cyan));

        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        let filtered = app.filtered_sessions();
        if filtered.is_empty() {
            if app.session_picker_filter.is_empty() {
                list_lines.push(Line::styled(
                    "  No sessions found.",
                    Style::default().fg(Color::DarkGray),
                ));
            } else {
                list_lines.push(Line::styled(
                    format!("  No sessions matching '{}'", app.session_picker_filter),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }

        for (display_idx, (_orig_idx, session)) in filtered.iter().enumerate() {
            let is_selected = display_idx == app.session_picker_selected;
            let is_active = app
                .session
                .as_ref()
                .map(|s| s.id == session.id)
                .unwrap_or(false);
            let title = session.title.as_deref().unwrap_or("(untitled)");
            let date = session.updated_at.format("%Y-%m-%d %H:%M");
            let active_marker = if is_active { " ●" } else { "" };
            let line_str = format!(
                " {} {}{} - {} ({} msgs)",
                if is_selected { "▶" } else { " " },
                title,
                active_marker,
                date,
                session.message_count
            );

            let style = if is_selected && app.session_picker_confirm_delete {
                Style::default().fg(Color::Red).add_modifier(Modifier::BOLD)
            } else if is_selected {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else if is_active {
                Style::default().fg(Color::Green)
            } else {
                Style::default()
            };

            list_lines.push(Line::styled(line_str, style));

            // Show details for selected item
            if is_selected {
                if app.session_picker_confirm_delete {
                    list_lines.push(Line::styled(
                        "   ⚠ Press d again to confirm delete, Esc to cancel",
                        Style::default().fg(Color::Red).add_modifier(Modifier::BOLD),
                    ));
                } else {
                    list_lines.push(Line::styled(
                        format!("   Agent: {} | ID: {}", session.agent, session.id),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
        }

        let list = Paragraph::new(list_lines)
            .block(list_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, chunks[0]);

        // Status bar with more actions
        let mut status_spans = vec![
            Span::styled(
                " SESSION PICKER ",
                Style::default().fg(Color::Black).bg(Color::Cyan),
            ),
            Span::raw(" "),
            Span::styled("↑↓", Style::default().fg(Color::Yellow)),
            Span::raw(": Nav "),
            Span::styled("Enter", Style::default().fg(Color::Yellow)),
            Span::raw(": Load "),
            Span::styled("d", Style::default().fg(Color::Yellow)),
            Span::raw(": Delete "),
            Span::styled("Esc", Style::default().fg(Color::Yellow)),
            Span::raw(": Cancel "),
        ];
        if !app.session_picker_filter.is_empty() || !app.session_picker_list.is_empty() {
            status_spans.push(Span::styled("Type", Style::default().fg(Color::Yellow)));
            status_spans.push(Span::raw(": Filter "));
        }
        let limit = std::env::var("CODETETHER_SESSION_PICKER_LIMIT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100);
        // Pagination info
        if app.session_picker_offset > 0 || app.session_picker_list.len() >= limit {
            status_spans.push(Span::styled("n", Style::default().fg(Color::Yellow)));
            status_spans.push(Span::raw(": Next "));
            if app.session_picker_offset > 0 {
                status_spans.push(Span::styled("p", Style::default().fg(Color::Yellow)));
                status_spans.push(Span::raw(": Prev "));
            }
        }
        let total = app.session_picker_list.len();
        let showing = filtered.len();
        let offset_display = if app.session_picker_offset > 0 {
            format!("+{}", app.session_picker_offset)
        } else {
            String::new()
        };
        if showing < total {
            status_spans.push(Span::styled(
                format!("{}{}/{}", offset_display, showing, total),
                Style::default().fg(Color::DarkGray),
            ));
        }

        let status = Paragraph::new(Line::from(status_spans));
        f.render_widget(status, chunks[1]);
        return;
    }

    // Agent picker view
    if app.view_mode == ViewMode::AgentPicker {
        let area = centered_rect(70, 70, f.area());
        f.render_widget(Clear, area);

        let filter_display = if app.agent_picker_filter.is_empty() {
            "type to filter".to_string()
        } else {
            format!("filter: {}", app.agent_picker_filter)
        };

        let picker_block = Block::default()
            .borders(Borders::ALL)
            .title(format!(
                " Select Agent (↑↓ navigate, Enter focus, m main chat, Esc cancel) [{}] ",
                filter_display
            ))
            .border_style(Style::default().fg(Color::Magenta));

        let filtered = app.filtered_spawned_agents();
        let mut list_lines: Vec<Line> = Vec::new();
        list_lines.push(Line::from(""));

        if let Some(ref active) = app.active_spawned_agent {
            list_lines.push(Line::styled(
                format!("  Current focus: @{}", active),
                Style::default()
                    .fg(Color::Green)
                    .add_modifier(Modifier::DIM),
            ));
            list_lines.push(Line::from(""));
        }

        if filtered.is_empty() {
            list_lines.push(Line::styled(
                "  No spawned agents match filter",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            for (display_idx, (name, instructions, is_processing, is_registered)) in
                filtered.iter().enumerate()
            {
                let is_selected = display_idx == app.agent_picker_selected;
                let is_focused = app.active_spawned_agent.as_deref() == Some(name.as_str());
                let marker = if is_selected { "▶" } else { " " };
                let focused_marker = if is_focused { " ✓" } else { "" };
                let status = if *is_processing { "⚡" } else { "●" };
                let protocol = if *is_registered { "🔗" } else { "⚠" };
                let avatar = agent_avatar(name);

                let style = if is_selected {
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                } else if is_focused {
                    Style::default().fg(Color::Green)
                } else {
                    Style::default()
                };

                list_lines.push(Line::styled(
                    format!("  {marker} {status} {protocol} {avatar} @{name}{focused_marker}"),
                    style,
                ));

                if is_selected {
                    let profile = agent_profile(name);
                    list_lines.push(Line::styled(
                        format!("     profile: {} — {}", profile.codename, profile.profile),
                        Style::default().fg(Color::Cyan),
                    ));
                    list_lines.push(Line::styled(
                        format!("     {}", instructions),
                        Style::default().fg(Color::DarkGray),
                    ));
                    list_lines.push(Line::styled(
                        format!(
                            "     protocol: {}",
                            if *is_registered {
                                "registered"
                            } else {
                                "not registered"
                            }
                        ),
                        if *is_registered {
                            Style::default().fg(Color::Green)
                        } else {
                            Style::default().fg(Color::Yellow)
                        },
                    ));
                }
            }
        }

        let list = Paragraph::new(list_lines)
            .block(picker_block)
            .wrap(Wrap { trim: false });
        f.render_widget(list, area);
        return;
    }

    if app.chat_layout == ChatLayoutMode::Webview {
        if render_webview_chat(f, app, theme) {
            render_help_overlay_if_needed(f, app, theme);
            return;
        }
    }

    // Chat view (default)
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(1),    // Messages
            Constraint::Length(3), // Input
            Constraint::Length(1), // Status bar
        ])
        .split(f.area());

    // Messages area with theme-based styling
    let messages_area = chunks[0];
    let model_label = app.active_model.as_deref().unwrap_or("auto");
    let target_label = app
        .active_spawned_agent
        .as_ref()
        .map(|name| format!(" @{}", name))
        .unwrap_or_default();
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(
            " CodeTether Agent [{}{}] model:{} ",
            app.current_agent, target_label, model_label
        ))
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let max_width = messages_area.width.saturating_sub(4) as usize;
    let message_lines = build_message_lines(app, theme, max_width);

    // Calculate scroll position
    let total_lines = message_lines.len();
    let visible_lines = messages_area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);
    // SCROLL_BOTTOM means "stick to bottom", otherwise clamp to max_scroll
    let scroll = if app.scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        app.scroll.min(max_scroll)
    };

    // Render messages with scrolling
    let messages_paragraph = Paragraph::new(
        message_lines[scroll..(scroll + visible_lines.min(total_lines)).min(total_lines)].to_vec(),
    )
    .block(messages_block.clone())
    .wrap(Wrap { trim: false });

    f.render_widget(messages_paragraph, messages_area);

    // Render scrollbar if needed
    if total_lines > visible_lines {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(ratatui::symbols::scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(total_lines).position(scroll);

        let scrollbar_area = Rect::new(
            messages_area.right() - 1,
            messages_area.top() + 1,
            1,
            messages_area.height - 2,
        );

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }

    // Input area
    let input_title = if app.is_processing {
        if let Some(started) = app.processing_started_at {
            let elapsed = started.elapsed();
            format!(" Processing ({:.0}s)... ", elapsed.as_secs_f64())
        } else {
            " Message (Processing...) ".to_string()
        }
    } else if app.autochat_running {
        format!(
            " {} ",
            app.autochat_status_label()
                .unwrap_or_else(|| "Autochat running…".to_string())
        )
    } else if app.input.starts_with('/') {
        let hint = match_slash_command_hint(&app.input);
        format!(" {} ", hint)
    } else if let Some(target) = &app.active_spawned_agent {
        format!(" Message to @{target} (use /agent main to exit) ")
    } else {
        " Message (Enter to send, / for commands) ".to_string()
    };
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(input_title)
        .border_style(Style::default().fg(if app.is_processing {
            Color::Yellow
        } else if app.autochat_running {
            Color::Cyan
        } else if app.input.starts_with('/') {
            Color::Magenta
        } else {
            theme.input_border_color.to_color()
        }));

    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input, chunks[1]);

    // Cursor
    f.set_cursor_position((
        chunks[1].x + app.cursor_position as u16 + 1,
        chunks[1].y + 1,
    ));

    // Enhanced status bar with token display and model info
    let token_display = TokenDisplay::new();
    let mut status_line = token_display.create_status_bar(theme);
    let model_status = if let Some(ref active) = app.active_model {
        let (provider, model) = crate::provider::parse_model_string(active);
        format!(" {}:{} ", provider.unwrap_or("auto"), model)
    } else {
        " auto ".to_string()
    };
    status_line.spans.insert(
        0,
        Span::styled(
            "│ ",
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        ),
    );
    status_line.spans.insert(
        0,
        Span::styled(model_status, Style::default().fg(Color::Cyan)),
    );
    if let Some(autochat_status) = app.autochat_status_label() {
        status_line.spans.insert(
            0,
            Span::styled(
                format!(" {autochat_status} "),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        );
    }
    let status = Paragraph::new(status_line);
    f.render_widget(status, chunks[2]);

    render_help_overlay_if_needed(f, app, theme);
}

fn render_webview_chat(f: &mut Frame, app: &App, theme: &Theme) -> bool {
    let area = f.area();
    if area.width < 90 || area.height < 18 {
        return false;
    }

    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(1),    // Body
            Constraint::Length(3), // Input
            Constraint::Length(1), // Status
        ])
        .split(area);

    render_webview_header(f, app, theme, main_chunks[0]);

    let body_constraints = if app.show_inspector {
        vec![
            Constraint::Length(26),
            Constraint::Min(40),
            Constraint::Length(30),
        ]
    } else {
        vec![Constraint::Length(26), Constraint::Min(40)]
    };

    let body_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(body_constraints)
        .split(main_chunks[1]);

    render_webview_sidebar(f, app, theme, body_chunks[0]);
    render_webview_chat_center(f, app, theme, body_chunks[1]);
    if app.show_inspector && body_chunks.len() > 2 {
        render_webview_inspector(f, app, theme, body_chunks[2]);
    }

    render_webview_input(f, app, theme, main_chunks[2]);

    let token_display = TokenDisplay::new();
    let mut status_line = token_display.create_status_bar(theme);
    let model_status = if let Some(ref active) = app.active_model {
        let (provider, model) = crate::provider::parse_model_string(active);
        format!(" {}:{} ", provider.unwrap_or("auto"), model)
    } else {
        " auto ".to_string()
    };
    status_line.spans.insert(
        0,
        Span::styled(
            "│ ",
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        ),
    );
    status_line.spans.insert(
        0,
        Span::styled(model_status, Style::default().fg(Color::Cyan)),
    );
    if let Some(autochat_status) = app.autochat_status_label() {
        status_line.spans.insert(
            0,
            Span::styled(
                format!(" {autochat_status} "),
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        );
    }
    let status = Paragraph::new(status_line);
    f.render_widget(status, main_chunks[3]);

    true
}

fn render_protocol_registry(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let cards = app.protocol_cards();
    let selected = app.protocol_selected.min(cards.len().saturating_sub(1));

    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Length(34), Constraint::Min(30)])
        .split(area);

    let list_block = Block::default()
        .borders(Borders::ALL)
        .title(" Registered Agents ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut list_lines: Vec<Line> = Vec::new();
    if cards.is_empty() {
        list_lines.push(Line::styled(
            "No protocol-registered agents.",
            Style::default().fg(Color::DarkGray),
        ));
        list_lines.push(Line::styled(
            "Spawn an agent with /spawn.",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for (idx, card) in cards.iter().enumerate() {
            let marker = if idx == selected { "▶" } else { " " };
            let style = if idx == selected {
                Style::default()
                    .fg(Color::Blue)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            let transport = card.preferred_transport.as_deref().unwrap_or("JSONRPC");
            list_lines.push(Line::styled(format!(" {marker} {}", card.name), style));
            list_lines.push(Line::styled(
                format!(
                    "    {transport} • {}",
                    truncate_with_ellipsis(&card.url, 22)
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let list = Paragraph::new(list_lines)
        .block(list_block)
        .wrap(Wrap { trim: false });
    f.render_widget(list, chunks[0]);

    let detail_block = Block::default()
        .borders(Borders::ALL)
        .title(" Agent Card Detail ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut detail_lines: Vec<Line> = Vec::new();
    if let Some(card) = cards.get(selected) {
        let label_style = Style::default().fg(Color::DarkGray);
        detail_lines.push(Line::from(vec![
            Span::styled("Name: ", label_style),
            Span::styled(
                card.name.clone(),
                Style::default().add_modifier(Modifier::BOLD),
            ),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled("Description: ", label_style),
            Span::raw(card.description.clone()),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled("URL: ", label_style),
            Span::styled(card.url.clone(), Style::default().fg(Color::Cyan)),
        ]));
        detail_lines.push(Line::from(vec![
            Span::styled("Version: ", label_style),
            Span::raw(format!(
                "{} (protocol {})",
                card.version, card.protocol_version
            )),
        ]));

        let preferred_transport = card.preferred_transport.as_deref().unwrap_or("JSONRPC");
        detail_lines.push(Line::from(vec![
            Span::styled("Transport: ", label_style),
            Span::raw(preferred_transport.to_string()),
        ]));
        if !card.additional_interfaces.is_empty() {
            detail_lines.push(Line::from(vec![
                Span::styled("Interfaces: ", label_style),
                Span::raw(format!("{} additional", card.additional_interfaces.len())),
            ]));
            for iface in &card.additional_interfaces {
                detail_lines.push(Line::styled(
                    format!("  • {} -> {}", iface.transport, iface.url),
                    Style::default().fg(Color::DarkGray),
                ));
            }
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::styled(
            "Capabilities",
            Style::default().add_modifier(Modifier::BOLD),
        ));
        detail_lines.push(Line::styled(
            format!(
                "  streaming={} push_notifications={} state_history={}",
                card.capabilities.streaming,
                card.capabilities.push_notifications,
                card.capabilities.state_transition_history
            ),
            Style::default().fg(Color::DarkGray),
        ));
        if !card.capabilities.extensions.is_empty() {
            detail_lines.push(Line::styled(
                format!(
                    "  extensions: {}",
                    card.capabilities
                        .extensions
                        .iter()
                        .map(|e| e.uri.as_str())
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::styled(
            format!("Skills ({})", card.skills.len()),
            Style::default().add_modifier(Modifier::BOLD),
        ));
        if card.skills.is_empty() {
            detail_lines.push(Line::styled("  none", Style::default().fg(Color::DarkGray)));
        } else {
            for skill in &card.skills {
                let tags = if skill.tags.is_empty() {
                    "".to_string()
                } else {
                    format!(" [{}]", skill.tags.join(","))
                };
                detail_lines.push(Line::styled(
                    format!("  • {}{}", skill.name, tags),
                    Style::default().fg(Color::Green),
                ));
                if !skill.description.is_empty() {
                    detail_lines.push(Line::styled(
                        format!("    {}", skill.description),
                        Style::default().fg(Color::DarkGray),
                    ));
                }
            }
        }

        detail_lines.push(Line::from(""));
        detail_lines.push(Line::styled(
            "Security",
            Style::default().add_modifier(Modifier::BOLD),
        ));
        if card.security_schemes.is_empty() {
            detail_lines.push(Line::styled(
                "  schemes: none",
                Style::default().fg(Color::DarkGray),
            ));
        } else {
            let mut names = card.security_schemes.keys().cloned().collect::<Vec<_>>();
            names.sort();
            detail_lines.push(Line::styled(
                format!("  schemes: {}", names.join(", ")),
                Style::default().fg(Color::DarkGray),
            ));
        }
        detail_lines.push(Line::styled(
            format!("  requirements: {}", card.security.len()),
            Style::default().fg(Color::DarkGray),
        ));
        detail_lines.push(Line::styled(
            format!(
                "  authenticated_extended_card: {}",
                card.supports_authenticated_extended_card
            ),
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        detail_lines.push(Line::styled(
            "No card selected.",
            Style::default().fg(Color::DarkGray),
        ));
    }

    let detail = Paragraph::new(detail_lines)
        .block(detail_block)
        .wrap(Wrap { trim: false })
        .scroll((app.protocol_scroll as u16, 0));
    f.render_widget(detail, chunks[1]);
}

fn render_webview_header(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let session_title = app
        .session
        .as_ref()
        .and_then(|s| s.title.clone())
        .unwrap_or_else(|| "Workspace Chat".to_string());
    let session_id = app
        .session
        .as_ref()
        .map(|s| s.id.chars().take(8).collect::<String>())
        .unwrap_or_else(|| "new".to_string());
    let model_label = app
        .session
        .as_ref()
        .and_then(|s| s.metadata.model.clone())
        .unwrap_or_else(|| "auto".to_string());
    let workspace_label = app.workspace.root_display.clone();
    let branch_label = app
        .workspace
        .git_branch
        .clone()
        .unwrap_or_else(|| "no-git".to_string());
    let dirty_label = if app.workspace.git_dirty_files > 0 {
        format!("{} dirty", app.workspace.git_dirty_files)
    } else {
        "clean".to_string()
    };

    let header_block = Block::default()
        .borders(Borders::ALL)
        .title(" CodeTether Webview ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let header_lines = vec![
        Line::from(vec![
            Span::styled(session_title, Style::default().add_modifier(Modifier::BOLD)),
            Span::raw(" "),
            Span::styled(
                format!("#{}", session_id),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]),
        Line::from(vec![
            Span::styled(
                "Workspace ",
                Style::default().fg(theme.timestamp_color.to_color()),
            ),
            Span::styled(workspace_label, Style::default()),
            Span::raw("  "),
            Span::styled(
                "Branch ",
                Style::default().fg(theme.timestamp_color.to_color()),
            ),
            Span::styled(
                branch_label,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                dirty_label,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw("  "),
            Span::styled(
                "Model ",
                Style::default().fg(theme.timestamp_color.to_color()),
            ),
            Span::styled(model_label, Style::default().fg(Color::Green)),
        ]),
    ];

    let header = Paragraph::new(header_lines)
        .block(header_block)
        .wrap(Wrap { trim: true });
    f.render_widget(header, area);
}

fn render_webview_sidebar(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let sidebar_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(8), Constraint::Min(6)])
        .split(area);

    let workspace_block = Block::default()
        .borders(Borders::ALL)
        .title(" Workspace ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut workspace_lines = Vec::new();
    workspace_lines.push(Line::from(vec![
        Span::styled(
            "Updated ",
            Style::default().fg(theme.timestamp_color.to_color()),
        ),
        Span::styled(
            app.workspace.captured_at.clone(),
            Style::default().fg(theme.timestamp_color.to_color()),
        ),
    ]));
    workspace_lines.push(Line::from(""));

    if app.workspace.entries.is_empty() {
        workspace_lines.push(Line::styled(
            "No entries found",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for entry in app.workspace.entries.iter().take(12) {
            let icon = match entry.kind {
                WorkspaceEntryKind::Directory => "📁",
                WorkspaceEntryKind::File => "📄",
            };
            workspace_lines.push(Line::from(vec![
                Span::styled(icon, Style::default().fg(Color::Cyan)),
                Span::raw(" "),
                Span::styled(entry.name.clone(), Style::default()),
            ]));
        }
    }

    workspace_lines.push(Line::from(""));
    workspace_lines.push(Line::styled(
        "Use /refresh to rescan",
        Style::default()
            .fg(Color::DarkGray)
            .add_modifier(Modifier::DIM),
    ));

    let workspace_panel = Paragraph::new(workspace_lines)
        .block(workspace_block)
        .wrap(Wrap { trim: true });
    f.render_widget(workspace_panel, sidebar_chunks[0]);

    let sessions_block = Block::default()
        .borders(Borders::ALL)
        .title(" Recent Sessions ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let mut session_lines = Vec::new();
    if app.session_picker_list.is_empty() {
        session_lines.push(Line::styled(
            "No sessions yet",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for session in app.session_picker_list.iter().take(6) {
            let is_active = app
                .session
                .as_ref()
                .map(|s| s.id == session.id)
                .unwrap_or(false);
            let title = session.title.as_deref().unwrap_or("(untitled)");
            let indicator = if is_active { "●" } else { "○" };
            let line_style = if is_active {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default()
            };
            session_lines.push(Line::from(vec![
                Span::styled(indicator, line_style),
                Span::raw(" "),
                Span::styled(title, line_style),
            ]));
            session_lines.push(Line::styled(
                format!(
                    "  {} msgs • {}",
                    session.message_count,
                    session.updated_at.format("%m-%d %H:%M")
                ),
                Style::default().fg(Color::DarkGray),
            ));
        }
    }

    let sessions_panel = Paragraph::new(session_lines)
        .block(sessions_block)
        .wrap(Wrap { trim: true });
    f.render_widget(sessions_panel, sidebar_chunks[1]);
}

fn render_webview_chat_center(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let messages_area = area;
    let focused_suffix = app
        .active_spawned_agent
        .as_ref()
        .map(|name| format!(" → @{name}"))
        .unwrap_or_default();
    let messages_block = Block::default()
        .borders(Borders::ALL)
        .title(format!(" Chat [{}{}] ", app.current_agent, focused_suffix))
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let max_width = messages_area.width.saturating_sub(4) as usize;
    let message_lines = build_message_lines(app, theme, max_width);

    let total_lines = message_lines.len();
    let visible_lines = messages_area.height.saturating_sub(2) as usize;
    let max_scroll = total_lines.saturating_sub(visible_lines);
    let scroll = if app.scroll >= SCROLL_BOTTOM {
        max_scroll
    } else {
        app.scroll.min(max_scroll)
    };

    let messages_paragraph = Paragraph::new(
        message_lines[scroll..(scroll + visible_lines.min(total_lines)).min(total_lines)].to_vec(),
    )
    .block(messages_block.clone())
    .wrap(Wrap { trim: false });

    f.render_widget(messages_paragraph, messages_area);

    if total_lines > visible_lines {
        let scrollbar = Scrollbar::default()
            .orientation(ScrollbarOrientation::VerticalRight)
            .symbols(ratatui::symbols::scrollbar::VERTICAL)
            .begin_symbol(Some("↑"))
            .end_symbol(Some("↓"));

        let mut scrollbar_state = ScrollbarState::new(total_lines).position(scroll);

        let scrollbar_area = Rect::new(
            messages_area.right() - 1,
            messages_area.top() + 1,
            1,
            messages_area.height - 2,
        );

        f.render_stateful_widget(scrollbar, scrollbar_area, &mut scrollbar_state);
    }
}

fn render_webview_inspector(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let block = Block::default()
        .borders(Borders::ALL)
        .title(" Inspector ")
        .border_style(Style::default().fg(theme.border_color.to_color()));

    let status_label = if app.is_processing {
        "Processing"
    } else if app.autochat_running {
        "Autochat"
    } else {
        "Idle"
    };
    let status_style = if app.is_processing {
        Style::default()
            .fg(Color::Yellow)
            .add_modifier(Modifier::BOLD)
    } else if app.autochat_running {
        Style::default()
            .fg(Color::Cyan)
            .add_modifier(Modifier::BOLD)
    } else {
        Style::default().fg(Color::Green)
    };
    let tool_label = app
        .current_tool
        .clone()
        .unwrap_or_else(|| "none".to_string());
    let message_count = app.messages.len();
    let session_id = app
        .session
        .as_ref()
        .map(|s| s.id.chars().take(8).collect::<String>())
        .unwrap_or_else(|| "new".to_string());
    let model_label = app
        .active_model
        .as_deref()
        .or_else(|| {
            app.session
                .as_ref()
                .and_then(|s| s.metadata.model.as_deref())
        })
        .unwrap_or("auto");
    let conversation_depth = app.session.as_ref().map(|s| s.messages.len()).unwrap_or(0);

    let label_style = Style::default().fg(theme.timestamp_color.to_color());

    let mut lines = Vec::new();
    lines.push(Line::from(vec![
        Span::styled("Status: ", label_style),
        Span::styled(status_label, status_style),
    ]));

    // Show elapsed time when processing
    if let Some(started) = app.processing_started_at {
        let elapsed = started.elapsed();
        let elapsed_str = if elapsed.as_secs() >= 60 {
            format!("{}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
        } else {
            format!("{:.1}s", elapsed.as_secs_f64())
        };
        lines.push(Line::from(vec![
            Span::styled("Elapsed: ", label_style),
            Span::styled(
                elapsed_str,
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
    }

    if app.autochat_running {
        if let Some(status) = app.autochat_status_label() {
            lines.push(Line::from(vec![
                Span::styled("Relay: ", label_style),
                Span::styled(status, Style::default().fg(Color::Cyan)),
            ]));
        }
    }

    lines.push(Line::from(vec![
        Span::styled("Tool: ", label_style),
        Span::styled(
            tool_label,
            if app.current_tool.is_some() {
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::DarkGray)
            },
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Session",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(vec![
        Span::styled("ID: ", label_style),
        Span::styled(format!("#{}", session_id), Style::default().fg(Color::Cyan)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Model: ", label_style),
        Span::styled(model_label.to_string(), Style::default().fg(Color::Green)),
    ]));
    let agent_display = if let Some(target) = &app.active_spawned_agent {
        format!("{} → @{} (focused)", app.current_agent, target)
    } else {
        app.current_agent.clone()
    };
    lines.push(Line::from(vec![
        Span::styled("Agent: ", label_style),
        Span::styled(agent_display, Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Messages: ", label_style),
        Span::styled(message_count.to_string(), Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Context: ", label_style),
        Span::styled(format!("{} turns", conversation_depth), Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Tools used: ", label_style),
        Span::styled(app.tool_call_count.to_string(), Style::default()),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Protocol: ", label_style),
        Span::styled(
            format!("{} registered", app.protocol_registered_count()),
            Style::default().fg(Color::Cyan),
        ),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Archive: ", label_style),
        Span::styled(
            format!("{} records", app.archived_message_count),
            Style::default(),
        ),
    ]));
    let sync_style = if app.chat_sync_last_error.is_some() {
        Style::default().fg(Color::Red)
    } else if app.chat_sync_rx.is_some() {
        Style::default().fg(Color::Green)
    } else {
        Style::default().fg(Color::DarkGray)
    };
    lines.push(Line::from(vec![
        Span::styled("Remote sync: ", label_style),
        Span::styled(
            app.chat_sync_status
                .as_deref()
                .unwrap_or("disabled")
                .to_string(),
            sync_style,
        ),
    ]));
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Sub-agents",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    if app.spawned_agents.is_empty() {
        lines.push(Line::styled(
            "None (use /spawn <name> <instructions>)",
            Style::default().fg(Color::DarkGray),
        ));
    } else {
        for (name, agent) in app.spawned_agents.iter().take(4) {
            let status = if agent.is_processing { "⚡" } else { "●" };
            let is_registered = app.is_agent_protocol_registered(name);
            let protocol = if is_registered { "🔗" } else { "⚠" };
            let focused = if app.active_spawned_agent.as_deref() == Some(name.as_str()) {
                " [focused]"
            } else {
                ""
            };
            lines.push(Line::styled(
                format!(
                    "{status} {protocol} {} @{name}{focused}",
                    agent_avatar(name)
                ),
                if focused.is_empty() {
                    Style::default().fg(Color::Magenta)
                } else {
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD)
                },
            ));
            let profile = agent_profile(name);
            lines.push(Line::styled(
                format!("   {} — {}", profile.codename, profile.profile),
                Style::default().fg(Color::Cyan).add_modifier(Modifier::DIM),
            ));
            lines.push(Line::styled(
                format!("   {}", agent.instructions),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ));
            if is_registered {
                lines.push(Line::styled(
                    format!("   bus://local/{name}"),
                    Style::default()
                        .fg(Color::Green)
                        .add_modifier(Modifier::DIM),
                ));
            }
        }
        if app.spawned_agents.len() > 4 {
            lines.push(Line::styled(
                format!("… and {} more", app.spawned_agents.len() - 4),
                Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM),
            ));
        }
    }
    lines.push(Line::from(""));
    lines.push(Line::styled(
        "Shortcuts",
        Style::default().add_modifier(Modifier::BOLD),
    ));
    lines.push(Line::from(vec![
        Span::styled("F3      ", Style::default().fg(Color::Yellow)),
        Span::styled("Inspector", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+B  ", Style::default().fg(Color::Yellow)),
        Span::styled("Layout", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+Y  ", Style::default().fg(Color::Yellow)),
        Span::styled("Copy", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+M  ", Style::default().fg(Color::Yellow)),
        Span::styled("Model", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("Ctrl+S  ", Style::default().fg(Color::Yellow)),
        Span::styled("Swarm", Style::default().fg(Color::DarkGray)),
    ]));
    lines.push(Line::from(vec![
        Span::styled("?       ", Style::default().fg(Color::Yellow)),
        Span::styled("Help", Style::default().fg(Color::DarkGray)),
    ]));

    let panel = Paragraph::new(lines).block(block).wrap(Wrap { trim: true });
    f.render_widget(panel, area);
}

fn render_webview_input(f: &mut Frame, app: &App, theme: &Theme, area: Rect) {
    let title = if app.is_processing {
        if let Some(started) = app.processing_started_at {
            let elapsed = started.elapsed();
            format!(" Processing ({:.0}s)... ", elapsed.as_secs_f64())
        } else {
            " Message (Processing...) ".to_string()
        }
    } else if app.autochat_running {
        format!(
            " {} ",
            app.autochat_status_label()
                .unwrap_or_else(|| "Autochat running…".to_string())
        )
    } else if app.input.starts_with('/') {
        // Show matching slash commands as hints
        let hint = match_slash_command_hint(&app.input);
        format!(" {} ", hint)
    } else if let Some(target) = &app.active_spawned_agent {
        format!(" Message to @{target} (use /agent main to exit) ")
    } else {
        " Message (Enter to send, / for commands) ".to_string()
    };

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title(title)
        .border_style(Style::default().fg(if app.is_processing {
            Color::Yellow
        } else if app.autochat_running {
            Color::Cyan
        } else if app.input.starts_with('/') {
            Color::Magenta
        } else {
            theme.input_border_color.to_color()
        }));

    let input = Paragraph::new(app.input.as_str())
        .block(input_block)
        .wrap(Wrap { trim: false });
    f.render_widget(input, area);

    f.set_cursor_position((area.x + app.cursor_position as u16 + 1, area.y + 1));
}

fn build_message_lines(app: &App, theme: &Theme, max_width: usize) -> Vec<Line<'static>> {
    let mut message_lines = Vec::new();
    let separator_width = max_width.min(60);

    for (idx, message) in app.messages.iter().enumerate() {
        let role_style = theme.get_role_style(&message.role);

        // Add a thin separator between messages (not before the first)
        if idx > 0 {
            let sep_char = match message.role.as_str() {
                "tool" => "·",
                _ => "─",
            };
            message_lines.push(Line::from(Span::styled(
                sep_char.repeat(separator_width),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            )));
        }

        // Role icons for better visual hierarchy
        let role_icon = match message.role.as_str() {
            "user" => "▸ ",
            "assistant" => "◆ ",
            "system" => "⚙ ",
            "tool" => "⚡",
            _ => "  ",
        };

        let header_line = {
            let mut spans = vec![
                Span::styled(
                    format!("[{}] ", message.timestamp),
                    Style::default()
                        .fg(theme.timestamp_color.to_color())
                        .add_modifier(Modifier::DIM),
                ),
                Span::styled(role_icon, role_style),
                Span::styled(message.role.clone(), role_style),
            ];
            if let Some(ref agent) = message.agent_name {
                let profile = agent_profile(agent);
                spans.push(Span::styled(
                    format!(" {} @{agent} ‹{}›", agent_avatar(agent), profile.codename),
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::BOLD),
                ));
            }
            Line::from(spans)
        };
        message_lines.push(header_line);

        match &message.message_type {
            MessageType::ToolCall {
                name,
                arguments_preview,
                arguments_len,
                truncated,
            } => {
                let tool_header = Line::from(vec![
                    Span::styled("  🔧 ", Style::default().fg(Color::Yellow)),
                    Span::styled(
                        format!("Tool: {}", name),
                        Style::default()
                            .fg(Color::Yellow)
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                message_lines.push(tool_header);

                if arguments_preview.trim().is_empty() {
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            "(no arguments)",
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                } else {
                    for line in arguments_preview.lines() {
                        let args_line = Line::from(vec![
                            Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled(line.to_string(), Style::default().fg(Color::DarkGray)),
                        ]);
                        message_lines.push(args_line);
                    }
                }

                if *truncated {
                    let args_line = Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("... (truncated; {} bytes)", arguments_len),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]);
                    message_lines.push(args_line);
                }
            }
            MessageType::ToolResult {
                name,
                output_preview,
                output_len,
                truncated,
                success,
                duration_ms,
            } => {
                let icon = if *success { "✅" } else { "❌" };
                let result_header = Line::from(vec![
                    Span::styled(
                        format!("  {icon} "),
                        Style::default().fg(if *success { Color::Green } else { Color::Red }),
                    ),
                    Span::styled(
                        format!("Result from {}", name),
                        Style::default()
                            .fg(if *success { Color::Green } else { Color::Red })
                            .add_modifier(Modifier::BOLD),
                    ),
                ]);
                message_lines.push(result_header);

                let status_line = format!(
                    "  │ status: {}{}",
                    if *success { "success" } else { "failure" },
                    duration_ms
                        .map(|ms| format!(" • {}", format_duration_ms(ms)))
                        .unwrap_or_default()
                );
                message_lines.push(Line::from(vec![
                    Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                    Span::styled(
                        status_line.trim_start_matches("  │ ").to_string(),
                        Style::default().fg(Color::DarkGray),
                    ),
                ]));

                if output_preview.trim().is_empty() {
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            "(empty output)",
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                } else {
                    for line in output_preview.lines() {
                        let output_line = Line::from(vec![
                            Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                            Span::styled(line.to_string(), Style::default().fg(Color::DarkGray)),
                        ]);
                        message_lines.push(output_line);
                    }
                }

                if *truncated {
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(
                            format!("... (truncated; {} bytes)", output_len),
                            Style::default()
                                .fg(Color::DarkGray)
                                .add_modifier(Modifier::DIM),
                        ),
                    ]));
                }
            }
            MessageType::Text(text) => {
                let formatter = MessageFormatter::new(max_width);
                let formatted_content = formatter.format_content(text, &message.role);
                message_lines.extend(formatted_content);
            }
            MessageType::Thinking(text) => {
                let thinking_style = Style::default()
                    .fg(Color::DarkGray)
                    .add_modifier(Modifier::DIM | Modifier::ITALIC);
                message_lines.push(Line::from(Span::styled(
                    "  💭 Thinking...",
                    Style::default()
                        .fg(Color::Magenta)
                        .add_modifier(Modifier::DIM),
                )));
                // Show truncated thinking content
                let max_thinking_lines = 8;
                let mut iter = text.lines();
                let mut shown = 0usize;
                while shown < max_thinking_lines {
                    let Some(line) = iter.next() else { break };
                    message_lines.push(Line::from(vec![
                        Span::styled("  │ ", Style::default().fg(Color::DarkGray)),
                        Span::styled(line.to_string(), thinking_style),
                    ]));
                    shown += 1;
                }
                if iter.next().is_some() {
                    message_lines.push(Line::from(Span::styled(
                        "  │ ... (truncated)",
                        thinking_style,
                    )));
                }
            }
            MessageType::Image { url, mime_type } => {
                let formatter = MessageFormatter::new(max_width);
                let image_line = formatter.format_image(url, mime_type.as_deref());
                message_lines.push(image_line);
            }
            MessageType::File { path, mime_type } => {
                let mime_label = mime_type.as_deref().unwrap_or("unknown type");
                let file_header = Line::from(vec![
                    Span::styled("  📎 ", Style::default().fg(Color::Cyan)),
                    Span::styled(
                        format!("File: {}", path),
                        Style::default()
                            .fg(Color::Cyan)
                            .add_modifier(Modifier::BOLD),
                    ),
                    Span::styled(
                        format!(" ({})", mime_label),
                        Style::default()
                            .fg(Color::DarkGray)
                            .add_modifier(Modifier::DIM),
                    ),
                ]);
                message_lines.push(file_header);
            }
        }

        // Show usage indicator after assistant messages
        if message.role == "assistant" {
            if let Some(ref meta) = message.usage_meta {
                let duration_str = if meta.duration_ms >= 60_000 {
                    format!(
                        "{}m{:02}.{}s",
                        meta.duration_ms / 60_000,
                        (meta.duration_ms % 60_000) / 1000,
                        (meta.duration_ms % 1000) / 100
                    )
                } else {
                    format!(
                        "{}.{}s",
                        meta.duration_ms / 1000,
                        (meta.duration_ms % 1000) / 100
                    )
                };
                let tokens_str =
                    format!("{}→{} tokens", meta.prompt_tokens, meta.completion_tokens);
                let cost_str = match meta.cost_usd {
                    Some(c) if c < 0.01 => format!("${:.4}", c),
                    Some(c) => format!("${:.2}", c),
                    None => String::new(),
                };
                let dim_style = Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM);
                let mut spans = vec![Span::styled(
                    format!("  ⏱ {} │ 📊 {}", duration_str, tokens_str),
                    dim_style,
                )];
                if !cost_str.is_empty() {
                    spans.push(Span::styled(format!(" │ 💰 {}", cost_str), dim_style));
                }
                message_lines.push(Line::from(spans));
            }
        }

        message_lines.push(Line::from(""));
    }

    // Show streaming text preview (text arriving before TextComplete finalizes it)
    if let Some(ref streaming) = app.streaming_text {
        if !streaming.is_empty() {
            message_lines.push(Line::from(Span::styled(
                "─".repeat(separator_width),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            )));
            message_lines.push(Line::from(vec![
                Span::styled(
                    format!("[{}] ", chrono::Local::now().format("%H:%M")),
                    Style::default()
                        .fg(theme.timestamp_color.to_color())
                        .add_modifier(Modifier::DIM),
                ),
                Span::styled("◆ ", theme.get_role_style("assistant")),
                Span::styled("assistant", theme.get_role_style("assistant")),
                Span::styled(
                    " (streaming...)",
                    Style::default()
                        .fg(theme.timestamp_color.to_color())
                        .add_modifier(Modifier::DIM),
                ),
            ]));
            let formatter = MessageFormatter::new(max_width);
            let formatted = formatter.format_content(streaming, "assistant");
            message_lines.extend(formatted);
            message_lines.push(Line::from(""));
        }
    }

    let mut agent_streams = app.streaming_agent_texts.iter().collect::<Vec<_>>();
    agent_streams.sort_by(|(a, _), (b, _)| a.to_lowercase().cmp(&b.to_lowercase()));
    for (agent, streaming) in agent_streams {
        if streaming.is_empty() {
            continue;
        }

        let profile = agent_profile(agent);

        message_lines.push(Line::from(Span::styled(
            "─".repeat(separator_width),
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        )));
        message_lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("◆ ", theme.get_role_style("assistant")),
            Span::styled("assistant", theme.get_role_style("assistant")),
            Span::styled(
                format!(" {} @{} ‹{}›", agent_avatar(agent), agent, profile.codename),
                Style::default()
                    .fg(Color::Magenta)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::styled(
                " (streaming...)",
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]));

        let formatter = MessageFormatter::new(max_width);
        let formatted = formatter.format_content(streaming, "assistant");
        message_lines.extend(formatted);
        message_lines.push(Line::from(""));
    }

    if app.is_processing {
        let spinner = current_spinner_frame();

        // Elapsed time display
        let elapsed_str = if let Some(started) = app.processing_started_at {
            let elapsed = started.elapsed();
            if elapsed.as_secs() >= 60 {
                format!(" {}m{:02}s", elapsed.as_secs() / 60, elapsed.as_secs() % 60)
            } else {
                format!(" {:.1}s", elapsed.as_secs_f64())
            }
        } else {
            String::new()
        };

        let processing_line = Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("◆ ", theme.get_role_style("assistant")),
            Span::styled("assistant", theme.get_role_style("assistant")),
            Span::styled(
                elapsed_str,
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
        ]);
        message_lines.push(processing_line);

        let (status_text, status_color) = if let Some(ref tool) = app.current_tool {
            (format!("  {spinner} Running: {}", tool), Color::Cyan)
        } else {
            (
                format!(
                    "  {} {}",
                    spinner,
                    app.processing_message.as_deref().unwrap_or("Thinking...")
                ),
                Color::Yellow,
            )
        };

        let indicator_line = Line::from(vec![Span::styled(
            status_text,
            Style::default()
                .fg(status_color)
                .add_modifier(Modifier::BOLD),
        )]);
        message_lines.push(indicator_line);
        message_lines.push(Line::from(""));
    }

    if app.autochat_running {
        let status_text = app
            .autochat_status_label()
            .unwrap_or_else(|| "Autochat running…".to_string());
        message_lines.push(Line::from(Span::styled(
            "─".repeat(separator_width),
            Style::default()
                .fg(theme.timestamp_color.to_color())
                .add_modifier(Modifier::DIM),
        )));
        message_lines.push(Line::from(vec![
            Span::styled(
                format!("[{}] ", chrono::Local::now().format("%H:%M")),
                Style::default()
                    .fg(theme.timestamp_color.to_color())
                    .add_modifier(Modifier::DIM),
            ),
            Span::styled("⚙ ", theme.get_role_style("system")),
            Span::styled(
                status_text,
                Style::default()
                    .fg(Color::Cyan)
                    .add_modifier(Modifier::BOLD),
            ),
        ]));
        message_lines.push(Line::from(""));
    }

    message_lines
}

fn match_slash_command_hint(input: &str) -> String {
    let commands = [
        (
            "/go ",
            "OKR-gated relay (requires approval, tracks outcomes)",
        ),
        ("/add ", "Easy mode: create a teammate"),
        ("/talk ", "Easy mode: message or focus a teammate"),
        ("/list", "Easy mode: list teammates"),
        ("/remove ", "Easy mode: remove a teammate"),
        ("/home", "Easy mode: return to main chat"),
        ("/help", "Open help"),
        ("/spawn ", "Create a named sub-agent"),
        ("/autochat ", "Tactical relay (fast path, no OKR tracking)"),
        ("/agents", "List spawned sub-agents"),
        ("/kill ", "Remove a spawned sub-agent"),
        ("/agent ", "Focus or message a spawned sub-agent"),
        ("/swarm ", "Run task in parallel swarm mode"),
        ("/ralph", "Start autonomous PRD loop"),
        ("/undo", "Undo last message and response"),
        ("/sessions", "Open session picker"),
        ("/resume", "Resume session or interrupted relay"),
        ("/new", "Start a new session"),
        ("/model", "Select or set model"),
        ("/webview", "Switch to webview layout"),
        ("/classic", "Switch to classic layout"),
        ("/inspector", "Toggle inspector pane"),
        ("/refresh", "Refresh workspace"),
        ("/archive", "Show persistent chat archive path"),
        ("/view", "Toggle swarm view"),
        ("/buslog", "Show protocol bus log"),
        ("/protocol", "Show protocol registry"),
    ];

    let trimmed = input.trim_start();
    let input_lower = trimmed.to_lowercase();

    // Exact command (or command + args) should always resolve to one hint.
    if let Some((cmd, desc)) = commands.iter().find(|(cmd, _)| {
        let key = cmd.trim_end().to_ascii_lowercase();
        input_lower == key || input_lower.starts_with(&(key + " "))
    }) {
        return format!("{} — {}", cmd.trim(), desc);
    }

    // Fallback to prefix matching while the user is still typing.
    let matches: Vec<_> = commands
        .iter()
        .filter(|(cmd, _)| cmd.starts_with(&input_lower))
        .collect();

    if matches.len() == 1 {
        format!("{} — {}", matches[0].0.trim(), matches[0].1)
    } else if matches.is_empty() {
        "Unknown command".to_string()
    } else {
        let cmds: Vec<_> = matches.iter().map(|(cmd, _)| cmd.trim()).collect();
        cmds.join(" | ")
    }
}

fn command_with_optional_args<'a>(input: &'a str, command: &str) -> Option<&'a str> {
    let trimmed = input.trim();
    let rest = trimmed.strip_prefix(command)?;

    if rest.is_empty() {
        return Some("");
    }

    let first = rest.chars().next()?;
    if first.is_whitespace() {
        Some(rest.trim())
    } else {
        None
    }
}

fn normalize_easy_command(input: &str) -> String {
    let trimmed = input.trim();
    if trimmed.is_empty() {
        return String::new();
    }

    if !trimmed.starts_with('/') {
        return input.to_string();
    }

    let mut parts = trimmed.splitn(2, char::is_whitespace);
    let command = parts.next().unwrap_or("");
    let args = parts.next().unwrap_or("").trim();

    match command.to_ascii_lowercase().as_str() {
        "/go" | "/team" => {
            if args.is_empty() {
                "/autochat".to_string()
            } else {
                let mut parts = args.splitn(2, char::is_whitespace);
                let first = parts.next().unwrap_or("").trim();
                if let Ok(count) = first.parse::<usize>() {
                    let rest = parts.next().unwrap_or("").trim();
                    if rest.is_empty() {
                        format!("/autochat {count} {AUTOCHAT_QUICK_DEMO_TASK}")
                    } else {
                        format!("/autochat {count} {rest}")
                    }
                } else {
                    format!("/autochat {AUTOCHAT_DEFAULT_AGENTS} {args}")
                }
            }
        }
        "/add" => {
            if args.is_empty() {
                "/spawn".to_string()
            } else {
                format!("/spawn {args}")
            }
        }
        "/list" | "/ls" => "/agents".to_string(),
        "/remove" | "/rm" => {
            if args.is_empty() {
                "/kill".to_string()
            } else {
                format!("/kill {args}")
            }
        }
        "/talk" | "/say" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {args}")
            }
        }
        "/focus" => {
            if args.is_empty() {
                "/agent".to_string()
            } else {
                format!("/agent {}", args.trim_start_matches('@'))
            }
        }
        "/home" | "/main" => "/agent main".to_string(),
        "/h" | "/?" => "/help".to_string(),
        _ => trimmed.to_string(),
    }
}

fn is_easy_go_command(input: &str) -> bool {
    let command = input
        .trim_start()
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(command.as_str(), "/go" | "/team")
}

fn is_glm5_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "zai/glm-5" | "z-ai/glm-5" | "openrouter/z-ai/glm-5"
    )
}

fn is_minimax_m25_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "minimax/minimax-m2.5"
            | "minimax-m2.5"
            | "minimax-credits/minimax-m2.5-highspeed"
            | "minimax-m2.5-highspeed"
    )
}

fn next_go_model(current_model: Option<&str>) -> String {
    match current_model {
        Some(model) if is_glm5_model(model) => GO_SWAP_MODEL_MINIMAX.to_string(),
        Some(model) if is_minimax_m25_model(model) => GO_SWAP_MODEL_GLM.to_string(),
        _ => GO_SWAP_MODEL_MINIMAX.to_string(),
    }
}

fn parse_autochat_args(rest: &str) -> Option<(usize, &str)> {
    let rest = rest.trim();
    if rest.is_empty() {
        return None;
    }

    let mut parts = rest.splitn(2, char::is_whitespace);
    let first = parts.next().unwrap_or("").trim();
    if first.is_empty() {
        return None;
    }

    if let Ok(count) = first.parse::<usize>() {
        let task = parts.next().unwrap_or("").trim();
        if task.is_empty() {
            Some((count, AUTOCHAT_QUICK_DEMO_TASK))
        } else {
            Some((count, task))
        }
    } else {
        Some((AUTOCHAT_DEFAULT_AGENTS, rest))
    }
}

fn normalize_for_convergence(text: &str) -> String {
    let mut normalized = String::with_capacity(text.len().min(512));
    let mut last_was_space = false;

    for ch in text.chars() {
        if ch.is_ascii_alphanumeric() {
            normalized.push(ch.to_ascii_lowercase());
            last_was_space = false;
        } else if ch.is_whitespace() && !last_was_space {
            normalized.push(' ');
            last_was_space = true;
        }

        if normalized.len() >= 280 {
            break;
        }
    }

    normalized.trim().to_string()
}

fn agent_profile(agent_name: &str) -> AgentProfile {
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

fn format_agent_profile_summary(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} — {} ({})",
        profile.codename, profile.profile, profile.personality
    )
}

fn agent_avatar(agent_name: &str) -> &'static str {
    let mut hash: u64 = 2_166_136_261;
    for byte in agent_name.bytes() {
        hash = (hash ^ u64::from(byte.to_ascii_lowercase())).wrapping_mul(16_777_619);
    }
    AGENT_AVATARS[hash as usize % AGENT_AVATARS.len()]
}

fn format_agent_identity(agent_name: &str) -> String {
    let profile = agent_profile(agent_name);
    format!(
        "{} @{} ‹{}›",
        agent_avatar(agent_name),
        agent_name,
        profile.codename
    )
}

fn format_relay_participant(participant: &str) -> String {
    if participant.eq_ignore_ascii_case("user") {
        "[you]".to_string()
    } else {
        format_agent_identity(participant)
    }
}

fn format_relay_handoff_line(relay_id: &str, round: usize, from: &str, to: &str) -> String {
    format!(
        "[relay {relay_id} • round {round}] {} → {}",
        format_relay_participant(from),
        format_relay_participant(to)
    )
}

fn format_tool_call_arguments(name: &str, arguments: &str) -> String {
    // Avoid expensive JSON parsing/pretty-printing for very large payloads.
    // Large tool arguments are common (e.g., patches) and reformatting them provides
    // little value in a terminal preview.
    if arguments.len() > TOOL_ARGS_PRETTY_JSON_MAX_BYTES {
        return arguments.to_string();
    }

    let parsed = match serde_json::from_str::<serde_json::Value>(arguments) {
        Ok(value) => value,
        Err(_) => return arguments.to_string(),
    };

    if name == "question"
        && let Some(question) = parsed.get("question").and_then(serde_json::Value::as_str)
    {
        return question.to_string();
    }

    serde_json::to_string_pretty(&parsed).unwrap_or_else(|_| arguments.to_string())
}

fn build_tool_arguments_preview(
    tool_name: &str,
    arguments: &str,
    max_lines: usize,
    max_bytes: usize,
) -> (String, bool) {
    // Pretty-print when reasonably sized; otherwise keep raw to avoid a heavy parse.
    let formatted = format_tool_call_arguments(tool_name, arguments);
    build_text_preview(&formatted, max_lines, max_bytes)
}

/// Build a stable, size-limited preview used by the renderer.
///
/// Returns (preview_text, truncated).
fn build_text_preview(text: &str, max_lines: usize, max_bytes: usize) -> (String, bool) {
    if max_lines == 0 || max_bytes == 0 || text.is_empty() {
        return (String::new(), !text.is_empty());
    }

    let mut out = String::new();
    let mut truncated = false;
    let mut remaining = max_bytes;

    let mut iter = text.lines();
    for i in 0..max_lines {
        let Some(line) = iter.next() else { break };

        // Add newline separator if needed
        if i > 0 {
            if remaining == 0 {
                truncated = true;
                break;
            }
            out.push('\n');
            remaining = remaining.saturating_sub(1);
        }

        if remaining == 0 {
            truncated = true;
            break;
        }

        if line.len() <= remaining {
            out.push_str(line);
            remaining = remaining.saturating_sub(line.len());
        } else {
            // Truncate this line to remaining bytes, respecting UTF-8 boundaries.
            let mut end = remaining;
            while end > 0 && !line.is_char_boundary(end) {
                end -= 1;
            }
            out.push_str(&line[..end]);
            truncated = true;
            break;
        }
    }

    // If there are still lines left, we truncated.
    if !truncated && iter.next().is_some() {
        truncated = true;
    }

    (out, truncated)
}

fn truncate_with_ellipsis(value: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }

    let mut chars = value.chars();
    let mut output = String::new();
    for _ in 0..max_chars {
        if let Some(ch) = chars.next() {
            output.push(ch);
        } else {
            return value.to_string();
        }
    }

    if chars.next().is_some() {
        format!("{output}...")
    } else {
        output
    }
}

fn message_clipboard_text(message: &ChatMessage) -> String {
    let mut prefix = String::new();
    if let Some(agent) = &message.agent_name {
        prefix = format!("@{agent}\n");
    }

    match &message.message_type {
        MessageType::Text(text) => format!("{prefix}{text}"),
        MessageType::Thinking(text) => format!("{prefix}{text}"),
        MessageType::Image { url, .. } => format!("{prefix}{url}"),
        MessageType::File { path, .. } => format!("{prefix}{path}"),
        MessageType::ToolCall {
            name,
            arguments_preview,
            ..
        } => format!("{prefix}Tool call: {name}\n{arguments_preview}"),
        MessageType::ToolResult {
            name,
            output_preview,
            ..
        } => format!("{prefix}Tool result: {name}\n{output_preview}"),
    }
}

fn copy_text_to_clipboard_best_effort(text: &str) -> Result<&'static str, String> {
    if text.trim().is_empty() {
        return Err("empty text".to_string());
    }

    // 1) Try system clipboard first (works locally when a clipboard provider is available)
    match arboard::Clipboard::new().and_then(|mut clipboard| clipboard.set_text(text.to_string())) {
        Ok(()) => return Ok("system clipboard"),
        Err(e) => {
            tracing::debug!(error = %e, "System clipboard unavailable; falling back to OSC52");
        }
    }

    // 2) Fallback: OSC52 (works in many terminals, including remote SSH sessions)
    osc52_copy(text).map_err(|e| format!("osc52 copy failed: {e}"))?;
    Ok("OSC52")
}

fn osc52_copy(text: &str) -> std::io::Result<()> {
    // OSC52 format: ESC ] 52 ; c ; <base64> BEL
    // Some terminals may disable OSC52 for security; we treat this as best-effort.
    let payload = base64::engine::general_purpose::STANDARD.encode(text.as_bytes());
    let seq = format!("\u{1b}]52;c;{payload}\u{07}");

    let mut stdout = std::io::stdout();
    crossterm::execute!(stdout, crossterm::style::Print(seq))?;
    use std::io::Write;
    stdout.flush()?;
    Ok(())
}

fn render_help_overlay_if_needed(f: &mut Frame, app: &App, theme: &Theme) {
    if !app.show_help {
        return;
    }

    let area = centered_rect(60, 60, f.area());
    f.render_widget(Clear, area);

    let token_display = TokenDisplay::new();
    let token_info = token_display.create_detailed_display();

    // Model / provider info
    let model_section: Vec<String> = if let Some(ref active) = app.active_model {
        let (provider, model) = crate::provider::parse_model_string(active);
        let provider_label = provider.unwrap_or("auto");
        vec![
            "".to_string(),
            "  ACTIVE MODEL".to_string(),
            "  ==============".to_string(),
            format!("  Provider:  {}", provider_label),
            format!("  Model:     {}", model),
            format!("  Agent:     {}", app.current_agent),
        ]
    } else {
        vec![
            "".to_string(),
            "  ACTIVE MODEL".to_string(),
            "  ==============".to_string(),
            format!("  Provider:  auto"),
            format!("  Model:     (default)"),
            format!("  Agent:     {}", app.current_agent),
        ]
    };

    let help_text: Vec<String> = vec![
        "".to_string(),
        "  KEYBOARD SHORTCUTS".to_string(),
        "  ==================".to_string(),
        "".to_string(),
        "  Enter        Send message".to_string(),
        "  Tab          Switch between build/plan agents".to_string(),
        "  Ctrl+A       Open spawned-agent picker".to_string(),
        "  Ctrl+M       Open model picker".to_string(),
        "  Ctrl+L       Protocol bus log".to_string(),
        "  Ctrl+P       Protocol registry".to_string(),
        "  Ctrl+S       Toggle swarm view".to_string(),
        "  Ctrl+B       Toggle webview layout".to_string(),
        "  Ctrl+Y       Copy latest assistant reply".to_string(),
        "  F3           Toggle inspector pane".to_string(),
        "  Ctrl+C       Quit".to_string(),
        "  ?            Toggle this help".to_string(),
        "".to_string(),
        "  SLASH COMMANDS (auto-complete hints shown while typing)".to_string(),
        "  OKR-GATED MODE (requires approval, tracks measurable outcomes)".to_string(),
        "  /go <task>      OKR-gated relay: draft → approve → execute → track KR progress"
            .to_string(),
        "".to_string(),
        "  TACTICAL MODE (fast path, no OKR tracking)".to_string(),
        "  /autochat [count] <task>  Immediate relay: no approval needed, no outcome tracking"
            .to_string(),
        "".to_string(),
        "  EASY MODE".to_string(),
        "  /add <name>     Create a helper teammate".to_string(),
        "  /talk <name> <message>  Message teammate".to_string(),
        "  /list           List teammates".to_string(),
        "  /remove <name>  Remove teammate".to_string(),
        "  /home           Return to main chat".to_string(),
        "  /help           Open this help".to_string(),
        "".to_string(),
        "  ADVANCED MODE".to_string(),
        "  /spawn <name> <instructions>  Create a named sub-agent".to_string(),
        "  /agents        List spawned sub-agents".to_string(),
        "  /kill <name>   Remove a spawned sub-agent".to_string(),
        "  /agent <name>  Focus chat on a spawned sub-agent".to_string(),
        "  /agent <name> <message>  Send one message to a spawned sub-agent".to_string(),
        "  /agent            Open spawned-agent picker".to_string(),
        "  /agent main|off  Exit focused sub-agent chat".to_string(),
        "  /swarm <task>   Run task in parallel swarm mode".to_string(),
        "  /ralph [path]   Start Ralph PRD loop (default: prd.json)".to_string(),
        "  /undo           Undo last message and response".to_string(),
        "  /sessions       Open session picker (filter, delete, load, n/p paginate)".to_string(),
        "  /resume         Resume interrupted relay or most recent session".to_string(),
        "  /resume <id>    Resume specific session by ID".to_string(),
        "  /new            Start a fresh session".to_string(),
        "  /model          Open model picker (or /model <name>)".to_string(),
        "  /view           Toggle swarm view".to_string(),
        "  /buslog         Show protocol bus log".to_string(),
        "  /protocol       Show protocol registry and AgentCards".to_string(),
        "  /webview        Web dashboard layout".to_string(),
        "  /classic        Single-pane layout".to_string(),
        "  /inspector      Toggle inspector pane".to_string(),
        "  /refresh        Refresh workspace and sessions".to_string(),
        "  /archive        Show persistent chat archive path".to_string(),
        "".to_string(),
        "  SESSION PICKER".to_string(),
        "  ↑/↓/j/k      Navigate sessions".to_string(),
        "  Enter         Load selected session".to_string(),
        "  d             Delete session (press twice to confirm)".to_string(),
        "  Type          Filter sessions by name/agent/ID".to_string(),
        "  Backspace     Clear filter character".to_string(),
        "  Esc           Close picker".to_string(),
        "".to_string(),
        "  VIM-STYLE NAVIGATION".to_string(),
        "  Alt+j        Scroll down".to_string(),
        "  Alt+k        Scroll up".to_string(),
        "  Ctrl+g       Go to top".to_string(),
        "  Ctrl+G       Go to bottom".to_string(),
        "".to_string(),
        "  SCROLLING".to_string(),
        "  Up/Down      Scroll messages".to_string(),
        "  PageUp/Dn    Scroll one page".to_string(),
        "  Alt+u/d      Scroll half page".to_string(),
        "".to_string(),
        "  COMMAND HISTORY".to_string(),
        "  Ctrl+R       Search history".to_string(),
        "  Ctrl+Up/Dn   Navigate history".to_string(),
        "".to_string(),
        "  Press ? or Esc to close".to_string(),
        "".to_string(),
    ];

    let mut combined_text = token_info;
    combined_text.extend(model_section);
    combined_text.extend(help_text);

    let help = Paragraph::new(combined_text.join("\n"))
        .block(
            Block::default()
                .borders(Borders::ALL)
                .title(" Help ")
                .border_style(Style::default().fg(theme.help_border_color.to_color())),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(help, area);
}

/// Helper to create a centered rect
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}

#[cfg(test)]
mod tests {
    use super::{
        AUTOCHAT_QUICK_DEMO_TASK, agent_avatar, agent_profile, command_with_optional_args,
        estimate_cost, format_agent_identity, format_relay_handoff_line, is_easy_go_command,
        is_secure_environment_from_values, match_slash_command_hint, minio_fallback_endpoint,
        next_go_model, normalize_easy_command, normalize_for_convergence, normalize_minio_endpoint,
        parse_autochat_args,
    };

    #[test]
    fn command_with_optional_args_handles_bare_command() {
        assert_eq!(command_with_optional_args("/spawn", "/spawn"), Some(""));
    }

    #[test]
    fn command_with_optional_args_handles_arguments() {
        assert_eq!(
            command_with_optional_args("/spawn planner you plan", "/spawn"),
            Some("planner you plan")
        );
    }

    #[test]
    fn command_with_optional_args_ignores_prefix_collisions() {
        assert_eq!(command_with_optional_args("/spawned", "/spawn"), None);
    }

    #[test]
    fn command_with_optional_args_ignores_autochat_prefix_collisions() {
        assert_eq!(command_with_optional_args("/autochatty", "/autochat"), None);
    }

    #[test]
    fn command_with_optional_args_trims_leading_whitespace_in_args() {
        assert_eq!(
            command_with_optional_args("/kill    local-agent-1", "/kill"),
            Some("local-agent-1")
        );
    }

    #[test]
    fn slash_hint_includes_protocol_command() {
        let hint = match_slash_command_hint("/protocol");
        assert!(hint.contains("/protocol"));
    }

    #[test]
    fn slash_hint_includes_autochat_command() {
        let hint = match_slash_command_hint("/autochat");
        assert!(hint.contains("/autochat"));
    }

    #[test]
    fn normalize_easy_command_maps_go_to_autochat() {
        assert_eq!(
            normalize_easy_command("/go build a calculator"),
            "/autochat 3 build a calculator"
        );
    }

    #[test]
    fn normalize_easy_command_maps_go_count_and_task() {
        assert_eq!(
            normalize_easy_command("/go 4 build a calculator"),
            "/autochat 4 build a calculator"
        );
    }

    #[test]
    fn normalize_easy_command_maps_go_count_only_to_demo_task() {
        assert_eq!(
            normalize_easy_command("/go 4"),
            format!("/autochat 4 {AUTOCHAT_QUICK_DEMO_TASK}")
        );
    }

    #[test]
    fn slash_hint_handles_command_with_args() {
        let hint = match_slash_command_hint("/go 4");
        assert!(hint.contains("/go"));
    }

    #[test]
    fn parse_autochat_args_supports_default_count() {
        assert_eq!(
            parse_autochat_args("build a calculator"),
            Some((3, "build a calculator"))
        );
    }

    #[test]
    fn parse_autochat_args_supports_explicit_count() {
        assert_eq!(
            parse_autochat_args("4 build a calculator"),
            Some((4, "build a calculator"))
        );
    }

    #[test]
    fn parse_autochat_args_count_only_uses_quick_demo_task() {
        assert_eq!(
            parse_autochat_args("4"),
            Some((4, AUTOCHAT_QUICK_DEMO_TASK))
        );
    }

    #[test]
    fn normalize_for_convergence_ignores_case_and_punctuation() {
        let a = normalize_for_convergence("Done! Next Step: Add tests.");
        let b = normalize_for_convergence("done next step add tests");
        assert_eq!(a, b);
    }

    #[test]
    fn agent_avatar_is_stable_and_ascii() {
        let avatar = agent_avatar("planner");
        assert_eq!(avatar, agent_avatar("planner"));
        assert!(avatar.is_ascii());
        assert!(avatar.starts_with('[') && avatar.ends_with(']'));
    }

    #[test]
    fn relay_handoff_line_shows_avatar_labels() {
        let line = format_relay_handoff_line("relay-1", 2, "planner", "coder");
        assert!(line.contains("relay relay-1"));
        assert!(line.contains("@planner"));
        assert!(line.contains("@coder"));
        assert!(line.contains('['));
    }

    #[test]
    fn relay_handoff_line_formats_user_sender() {
        let line = format_relay_handoff_line("relay-2", 1, "user", "planner");
        assert!(line.contains("[you]"));
        assert!(line.contains("@planner"));
    }

    #[test]
    fn planner_profile_has_expected_personality() {
        let profile = agent_profile("auto-planner");
        assert_eq!(profile.codename, "Strategist");
        assert!(profile.profile.contains("decomposition"));
    }

    #[test]
    fn formatted_identity_includes_codename() {
        let identity = format_agent_identity("auto-coder");
        assert!(identity.contains("@auto-coder"));
        assert!(identity.contains("‹Forge›"));
    }

    #[test]
    fn normalize_minio_endpoint_strips_login_path() {
        assert_eq!(
            normalize_minio_endpoint("http://192.168.50.223:9001/login"),
            "http://192.168.50.223:9001"
        );
    }

    #[test]
    fn normalize_minio_endpoint_adds_default_scheme() {
        assert_eq!(
            normalize_minio_endpoint("192.168.50.223:9000"),
            "http://192.168.50.223:9000"
        );
    }

    #[test]
    fn fallback_endpoint_maps_console_port_to_s3_port() {
        assert_eq!(
            minio_fallback_endpoint("http://192.168.50.223:9001"),
            Some("http://192.168.50.223:9000".to_string())
        );
        assert_eq!(minio_fallback_endpoint("http://192.168.50.223:9000"), None);
    }

    #[test]
    fn secure_environment_detection_respects_explicit_flags() {
        assert!(is_secure_environment_from_values(Some(true), None, None));
        assert!(!is_secure_environment_from_values(
            Some(false),
            Some(true),
            Some("secure")
        ));
    }

    #[test]
    fn secure_environment_detection_uses_environment_name_fallback() {
        assert!(is_secure_environment_from_values(None, None, Some("PROD")));
        assert!(is_secure_environment_from_values(
            None,
            None,
            Some("production")
        ));
        assert!(!is_secure_environment_from_values(None, None, Some("dev")));
    }

    #[test]
    fn minimax_m25_pricing_estimate_matches_rates() {
        let cost = estimate_cost("minimax/MiniMax-M2.5", 1_000_000, 1_000_000)
            .expect("MiniMax M2.5 cost should be available");
        assert!((cost - 1.5).abs() < 1e-9); // $0.3 + $1.2 = $1.5
    }

    #[test]
    fn minimax_m25_highspeed_pricing() {
        let cost = estimate_cost("MiniMax-M2.5-highspeed", 1_000_000, 1_000_000)
            .expect("MiniMax M2.5 Highspeed cost should be available");
        assert!((cost - 3.0).abs() < 1e-9); // $0.6 + $2.4 = $3.0
    }

    #[test]
    fn easy_go_command_detects_go_and_team_aliases() {
        assert!(is_easy_go_command("/go build indexing"));
        assert!(is_easy_go_command("/team 4 implement auth"));
        assert!(!is_easy_go_command("/autochat build indexing"));
    }

    #[test]
    fn next_go_model_toggles_between_glm_and_minimax() {
        assert_eq!(
            next_go_model(Some("zai/glm-5")),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
        assert_eq!(
            next_go_model(Some("z-ai/glm-5")),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
        assert_eq!(
            next_go_model(Some("minimax-credits/MiniMax-M2.5-highspeed")),
            "zai/glm-5"
        );
        assert_eq!(
            next_go_model(Some("unknown/model")),
            "minimax-credits/MiniMax-M2.5-highspeed"
        );
    }
}
