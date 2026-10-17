//! Remaining Bedrock output-capacity families.
pub(super) fn estimate(id: &str) -> usize {
    if id.contains("llama") {
        4_096
    } else if id.contains("mistral")
        || id.contains("mixtral")
        || id.contains("qwen")
        || id.contains("kimi")
        || id.contains("glm")
        || id.contains("minimax")
        || id.contains("gemma")
        || id.contains("cohere")
        || id.contains("amazon")
    {
        8_192
    } else {
        4_096
    }
}
