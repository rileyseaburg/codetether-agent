//! Provider regression coverage.

use super::*;

#[test]
fn parses_string_only_models_payload() {
    let payload = json!({
        "data": ["glm-5", "glm-5-turbo"]
    });

    let models = OpenAIProvider::parse_models_payload(&payload, "custom-openapi");

    assert_eq!(models.len(), 2);
    assert_eq!(models[0].id, "glm-5");
    assert_eq!(models[1].id, "glm-5-turbo");
}
