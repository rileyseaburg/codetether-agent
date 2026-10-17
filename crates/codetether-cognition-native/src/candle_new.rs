//! Dispatch custom packed formats before the stock GGUF parser.
use super::{ThinkerConfig, candle_runtime::CandleThinker};
impl CandleThinker {
    /// Load the selected native Candle model.
    /// Returns an error for invalid/unsupported metadata or inaccessible weights.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_cognition_native::{CandleThinker, ThinkerConfig};
    /// # fn example(config: &ThinkerConfig) -> anyhow::Result<()> {
    /// let _runtime = CandleThinker::new(config)?;
    /// # Ok(())
    /// # }
    /// ```
    pub fn new(config: &ThinkerConfig) -> anyhow::Result<Self> {
        if let Some(model) = super::bonsai::try_load(config)? {
            return Ok(model);
        }
        Self::new_generic(config)
    }
}
