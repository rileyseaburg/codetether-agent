//! Provider regression coverage.

use super::*;

#[test]
fn parses_openai_compatible_models_payload() {
    let payload = json!({
        "object": "list",
        "data": [
            {
                "id": "GLM-5-Turbo",
                "name": "GLM-5-Turbo",
                "limits": {
                    "max_context_window_tokens": 200000,
                    "max_output_tokens": 16384
                },
                "input_modalities": ["text"]
            }
        ]
    });

    let models = OpenAIProvider::parse_models_payload(&payload, "custom-openapi");

    assert_eq!(models.len(), 1);
    assert_eq!(models[0].id, "GLM-5-Turbo");
    assert_eq!(models[0].name, "GLM-5-Turbo");
    assert_eq!(models[0].provider, "custom-openapi");
    assert_eq!(models[0].context_window, 200_000);
    assert_eq!(models[0].max_output_tokens, Some(16_384));
}
