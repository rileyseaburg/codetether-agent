//! Shared state held by the in-memory cognition runtime.

use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::{Mutex, RwLock, broadcast};
use tokio::task::JoinHandle;

use super::beliefs::Belief;
use super::executor::DecisionReceipt;
use super::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaPolicy, PersonaRuntimeState, Proposal,
    SwarmGovernance, ThinkerClient, ThoughtEvent,
};
use crate::tool::ToolRegistry;

/// In-memory cognition runtime for perpetual persona swarms.
#[derive(Debug)]
pub struct CognitionRuntime {
    pub(super) enabled: bool,
    pub(super) max_events: usize,
    pub(super) max_snapshots: usize,
    pub(super) default_policy: PersonaPolicy,
    pub(super) running: Arc<AtomicBool>,
    pub(super) loop_interval_ms: Arc<RwLock<u64>>,
    pub(super) started_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    pub(super) last_tick_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    pub(super) personas: Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    pub(super) proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    pub(super) events: Arc<RwLock<VecDeque<ThoughtEvent>>>,
    pub(super) snapshots: Arc<RwLock<VecDeque<MemorySnapshot>>>,
    pub(super) loop_handle: Arc<Mutex<Option<JoinHandle<()>>>>,
    pub(super) event_tx: broadcast::Sender<ThoughtEvent>,
    pub(super) thinker: Option<Arc<ThinkerClient>>,
    pub(super) beliefs: Arc<RwLock<HashMap<String, Belief>>>,
    pub(super) attention_queue: Arc<RwLock<Vec<AttentionItem>>>,
    pub(super) governance: Arc<RwLock<SwarmGovernance>>,
    pub(super) workspace: Arc<RwLock<GlobalWorkspace>>,
    pub(super) tools: Option<Arc<ToolRegistry>>,
    pub(super) receipts: Arc<RwLock<Vec<DecisionReceipt>>>,
    /// Proposals pending human approval for Critical risk.
    pub(super) pending_approvals: Arc<RwLock<HashMap<String, bool>>>,
}
