//! Runtime field assembly from options and restored state.

use std::collections::{HashMap, VecDeque};
use std::sync::Arc;
use std::sync::atomic::AtomicBool;
use tokio::sync::{Mutex, RwLock, broadcast};

use super::initial_state::InitialState;
use super::{
    CognitionRuntime, CognitionRuntimeOptions, SwarmGovernance, ThinkerClient, ThoughtEvent,
};

/// Minimum retained events and snapshots, regardless of configuration.
const MIN_EVENTS: usize = 32;
const MIN_SNAPSHOTS: usize = 8;
/// Minimum loop interval, in milliseconds.
const MIN_INTERVAL_MS: u64 = 100;

/// Assemble a runtime from options, restored state, and an optional thinker.
pub(super) fn assemble(
    options: CognitionRuntimeOptions,
    init: InitialState,
    thinker: Option<Arc<ThinkerClient>>,
    event_tx: broadcast::Sender<ThoughtEvent>,
) -> CognitionRuntime {
    CognitionRuntime {
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
