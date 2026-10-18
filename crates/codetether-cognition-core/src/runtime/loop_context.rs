use crate::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, SwarmGovernance,
    ThoughtEvent, beliefs::Belief, execution::DecisionReceipt,
};
use chrono::{DateTime, Utc};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, atomic::AtomicBool},
};
use tokio::sync::{RwLock, broadcast};
/// Cloned shared state retained by a spawned cognition loop.
/// # Examples
/// Obtain this view with [super::CognitionRuntime::loop_context].
pub struct LoopContext<T, R> {
    /// Shared running flag observed by the loop.
    pub running: Arc<AtomicBool>,
    /// Shared cadence in milliseconds.
    pub loop_interval_ms: Arc<RwLock<u64>>,
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
    /// Maximum buffered thought events.
    pub max_events: usize,
    /// Maximum buffered memory snapshots.
    pub max_snapshots: usize,
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
    /// Proposals awaiting human approval.
    pub pending_approvals: Arc<RwLock<HashMap<String, bool>>>,
}
