use super::{CognitionRuntime, InitialState};
use crate::{CognitionRuntimeOptions, SwarmGovernance, ThoughtEvent};
use std::{
    collections::{HashMap, VecDeque},
    sync::{Arc, atomic::AtomicBool},
};
use tokio::sync::{Mutex, RwLock, broadcast};
/// Minimum retained events and snapshots, regardless of configuration.
const MIN_EVENTS: usize = 32;
const MIN_SNAPSHOTS: usize = 8;
/// Minimum loop interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 100;

impl<T, R> CognitionRuntime<T, R> {
    /// Assemble shared state from options, restored data and an optional thinker.
    /// # Arguments
    /// Options choose bounds/cadence; initial state supplies restored stores; thinker
    /// and event sender are caller-owned service handles.
    /// # Returns
    /// State with normalized limits and independently shared runtime stores.
    /// # Examples
    /// See [CognitionRuntime] for complete construction.
    pub fn assemble(
        options: CognitionRuntimeOptions,
        init: InitialState,
        thinker: Option<Arc<T>>,
        event_tx: broadcast::Sender<ThoughtEvent>,
    ) -> Self {
        Self {
            enabled: options.enabled,
            max_events: options.max_events.max(MIN_EVENTS),
            max_snapshots: options.max_snapshots.max(MIN_SNAPSHOTS),
            default_policy: options.default_policy,
            running: Arc::new(AtomicBool::new(false)),
            loop_interval_ms: Arc::new(RwLock::new(options.loop_interval_ms.max(MIN_INTERVAL_MS))),
            started_at: Arc::new(RwLock::new(None)),
            last_tick_at: Arc::new(RwLock::new(None)),
            personas: Arc::new(RwLock::new(init.personas)),
            proposals: Arc::new(RwLock::new(init.proposals)),
            events: Arc::new(RwLock::new(VecDeque::new())),
            snapshots: Arc::new(RwLock::new(VecDeque::new())),
            loop_handle: Arc::new(Mutex::new(None)),
            event_tx,
            thinker,
            beliefs: Arc::new(RwLock::new(init.beliefs)),
            attention_queue: Arc::new(RwLock::new(init.attention)),
            governance: Arc::new(RwLock::new(SwarmGovernance::default())),
            workspace: Arc::new(RwLock::new(init.workspace)),
            tools: None,
            receipts: Arc::new(RwLock::new(Vec::new())),
            pending_approvals: Arc::new(RwLock::new(HashMap::new())),
        }
    }
}
