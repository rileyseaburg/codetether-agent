use super::merge_known;
use crate::provider::ModelInfo;

#[test]
fn merge_known_adds_glm_5_3_to_discovery() {
    let models = merge_known(vec![model("glm-5")]);

    assert!(models.iter().any(|model| model.id == "glm-5.3"));
    assert_eq!(models.iter().filter(|model| model.id == "glm-5").count(), 1);
}

#[test]
fn merge_known_excludes_deprecated_pony_alpha_2() {
    for discovered in [vec![], vec![model("pony-alpha-2"), model("PONY-ALPHA-2")]] {
        let models = merge_known(discovered);
        assert!(
            !models
                .iter()
                .any(|m| m.id.eq_ignore_ascii_case("pony-alpha-2"))
        );
        assert!(models.iter().any(|m| m.id == "glm-5.3"));
    }
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
