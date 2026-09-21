//! The first native graph only supports inverse rotation on token lookup.
use super::Index;
use anyhow::{Context, Result, ensure};
use serde_json::Value;
pub(super) fn validate(index: &Index) -> Result<()> {
    let inverse = index
        .metadata
        .get("prism.hadamard.inverse_weight_names")
        .and_then(Value::as_array)
        .context("Missing inverse rotation mapping")?;
    ensure!(
        inverse.len() == 1 && inverse[0].as_str() == Some("token_embd.weight"),
        "Unsupported inverse Hadamard tensor mapping"
    );
    let rows = index
        .tensors
        .get("token_embd.weight")
        .context("Missing embeddings")?
        .shape
        .get(1)
        .copied()
        .context("Invalid embedding rank")?;
    for key in ["tokenizer.ggml.tokens", "tokenizer.ggml.token_type"] {
        ensure!(
            index
                .metadata
                .get(key)
                .and_then(Value::as_array)
                .is_some_and(|v| v.len() == rows),
            "Vocabulary metadata does not match embedding rows"
        );
    }
    Ok(())
}
