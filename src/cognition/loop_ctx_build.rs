//! Construction of [`LoopCtx`](super::LoopCtx) from a runtime.

use std::sync::Arc;

use super::LoopCtx;
use crate::cognition::CognitionRuntime;

impl LoopCtx {
    /// Snapshot the runtime's shared handles for the loop task.
    pub(in crate::cognition) fn from_runtime(runtime: &CognitionRuntime) -> Self {
        Self {
            running: Arc::clone(&runtime.running),
            loop_interval_ms: Arc::clone(&runtime.loop_interval_ms),
            last_tick_at: Arc::clone(&runtime.last_tick_at),
            personas: Arc::clone(&runtime.personas),
            proposals: Arc::clone(&runtime.proposals),
            events: Arc::clone(&runtime.events),
            snapshots: Arc::clone(&runtime.snapshots),
            max_events: runtime.max_events,
            max_snapshots: runtime.max_snapshots,
            event_tx: runtime.event_tx.clone(),
            thinker: runtime.thinker.clone(),
            beliefs: Arc::clone(&runtime.beliefs),
            attention_queue: Arc::clone(&runtime.attention_queue),
            governance: Arc::clone(&runtime.governance),
            workspace: Arc::clone(&runtime.workspace),
            tools: runtime.tools.clone(),
            receipts: Arc::clone(&runtime.receipts),
            pending_approvals: Arc::clone(&runtime.pending_approvals),
        }
    }
}
