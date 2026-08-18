//! Per-architecture chat prompt templates for Candle inference.

/// Build a chat prompt using the proper template for each model architecture.
///
/// Unknown architectures use a plain labelled transcript.
pub(super) fn format_chat_prompt(
    architecture: &str,
    system_prompt: &str,
    user_prompt: &str,
) -> String {
    match architecture {
        // ChatML template (Qwen2, Yi, etc.)
        "qwen2" | "qwen3" | "qwen3moe" | "qwen3_moe" => format!(
            "<|im_start|>system\n{system_prompt}<|im_end|>\n<|im_start|>user\n{user_prompt}<|im_end|>\n<|im_start|>assistant\n"
        ),
        // Llama 3 instruct template
        "llama" => format!(
            "<|begin_of_text|><|start_header_id|>system<|end_header_id|>\n\n{system_prompt}<|eot_id|><|start_header_id|>user<|end_header_id|>\n\n{user_prompt}<|eot_id|><|start_header_id|>assistant<|end_header_id|>\n\n"
        ),
        // Gemma instruct template
        "gemma" | "gemma2" | "gemma3" | "gemma-embedding" => format!(
            "<start_of_turn>user\n{system_prompt}\n\n{user_prompt}<end_of_turn>\n<start_of_turn>model\n"
        ),
        _ => format!("System:\n{system_prompt}\n\nUser:\n{user_prompt}\n\nAssistant:\n"),
    }
}
