//! Routing identity mirrors adapter model normalization without credentials or HTTP.
use super::resolve;

#[path = "tests/adapters.rs"]
mod adapters;

#[test]
fn routed_identity_normalizes_glm5_aliases_and_preserves_variants() {
    for model in ["", "glm5", "glm-5", "glm5/glm-5", "glm5:glm-5", " glm5 "] {
        assert_eq!(resolve("glm5", model), "glm-5-fp8");
    }
    assert_eq!(resolve("glm5", "glm5/glm-4.7"), "glm-4.7");
}

#[test]
fn routed_identity_normalizes_cerebras_and_vertex_models() {
    assert_eq!(resolve("cerebras", "glm-4.7"), "zai-glm-4.7");
    assert_eq!(resolve("cerebras", "zai-glm-4.7"), "zai-glm-4.7");
    assert_eq!(resolve("vertex-glm", "glm-5"), "zai-org/glm-5-maas");
    assert_eq!(
        resolve("vertex-glm", "zai-org-other"),
        "zai-org/zai-org-other-maas"
    );
    assert_eq!(
        resolve("vertex-glm", "zai-org/glm-5-maas"),
        "zai-org/glm-5-maas"
    );
}

#[test]
fn routed_identity_preserves_unrecognized_provider_model_ids() {
    assert_eq!(
        resolve("openrouter", "anthropic/claude-sonnet"),
        "anthropic/claude-sonnet"
    );
    assert_eq!(resolve("local_cuda", "model-a"), "model-a");
}
