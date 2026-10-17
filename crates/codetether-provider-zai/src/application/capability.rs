//! Model selection for streamed tool-call output.
use super::ZaiProvider;
impl ZaiProvider {
    pub(super) fn model_supports_tool_stream(model: &str) -> bool {
        model.contains("glm-5") || model.contains("glm-4.7") || model.contains("glm-4.6")
    }
}
