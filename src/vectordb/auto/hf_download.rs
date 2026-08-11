//! Download model + tokenizer weights from the HuggingFace Hub.

use super::catalog::ModelSpec;
use anyhow::{Context, Result};
use hf_hub::{HFClient, split_id};
use std::path::PathBuf;

/// Local paths to a downloaded model's files.
#[derive(Debug, Clone)]
pub struct ModelFiles {
    /// `config.json`.
    pub config: PathBuf,
    /// `tokenizer.json`.
    pub tokenizer: PathBuf,
    /// `model.safetensors`.
    pub weights: PathBuf,
}

/// Fetch `config.json`, `tokenizer.json`, and `model.safetensors` for `spec`.
///
/// Files are cached by hf-hub under the standard HF cache dir; repeated calls
/// resolve from disk without re-downloading. The catalog models are public and
/// ungated (Apache-2.0), so no token is required; `HFClient::new` still picks
/// up `HF_TOKEN`/`HF_HOME` when present to avoid anonymous rate limits.
pub async fn download(spec: &ModelSpec) -> Result<ModelFiles> {
    let api = HFClient::new().context("failed to initialize HuggingFace Hub client")?;
    let (owner, name) = split_id(spec.repo);
    let repo = api.model(owner, name);

    let config = repo
        .download_file()
        .filename("config.json")
        .send()
        .await
        .with_context(|| format!("download config.json for {}", spec.repo))?;
    let tokenizer = repo
        .download_file()
        .filename("tokenizer.json")
        .send()
        .await
        .with_context(|| format!("download tokenizer.json for {}", spec.repo))?;
    let weights = repo
        .download_file()
        .filename("model.safetensors")
        .send()
        .await
        .with_context(|| format!("download model.safetensors for {}", spec.repo))?;

    Ok(ModelFiles {
        config,
        tokenizer,
        weights,
    })
}
