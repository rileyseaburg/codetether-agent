//! Bind an external tokenizer to the GGUF's explicit vocabulary IDs.
use super::Index;
use anyhow::{Context, Result, ensure};
use tokenizers::Tokenizer;
pub(super) fn validate(tokenizer: &Tokenizer, index: &Index) -> Result<()> {
    let tokens = index
        .metadata
        .get("tokenizer.ggml.tokens")
        .and_then(serde_json::Value::as_array)
        .context("Missing GGUF vocabulary")?;
    for (text, id) in tokenizer.get_vocab(true) {
        ensure!(
            tokens.get(id as usize).and_then(serde_json::Value::as_str) == Some(text.as_str()),
            "Tokenizer ID does not match Bonsai GGUF"
        );
    }
    ensure!(
        index
            .metadata
            .get("tokenizer.ggml.pre")
            .and_then(serde_json::Value::as_str)
            == Some("qwen35"),
        "Unsupported Bonsai pretokenizer"
    );
    Ok(())
}
