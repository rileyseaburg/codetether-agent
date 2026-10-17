//! Model catalog entry.
use crate::*;
pub(super) fn info() -> ModelInfo {
    ModelInfo {
        id: "step-1-32k".to_string(),
        name: "Step 1 32K".to_string(),
        provider: "stepfun".to_string(),
        context_window: 32_000,
        max_output_tokens: Some(8192),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(1.0),
        output_cost_per_million: Some(3.0),
    }
}
