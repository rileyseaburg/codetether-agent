//! The dedicated provider facade; one resident CUDA model per provider instance.
use super::{BonsaiConfig, GenerationTiming};
use anyhow::{Result, ensure};
use std::sync::{Arc, Mutex};
/// Direct Bonsai CUDA provider, with a TetherScript-owned prompt contract.
/// # Examples
/// ```rust,no_run
/// use codetether_agent::provider::bonsai::{BonsaiProvider, BonsaiConfig};
/// let provider = BonsaiProvider::new(BonsaiConfig::from_environment().unwrap()).unwrap();
/// ```
#[derive(Clone)]
pub struct BonsaiProvider {
    pub(super) config: BonsaiConfig,
    pub(super) timing: Arc<Mutex<Option<GenerationTiming>>>,
    #[cfg(feature = "candle-cuda")]
    pub(super) runtime: Arc<Mutex<Option<super::runtime::Runtime>>>,
}
impl BonsaiProvider {
    /// Construct a provider without loading weights; requires candle-cuda and tetherscript.
    /// # Errors
    /// Returns an error if compiled features or checkpoint files are missing.
    pub fn new(config: BonsaiConfig) -> Result<Self> {
        ensure!(
            cfg!(all(feature = "candle-cuda", feature = "tetherscript")),
            "Bonsai requires --features candle-cuda,tetherscript"
        );
        ensure!(
            config.model_path.is_file() && config.tokenizer_path.is_file(),
            "Bonsai model files missing"
        );
        Ok(Self {
            config,
            timing: Arc::new(Mutex::new(None)),
            #[cfg(feature = "candle-cuda")]
            runtime: Arc::new(Mutex::new(None)),
        })
    }
    /// Return timing from the most recent successful request.
    pub fn last_timing(&self) -> Option<GenerationTiming> {
        self.timing.lock().ok()?.clone()
    }
}
