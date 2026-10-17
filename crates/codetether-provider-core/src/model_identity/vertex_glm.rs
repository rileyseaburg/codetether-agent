//! Model IDs shared by Vertex GLM requests and harness routing identity.
/// Preserve qualified IDs and qualify short Vertex GLM model names.
///
/// # Arguments
/// * `model` - A short model name or an existing `zai-org/` ID.
/// # Returns
/// A qualified model ID.
/// # Examples
/// ```
/// assert_eq!(codetether_provider_core::model_identity::vertex_glm::normalize("glm-5"), "zai-org/glm-5-maas");
/// ```
pub fn normalize(model: &str) -> String {
    if model.starts_with("zai-org/") {
        model.to_owned()
    } else {
        format!("zai-org/{model}-maas")
    }
}
