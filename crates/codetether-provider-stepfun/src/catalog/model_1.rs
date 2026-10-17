//! Model catalog entry.
use crate::*;
pub(super) fn info() -> ModelInfo {
    ModelInfo {
        id: "step-1-8k".to_string(),
        name: "Step 1 8K".to_string(),
        provider: "stepfun".to_string(),
        context_window: 8_000,
        max_output_tokens: Some(4096),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.5),
        output_cost_per_million: Some(1.5),
    }
}
