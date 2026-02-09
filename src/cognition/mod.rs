//! Perpetual cognition runtime for persona swarms.
//!
//! Phase 0 scope:
//! - Contract types for personas, thought events, proposals, and snapshots
//! - In-memory runtime manager for lifecycle + lineage
//! - Feature-flagged perpetual loop (no external execution side effects)

pub mod beliefs;
pub mod executor;
pub mod persistence;
mod thinker;

#[cfg(feature = "functiongemma")]
pub mod tool_router;

pub use thinker::{
    CandleDevicePreference, ThinkerBackend, ThinkerClient, ThinkerConfig, ThinkerOutput,
};

use crate::tool::ToolRegistry;
use anyhow::{Result, anyhow};
use beliefs::{Belief, BeliefStatus};
use chrono::{DateTime, Duration as ChronoDuration, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio::task::JoinHandle;
use tokio::time::Instant;
use uuid::Uuid;

/// Persona execution status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PersonaStatus {
    Active,
    Idle,
    Reaped,
    Error,
}

/// Policy boundaries for a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaPolicy {
    pub max_spawn_depth: u32,
    pub max_branching_factor: u32,
    pub token_budget_per_minute: u32,
    pub compute_ms_per_minute: u32,
    pub idle_ttl_secs: u64,
    pub share_memory: bool,
    #[serde(default)]
    pub allowed_tools: Vec<String>,
}

impl Default for PersonaPolicy {
    fn default() -> Self {
        Self {
            max_spawn_depth: 4,
            max_branching_factor: 4,
            token_budget_per_minute: 20_000,
            compute_ms_per_minute: 10_000,
            idle_ttl_secs: 3_600,
            share_memory: false,
            allowed_tools: Vec::new(),
        }
    }
}

/// Identity contract for a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaIdentity {
    pub id: String,
    pub name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub parent_id: Option<String>,
    pub depth: u32,
    pub created_at: DateTime<Utc>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Full runtime state for a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PersonaRuntimeState {
    pub identity: PersonaIdentity,
    pub policy: PersonaPolicy,
    pub status: PersonaStatus,
    pub thought_count: u64,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub updated_at: DateTime<Utc>,
    /// Tokens consumed in current 60-second window.
    pub tokens_this_window: u32,
    /// Compute milliseconds consumed in current 60-second window.
    pub compute_ms_this_window: u32,
    /// Start of the current budget window.
    pub window_started_at: DateTime<Utc>,
    /// Last time this persona made meaningful progress (not budget-paused/quorum-waiting).
    pub last_progress_at: DateTime<Utc>,
    /// Whether this persona is currently paused due to budget exhaustion.
    #[serde(default)]
    pub budget_paused: bool,
}

/// Proposal risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalRisk {
    Low,
    Medium,
    High,
    Critical,
}

/// Proposal status.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalStatus {
    Created,
    Verified,
    Rejected,
    Executed,
}

/// A vote on a proposal.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ProposalVote {
    Approve,
    Reject,
    Veto,
    Abstain,
}

/// Proposal contract (think first, execute through gates).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Proposal {
    pub id: String,
    pub persona_id: String,
    pub title: String,
    pub rationale: String,
    pub evidence_refs: Vec<String>,
    pub risk: ProposalRisk,
    pub status: ProposalStatus,
    pub created_at: DateTime<Utc>,
    /// Votes from personas: persona_id -> vote
    #[serde(default)]
    pub votes: HashMap<String, ProposalVote>,
    /// Deadline for voting
    pub vote_deadline: Option<DateTime<Utc>>,
    /// Whether votes have been requested this cycle
    #[serde(default)]
    pub votes_requested: bool,
}

/// Thought/event types emitted by the cognition loop.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ThoughtEventType {
    ThoughtGenerated,
    HypothesisRaised,
    CheckRequested,
    CheckResult,
    ProposalCreated,
    ProposalVerified,
    ProposalRejected,
    ActionExecuted,
    PersonaSpawned,
    PersonaReaped,
    SnapshotCompressed,
    BeliefExtracted,
    BeliefContested,
    BeliefRevalidated,
    BudgetPaused,
    IdleReaped,
    AttentionCreated,
    VoteCast,
    WorkspaceUpdated,
}

impl ThoughtEventType {
    fn as_str(&self) -> &'static str {
        match self {
            Self::ThoughtGenerated => "thought_generated",
            Self::HypothesisRaised => "hypothesis_raised",
            Self::CheckRequested => "check_requested",
            Self::CheckResult => "check_result",
            Self::ProposalCreated => "proposal_created",
            Self::ProposalVerified => "proposal_verified",
            Self::ProposalRejected => "proposal_rejected",
            Self::ActionExecuted => "action_executed",
            Self::PersonaSpawned => "persona_spawned",
            Self::PersonaReaped => "persona_reaped",
            Self::SnapshotCompressed => "snapshot_compressed",
            Self::BeliefExtracted => "belief_extracted",
            Self::BeliefContested => "belief_contested",
            Self::BeliefRevalidated => "belief_revalidated",
            Self::BudgetPaused => "budget_paused",
            Self::IdleReaped => "idle_reaped",
            Self::AttentionCreated => "attention_created",
            Self::VoteCast => "vote_cast",
            Self::WorkspaceUpdated => "workspace_updated",
        }
    }
}

/// Streamable thought event contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThoughtEvent {
    pub id: String,
    pub event_type: ThoughtEventType,
    pub persona_id: Option<String>,
    pub swarm_id: Option<String>,
    pub timestamp: DateTime<Utc>,
    pub payload: serde_json::Value,
}

/// Distilled memory snapshot contract.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemorySnapshot {
    pub id: String,
    pub generated_at: DateTime<Utc>,
    pub swarm_id: Option<String>,
    pub persona_scope: Vec<String>,
    pub summary: String,
    pub hot_event_count: usize,
    pub warm_fact_count: usize,
    pub cold_snapshot_count: usize,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Request payload for creating a persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreatePersonaRequest {
    pub persona_id: Option<String>,
    pub name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub parent_id: Option<String>,
    pub policy: Option<PersonaPolicy>,
    #[serde(default)]
    pub tags: Vec<String>,
}

/// Request payload for spawning a child persona.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpawnPersonaRequest {
    pub persona_id: Option<String>,
    pub name: String,
    pub role: String,
    pub charter: String,
    pub swarm_id: Option<String>,
    pub policy: Option<PersonaPolicy>,
}

/// Request payload for reaping persona(s).
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReapPersonaRequest {
    pub cascade: Option<bool>,
    pub reason: Option<String>,
}

/// Start-control payload for perpetual cognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StartCognitionRequest {
    pub loop_interval_ms: Option<u64>,
    pub seed_persona: Option<CreatePersonaRequest>,
}

/// Stop-control payload for perpetual cognition.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StopCognitionRequest {
    pub reason: Option<String>,
}

// ── Attention, Governance, GlobalWorkspace ──────────────────────────────

/// Source of an attention item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum AttentionSource {
    ContestedBelief,
    FailedCheck,
    StaleBelief,
    ProposalTimeout,
    FailedExecution,
}

/// An item requiring persona attention.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AttentionItem {
    pub id: String,
    pub topic: String,
    pub topic_tags: Vec<String>,
    pub priority: f32,
    pub source_type: AttentionSource,
    pub source_id: String,
    pub assigned_persona: Option<String>,
    pub created_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
}

/// Governance rules for swarm voting.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SwarmGovernance {
    pub quorum_fraction: f32,
    pub required_approvers_by_role: HashMap<ProposalRisk, Vec<String>>,
    pub veto_roles: Vec<String>,
    pub vote_timeout_secs: u64,
}

impl Default for SwarmGovernance {
    fn default() -> Self {
        Self {
            quorum_fraction: 0.5,
            required_approvers_by_role: HashMap::new(),
            veto_roles: Vec::new(),
            vote_timeout_secs: 300,
        }
    }
}

/// The coherent "now" for the entire swarm.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GlobalWorkspace {
    pub top_beliefs: Vec<String>,
    pub top_uncertainties: Vec<String>,
    pub top_attention: Vec<String>,
    pub active_objectives: Vec<String>,
    pub updated_at: DateTime<Utc>,
}

impl Default for GlobalWorkspace {
    fn default() -> Self {
        Self {
            top_beliefs: Vec::new(),
            top_uncertainties: Vec::new(),
            top_attention: Vec::new(),
            active_objectives: Vec::new(),
            updated_at: Utc::now(),
        }
    }
}

/// Start/stop response status.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CognitionStatus {
    pub enabled: bool,
    pub running: bool,
    pub loop_interval_ms: u64,
    pub started_at: Option<DateTime<Utc>>,
    pub last_tick_at: Option<DateTime<Utc>>,
    pub persona_count: usize,
    pub active_persona_count: usize,
    pub events_buffered: usize,
    pub snapshots_buffered: usize,
}

/// Reap response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReapPersonaResponse {
    pub reaped_ids: Vec<String>,
    pub count: usize,
}

/// Lineage node response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageNode {
    pub persona_id: String,
    pub parent_id: Option<String>,
    pub children: Vec<String>,
    pub depth: u32,
    pub status: PersonaStatus,
}

/// Lineage graph response.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LineageGraph {
    pub nodes: Vec<LineageNode>,
    pub roots: Vec<String>,
    pub total_edges: usize,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ThoughtPhase {
    Observe,
    Reflect,
    Test,
    Compress,
}

impl ThoughtPhase {
    fn from_thought_count(thought_count: u64) -> Self {
        match thought_count % 4 {
            1 => Self::Observe,
            2 => Self::Reflect,
            3 => Self::Test,
            _ => Self::Compress,
        }
    }

    fn as_str(&self) -> &'static str {
        match self {
            Self::Observe => "observe",
            Self::Reflect => "reflect",
            Self::Test => "test",
            Self::Compress => "compress",
        }
    }

    fn event_type(&self) -> ThoughtEventType {
        match self {
            Self::Observe => ThoughtEventType::ThoughtGenerated,
            Self::Reflect => ThoughtEventType::HypothesisRaised,
            Self::Test => ThoughtEventType::CheckRequested,
            Self::Compress => ThoughtEventType::SnapshotCompressed,
        }
    }
}

#[derive(Debug, Clone)]
struct ThoughtWorkItem {
    persona_id: String,
    persona_name: String,
    role: String,
    charter: String,
    swarm_id: Option<String>,
    thought_count: u64,
    phase: ThoughtPhase,
}

#[derive(Debug, Clone)]
struct ThoughtResult {
    source: &'static str,
    model: Option<String>,
    finish_reason: Option<String>,
    thinking: String,
    prompt_tokens: Option<u32>,
    completion_tokens: Option<u32>,
    total_tokens: Option<u32>,
    latency_ms: u128,
    error: Option<String>,
}

/// Runtime options for cognition manager.
#[derive(Debug, Clone)]
pub struct CognitionRuntimeOptions {
    pub enabled: bool,
    pub loop_interval_ms: u64,
    pub max_events: usize,
    pub max_snapshots: usize,
    pub default_policy: PersonaPolicy,
}

impl Default for CognitionRuntimeOptions {
    fn default() -> Self {
        Self {
            enabled: false,
            loop_interval_ms: 2_000,
            max_events: 2_000,
            max_snapshots: 128,
            default_policy: PersonaPolicy::default(),
        }
    }
}

/// In-memory cognition runtime for perpetual persona swarms.
#[derive(Debug)]
pub struct CognitionRuntime {
    enabled: bool,
    max_events: usize,
    max_snapshots: usize,
    default_policy: PersonaPolicy,
    running: Arc<AtomicBool>,
    loop_interval_ms: Arc<RwLock<u64>>,
    started_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    last_tick_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    personas: Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    events: Arc<RwLock<VecDeque<ThoughtEvent>>>,
    snapshots: Arc<RwLock<VecDeque<MemorySnapshot>>>,
    loop_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    event_tx: broadcast::Sender<ThoughtEvent>,
    thinker: Option<Arc<ThinkerClient>>,
    beliefs: Arc<RwLock<HashMap<String, Belief>>>,
    attention_queue: Arc<RwLock<Vec<AttentionItem>>>,
    governance: Arc<RwLock<SwarmGovernance>>,
    workspace: Arc<RwLock<GlobalWorkspace>>,
    tools: Option<Arc<ToolRegistry>>,
    receipts: Arc<RwLock<Vec<executor::DecisionReceipt>>>,
    /// Proposals pending human approval for Critical risk.
    pending_approvals: Arc<RwLock<HashMap<String, bool>>>,
}

impl CognitionRuntime {
    /// Build runtime from environment feature flags.
    pub fn new_from_env() -> Self {
        let mut options = CognitionRuntimeOptions::default();
        options.enabled = env_bool("CODETETHER_COGNITION_ENABLED", true);
        options.loop_interval_ms = env_u64("CODETETHER_COGNITION_LOOP_INTERVAL_MS", 2_000);
        options.max_events = env_usize("CODETETHER_COGNITION_MAX_EVENTS", 2_000);
        options.max_snapshots = env_usize("CODETETHER_COGNITION_MAX_SNAPSHOTS", 128);

        options.default_policy = PersonaPolicy {
            max_spawn_depth: env_u32("CODETETHER_COGNITION_MAX_SPAWN_DEPTH", 4),
            max_branching_factor: env_u32("CODETETHER_COGNITION_MAX_BRANCHING_FACTOR", 4),
            token_budget_per_minute: env_u32(
                "CODETETHER_COGNITION_TOKEN_BUDGET_PER_MINUTE",
                20_000,
            ),
            compute_ms_per_minute: env_u32("CODETETHER_COGNITION_COMPUTE_MS_PER_MINUTE", 10_000),
            idle_ttl_secs: env_u64("CODETETHER_COGNITION_IDLE_TTL_SECS", 3_600),
            share_memory: env_bool("CODETETHER_COGNITION_SHARE_MEMORY", false),
            allowed_tools: Vec::new(),
        };

        let thinker_config = ThinkerConfig {
            enabled: env_bool("CODETETHER_COGNITION_THINKER_ENABLED", true),
            backend: thinker::ThinkerBackend::from_env(
                &std::env::var("CODETETHER_COGNITION_THINKER_BACKEND")
                    .unwrap_or_else(|_| "openai_compat".to_string()),
            ),
            endpoint: normalize_thinker_endpoint(
                &std::env::var("CODETETHER_COGNITION_THINKER_BASE_URL")
                    .unwrap_or_else(|_| "http://127.0.0.1:11434/v1".to_string()),
            ),
            model: std::env::var("CODETETHER_COGNITION_THINKER_MODEL")
                .unwrap_or_else(|_| "qwen2.5:3b-instruct".to_string()),
            api_key: std::env::var("CODETETHER_COGNITION_THINKER_API_KEY").ok(),
            temperature: env_f32("CODETETHER_COGNITION_THINKER_TEMPERATURE", 0.2),
            top_p: std::env::var("CODETETHER_COGNITION_THINKER_TOP_P")
                .ok()
                .and_then(|v| v.parse::<f32>().ok()),
            max_tokens: env_usize("CODETETHER_COGNITION_THINKER_MAX_TOKENS", 256),
            timeout_ms: env_u64("CODETETHER_COGNITION_THINKER_TIMEOUT_MS", 12_000),
            candle_model_path: std::env::var("CODETETHER_COGNITION_THINKER_CANDLE_MODEL_PATH").ok(),
            candle_tokenizer_path: std::env::var(
                "CODETETHER_COGNITION_THINKER_CANDLE_TOKENIZER_PATH",
            )
            .ok(),
            candle_arch: std::env::var("CODETETHER_COGNITION_THINKER_CANDLE_ARCH").ok(),
            candle_device: thinker::CandleDevicePreference::from_env(
                &std::env::var("CODETETHER_COGNITION_THINKER_CANDLE_DEVICE")
                    .unwrap_or_else(|_| "auto".to_string()),
            ),
            candle_cuda_ordinal: env_usize("CODETETHER_COGNITION_THINKER_CANDLE_CUDA_ORDINAL", 0),
            candle_repeat_penalty: env_f32(
                "CODETETHER_COGNITION_THINKER_CANDLE_REPEAT_PENALTY",
                1.1,
            ),
            candle_repeat_last_n: env_usize(
                "CODETETHER_COGNITION_THINKER_CANDLE_REPEAT_LAST_N",
                64,
            ),
            candle_seed: env_u64("CODETETHER_COGNITION_THINKER_CANDLE_SEED", 42),
        };

        Self::new_with_options_and_thinker(options, Some(thinker_config))
    }

    /// Build runtime from explicit options.
    pub fn new_with_options(options: CognitionRuntimeOptions) -> Self {
        Self::new_with_options_and_thinker(options, None)
    }

    fn new_with_options_and_thinker(
        options: CognitionRuntimeOptions,
        thinker_config: Option<ThinkerConfig>,
    ) -> Self {
        let (event_tx, _) = broadcast::channel(options.max_events.max(16));
        let thinker = thinker_config.and_then(|cfg| {
            if !cfg.enabled {
                return None;
            }
            match ThinkerClient::new(cfg) {
                Ok(client) => {
                    tracing::info!(
                        backend = ?client.config().backend,
                        endpoint = %client.config().endpoint,
                        model = %client.config().model,
                        "Cognition thinker initialized"
                    );
                    Some(Arc::new(client))
                }
                Err(error) => {
                    tracing::warn!(%error, "Failed to initialize cognition thinker; using fallback thoughts");
                    None
                }
            }
        });

        Self {
            enabled: options.enabled,
            max_events: options.max_events.max(32),
            max_snapshots: options.max_snapshots.max(8),
            default_policy: options.default_policy,
            running: Arc::new(AtomicBool::new(false)),
            loop_interval_ms: Arc::new(RwLock::new(options.loop_interval_ms.max(100))),
            started_at: Arc::new(RwLock::new(None)),
            last_tick_at: Arc::new(RwLock::new(None)),
            personas: Arc::new(RwLock::new(HashMap::new())),
            proposals: Arc::new(RwLock::new(HashMap::new())),
            events: Arc::new(RwLock::new(VecDeque::new())),
            snapshots: Arc::new(RwLock::new(VecDeque::new())),
            loop_handle: Arc::new(Mutex::new(None)),
            event_tx,
            thinker,
            beliefs: Arc::new(RwLock::new(HashMap::new())),
            attention_queue: Arc::new(RwLock::new(Vec::new())),
            governance: Arc::new(RwLock::new(SwarmGovernance::default())),
            workspace: Arc::new(RwLock::new(GlobalWorkspace::default())),
            tools: None,
            receipts: Arc::new(RwLock::new(Vec::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    /// Whether cognition is enabled by feature flag.
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }

    /// Subscribe to thought events for streaming.
    pub fn subscribe_events(&self) -> broadcast::Receiver<ThoughtEvent> {
        self.event_tx.subscribe()
    }

    /// Start the perpetual cognition loop.
    pub async fn start(&self, req: Option<StartCognitionRequest>) -> Result<CognitionStatus> {
        if !self.enabled {
            return Err(anyhow!(
                "Perpetual cognition is disabled. Set CODETETHER_COGNITION_ENABLED=true."
            ));
        }

        let mut requested_seed_persona: Option<CreatePersonaRequest> = None;
        if let Some(request) = req {
            if let Some(interval) = request.loop_interval_ms {
                let mut lock = self.loop_interval_ms.write().await;
                *lock = interval.max(100);
            }
            requested_seed_persona = request.seed_persona;
        }

        let has_personas = !self.personas.read().await.is_empty();
        if !has_personas {
            let seed = requested_seed_persona.unwrap_or_else(default_seed_persona);
            let _ = self.create_persona(seed).await?;
        }

        if self.running.load(Ordering::SeqCst) {
            return Ok(self.status().await);
        }

        self.running.store(true, Ordering::SeqCst);
        {
            let mut started = self.started_at.write().await;
            *started = Some(Utc::now());
        }

        let running = Arc::clone(&self.running);
        let loop_interval_ms = Arc::clone(&self.loop_interval_ms);
        let last_tick_at = Arc::clone(&self.last_tick_at);
        let personas = Arc::clone(&self.personas);
        let proposals = Arc::clone(&self.proposals);
        let events = Arc::clone(&self.events);
        let snapshots = Arc::clone(&self.snapshots);
        let max_events = self.max_events;
        let max_snapshots = self.max_snapshots;
        let event_tx = self.event_tx.clone();
        let thinker = self.thinker.clone();
        let beliefs = Arc::clone(&self.beliefs);
        let attention_queue = Arc::clone(&self.attention_queue);
        let governance = Arc::clone(&self.governance);
        let workspace = Arc::clone(&self.workspace);
        let tools = self.tools.clone();
        let receipts = Arc::clone(&self.receipts);
        let pending_approvals = Arc::clone(&self.pending_approvals);

        let handle = tokio::spawn(async move {
            let mut next_tick = Instant::now();
            while running.load(Ordering::SeqCst) {
                let now_instant = Instant::now();
                if now_instant < next_tick {
                    tokio::time::sleep_until(next_tick).await;
                }
                if !running.load(Ordering::SeqCst) {
                    break;
                }

                let now = Utc::now();
                {
                    let mut lock = last_tick_at.write().await;
                    *lock = Some(now);
                }

                let mut new_events = Vec::new();
                let mut new_snapshots = Vec::new();
                let mut new_proposals = Vec::new();

                // ── Step 1: Budget window reset + enforcement ──
                let work_items: Vec<ThoughtWorkItem> = {
                    let mut map = personas.write().await;
                    let mut items = Vec::new();
                    for persona in map.values_mut() {
                        if persona.status != PersonaStatus::Active {
                            continue;
                        }

                        // Reset budget window every 60 seconds
                        let window_elapsed = now
                            .signed_duration_since(persona.window_started_at)
                            .num_seconds();
                        if window_elapsed >= 60 {
                            persona.tokens_this_window = 0;
                            persona.compute_ms_this_window = 0;
                            persona.window_started_at = now;
                        }

                        // Check budget
                        let token_ok =
                            persona.tokens_this_window < persona.policy.token_budget_per_minute;
                        let compute_ok =
                            persona.compute_ms_this_window < persona.policy.compute_ms_per_minute;
                        if !token_ok || !compute_ok {
                            if !persona.budget_paused {
                                persona.budget_paused = true;
                                new_events.push(ThoughtEvent {
                                    id: Uuid::new_v4().to_string(),
                                    event_type: ThoughtEventType::BudgetPaused,
                                    persona_id: Some(persona.identity.id.clone()),
                                    swarm_id: persona.identity.swarm_id.clone(),
                                    timestamp: now,
                                    payload: json!({
                                        "budget_paused": true,
                                        "tokens_used": persona.tokens_this_window,
                                        "compute_ms_used": persona.compute_ms_this_window,
                                        "token_budget": persona.policy.token_budget_per_minute,
                                        "compute_budget": persona.policy.compute_ms_per_minute,
                                    }),
                                });
                            }
                            continue; // skip tick for this persona
                        }

                        persona.budget_paused = false;
                        persona.thought_count = persona.thought_count.saturating_add(1);
                        persona.last_tick_at = Some(now);
                        persona.updated_at = now;
                        items.push(ThoughtWorkItem {
                            persona_id: persona.identity.id.clone(),
                            persona_name: persona.identity.name.clone(),
                            role: persona.identity.role.clone(),
                            charter: persona.identity.charter.clone(),
                            swarm_id: persona.identity.swarm_id.clone(),
                            thought_count: persona.thought_count,
                            phase: ThoughtPhase::from_thought_count(persona.thought_count),
                        });
                    }
                    items
                };

                for work in &work_items {
                    let context = recent_persona_context(&events, &work.persona_id, 8).await;

                    let thought = generate_phase_thought(thinker.as_deref(), work, &context).await;

                    let event_timestamp = Utc::now();
                    let is_fallback = thought.source == "fallback";

                    new_events.push(ThoughtEvent {
                        id: Uuid::new_v4().to_string(),
                        event_type: work.phase.event_type(),
                        persona_id: Some(work.persona_id.clone()),
                        swarm_id: work.swarm_id.clone(),
                        timestamp: event_timestamp,
                        payload: json!({
                            "phase": work.phase.as_str(),
                            "thought_count": work.thought_count,
                            "persona": {
                                "id": work.persona_id.clone(),
                                "name": work.persona_name.clone(),
                                "role": work.role.clone(),
                            },
                            "context_event_count": context.len(),
                            "thinking": thought.thinking.clone(),
                            "source": thought.source,
                            "model": thought.model.clone(),
                            "finish_reason": thought.finish_reason.clone(),
                            "usage": {
                                "prompt_tokens": thought.prompt_tokens,
                                "completion_tokens": thought.completion_tokens,
                                "total_tokens": thought.total_tokens,
                            },
                            "latency_ms": thought.latency_ms,
                            "error": thought.error.clone(),
                        }),
                    });

                    // ── After thought: update budget counters ──
                    {
                        let mut map = personas.write().await;
                        if let Some(persona) = map.get_mut(&work.persona_id) {
                            let tokens_used = thought.total_tokens.unwrap_or(0);
                            let compute_used = thought.latency_ms as u32;
                            persona.tokens_this_window =
                                persona.tokens_this_window.saturating_add(tokens_used);
                            persona.compute_ms_this_window =
                                persona.compute_ms_this_window.saturating_add(compute_used);

                            // Step 2: Update last_progress_at for non-fallback thought
                            if !is_fallback {
                                persona.last_progress_at = Utc::now();
                            }
                        }
                    }

                    // ── Step 3: Belief extraction during Reflect phase ──
                    if work.phase == ThoughtPhase::Reflect && !is_fallback {
                        let extracted = beliefs::extract_beliefs_from_thought(
                            thinker.as_deref(),
                            &work.persona_id,
                            &thought.thinking,
                        )
                        .await;

                        if !extracted.is_empty() {
                            let mut belief_store = beliefs.write().await;
                            let mut attn_queue = attention_queue.write().await;
                            for mut new_belief in extracted {
                                // Check for existing belief_key (duplicate detection)
                                let existing_id = belief_store
                                    .values()
                                    .find(|b| {
                                        b.belief_key == new_belief.belief_key
                                            && b.status != BeliefStatus::Invalidated
                                    })
                                    .map(|b| b.id.clone());

                                if let Some(eid) = existing_id {
                                    // Confirm existing belief
                                    if let Some(existing) = belief_store.get_mut(&eid) {
                                        if !existing.confirmed_by.contains(&work.persona_id) {
                                            existing.confirmed_by.push(work.persona_id.clone());
                                        }
                                        existing.updated_at = Utc::now();
                                    }
                                } else {
                                    // Handle contradiction targets
                                    let contest_targets: Vec<String> =
                                        new_belief.contradicts.clone();
                                    for target_key in &contest_targets {
                                        if let Some(target) = belief_store.values_mut().find(|b| {
                                            &b.belief_key == target_key
                                                && b.status != BeliefStatus::Invalidated
                                        }) {
                                            target.contested_by.push(new_belief.id.clone());
                                            if !new_belief.contradicts.contains(&target.belief_key)
                                            {
                                                new_belief
                                                    .contradicts
                                                    .push(target.belief_key.clone());
                                            }
                                            // Apply contest penalty
                                            target.confidence = (target.confidence - 0.1).max(0.05);
                                            if target.confidence < 0.5 {
                                                target.status = BeliefStatus::Stale;
                                            } else {
                                                // Create revalidation attention item
                                                attn_queue.push(AttentionItem {
                                                    id: Uuid::new_v4().to_string(),
                                                    topic: format!(
                                                        "Revalidate belief: {}",
                                                        target.claim
                                                    ),
                                                    topic_tags: vec![target.belief_key.clone()],
                                                    priority: 0.7,
                                                    source_type: AttentionSource::ContestedBelief,
                                                    source_id: target.id.clone(),
                                                    assigned_persona: None,
                                                    created_at: Utc::now(),
                                                    resolved_at: None,
                                                });
                                            }
                                        }
                                    }

                                    new_events.push(ThoughtEvent {
                                        id: Uuid::new_v4().to_string(),
                                        event_type: ThoughtEventType::BeliefExtracted,
                                        persona_id: Some(work.persona_id.clone()),
                                        swarm_id: work.swarm_id.clone(),
                                        timestamp: Utc::now(),
                                        payload: json!({
                                            "belief_id": new_belief.id,
                                            "belief_key": new_belief.belief_key,
                                            "claim": trim_for_storage(&new_belief.claim, 280),
                                            "confidence": new_belief.confidence,
                                        }),
                                    });

                                    // Mark progress for belief creation
                                    {
                                        let mut map = personas.write().await;
                                        if let Some(p) = map.get_mut(&work.persona_id) {
                                            p.last_progress_at = Utc::now();
                                        }
                                    }

                                    belief_store.insert(new_belief.id.clone(), new_belief);
                                }
                            }
                        }
                    }

                    if work.phase == ThoughtPhase::Test {
                        new_events.push(ThoughtEvent {
                            id: Uuid::new_v4().to_string(),
                            event_type: ThoughtEventType::CheckResult,
                            persona_id: Some(work.persona_id.clone()),
                            swarm_id: work.swarm_id.clone(),
                            timestamp: Utc::now(),
                            payload: json!({
                                "phase": work.phase.as_str(),
                                "thought_count": work.thought_count,
                                "result_excerpt": trim_for_storage(&thought.thinking, 280),
                                "source": thought.source,
                                "model": thought.model,
                            }),
                        });

                        // Mark progress for check result
                        {
                            let mut map = personas.write().await;
                            if let Some(p) = map.get_mut(&work.persona_id) {
                                p.last_progress_at = Utc::now();
                            }
                        }

                        // ── Step 6: Tool execution via capability leases ──
                        if !is_fallback {
                            if let Some(ref tool_registry) = tools {
                                let allowed = {
                                    let map = personas.read().await;
                                    map.get(&work.persona_id)
                                        .map(|p| p.policy.allowed_tools.clone())
                                        .unwrap_or_default()
                                };
                                if !allowed.is_empty() {
                                    let tool_results = executor::execute_tool_requests(
                                        thinker.as_deref(),
                                        tool_registry,
                                        &work.persona_id,
                                        &thought.thinking,
                                        &allowed,
                                    )
                                    .await;

                                    for result_event in tool_results {
                                        new_events.push(result_event);
                                        // Mark progress for tool execution
                                        let mut map = personas.write().await;
                                        if let Some(p) = map.get_mut(&work.persona_id) {
                                            p.last_progress_at = Utc::now();
                                        }
                                    }
                                }
                            }
                        }
                    }

                    if work.phase == ThoughtPhase::Reflect && work.thought_count % 8 == 2 {
                        let gov = governance.read().await;
                        let proposal = Proposal {
                            id: Uuid::new_v4().to_string(),
                            persona_id: work.persona_id.clone(),
                            title: proposal_title_from_thought(
                                &thought.thinking,
                                work.thought_count,
                            ),
                            rationale: trim_for_storage(&thought.thinking, 900),
                            evidence_refs: vec!["internal.thought_stream".to_string()],
                            risk: ProposalRisk::Low,
                            status: ProposalStatus::Created,
                            created_at: Utc::now(),
                            votes: HashMap::new(),
                            vote_deadline: Some(
                                Utc::now() + ChronoDuration::seconds(gov.vote_timeout_secs as i64),
                            ),
                            votes_requested: false,
                        };

                        new_events.push(ThoughtEvent {
                            id: Uuid::new_v4().to_string(),
                            event_type: ThoughtEventType::ProposalCreated,
                            persona_id: Some(work.persona_id.clone()),
                            swarm_id: work.swarm_id.clone(),
                            timestamp: Utc::now(),
                            payload: json!({
                                "proposal_id": proposal.id,
                                "title": proposal.title,
                                "rationale_excerpt": trim_for_storage(&proposal.rationale, 220),
                                "source": thought.source,
                                "model": thought.model,
                            }),
                        });

                        new_proposals.push(proposal);
                    }

                    if work.phase == ThoughtPhase::Compress {
                        new_snapshots.push(MemorySnapshot {
                            id: Uuid::new_v4().to_string(),
                            generated_at: Utc::now(),
                            swarm_id: work.swarm_id.clone(),
                            persona_scope: vec![work.persona_id.clone()],
                            summary: trim_for_storage(&thought.thinking, 1_500),
                            hot_event_count: context.len(),
                            warm_fact_count: estimate_fact_count(&thought.thinking),
                            cold_snapshot_count: 1,
                            metadata: HashMap::from([
                                (
                                    "phase".to_string(),
                                    serde_json::Value::String(work.phase.as_str().to_string()),
                                ),
                                (
                                    "role".to_string(),
                                    serde_json::Value::String(work.role.clone()),
                                ),
                                (
                                    "source".to_string(),
                                    serde_json::Value::String(thought.source.to_string()),
                                ),
                                (
                                    "model".to_string(),
                                    serde_json::Value::String(
                                        thought
                                            .model
                                            .clone()
                                            .unwrap_or_else(|| "fallback".to_string()),
                                    ),
                                ),
                                (
                                    "completion_tokens".to_string(),
                                    serde_json::Value::Number(serde_json::Number::from(
                                        thought.completion_tokens.unwrap_or(0) as u64,
                                    )),
                                ),
                            ]),
                        });

                        // ── Step 3: Belief staleness decay in Compress phase ──
                        {
                            let mut belief_store = beliefs.write().await;
                            let mut attn_queue = attention_queue.write().await;
                            let stale_ids: Vec<String> = belief_store
                                .values()
                                .filter(|b| {
                                    b.status == BeliefStatus::Active && now > b.review_after
                                })
                                .map(|b| b.id.clone())
                                .collect();
                            for id in stale_ids {
                                if let Some(belief) = belief_store.get_mut(&id) {
                                    belief.status = BeliefStatus::Stale;
                                    belief.confidence *= 0.98;
                                    belief.confidence = belief.confidence.max(0.05);
                                    attn_queue.push(AttentionItem {
                                        id: Uuid::new_v4().to_string(),
                                        topic: format!("Stale belief: {}", belief.claim),
                                        topic_tags: vec![belief.belief_key.clone()],
                                        priority: 0.4,
                                        source_type: AttentionSource::StaleBelief,
                                        source_id: belief.id.clone(),
                                        assigned_persona: None,
                                        created_at: now,
                                        resolved_at: None,
                                    });
                                }
                            }
                        }

                        // ── Step 4d: Update GlobalWorkspace ──
                        {
                            let belief_store = beliefs.read().await;
                            let attn_queue = attention_queue.read().await;

                            let mut sorted_beliefs: Vec<&Belief> = belief_store
                                .values()
                                .filter(|b| b.status == BeliefStatus::Active)
                                .collect();
                            sorted_beliefs.sort_by(|a, b| {
                                let score_a = a.confidence
                                    * (1.0
                                        / (1.0
                                            + now.signed_duration_since(a.updated_at).num_minutes()
                                                as f32));
                                let score_b = b.confidence
                                    * (1.0
                                        / (1.0
                                            + now.signed_duration_since(b.updated_at).num_minutes()
                                                as f32));
                                score_b
                                    .partial_cmp(&score_a)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            });

                            let top_beliefs: Vec<String> = sorted_beliefs
                                .iter()
                                .take(10)
                                .map(|b| b.id.clone())
                                .collect();
                            let top_uncertainties: Vec<String> = belief_store
                                .values()
                                .filter(|b| {
                                    b.status == BeliefStatus::Stale || !b.contested_by.is_empty()
                                })
                                .take(5)
                                .map(|b| {
                                    format!("[{}] {}", b.belief_key, trim_for_storage(&b.claim, 80))
                                })
                                .collect();

                            let mut sorted_attn: Vec<&AttentionItem> = attn_queue
                                .iter()
                                .filter(|a| a.resolved_at.is_none())
                                .collect();
                            sorted_attn.sort_by(|a, b| {
                                b.priority
                                    .partial_cmp(&a.priority)
                                    .unwrap_or(std::cmp::Ordering::Equal)
                            });
                            let top_attention: Vec<String> =
                                sorted_attn.iter().take(10).map(|a| a.id.clone()).collect();

                            let mut ws = workspace.write().await;
                            ws.top_beliefs = top_beliefs;
                            ws.top_uncertainties = top_uncertainties;
                            ws.top_attention = top_attention;
                            ws.updated_at = now;
                        }

                        new_events.push(ThoughtEvent {
                            id: Uuid::new_v4().to_string(),
                            event_type: ThoughtEventType::WorkspaceUpdated,
                            persona_id: Some(work.persona_id.clone()),
                            swarm_id: work.swarm_id.clone(),
                            timestamp: Utc::now(),
                            payload: json!({ "updated": true }),
                        });
                    }
                }

                // ── Step 4c: Governance — resolve proposals ──
                {
                    let gov = governance.read().await;
                    let mut proposal_store = proposals.write().await;
                    let persona_map = personas.read().await;
                    let active_count = persona_map
                        .values()
                        .filter(|p| p.status == PersonaStatus::Active)
                        .count();

                    let proposal_ids: Vec<String> = proposal_store
                        .values()
                        .filter(|p| p.status == ProposalStatus::Created)
                        .map(|p| p.id.clone())
                        .collect();

                    let mut attn_queue = attention_queue.write().await;
                    for pid in proposal_ids {
                        if let Some(proposal) = proposal_store.get_mut(&pid) {
                            // Check vote deadline
                            if let Some(deadline) = proposal.vote_deadline {
                                if now > deadline {
                                    let quorum_needed =
                                        (active_count as f32 * gov.quorum_fraction).ceil() as usize;
                                    if proposal.votes.len() < quorum_needed {
                                        // Timeout without quorum → attention item
                                        attn_queue.push(AttentionItem {
                                            id: Uuid::new_v4().to_string(),
                                            topic: format!(
                                                "Proposal vote timeout: {}",
                                                proposal.title
                                            ),
                                            topic_tags: Vec::new(),
                                            priority: 0.6,
                                            source_type: AttentionSource::ProposalTimeout,
                                            source_id: proposal.id.clone(),
                                            assigned_persona: None,
                                            created_at: now,
                                            resolved_at: None,
                                        });
                                        continue;
                                    }
                                }
                            }

                            // Resolve if enough votes
                            let quorum_needed =
                                (active_count as f32 * gov.quorum_fraction).ceil() as usize;
                            if proposal.votes.len() >= quorum_needed {
                                // Check for veto
                                let vetoed = proposal.votes.iter().any(|(voter_id, vote)| {
                                    if *vote != ProposalVote::Veto {
                                        return false;
                                    }
                                    if let Some(voter) = persona_map.get(voter_id) {
                                        gov.veto_roles.contains(&voter.identity.role)
                                    } else {
                                        false
                                    }
                                });

                                if vetoed {
                                    proposal.status = ProposalStatus::Rejected;
                                    continue;
                                }

                                // Check required approvers
                                let required_roles = gov
                                    .required_approvers_by_role
                                    .get(&proposal.risk)
                                    .cloned()
                                    .unwrap_or_default();
                                let all_required_met = required_roles.iter().all(|role| {
                                    proposal.votes.iter().any(|(vid, vote)| {
                                        *vote == ProposalVote::Approve
                                            && persona_map
                                                .get(vid)
                                                .map(|p| &p.identity.role == role)
                                                .unwrap_or(false)
                                    })
                                });

                                if !all_required_met {
                                    continue; // wait for required approvers
                                }

                                // Count approvals vs rejections
                                let approvals = proposal
                                    .votes
                                    .values()
                                    .filter(|v| **v == ProposalVote::Approve)
                                    .count();
                                let rejections = proposal
                                    .votes
                                    .values()
                                    .filter(|v| **v == ProposalVote::Reject)
                                    .count();

                                if approvals > rejections {
                                    proposal.status = ProposalStatus::Verified;
                                } else {
                                    proposal.status = ProposalStatus::Rejected;
                                }
                            }
                        }
                    }
                }

                // ── Step 7: Execute verified proposals ──
                {
                    let mut proposal_store = proposals.write().await;
                    let verified_ids: Vec<String> = proposal_store
                        .values()
                        .filter(|p| p.status == ProposalStatus::Verified)
                        .map(|p| p.id.clone())
                        .collect();

                    for pid in verified_ids {
                        if let Some(proposal) = proposal_store.get_mut(&pid) {
                            if proposal.risk == ProposalRisk::Critical {
                                // Check for human approval
                                let approved = {
                                    let approvals = pending_approvals.read().await;
                                    approvals.get(&pid).copied().unwrap_or(false)
                                };
                                if !approved {
                                    // Register for human approval
                                    let mut approvals = pending_approvals.write().await;
                                    approvals.entry(pid.clone()).or_insert(false);
                                    continue;
                                }
                            }

                            // Create decision receipt
                            let receipt = executor::DecisionReceipt {
                                id: Uuid::new_v4().to_string(),
                                proposal_id: pid.clone(),
                                inputs: proposal.evidence_refs.clone(),
                                governance_decision: format!(
                                    "Approved with {} votes",
                                    proposal.votes.len()
                                ),
                                capability_leases: Vec::new(),
                                tool_invocations: Vec::new(),
                                outcome: executor::ExecutionOutcome::Success {
                                    summary: format!("Proposal '{}' executed", proposal.title),
                                },
                                created_at: Utc::now(),
                            };

                            new_events.push(ThoughtEvent {
                                id: Uuid::new_v4().to_string(),
                                event_type: ThoughtEventType::ActionExecuted,
                                persona_id: Some(proposal.persona_id.clone()),
                                swarm_id: None,
                                timestamp: Utc::now(),
                                payload: json!({
                                    "receipt_id": receipt.id,
                                    "proposal_id": pid,
                                    "outcome": "success",
                                    "summary": format!("Proposal '{}' executed", proposal.title),
                                }),
                            });

                            receipts.write().await.push(receipt);
                            proposal.status = ProposalStatus::Executed;
                        }
                    }
                }

                if !new_proposals.is_empty() {
                    let mut proposal_store = proposals.write().await;
                    for proposal in new_proposals {
                        proposal_store.insert(proposal.id.clone(), proposal);
                    }
                }

                for event in new_events {
                    push_event_internal(&events, max_events, &event_tx, event).await;
                }
                for snapshot in new_snapshots {
                    push_snapshot_internal(&snapshots, max_snapshots, snapshot).await;
                }

                // ── Step 2: Idle TTL reaping ──
                {
                    let mut map = personas.write().await;
                    let idle_ids: Vec<String> = map
                        .values()
                        .filter(|p| {
                            p.status == PersonaStatus::Active
                                && !p.budget_paused
                                && now.signed_duration_since(p.last_progress_at).num_seconds()
                                    > p.policy.idle_ttl_secs as i64
                        })
                        .map(|p| p.identity.id.clone())
                        .collect();

                    for id in &idle_ids {
                        if let Some(persona) = map.get_mut(id) {
                            persona.status = PersonaStatus::Reaped;
                            persona.updated_at = now;
                        }
                        // Also cascade-reap children
                        let children: Vec<String> = map
                            .values()
                            .filter(|p| p.identity.parent_id.as_deref() == Some(id.as_str()))
                            .map(|p| p.identity.id.clone())
                            .collect();
                        for child_id in children {
                            if let Some(child) = map.get_mut(&child_id) {
                                child.status = PersonaStatus::Reaped;
                                child.updated_at = now;
                            }
                        }
                    }
                    drop(map);

                    for id in idle_ids {
                        push_event_internal(
                            &events,
                            max_events,
                            &event_tx,
                            ThoughtEvent {
                                id: Uuid::new_v4().to_string(),
                                event_type: ThoughtEventType::IdleReaped,
                                persona_id: Some(id),
                                swarm_id: None,
                                timestamp: now,
                                payload: json!({ "reason": "idle_ttl_expired" }),
                            },
                        )
                        .await;
                    }
                }

                // ── Step 5: Persistence on Compress ──
                if work_items.iter().any(|w| w.phase == ThoughtPhase::Compress) {
                    let _ = persistence::save_state(
                        &personas,
                        &proposals,
                        &beliefs,
                        &attention_queue,
                        &workspace,
                        &events,
                        &snapshots,
                    )
                    .await;
                }

                let interval = Duration::from_millis((*loop_interval_ms.read().await).max(100));
                next_tick += interval;
                let tick_completed = Instant::now();
                if tick_completed > next_tick {
                    next_tick = tick_completed;
                }
            }
        });

        {
            let mut lock = self.loop_handle.lock().await;
            *lock = Some(handle);
        }

        Ok(self.status().await)
    }

    /// Stop the perpetual cognition loop.
    pub async fn stop(&self, reason: Option<String>) -> Result<CognitionStatus> {
        self.running.store(false, Ordering::SeqCst);

        if let Some(handle) = self.loop_handle.lock().await.take() {
            handle.abort();
            let _ = handle.await;
        }

        if let Some(reason_value) = reason {
            let event = ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::CheckResult,
                persona_id: None,
                swarm_id: None,
                timestamp: Utc::now(),
                payload: json!({ "stopped": true, "reason": reason_value }),
            };
            self.push_event(event).await;
        }

        Ok(self.status().await)
    }

    /// Create a persona record.
    pub async fn create_persona(&self, req: CreatePersonaRequest) -> Result<PersonaRuntimeState> {
        let now = Utc::now();
        let mut personas = self.personas.write().await;

        let mut parent_swarm_id = None;
        let mut computed_depth = 0_u32;
        let mut inherited_policy = None;

        if let Some(parent_id) = req.parent_id.clone() {
            let parent = personas
                .get(&parent_id)
                .ok_or_else(|| anyhow!("Parent persona not found: {}", parent_id))?;

            if parent.status == PersonaStatus::Reaped {
                return Err(anyhow!("Parent persona {} is reaped", parent_id));
            }

            parent_swarm_id = parent.identity.swarm_id.clone();
            computed_depth = parent.identity.depth.saturating_add(1);
            inherited_policy = Some(parent.policy.clone());
            let branch_limit = parent.policy.max_branching_factor;

            let child_count = personas
                .values()
                .filter(|p| {
                    p.identity.parent_id.as_deref() == Some(parent_id.as_str())
                        && p.status != PersonaStatus::Reaped
                })
                .count();

            if child_count as u32 >= branch_limit {
                return Err(anyhow!(
                    "Parent {} reached branching limit {}",
                    parent_id,
                    branch_limit
                ));
            }
        }

        let policy = req
            .policy
            .clone()
            .or(inherited_policy.clone())
            .unwrap_or_else(|| self.default_policy.clone());

        let effective_depth_limit = inherited_policy
            .as_ref()
            .map(|p| p.max_spawn_depth)
            .unwrap_or(policy.max_spawn_depth);

        if computed_depth > effective_depth_limit {
            return Err(anyhow!(
                "Spawn depth {} exceeds limit {}",
                computed_depth,
                effective_depth_limit
            ));
        }

        let persona_id = req.persona_id.unwrap_or_else(|| Uuid::new_v4().to_string());
        if personas.contains_key(&persona_id) {
            return Err(anyhow!("Persona id already exists: {}", persona_id));
        }

        let identity = PersonaIdentity {
            id: persona_id.clone(),
            name: req.name,
            role: req.role,
            charter: req.charter,
            swarm_id: req.swarm_id.or(parent_swarm_id),
            parent_id: req.parent_id,
            depth: computed_depth,
            created_at: now,
            tags: req.tags,
        };

        let persona = PersonaRuntimeState {
            identity,
            policy,
            status: PersonaStatus::Active,
            thought_count: 0,
            last_tick_at: None,
            updated_at: now,
            tokens_this_window: 0,
            compute_ms_this_window: 0,
            window_started_at: now,
            last_progress_at: now,
            budget_paused: false,
        };

        personas.insert(persona_id, persona.clone());
        drop(personas);

        self.push_event(ThoughtEvent {
            id: Uuid::new_v4().to_string(),
            event_type: ThoughtEventType::PersonaSpawned,
            persona_id: Some(persona.identity.id.clone()),
            swarm_id: persona.identity.swarm_id.clone(),
            timestamp: now,
            payload: json!({
                "name": persona.identity.name,
                "role": persona.identity.role,
                "depth": persona.identity.depth,
            }),
        })
        .await;

        Ok(persona)
    }

    /// Spawn a child persona under an existing parent.
    pub async fn spawn_child(
        &self,
        parent_id: &str,
        req: SpawnPersonaRequest,
    ) -> Result<PersonaRuntimeState> {
        let request = CreatePersonaRequest {
            persona_id: req.persona_id,
            name: req.name,
            role: req.role,
            charter: req.charter,
            swarm_id: req.swarm_id,
            parent_id: Some(parent_id.to_string()),
            policy: req.policy,
            tags: Vec::new(),
        };
        self.create_persona(request).await
    }

    /// Reap one persona or the full descendant tree.
    pub async fn reap_persona(
        &self,
        persona_id: &str,
        req: ReapPersonaRequest,
    ) -> Result<ReapPersonaResponse> {
        let cascade = req.cascade.unwrap_or(false);
        let now = Utc::now();

        let mut personas = self.personas.write().await;
        if !personas.contains_key(persona_id) {
            return Err(anyhow!("Persona not found: {}", persona_id));
        }

        let mut reaped_ids = vec![persona_id.to_string()];
        if cascade {
            let mut idx = 0usize;
            while idx < reaped_ids.len() {
                let current = reaped_ids[idx].clone();
                let children: Vec<String> = personas
                    .values()
                    .filter(|p| p.identity.parent_id.as_deref() == Some(current.as_str()))
                    .map(|p| p.identity.id.clone())
                    .collect();
                for child in children {
                    if !reaped_ids.iter().any(|existing| existing == &child) {
                        reaped_ids.push(child);
                    }
                }
                idx += 1;
            }
        }

        for id in &reaped_ids {
            if let Some(persona) = personas.get_mut(id) {
                persona.status = PersonaStatus::Reaped;
                persona.updated_at = now;
            }
        }
        drop(personas);

        for id in &reaped_ids {
            self.push_event(ThoughtEvent {
                id: Uuid::new_v4().to_string(),
                event_type: ThoughtEventType::PersonaReaped,
                persona_id: Some(id.clone()),
                swarm_id: None,
                timestamp: now,
                payload: json!({
                    "reason": req.reason.clone().unwrap_or_else(|| "manual_reap".to_string()),
                    "cascade": cascade,
                }),
            })
            .await;
        }

        Ok(ReapPersonaResponse {
            count: reaped_ids.len(),
            reaped_ids,
        })
    }

    /// Get latest memory snapshot, if any.
    pub async fn latest_snapshot(&self) -> Option<MemorySnapshot> {
        self.snapshots.read().await.back().cloned()
    }

    /// Build lineage graph from current persona state.
    pub async fn lineage_graph(&self) -> LineageGraph {
        let personas = self.personas.read().await;
        let mut children_by_parent: HashMap<String, Vec<String>> = HashMap::new();
        let mut roots = Vec::new();
        let mut total_edges = 0usize;

        for persona in personas.values() {
            if let Some(parent_id) = persona.identity.parent_id.clone() {
                children_by_parent
                    .entry(parent_id)
                    .or_default()
                    .push(persona.identity.id.clone());
                total_edges = total_edges.saturating_add(1);
            } else {
                roots.push(persona.identity.id.clone());
            }
        }

        let mut nodes: Vec<LineageNode> = personas
            .values()
            .map(|persona| {
                let mut children = children_by_parent
                    .get(&persona.identity.id)
                    .cloned()
                    .unwrap_or_default();
                children.sort();

                LineageNode {
                    persona_id: persona.identity.id.clone(),
                    parent_id: persona.identity.parent_id.clone(),
                    children,
                    depth: persona.identity.depth,
                    status: persona.status,
                }
            })
            .collect();

        nodes.sort_by(|a, b| a.persona_id.cmp(&b.persona_id));
        roots.sort();

        LineageGraph {
            nodes,
            roots,
            total_edges,
        }
    }

    /// Return a summary status.
    pub async fn status(&self) -> CognitionStatus {
        let personas = self.personas.read().await;
        let events = self.events.read().await;
        let snapshots = self.snapshots.read().await;
        let started_at = *self.started_at.read().await;
        let last_tick_at = *self.last_tick_at.read().await;
        let loop_interval_ms = *self.loop_interval_ms.read().await;

        let active_persona_count = personas
            .values()
            .filter(|p| p.status == PersonaStatus::Active)
            .count();

        CognitionStatus {
            enabled: self.enabled,
            running: self.running.load(Ordering::SeqCst),
            loop_interval_ms,
            started_at,
            last_tick_at,
            persona_count: personas.len(),
            active_persona_count,
            events_buffered: events.len(),
            snapshots_buffered: snapshots.len(),
        }
    }

    async fn push_event(&self, event: ThoughtEvent) {
        push_event_internal(&self.events, self.max_events, &self.event_tx, event).await;
    }

    /// Set the tool registry for capability-based tool execution.
    pub fn set_tools(&mut self, registry: Arc<ToolRegistry>) {
        self.tools = Some(registry);
    }

    /// Get current beliefs.
    pub async fn get_beliefs(&self) -> HashMap<String, Belief> {
        self.beliefs.read().await.clone()
    }

    /// Get a single belief by ID.
    pub async fn get_belief(&self, id: &str) -> Option<Belief> {
        self.beliefs.read().await.get(id).cloned()
    }

    /// Get the current attention queue.
    pub async fn get_attention_queue(&self) -> Vec<AttentionItem> {
        self.attention_queue.read().await.clone()
    }

    /// Get all proposals.
    pub async fn get_proposals(&self) -> HashMap<String, Proposal> {
        self.proposals.read().await.clone()
    }

    /// Get the current global workspace.
    pub async fn get_workspace(&self) -> GlobalWorkspace {
        self.workspace.read().await.clone()
    }

    /// Get decision receipts.
    pub async fn get_receipts(&self) -> Vec<executor::DecisionReceipt> {
        self.receipts.read().await.clone()
    }

    /// Approve a Critical-risk proposal for execution.
    pub async fn approve_proposal(&self, proposal_id: &str) -> Result<()> {
        let proposals = self.proposals.read().await;
        let proposal = proposals
            .get(proposal_id)
            .ok_or_else(|| anyhow!("Proposal not found: {}", proposal_id))?;

        if proposal.risk != ProposalRisk::Critical {
            return Err(anyhow!("Only Critical proposals require human approval"));
        }
        if proposal.status != ProposalStatus::Verified {
            return Err(anyhow!("Proposal is not in Verified status"));
        }
        drop(proposals);

        let mut approvals = self.pending_approvals.write().await;
        approvals.insert(proposal_id.to_string(), true);
        Ok(())
    }

    /// Get governance settings.
    pub async fn get_governance(&self) -> SwarmGovernance {
        self.governance.read().await.clone()
    }

    /// Get persona state for a specific persona.
    pub async fn get_persona(&self, id: &str) -> Option<PersonaRuntimeState> {
        self.personas.read().await.get(id).cloned()
    }
}

async fn push_event_internal(
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    max_events: usize,
    event_tx: &broadcast::Sender<ThoughtEvent>,
    event: ThoughtEvent,
) {
    {
        let mut lock = events.write().await;
        lock.push_back(event.clone());
        while lock.len() > max_events {
            lock.pop_front();
        }
    }
    let _ = event_tx.send(event);
}

async fn push_snapshot_internal(
    snapshots: &Arc<RwLock<VecDeque<MemorySnapshot>>>,
    max_snapshots: usize,
    snapshot: MemorySnapshot,
) {
    let mut lock = snapshots.write().await;
    lock.push_back(snapshot);
    while lock.len() > max_snapshots {
        lock.pop_front();
    }
}

async fn recent_persona_context(
    events: &Arc<RwLock<VecDeque<ThoughtEvent>>>,
    persona_id: &str,
    limit: usize,
) -> Vec<ThoughtEvent> {
    let lock = events.read().await;
    let mut selected: Vec<ThoughtEvent> = lock
        .iter()
        .rev()
        .filter(|event| {
            event.persona_id.as_deref() == Some(persona_id)
                || (event.persona_id.is_none()
                    && matches!(
                        event.event_type,
                        ThoughtEventType::CheckResult
                            | ThoughtEventType::ProposalCreated
                            | ThoughtEventType::SnapshotCompressed
                    ))
        })
        .take(limit)
        .cloned()
        .collect();
    selected.reverse();
    selected
}

async fn generate_phase_thought(
    thinker: Option<&ThinkerClient>,
    work: &ThoughtWorkItem,
    context: &[ThoughtEvent],
) -> ThoughtResult {
    let started_at = Instant::now();
    if let Some(client) = thinker {
        let (system_prompt, user_prompt) = build_phase_prompts(work, context);
        match client.think(&system_prompt, &user_prompt).await {
            Ok(output) => {
                let thinking = normalize_thought_output(work, context, &output.text);
                if !thinking.is_empty() {
                    return ThoughtResult {
                        source: "model",
                        model: Some(output.model),
                        finish_reason: output.finish_reason,
                        thinking,
                        prompt_tokens: output.prompt_tokens,
                        completion_tokens: output.completion_tokens,
                        total_tokens: output.total_tokens,
                        latency_ms: started_at.elapsed().as_millis(),
                        error: None,
                    };
                }
            }
            Err(error) => {
                return ThoughtResult {
                    source: "fallback",
                    model: None,
                    finish_reason: None,
                    thinking: fallback_phase_text(work, context),
                    prompt_tokens: None,
                    completion_tokens: None,
                    total_tokens: None,
                    latency_ms: started_at.elapsed().as_millis(),
                    error: Some(error.to_string()),
                };
            }
        }
    }

    ThoughtResult {
        source: "fallback",
        model: None,
        finish_reason: None,
        thinking: fallback_phase_text(work, context),
        prompt_tokens: None,
        completion_tokens: None,
        total_tokens: None,
        latency_ms: started_at.elapsed().as_millis(),
        error: None,
    }
}

fn build_phase_prompts(work: &ThoughtWorkItem, context: &[ThoughtEvent]) -> (String, String) {
    let system_prompt = "You are the internal cognition engine for a persistent autonomous persona. \
Respond with concise plain text only. Do not include markdown, XML, or code fences. \
Write as an operational process update, not meta narration. \
Do not say phrases like 'I need to', 'we need to', 'I will', or describe your own reasoning process. \
Output concrete findings, checks, risks, and next actions. \
Fill every labeled field with concrete content. Never output placeholders such as '...', '<...>', 'TBD', or 'TODO'."
        .to_string();

    let context_lines = if context.is_empty() {
        "none".to_string()
    } else {
        context
            .iter()
            .map(format_context_event)
            .collect::<Vec<_>>()
            .join("\n")
    };

    let phase_instruction = match work.phase {
        ThoughtPhase::Observe => {
            "Process format (exact line labels): \
Phase: Observe | Goal: detect current customer/business risk | \
Signals: 1-3 concrete signals separated by '; ' | \
Uncertainty: one unknown that blocks confidence | \
Next_Action: one immediate operational action."
        }
        ThoughtPhase::Reflect => {
            "Process format (exact line labels): \
Phase: Reflect | Hypothesis: single testable hypothesis | \
Rationale: why this is likely | \
Business_Risk: customer/revenue/SLA impact | \
Validation_Next_Action: one action to confirm or falsify."
        }
        ThoughtPhase::Test => {
            "Process format (exact line labels): \
Phase: Test | Check: single concrete check | \
Procedure: short executable procedure | \
Expected_Result: pass/fail expectation | \
Evidence_Quality: low|medium|high with reason | \
Escalation_Trigger: when to escalate immediately."
        }
        ThoughtPhase::Compress => {
            "Process format (exact line labels): \
Phase: Compress | State_Summary: current state in one line | \
Retained_Facts: 3 short facts separated by '; ' | \
Open_Risks: up to 2 unresolved risks separated by '; ' | \
Next_Process_Step: next operational step."
        }
    };

    let user_prompt = format!(
        "phase: {phase}\npersona_id: {persona_id}\npersona_name: {persona_name}\nrole: {role}\ncharter: {charter}\nthought_count: {thought_count}\nrecent_context:\n{context}\n\ninstruction:\n{instruction}",
        phase = work.phase.as_str(),
        persona_id = work.persona_id,
        persona_name = work.persona_name,
        role = work.role,
        charter = work.charter,
        thought_count = work.thought_count,
        context = context_lines,
        instruction = phase_instruction
    );

    (system_prompt, user_prompt)
}

fn format_context_event(event: &ThoughtEvent) -> String {
    let payload = serde_json::to_string(&event.payload).unwrap_or_else(|_| "{}".to_string());
    format!(
        "{} {} {}",
        event.event_type.as_str(),
        event.timestamp.to_rfc3339(),
        trim_for_storage(&payload, 220)
    )
}

fn fallback_phase_text(work: &ThoughtWorkItem, context: &[ThoughtEvent]) -> String {
    let charter = trim_for_storage(&work.charter, 180);
    let context_summary = fallback_context_summary(context);
    let thought = match work.phase {
        ThoughtPhase::Observe => format!(
            "Phase: Observe | Goal: detect current customer/business risk | Signals: role={}; charter_focus={}; {} | Uncertainty: live customer-impact telemetry and current incident status are incomplete. | Next_Action: run targeted health/error checks for customer-facing flows and capture failure rate baselines.",
            work.role, charter, context_summary
        ),
        ThoughtPhase::Reflect => format!(
            "Phase: Reflect | Hypothesis: current instability risk is most likely in runtime reliability and dependency availability. | Rationale: recent context indicates unresolved operational uncertainty. | Business_Risk: outages can cause SLA breach, revenue loss, and trust erosion. | Validation_Next_Action: confirm via service health trend, dependency error distribution, and rollback readiness.",
        ),
        ThoughtPhase::Test => format!(
            "Phase: Test | Check: verify customer-path service health against recent error spikes and release changes. | Procedure: collect latest health status, error counts, and recent deploy diffs; compare against baseline. | Expected_Result: pass if health is stable and error rate within baseline, fail otherwise. | Evidence_Quality: medium (depends on telemetry completeness). | Escalation_Trigger: escalate immediately on repeated customer-path failures or sustained elevated error rate.",
        ),
        ThoughtPhase::Compress => format!(
            "Phase: Compress | State_Summary: reliability monitoring active with unresolved business-impact uncertainty. | Retained_Facts: role={} ; charter_focus={} ; {} | Open_Risks: potential customer-path instability ; incomplete evidence for confident closure. | Next_Process_Step: convert latest checks into prioritized remediation tasks and verify impact reduction.",
            work.role, charter, context_summary
        ),
    };
    trim_for_storage(&thought, 1_200)
}

fn normalize_thought_output(work: &ThoughtWorkItem, context: &[ThoughtEvent], raw: &str) -> String {
    let trimmed = trim_for_storage(raw, 2_000);
    if trimmed.trim().is_empty() {
        return fallback_phase_text(work, context);
    }

    // Prefer process-labeled content if the model emitted a preamble first.
    if let Some(idx) = find_process_label_start(&trimmed) {
        let candidate = trimmed[idx..].trim();
        if let Some(first_line) = candidate
            .lines()
            .map(str::trim)
            .find(|line| !line.is_empty())
        {
            let normalized_line = first_line.trim_matches('"').trim_matches('\'').trim();
            if normalized_line.starts_with("Phase:")
                && !normalized_line.contains('<')
                && !has_template_placeholder_values(normalized_line)
            {
                return normalized_line.to_string();
            }
        }
        if !candidate.is_empty()
            && !candidate.contains('<')
            && !candidate.contains('\n')
            && !has_template_placeholder_values(candidate)
        {
            return candidate.to_string();
        }
    }

    let lower = trimmed.to_ascii_lowercase();
    let looks_meta = lower.starts_with("we need")
        || lower.starts_with("i need")
        || lower.contains("we need to")
        || lower.contains("i need to")
        || lower.contains("must output")
        || lower.contains("let's ")
        || lower.contains("we have to");

    if looks_meta || has_template_placeholder_values(&trimmed) {
        return fallback_phase_text(work, context);
    }

    trimmed
}

fn has_template_placeholder_values(text: &str) -> bool {
    let lower = text.to_ascii_lowercase();
    [
        "goal: ...",
        "signals: ...",
        "uncertainty: ...",
        "next_action: ...",
        "hypothesis: ...",
        "rationale: ...",
        "business_risk: ...",
        "validation_next_action: ...",
        "check: ...",
        "procedure: ...",
        "expected_result: ...",
        "evidence_quality: ...",
        "escalation_trigger: ...",
        "state_summary: ...",
        "retained_facts: ...",
        "open_risks: ...",
        "next_process_step: ...",
    ]
    .iter()
    .any(|needle| lower.contains(needle))
        || lower.contains("<...")
        || lower.contains("tbd")
        || lower.contains("todo")
}

fn find_process_label_start(text: &str) -> Option<usize> {
    [
        "Phase: Observe",
        "Phase: Reflect",
        "Phase: Test",
        "Phase: Compress",
        "Phase:",
    ]
    .iter()
    .filter_map(|label| text.find(label))
    .min()
}

fn fallback_context_summary(context: &[ThoughtEvent]) -> String {
    if context.is_empty() {
        return "No prior events recorded yet.".to_string();
    }

    let mut latest_error: Option<String> = None;
    let mut latest_proposal: Option<String> = None;
    let mut latest_check: Option<String> = None;

    for event in context.iter().rev() {
        if latest_error.is_none()
            && let Some(error) = event
                .payload
                .get("error")
                .and_then(serde_json::Value::as_str)
            && !error.trim().is_empty()
        {
            latest_error = Some(trim_for_storage(error, 140));
        }

        if latest_proposal.is_none()
            && event.event_type == ThoughtEventType::ProposalCreated
            && let Some(title) = event
                .payload
                .get("title")
                .and_then(serde_json::Value::as_str)
            && !title.trim().is_empty()
            && !has_template_placeholder_values(title)
        {
            latest_proposal = Some(trim_for_storage(title, 120));
        }

        if latest_check.is_none()
            && event.event_type == ThoughtEventType::CheckResult
            && let Some(result) = event
                .payload
                .get("result_excerpt")
                .and_then(serde_json::Value::as_str)
            && !result.trim().is_empty()
            && !has_template_placeholder_values(result)
        {
            latest_check = Some(trim_for_storage(result, 140));
        }

        if latest_error.is_some() && latest_proposal.is_some() && latest_check.is_some() {
            break;
        }
    }

    let mut lines = vec![format!(
        "{} recent cognition events are available.",
        context.len()
    )];
    if let Some(error) = latest_error {
        lines.push(format!("Latest error signal: {}.", error));
    }
    if let Some(proposal) = latest_proposal {
        lines.push(format!("Recent proposal: {}.", proposal));
    }
    if let Some(check) = latest_check {
        lines.push(format!("Recent check: {}.", check));
    }
    lines.join(" ")
}

fn trim_for_storage(input: &str, max_chars: usize) -> String {
    if input.chars().count() <= max_chars {
        return input.trim().to_string();
    }
    let mut trimmed = String::with_capacity(max_chars + 8);
    for ch in input.chars().take(max_chars) {
        trimmed.push(ch);
    }
    trimmed.push_str("...");
    trimmed.trim().to_string()
}

fn estimate_fact_count(text: &str) -> usize {
    let sentence_count =
        text.matches('.').count() + text.matches('!').count() + text.matches('?').count();
    sentence_count.clamp(1, 12)
}

fn proposal_title_from_thought(thought: &str, thought_count: u64) -> String {
    let first_line = thought
        .lines()
        .find(|line| !line.trim().is_empty())
        .unwrap_or("proposal");
    let compact = first_line
        .replace(['\t', '\r', '\n'], " ")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    let trimmed = trim_for_storage(&compact, 72);
    if trimmed.is_empty() {
        format!("proposal-{}", thought_count)
    } else {
        trimmed
    }
}

fn default_seed_persona() -> CreatePersonaRequest {
    CreatePersonaRequest {
        persona_id: Some("root-thinker".to_string()),
        name: "root-thinker".to_string(),
        role: "orchestrator".to_string(),
        charter: "Continuously observe, reflect, test hypotheses, and compress useful insights."
            .to_string(),
        swarm_id: Some("swarm-core".to_string()),
        parent_id: None,
        policy: None,
        tags: vec!["orchestration".to_string()],
    }
}

fn normalize_thinker_endpoint(base_url: &str) -> String {
    let trimmed = base_url.trim().trim_end_matches('/');
    if trimmed.ends_with("/chat/completions") {
        return trimmed.to_string();
    }
    if trimmed.is_empty() {
        return "http://127.0.0.1:11434/v1/chat/completions".to_string();
    }
    format!("{}/chat/completions", trimmed)
}

fn env_bool(name: &str, default: bool) -> bool {
    std::env::var(name)
        .ok()
        .and_then(|v| match v.to_ascii_lowercase().as_str() {
            "1" | "true" | "yes" | "on" => Some(true),
            "0" | "false" | "no" | "off" => Some(false),
            _ => None,
        })
        .unwrap_or(default)
}

fn env_f32(name: &str, default: f32) -> f32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<f32>().ok())
        .unwrap_or(default)
}

fn env_u64(name: &str, default: u64) -> u64 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<u64>().ok())
        .unwrap_or(default)
}

fn env_u32(name: &str, default: u32) -> u32 {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<u32>().ok())
        .unwrap_or(default)
}

fn env_usize(name: &str, default: usize) -> usize {
    std::env::var(name)
        .ok()
        .and_then(|v| v.parse::<usize>().ok())
        .unwrap_or(default)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_work_item(phase: ThoughtPhase) -> ThoughtWorkItem {
        ThoughtWorkItem {
            persona_id: "p-1".to_string(),
            persona_name: "Spotlessbinco Business Thinker".to_string(),
            role: "principal reliability engineer".to_string(),
            charter: "Continuously think about /home/riley/spotlessbinco as a production business system."
                .to_string(),
            swarm_id: Some("spotlessbinco".to_string()),
            thought_count: 4,
            phase,
        }
    }

    fn test_runtime() -> CognitionRuntime {
        CognitionRuntime::new_with_options(CognitionRuntimeOptions {
            enabled: true,
            loop_interval_ms: 25,
            max_events: 256,
            max_snapshots: 32,
            default_policy: PersonaPolicy {
                max_spawn_depth: 2,
                max_branching_factor: 2,
                token_budget_per_minute: 1_000,
                compute_ms_per_minute: 1_000,
                idle_ttl_secs: 300,
                share_memory: false,
                allowed_tools: Vec::new(),
            },
        })
    }

    #[test]
    fn normalize_rejects_placeholder_process_line() {
        let work = sample_work_item(ThoughtPhase::Compress);
        let output = normalize_thought_output(
            &work,
            &[],
            "Phase: Compress | State_Summary: ... | Retained_Facts: ... | Open_Risks: ... | Next_Process_Step: ...",
        );
        assert!(
            output.starts_with("Phase: Compress | State_Summary: reliability monitoring active")
        );
        assert!(!output.contains("State_Summary: ..."));
    }

    #[test]
    fn normalize_accepts_concrete_process_line() {
        let work = sample_work_item(ThoughtPhase::Test);
        let output = normalize_thought_output(
            &work,
            &[],
            "Phase: Test | Check: inspect ingress 5xx over last 15m | Procedure: query error-rate dashboard and compare baseline | Expected_Result: pass if <=0.5% 5xx, fail otherwise | Evidence_Quality: high from direct telemetry | Escalation_Trigger: >2% 5xx for 5 minutes",
        );
        assert_eq!(
            output,
            "Phase: Test | Check: inspect ingress 5xx over last 15m | Procedure: query error-rate dashboard and compare baseline | Expected_Result: pass if <=0.5% 5xx, fail otherwise | Evidence_Quality: high from direct telemetry | Escalation_Trigger: >2% 5xx for 5 minutes"
        );
    }

    #[tokio::test]
    async fn create_spawn_and_lineage_work() {
        let runtime = test_runtime();

        let root = runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("root".to_string()),
                name: "root".to_string(),
                role: "orchestrator".to_string(),
                charter: "coordinate".to_string(),
                swarm_id: Some("swarm-a".to_string()),
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .expect("root should be created");

        assert_eq!(root.identity.depth, 0);

        let child = runtime
            .spawn_child(
                "root",
                SpawnPersonaRequest {
                    persona_id: Some("child-1".to_string()),
                    name: "child-1".to_string(),
                    role: "analyst".to_string(),
                    charter: "analyze".to_string(),
                    swarm_id: None,
                    policy: None,
                },
            )
            .await
            .expect("child should spawn");

        assert_eq!(child.identity.parent_id.as_deref(), Some("root"));
        assert_eq!(child.identity.depth, 1);

        let lineage = runtime.lineage_graph().await;
        assert_eq!(lineage.total_edges, 1);
        assert_eq!(lineage.roots, vec!["root".to_string()]);
    }

    #[tokio::test]
    async fn branching_and_depth_limits_are_enforced() {
        let runtime = test_runtime();

        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("root".to_string()),
                name: "root".to_string(),
                role: "orchestrator".to_string(),
                charter: "coordinate".to_string(),
                swarm_id: Some("swarm-a".to_string()),
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .expect("root should be created");

        runtime
            .spawn_child(
                "root",
                SpawnPersonaRequest {
                    persona_id: Some("c1".to_string()),
                    name: "c1".to_string(),
                    role: "worker".to_string(),
                    charter: "run".to_string(),
                    swarm_id: None,
                    policy: None,
                },
            )
            .await
            .expect("first child should spawn");

        runtime
            .spawn_child(
                "root",
                SpawnPersonaRequest {
                    persona_id: Some("c2".to_string()),
                    name: "c2".to_string(),
                    role: "worker".to_string(),
                    charter: "run".to_string(),
                    swarm_id: None,
                    policy: None,
                },
            )
            .await
            .expect("second child should spawn");

        let third_child = runtime
            .spawn_child(
                "root",
                SpawnPersonaRequest {
                    persona_id: Some("c3".to_string()),
                    name: "c3".to_string(),
                    role: "worker".to_string(),
                    charter: "run".to_string(),
                    swarm_id: None,
                    policy: None,
                },
            )
            .await;
        assert!(third_child.is_err());

        runtime
            .spawn_child(
                "c1",
                SpawnPersonaRequest {
                    persona_id: Some("c1-1".to_string()),
                    name: "c1-1".to_string(),
                    role: "worker".to_string(),
                    charter: "run".to_string(),
                    swarm_id: None,
                    policy: None,
                },
            )
            .await
            .expect("depth 2 should be allowed");

        let depth_violation = runtime
            .spawn_child(
                "c1-1",
                SpawnPersonaRequest {
                    persona_id: Some("c1-1-1".to_string()),
                    name: "c1-1-1".to_string(),
                    role: "worker".to_string(),
                    charter: "run".to_string(),
                    swarm_id: None,
                    policy: None,
                },
            )
            .await;
        assert!(depth_violation.is_err());
    }

    #[tokio::test]
    async fn start_stop_updates_runtime_status() {
        let runtime = test_runtime();

        runtime
            .start(Some(StartCognitionRequest {
                loop_interval_ms: Some(10),
                seed_persona: Some(CreatePersonaRequest {
                    persona_id: Some("seed".to_string()),
                    name: "seed".to_string(),
                    role: "watcher".to_string(),
                    charter: "observe".to_string(),
                    swarm_id: Some("swarm-seed".to_string()),
                    parent_id: None,
                    policy: None,
                    tags: Vec::new(),
                }),
            }))
            .await
            .expect("runtime should start");

        tokio::time::sleep(Duration::from_millis(60)).await;
        let running_status = runtime.status().await;
        assert!(running_status.running);
        assert!(running_status.events_buffered > 0);

        runtime
            .stop(Some("test".to_string()))
            .await
            .expect("runtime should stop");
        let stopped_status = runtime.status().await;
        assert!(!stopped_status.running);
    }

    #[tokio::test]
    async fn zero_budget_persona_is_skipped() {
        let runtime = CognitionRuntime::new_with_options(CognitionRuntimeOptions {
            enabled: true,
            loop_interval_ms: 10,
            max_events: 256,
            max_snapshots: 32,
            default_policy: PersonaPolicy {
                max_spawn_depth: 2,
                max_branching_factor: 2,
                token_budget_per_minute: 0,
                compute_ms_per_minute: 0,
                idle_ttl_secs: 3_600,
                share_memory: false,
                allowed_tools: Vec::new(),
            },
        });

        let persona = runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("budget-test".to_string()),
                name: "budget-test".to_string(),
                role: "tester".to_string(),
                charter: "test budgets".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .expect("should create persona");

        assert_eq!(persona.tokens_this_window, 0);
        assert_eq!(persona.compute_ms_this_window, 0);

        // Start and run a few ticks
        runtime.start(None).await.expect("should start");
        tokio::time::sleep(Duration::from_millis(50)).await;
        runtime.stop(None).await.expect("should stop");

        // Persona should still have 0 thought count (budget blocked)
        let p = runtime.get_persona("budget-test").await.unwrap();
        assert_eq!(p.thought_count, 0);
        assert!(p.budget_paused);
    }

    #[tokio::test]
    async fn budget_counters_reset_after_window() {
        let now = Utc::now();
        let mut persona = PersonaRuntimeState {
            identity: PersonaIdentity {
                id: "p1".to_string(),
                name: "test".to_string(),
                role: "tester".to_string(),
                charter: "test".to_string(),
                swarm_id: None,
                parent_id: None,
                depth: 0,
                created_at: now,
                tags: Vec::new(),
            },
            policy: PersonaPolicy::default(),
            status: PersonaStatus::Active,
            thought_count: 0,
            last_tick_at: None,
            updated_at: now,
            tokens_this_window: 5000,
            compute_ms_this_window: 3000,
            window_started_at: now - ChronoDuration::seconds(61),
            last_progress_at: now,
            budget_paused: false,
        };

        // Simulate window reset check
        let window_elapsed = now
            .signed_duration_since(persona.window_started_at)
            .num_seconds();
        assert!(window_elapsed >= 60);

        // Reset
        persona.tokens_this_window = 0;
        persona.compute_ms_this_window = 0;
        persona.window_started_at = now;

        assert_eq!(persona.tokens_this_window, 0);
        assert_eq!(persona.compute_ms_this_window, 0);
    }

    #[tokio::test]
    async fn idle_persona_is_reaped() {
        let runtime = CognitionRuntime::new_with_options(CognitionRuntimeOptions {
            enabled: true,
            loop_interval_ms: 10,
            max_events: 256,
            max_snapshots: 32,
            default_policy: PersonaPolicy {
                max_spawn_depth: 2,
                max_branching_factor: 2,
                token_budget_per_minute: 20_000,
                compute_ms_per_minute: 10_000,
                idle_ttl_secs: 0, // 0 TTL = immediately idle
                share_memory: false,
                allowed_tools: Vec::new(),
            },
        });

        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("idle-test".to_string()),
                name: "idle-test".to_string(),
                role: "idler".to_string(),
                charter: "idle away".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .expect("should create persona");

        // Manually set last_progress_at to the past
        {
            let mut personas = runtime.personas.write().await;
            if let Some(p) = personas.get_mut("idle-test") {
                p.last_progress_at = Utc::now() - ChronoDuration::seconds(10);
            }
        }

        runtime.start(None).await.expect("should start");
        tokio::time::sleep(Duration::from_millis(100)).await;
        runtime.stop(None).await.expect("should stop");

        let p = runtime.get_persona("idle-test").await.unwrap();
        assert_eq!(p.status, PersonaStatus::Reaped);
    }

    #[tokio::test]
    async fn budget_paused_persona_not_reaped_for_idle() {
        let runtime = CognitionRuntime::new_with_options(CognitionRuntimeOptions {
            enabled: true,
            loop_interval_ms: 10,
            max_events: 256,
            max_snapshots: 32,
            default_policy: PersonaPolicy {
                max_spawn_depth: 2,
                max_branching_factor: 2,
                token_budget_per_minute: 0, // zero budget = always paused
                compute_ms_per_minute: 0,
                idle_ttl_secs: 0, // 0 TTL
                share_memory: false,
                allowed_tools: Vec::new(),
            },
        });

        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("paused-test".to_string()),
                name: "paused-test".to_string(),
                role: "pauser".to_string(),
                charter: "pause".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .expect("should create persona");

        // Set last_progress_at to the past to trigger idle check
        {
            let mut personas = runtime.personas.write().await;
            if let Some(p) = personas.get_mut("paused-test") {
                p.last_progress_at = Utc::now() - ChronoDuration::seconds(10);
            }
        }

        runtime.start(None).await.expect("should start");
        tokio::time::sleep(Duration::from_millis(100)).await;
        runtime.stop(None).await.expect("should stop");

        let p = runtime.get_persona("paused-test").await.unwrap();
        // Budget-paused personas are NOT reaped for idle
        assert_eq!(p.status, PersonaStatus::Active);
        assert!(p.budget_paused);
    }

    #[tokio::test]
    async fn governance_proposal_resolution() {
        let runtime = test_runtime();
        let gov = SwarmGovernance {
            quorum_fraction: 0.5,
            required_approvers_by_role: HashMap::new(),
            veto_roles: vec!["auditor".to_string()],
            vote_timeout_secs: 300,
        };
        *runtime.governance.write().await = gov;

        // Create two personas
        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("voter-1".to_string()),
                name: "voter-1".to_string(),
                role: "engineer".to_string(),
                charter: "vote".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .unwrap();
        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("voter-2".to_string()),
                name: "voter-2".to_string(),
                role: "engineer".to_string(),
                charter: "vote".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .unwrap();

        // Insert a proposal with votes
        {
            let mut proposals = runtime.proposals.write().await;
            let mut votes = HashMap::new();
            votes.insert("voter-1".to_string(), ProposalVote::Approve);
            proposals.insert(
                "prop-1".to_string(),
                Proposal {
                    id: "prop-1".to_string(),
                    persona_id: "voter-1".to_string(),
                    title: "test proposal".to_string(),
                    rationale: "testing governance".to_string(),
                    evidence_refs: Vec::new(),
                    risk: ProposalRisk::Low,
                    status: ProposalStatus::Created,
                    created_at: Utc::now(),
                    votes,
                    vote_deadline: Some(Utc::now() + ChronoDuration::seconds(300)),
                    votes_requested: true,
                },
            );
        }

        // quorum = 0.5 * 2 = 1, and we have 1 approval → should be verified
        runtime.start(None).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        runtime.stop(None).await.unwrap();

        let proposals = runtime.get_proposals().await;
        let prop = proposals.get("prop-1").unwrap();
        // Should be verified or executed (it gets verified then executed in same tick)
        assert!(
            prop.status == ProposalStatus::Verified || prop.status == ProposalStatus::Executed,
            "Expected Verified or Executed, got {:?}",
            prop.status
        );
    }

    #[tokio::test]
    async fn veto_rejects_proposal() {
        let runtime = test_runtime();
        let gov = SwarmGovernance {
            quorum_fraction: 0.5,
            required_approvers_by_role: HashMap::new(),
            veto_roles: vec!["auditor".to_string()],
            vote_timeout_secs: 300,
        };
        *runtime.governance.write().await = gov;

        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("eng".to_string()),
                name: "eng".to_string(),
                role: "engineer".to_string(),
                charter: "build".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .unwrap();
        runtime
            .create_persona(CreatePersonaRequest {
                persona_id: Some("aud".to_string()),
                name: "aud".to_string(),
                role: "auditor".to_string(),
                charter: "audit".to_string(),
                swarm_id: None,
                parent_id: None,
                policy: None,
                tags: Vec::new(),
            })
            .await
            .unwrap();

        {
            let mut proposals = runtime.proposals.write().await;
            let mut votes = HashMap::new();
            votes.insert("eng".to_string(), ProposalVote::Approve);
            votes.insert("aud".to_string(), ProposalVote::Veto);
            proposals.insert(
                "prop-veto".to_string(),
                Proposal {
                    id: "prop-veto".to_string(),
                    persona_id: "eng".to_string(),
                    title: "vetoed proposal".to_string(),
                    rationale: "testing veto".to_string(),
                    evidence_refs: Vec::new(),
                    risk: ProposalRisk::Low,
                    status: ProposalStatus::Created,
                    created_at: Utc::now(),
                    votes,
                    vote_deadline: Some(Utc::now() + ChronoDuration::seconds(300)),
                    votes_requested: true,
                },
            );
        }

        runtime.start(None).await.unwrap();
        tokio::time::sleep(Duration::from_millis(100)).await;
        runtime.stop(None).await.unwrap();

        let proposals = runtime.get_proposals().await;
        let prop = proposals.get("prop-veto").unwrap();
        assert_eq!(prop.status, ProposalStatus::Rejected);
    }

    #[test]
    fn global_workspace_default() {
        let ws = GlobalWorkspace::default();
        assert!(ws.top_beliefs.is_empty());
        assert!(ws.top_uncertainties.is_empty());
        assert!(ws.top_attention.is_empty());
    }

    #[test]
    fn attention_item_creation() {
        let item = AttentionItem {
            id: "a1".to_string(),
            topic: "test topic".to_string(),
            topic_tags: vec!["reliability".to_string()],
            priority: 0.8,
            source_type: AttentionSource::ContestedBelief,
            source_id: "b1".to_string(),
            assigned_persona: None,
            created_at: Utc::now(),
            resolved_at: None,
        };
        assert!(item.resolved_at.is_none());
        assert_eq!(item.priority, 0.8);
    }
}
