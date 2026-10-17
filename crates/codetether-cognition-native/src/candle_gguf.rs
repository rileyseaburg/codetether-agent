//! GGUF metadata extraction for Candle model loading.

use candle_core::quantized::gguf_file;

/// Read the trained context length for `architecture` from GGUF metadata.
pub(super) fn detect_context_window(
    content: &gguf_file::Content,
    architecture: &str,
) -> Option<usize> {
    let key = match architecture {
        "qwen2" => "qwen2.context_length",
        "qwen3" | "qwen3moe" | "qwen3_moe" => "qwen3.context_length",
        "gemma" | "gemma2" | "gemma3" | "gemma-embedding" => return gemma_context_window(content),
        _ => "llama.context_length",
    };
    content
        .metadata
        .get(key)
        .and_then(|v| v.to_u32().ok())
        .map(|v| v as usize)
}

/// Gemma GGUFs key context length by generation; try newest first.
fn gemma_context_window(content: &gguf_file::Content) -> Option<usize> {
    for prefix in ["gemma3", "gemma2", "gemma"] {
        let key = format!("{prefix}.context_length");
        if let Some(value) = content.metadata.get(&key) {
            return value.to_u32().ok().map(|v| v as usize);
        }
    }
    None
}

/// Extract EOS token IDs from GGUF metadata before the content is consumed.
pub(super) fn extract_gguf_eos_ids(content: &gguf_file::Content) -> Vec<u32> {
    let mut ids = Vec::new();
    for key in ["tokenizer.ggml.eos_token_id", "tokenizer.ggml.eot_token_id"] {
        if let Some(v) = content.metadata.get(key)
            && let Ok(id) = v.to_u32()
            && !ids.contains(&id)
        {
            ids.push(id);
        }
    }
    ids
}
