//! Perpetual cognition runtime for persona swarms.
//!
//! Phase 0 scope:
//! - Contract types for personas, thought events, proposals, and snapshots
//! - In-memory runtime manager for lifecycle + lineage
//! - Feature-flagged perpetual loop (no external execution side effects)

use anyhow::{Result, anyhow};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio::task::JoinHandle;
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
    pub token_credits_per_minute: u32,
    pub cpu_credits_per_minute: u32,
    pub idle_ttl_secs: u64,
    pub share_memory: bool,
}

impl Default for PersonaPolicy {
    fn default() -> Self {
        Self {
            max_spawn_depth: 4,
            max_branching_factor: 4,
            token_credits_per_minute: 20_000,
            cpu_credits_per_minute: 10_000,
            idle_ttl_secs: 3_600,
            share_memory: false,
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
}

/// Proposal risk level.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
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
}

impl CognitionRuntime {
    /// Build runtime from environment feature flags.
    pub fn new_from_env() -> Self {
        let mut options = CognitionRuntimeOptions::default();
        options.enabled = env_bool("CODETETHER_COGNITION_ENABLED", false);
        options.loop_interval_ms = env_u64("CODETETHER_COGNITION_LOOP_INTERVAL_MS", 2_000);
        options.max_events = env_usize("CODETETHER_COGNITION_MAX_EVENTS", 2_000);
        options.max_snapshots = env_usize("CODETETHER_COGNITION_MAX_SNAPSHOTS", 128);

        options.default_policy = PersonaPolicy {
            max_spawn_depth: env_u32("CODETETHER_COGNITION_MAX_SPAWN_DEPTH", 4),
            max_branching_factor: env_u32("CODETETHER_COGNITION_MAX_BRANCHING_FACTOR", 4),
            token_credits_per_minute: env_u32(
                "CODETETHER_COGNITION_TOKEN_CREDITS_PER_MINUTE",
                20_000,
            ),
            cpu_credits_per_minute: env_u32("CODETETHER_COGNITION_CPU_CREDITS_PER_MINUTE", 10_000),
            idle_ttl_secs: env_u64("CODETETHER_COGNITION_IDLE_TTL_SECS", 3_600),
            share_memory: env_bool("CODETETHER_COGNITION_SHARE_MEMORY", false),
        };

        Self::new_with_options(options)
    }

    /// Build runtime from explicit options.
    pub fn new_with_options(options: CognitionRuntimeOptions) -> Self {
        let (event_tx, _) = broadcast::channel(options.max_events.max(16));
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

        if let Some(request) = req.clone() {
            if let Some(interval) = request.loop_interval_ms {
                let mut lock = self.loop_interval_ms.write().await;
                *lock = interval.max(100);
            }
            if let Some(seed) = request.seed_persona {
                let has_personas = !self.personas.read().await.is_empty();
                if !has_personas {
                    let _ = self.create_persona(seed).await?;
                }
            }
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

        let handle = tokio::spawn(async move {
            while running.load(Ordering::SeqCst) {
                let now = Utc::now();
                {
                    let mut lock = last_tick_at.write().await;
                    *lock = Some(now);
                }

                let mut new_events = Vec::new();
                let mut new_snapshots = Vec::new();
                let mut new_proposals = Vec::new();

                {
                    let mut map = personas.write().await;
                    for persona in map.values_mut() {
                        if persona.status != PersonaStatus::Active {
                            continue;
                        }

                        persona.thought_count = persona.thought_count.saturating_add(1);
                        persona.last_tick_at = Some(now);
                        persona.updated_at = now;

                        let phase = match persona.thought_count % 4 {
                            1 => "observe",
                            2 => "reflect",
                            3 => "test",
                            _ => "compress",
                        };

                        let event_type = match phase {
                            "observe" => ThoughtEventType::ThoughtGenerated,
                            "reflect" => ThoughtEventType::HypothesisRaised,
                            "test" => ThoughtEventType::CheckRequested,
                            _ => ThoughtEventType::SnapshotCompressed,
                        };

                        new_events.push(ThoughtEvent {
                            id: Uuid::new_v4().to_string(),
                            event_type,
                            persona_id: Some(persona.identity.id.clone()),
                            swarm_id: persona.identity.swarm_id.clone(),
                            timestamp: now,
                            payload: json!({
                                "phase": phase,
                                "thought_count": persona.thought_count,
                                "role": persona.identity.role,
                            }),
                        });

                        if phase == "reflect" && persona.thought_count % 8 == 2 {
                            let proposal = Proposal {
                                id: Uuid::new_v4().to_string(),
                                persona_id: persona.identity.id.clone(),
                                title: format!("proposal-{}", persona.thought_count),
                                rationale: format!(
                                    "Generated from reflective loop in persona {}",
                                    persona.identity.id
                                ),
                                evidence_refs: vec!["internal.thought_stream".to_string()],
                                risk: ProposalRisk::Low,
                                status: ProposalStatus::Created,
                                created_at: now,
                            };

                            new_events.push(ThoughtEvent {
                                id: Uuid::new_v4().to_string(),
                                event_type: ThoughtEventType::ProposalCreated,
                                persona_id: Some(persona.identity.id.clone()),
                                swarm_id: persona.identity.swarm_id.clone(),
                                timestamp: now,
                                payload: json!({
                                    "proposal_id": proposal.id,
                                    "title": proposal.title,
                                }),
                            });

                            new_proposals.push(proposal);
                        }

                        if phase == "compress" {
                            new_snapshots.push(MemorySnapshot {
                                id: Uuid::new_v4().to_string(),
                                generated_at: now,
                                swarm_id: persona.identity.swarm_id.clone(),
                                persona_scope: vec![persona.identity.id.clone()],
                                summary: format!(
                                    "Compressed cognition state for persona {} at thought {}",
                                    persona.identity.id, persona.thought_count
                                ),
                                hot_event_count: 1,
                                warm_fact_count: 1,
                                cold_snapshot_count: 1,
                                metadata: HashMap::from([
                                    (
                                        "phase".to_string(),
                                        serde_json::Value::String("compress".to_string()),
                                    ),
                                    (
                                        "role".to_string(),
                                        serde_json::Value::String(persona.identity.role.clone()),
                                    ),
                                ]),
                            });
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

                let interval = *loop_interval_ms.read().await;
                tokio::time::sleep(Duration::from_millis(interval.max(100))).await;
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
        };

        let persona = PersonaRuntimeState {
            identity,
            policy,
            status: PersonaStatus::Active,
            thought_count: 0,
            last_tick_at: None,
            updated_at: now,
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

    fn test_runtime() -> CognitionRuntime {
        CognitionRuntime::new_with_options(CognitionRuntimeOptions {
            enabled: true,
            loop_interval_ms: 25,
            max_events: 256,
            max_snapshots: 32,
            default_policy: PersonaPolicy {
                max_spawn_depth: 2,
                max_branching_factor: 2,
                token_credits_per_minute: 1_000,
                cpu_credits_per_minute: 1_000,
                idle_ttl_secs: 300,
                share_memory: false,
            },
        })
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
}
