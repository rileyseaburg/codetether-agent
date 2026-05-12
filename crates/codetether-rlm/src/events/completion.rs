//! RLM completion record.

use serde::{Deserialize, Serialize};
use uuid::Uuid;

use super::RlmOutcome;

/// Terminal record for a single RLM invocation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RlmCompletion {
    pub trace_id: Uuid,
    pub outcome: RlmOutcome,
    pub iterations: usize,
    pub subcalls: usize,
    pub input_tokens: usize,
    pub output_tokens: usize,
    pub elapsed_ms: u64,
    pub reason: Option<String>,
    pub root_model: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub subcall_model_used: Option<String>,
}

impl RlmCompletion {
    /// output_tokens / input_tokens, 0.0 when input is zero.
    pub fn compression_ratio(&self) -> f64 {
        if self.input_tokens == 0 { 0.0 } else {
            self.output_tokens as f64 / self.input_tokens as f64
        }
    }
}
