//! Model catalog entry.
use crate::*;
pub(super) fn info() -> ModelInfo {
    ModelInfo {
        id: "step-1-128k".to_string(),
        name: "Step 1 128K".to_string(),
        provider: "stepfun".to_string(),
        context_window: 128_000,
        max_output_tokens: Some(8192),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(2.0),
        output_cost_per_million: Some(6.0),
    }
}
