use super::select;
use crate::provider::ModelInfo;

fn model(id: &str, cost: Option<f64>, tools: bool) -> ModelInfo {
    ModelInfo {
        id: id.into(),
        name: id.into(),
        provider: "same".into(),
        context_window: 128_000,
        max_output_tokens: Some(8_192),
        supports_vision: false,
        supports_tools: tools,
        supports_streaming: true,
        input_cost_per_million: cost,
        output_cost_per_million: None,
    }
}

#[test]
fn prefers_cheapest_capable_non_caller_model() {
    let models = [
        model("caller", Some(10.0), true),
        model("cheap", Some(1.0), true),
        model("cheaper-no-tools", Some(0.1), false),
    ];
    assert_eq!(select(&models, "caller"), Some("cheap"));
}

#[test]
fn uses_caller_when_no_other_model_is_capable() {
    let models = [
        model("caller", Some(10.0), true),
        model("tiny", Some(1.0), false),
    ];
    assert_eq!(select(&models, "caller"), Some("caller"));
}
