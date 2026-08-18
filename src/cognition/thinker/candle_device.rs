//! Candle compute-device selection.

use anyhow::{Context, Result, anyhow};
use candle_core::Device;

use super::{CandleDevicePreference, ThinkerConfig};

/// Resolve the compute device and a human-readable label for logging.
///
/// # Errors
///
/// Returns an error when CUDA is explicitly required but unavailable.
pub(super) fn select_candle_device(config: &ThinkerConfig) -> Result<(Device, String)> {
    match config.candle_device {
        CandleDevicePreference::Cpu => Ok((Device::Cpu, "cpu".to_string())),
        CandleDevicePreference::Cuda => {
            let device = try_cuda_device(config.candle_cuda_ordinal)?;
            Ok((device, format!("cuda:{}", config.candle_cuda_ordinal)))
        }
        CandleDevicePreference::Auto => Ok(auto_device(config)),
    }
}

#[cfg(not(feature = "candle-cuda"))]
fn auto_device(_config: &ThinkerConfig) -> (Device, String) {
    (Device::Cpu, "cpu".to_string())
}

#[cfg(feature = "candle-cuda")]
fn auto_device(config: &ThinkerConfig) -> (Device, String) {
    match try_cuda_device(config.candle_cuda_ordinal) {
        Ok(device) => {
            tracing::info!(
                ordinal = config.candle_cuda_ordinal,
                "Candle thinker selected CUDA device"
            );
            (device, format!("cuda:{}", config.candle_cuda_ordinal))
        }
        Err(error) => {
            tracing::warn!(%error, "CUDA unavailable for Candle thinker, falling back to CPU");
            (Device::Cpu, "cpu".to_string())
        }
    }
}

#[cfg(feature = "candle-cuda")]
fn try_cuda_device(ordinal: usize) -> Result<Device> {
    Device::new_cuda(ordinal)
        .with_context(|| format!("failed to initialize CUDA device ordinal {ordinal}"))
}

#[cfg(not(feature = "candle-cuda"))]
fn try_cuda_device(_ordinal: usize) -> Result<Device> {
    Err(anyhow!("rebuild with --features candle-cuda"))
}
