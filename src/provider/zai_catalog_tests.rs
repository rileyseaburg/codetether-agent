use super::merge_known;
use crate::provider::ModelInfo;

#[test]
fn merge_known_adds_glm_5_3_to_discovery() {
    let models = merge_known(vec![model("glm-5")]);

    assert!(models.iter().any(|model| model.id == "glm-5.3"));
    assert_eq!(models.iter().filter(|model| model.id == "glm-5").count(), 1);
}

fn model(id: &str) -> ModelInfo {
    ModelInfo {
        id: id.to_string(),
        name: id.to_string(),
        provider: "zai".to_string(),
        context_window: 200_000,
        max_output_tokens: Some(128_000),
        supports_vision: false,
        supports_tools: true,
        supports_streaming: true,
        input_cost_per_million: None,
        output_cost_per_million: None,
    }
}
