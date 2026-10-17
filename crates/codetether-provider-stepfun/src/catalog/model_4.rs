//! Model catalog entry.
use crate::*;
pub(super) fn info() -> ModelInfo {
    ModelInfo {
        id: "step-1v-8k".to_string(),
        name: "Step 1 Vision 8K".to_string(),
        provider: "stepfun".to_string(),
        context_window: 8_000,
        max_output_tokens: Some(4096),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(1.0),
        output_cost_per_million: Some(3.0),
    }
}
