//! build model selector for codebase indexing.

use super::*;

pub(in crate::indexer) fn build_model_selector(provider: &str, model: &str) -> Result<String> {
    let provider = provider.trim();
    let model = model.trim();

    if model.is_empty() {
        anyhow::bail!("embedding model cannot be empty");
    }

    if model.contains('/') {
        return Ok(model.to_string());
    }

    if provider.is_empty() {
        anyhow::bail!(
            "embedding provider cannot be empty when model does not include provider prefix"
        );
    }

    Ok(format!("{provider}/{model}"))
}
