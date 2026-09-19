//! Dispatch custom packed formats before the stock GGUF parser.
use super::{ThinkerConfig, candle_runtime::CandleThinker};
impl CandleThinker {
    /// Load the selected native Candle model.
    /// Returns an error for invalid/unsupported metadata or inaccessible weights.
    pub(crate) fn new(config: &ThinkerConfig) -> anyhow::Result<Self> {
        if let Some(model) = super::bonsai::try_load(config)? {
            return Ok(model);
        }
        Self::new_generic(config)
    }
}
