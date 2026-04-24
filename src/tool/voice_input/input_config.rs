use anyhow::{Result, anyhow};
use cpal::traits::DeviceTrait;

pub struct InputConfig {
    pub format: cpal::SampleFormat,
    pub stream: cpal::StreamConfig,
}

pub fn select(device: &cpal::Device) -> Result<InputConfig> {
    let supported = device
        .supported_input_configs()?
        .find(|range| {
            range.channels() == 1
                && range.min_sample_rate() <= cpal::SampleRate(16000)
                && range.max_sample_rate() >= cpal::SampleRate(16000)
        })
        .ok_or_else(|| anyhow!("Default input device does not support 16kHz mono capture."))?;
    Ok(InputConfig {
        format: supported.sample_format(),
        stream: supported.with_sample_rate(cpal::SampleRate(16000)).config(),
    })
}
