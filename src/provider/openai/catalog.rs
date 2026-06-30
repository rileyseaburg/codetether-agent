//! Static default model catalogs for OpenAI-compatible providers.

use crate::provider::ModelInfo;

use super::OpenAIProvider;

impl OpenAIProvider {
    /// Return known models for specific OpenAI-compatible providers.
    pub(super) fn provider_default_models(&self) -> Vec<ModelInfo> {
        default_pairs(&self.provider_name)
            .into_iter()
            .map(|(id, name)| ModelInfo {
                id: id.to_string(),
                name: name.to_string(),
                provider: self.provider_name.clone(),
                context_window: 128_000,
                max_output_tokens: Some(16_384),
                supports_vision: false,
                supports_tools: true,
                supports_streaming: true,
                input_cost_per_million: None,
                output_cost_per_million: None,
            })
            .collect()
    }
}

fn default_pairs(provider_name: &str) -> Vec<(&'static str, &'static str)> {
    match provider_name {
        "cerebras" => vec![
            ("gpt-oss-120b", "GPT-OSS 120B"),
            ("gemma-4-31b", "Gemma 4 31B (Preview)"),
            ("zai-glm-4.7", "GLM-4.7 (Z.ai on Cerebras)"),
        ],
        "minimax" => vec![
            ("MiniMax-M2.5", "MiniMax M2.5"),
            ("MiniMax-M2.5-highspeed", "MiniMax M2.5 Highspeed"),
            ("MiniMax-M2.1", "MiniMax M2.1"),
            ("MiniMax-M2.1-highspeed", "MiniMax M2.1 Highspeed"),
            ("MiniMax-M2", "MiniMax M2"),
        ],
        "novita" => vec![
            ("Qwen/Qwen3.5-35B-A3B", "Qwen 3.5 35B A3B"),
            ("deepseek/deepseek-v3-0324", "DeepSeek V3"),
            ("meta-llama/llama-3.1-70b-instruct", "Llama 3.1 70B"),
            ("meta-llama/llama-3.1-8b-instruct", "Llama 3.1 8B"),
        ],
        _ => vec![],
    }
}
