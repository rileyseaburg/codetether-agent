//! Existing regression test.
use super::*;

#[test]
fn enrich_with_pricing_sets_costs() {
    let provider = CopilotProvider::new("test-token".to_string(), "test-client".into()).unwrap();
    let mut models = vec![ModelInfo {
        id: "gpt-4o".to_string(),
        name: "gpt-4o".to_string(),
        provider: "github-copilot".to_string(),
        context_window: 128_000,
        max_output_tokens: Some(16_384),
        supports_vision: true,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    }];
    provider.enrich_with_pricing(&mut models);
    // GPT-4o is 0x premium (free), so cost = 0.0
    assert_eq!(models[0].input_cost_per_million, Some(0.0));
    assert_eq!(models[0].output_cost_per_million, Some(0.0));
    // Name should be enriched
    assert_eq!(models[0].name, "GPT-4o");
}
