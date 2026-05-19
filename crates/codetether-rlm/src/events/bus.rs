//! RLM event bus trait.

use crate::{RlmCompletion, RlmProgressEvent};

/// Trait for emitting events during an RLM run.
pub trait RlmEventBus: Send + Sync {
    /// Emit a progress tick (iteration boundary).
    fn emit_progress(&self, event: RlmProgressEvent);
    /// Emit the terminal completion record.
    fn emit_completion(&self, event: RlmCompletion);
}
