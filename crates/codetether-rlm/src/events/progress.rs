//! RLM progress event.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Progress tick from an in-flight RLM analysis loop.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmProgressEvent {
    pub trace_id: Uuid,
    pub iteration: usize,
    pub max_iterations: usize,
    pub status: String,
}

impl RlmProgressEvent {
    /// Fraction of iteration budget consumed (0.0–1.0).
    pub fn fraction(&self) -> f64 {
        if self.max_iterations == 0 {
            0.0
        } else {
            (self.iteration as f64 / self.max_iterations as f64).clamp(0.0, 1.0)
        }
    }
}
