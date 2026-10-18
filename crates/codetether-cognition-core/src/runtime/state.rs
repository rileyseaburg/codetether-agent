use crate::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaPolicy, PersonaRuntimeState, Proposal,
    SwarmGovernance, ThoughtEvent, beliefs::Belief, execution::DecisionReceipt,
};
use chrono::{DateTime, Utc};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, atomic::AtomicBool},
};
use tokio::{
    sync::{Mutex, RwLock, broadcast},
    task::JoinHandle,
};
/// In-memory cognition state with caller-owned thinker and tool types.
/// # Examples
/// ```
/// use codetether_cognition_core::{CognitionRuntimeOptions,runtime::{CognitionRuntime,InitialState}};
/// let (events, _) = tokio::sync::broadcast::channel(256);
/// let state = CognitionRuntime::<(),()>::assemble(CognitionRuntimeOptions::default(),InitialState::default(),None,events);
/// assert!(!state.enabled);
/// ```
#[derive(Debug)]
pub struct CognitionRuntime<T, R> {
    /// Whether perpetual cognition is enabled.
    pub enabled: bool,
    /// Maximum buffered thought events.
    pub max_events: usize,
    /// Maximum buffered memory snapshots.
    pub max_snapshots: usize,
    /// Policy used when creating personas.
    pub default_policy: PersonaPolicy,
    /// Shared running flag observed by the loop.
    pub running: Arc<AtomicBool>,
    /// Shared cadence in milliseconds.
    pub loop_interval_ms: Arc<RwLock<u64>>,
    /// Last startup timestamp.
    pub started_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    /// Last completed tick timestamp.
    pub last_tick_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    /// Persona states indexed by identity.
    pub personas: Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    /// Current proposals indexed by identity.
    pub proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    /// Bounded recent thought events.
    pub events: Arc<RwLock<VecDeque<ThoughtEvent>>>,
    /// Bounded recent memory snapshots.
    pub snapshots: Arc<RwLock<VecDeque<MemorySnapshot>>>,
    /// Owned background loop task.
    pub loop_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    /// Thought event broadcast sender.
    pub event_tx: broadcast::Sender<ThoughtEvent>,
    /// Optional shared inference client.
    pub thinker: Option<Arc<T>>,
    /// Shared belief store.
    pub beliefs: Arc<RwLock<HashMap<String, Belief>>>,
    /// Pending attention items.
    pub attention_queue: Arc<RwLock<Vec<AttentionItem>>>,
    /// Current proposal governance rules.
    pub governance: Arc<RwLock<SwarmGovernance>>,
    /// Current global workspace.
    pub workspace: Arc<RwLock<GlobalWorkspace>>,
    /// Optional shared native tool registry.
    pub tools: Option<Arc<R>>,
    /// Recorded execution decisions.
    pub receipts: Arc<RwLock<Vec<DecisionReceipt>>>,
    /// Proposals pending human approval for Critical risk.
    /// Proposals awaiting human approval.
    pub pending_approvals: Arc<RwLock<HashMap<String, bool>>>,
}
