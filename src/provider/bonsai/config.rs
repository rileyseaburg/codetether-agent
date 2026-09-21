//! Checkpoint and CUDA device configuration without network credentials.
use anyhow::{Context, Result, ensure};
use std::path::PathBuf;
/// Local Bonsai paths and physical device selection.
/// # Examples
/// ```rust,no_run
/// let config = codetether_agent::provider::bonsai::BonsaiConfig::from_environment().unwrap();
/// assert!(config.model_path.is_file());
/// ```
#[derive(Clone, Debug)]
pub struct BonsaiConfig {
    /// Validated Prism PQ2_0 checkpoint path.
    pub model_path: PathBuf,
    /// Tokenizer with matching checkpoint token IDs.
    pub tokenizer_path: PathBuf,
    /// CUDA device ordinal after CUDA_VISIBLE_DEVICES remapping.
    pub cuda_ordinal: usize,
}
impl BonsaiConfig {
    /// Resolve BONSAI_MODEL_PATH/BONSAI_TOKENIZER_PATH, or the installed model directory.
    /// # Errors
    /// Returns an error for missing files, home directory or invalid device ordinal.
    pub fn from_environment() -> Result<Self> {
        let root = std::env::var_os("HOME")
            .or_else(|| std::env::var_os("USERPROFILE"))
            .map(PathBuf::from)
            .context("No home directory")?
            .join(".local/share/codetether/bonsai2/models");
        let model_path = std::env::var_os("BONSAI_MODEL_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("Ternary-Bonsai-2-27B-PQ2_0.gguf"));
        let tokenizer_path = std::env::var_os("BONSAI_TOKENIZER_PATH")
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("tokenizer.json"));
        ensure!(
            model_path.is_file() && tokenizer_path.is_file(),
            "Bonsai checkpoint/tokenizer missing"
        );
        let cuda_ordinal = std::env::var("BONSAI_CUDA_DEVICE")
            .unwrap_or_else(|_| "0".into())
            .parse()?;
        Ok(Self {
            model_path,
            tokenizer_path,
            cuda_ordinal,
        })
    }
}
