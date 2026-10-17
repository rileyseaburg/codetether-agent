//! Persistent in-process runtime; this path does not construct a ThinkerClient.
use super::{BonsaiConfig, native::Loaded};
use anyhow::Result;
use candle_core::Device;
pub(super) struct Runtime {
    pub loaded: Loaded,
    pub requests: u64,
}
impl Runtime {
    pub fn load(config: &BonsaiConfig) -> Result<Self> {
        let device = Device::new_cuda(config.cuda_ordinal)?;
        let loaded = super::native::open(&config.model_path, &config.tokenizer_path, device)?;
        loaded.device.synchronize()?;
        Ok(Self {
            loaded,
            requests: 0,
        })
    }
}
/// Load weights exactly once; sampling changes never reload the model.
pub(super) fn ensure_loaded(slot: &mut Option<Runtime>, config: &BonsaiConfig) -> Result<f64> {
    if slot.is_some() {
        return Ok(0.0);
    }
    let started = std::time::Instant::now();
    *slot = Some(Runtime::load(config)?);
    Ok(started.elapsed().as_secs_f64() * 1000.0)
}
