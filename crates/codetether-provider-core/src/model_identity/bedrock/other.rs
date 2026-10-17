//! Resolve one family of Bedrock model aliases.

pub(super) fn resolve(model: &str) -> Option<&'static str> {
    Some(match model {
        // --- DeepSeek ---
        "deepseek-r1" => "us.deepseek.r1-v1:0",
        "deepseek-v3" | "deepseek-v3.2" => "deepseek.v3.2",

        // --- Cohere ---
        "command-r" => "cohere.command-r-v1:0",
        "command-r-plus" => "cohere.command-r-plus-v1:0",

        // --- Qwen ---
        "qwen3-32b" => "qwen.qwen3-32b-v1:0",
        "qwen3-coder" | "qwen3-coder-next" => "qwen.qwen3-coder-next",
        "qwen3-coder-30b" => "qwen.qwen3-coder-30b-a3b-v1:0",

        // --- Google Gemma ---
        "gemma-3-27b" => "google.gemma-3-27b-it",
        "gemma-3-12b" => "google.gemma-3-12b-it",
        "gemma-3-4b" => "google.gemma-3-4b-it",

        // --- Moonshot / Kimi ---
        "kimi-k2" | "kimi-k2-thinking" => "moonshot.kimi-k2-thinking",
        "kimi-k2.5" => "moonshotai.kimi-k2.5",

        // --- AI21 Jamba ---
        "jamba-1.5-large" => "ai21.jamba-1-5-large-v1:0",
        "jamba-1.5-mini" => "ai21.jamba-1-5-mini-v1:0",

        // --- MiniMax ---
        "minimax-m2" => "minimax.minimax-m2",
        "minimax-m2.1" => "minimax.minimax-m2.1",

        // --- NVIDIA ---
        "nemotron-nano-30b" => "nvidia.nemotron-nano-3-30b",
        "nemotron-nano-12b" => "nvidia.nemotron-nano-12b-v2",
        "nemotron-nano-9b" => "nvidia.nemotron-nano-9b-v2",

        // --- Z.AI / GLM ---
        "glm-5" => "zai.glm-5",
        "glm-4.7" => "zai.glm-4.7",
        "glm-4.7-flash" => "zai.glm-4.7-flash",

        _ => return None,
    })
}
