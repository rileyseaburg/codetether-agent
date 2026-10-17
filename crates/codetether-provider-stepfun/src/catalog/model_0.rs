//! Model catalog entry.
use crate::*;
pub(super) fn info() -> ModelInfo {
    ModelInfo {
        id: "step-3.5-flash".to_string(),
        name: "Step 3.5 Flash".to_string(),
        provider: "stepfun".to_string(),
        context_window: 128_000,
        max_output_tokens: Some(8192),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: Some(0.0), // Free tier
        output_cost_per_million: Some(0.0),
    }
}
