//! Shared handles the cognition loop task owns.

use chrono::{DateTime, Utc};
use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::{RwLock, broadcast};

use super::beliefs::Belief;
use super::executor::DecisionReceipt;
use super::{
    AttentionItem, GlobalWorkspace, MemorySnapshot, PersonaRuntimeState, Proposal, SwarmGovernance,
    ThinkerClient, ThoughtEvent,
};
use crate::tool::ToolRegistry;

/// Cloned state shared with the spawned loop task.
pub(super) struct LoopCtx {
    pub running: Arc<AtomicBool>,
    pub loop_interval_ms: Arc<RwLock<u64>>,
    pub last_tick_at: Arc<RwLock<Option<DateTime<Utc>>>>,
    pub personas: Arc<RwLock<HashMap<String, PersonaRuntimeState>>>,
    pub proposals: Arc<RwLock<HashMap<String, Proposal>>>,
    pub events: Arc<RwLock<VecDeque<ThoughtEvent>>>,
    pub snapshots: Arc<RwLock<VecDeque<MemorySnapshot>>>,
    pub max_events: usize,
    pub max_snapshots: usize,
    pub event_tx: broadcast::Sender<ThoughtEvent>,
    pub thinker: Option<Arc<ThinkerClient>>,
    pub beliefs: Arc<RwLock<HashMap<String, Belief>>>,
    pub attention_queue: Arc<RwLock<Vec<AttentionItem>>>,
    pub governance: Arc<RwLock<SwarmGovernance>>,
    pub workspace: Arc<RwLock<GlobalWorkspace>>,
    pub tools: Option<Arc<ToolRegistry>>,
    pub receipts: Arc<RwLock<Vec<DecisionReceipt>>>,
    pub pending_approvals: Arc<RwLock<HashMap<String, bool>>>,
}

#[path = "loop_ctx_build.rs"]
mod build;
