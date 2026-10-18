use super::{CognitionRuntime, LoopContext};
use std::sync::Arc;
impl<T, R> CognitionRuntime<T, R> {
    /// Snapshot the runtime's shared handles for the loop task.
    pub fn loop_context(&self) -> LoopContext<T, R> {
        LoopContext {
            running: Arc::clone(&self.running),
            loop_interval_ms: Arc::clone(&self.loop_interval_ms),
            last_tick_at: Arc::clone(&self.last_tick_at),
            personas: Arc::clone(&self.personas),
            proposals: Arc::clone(&self.proposals),
            events: Arc::clone(&self.events),
            snapshots: Arc::clone(&self.snapshots),
            max_events: self.max_events,
            max_snapshots: self.max_snapshots,
            event_tx: self.event_tx.clone(),
            thinker: self.thinker.clone(),
            beliefs: Arc::clone(&self.beliefs),
            attention_queue: Arc::clone(&self.attention_queue),
            governance: Arc::clone(&self.governance),
            workspace: Arc::clone(&self.workspace),
            tools: self.tools.clone(),
            receipts: Arc::clone(&self.receipts),
            pending_approvals: Arc::clone(&self.pending_approvals),
        }
    }
}
