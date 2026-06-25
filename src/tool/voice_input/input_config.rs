use anyhow::Result;
use cpal::traits::DeviceTrait;

pub struct InputConfig {
    pub format: cpal::SampleFormat,
    pub stream: cpal::StreamConfig,
}

pub fn select(device: &cpal::Device) -> Result<InputConfig> {
    let supported = device.default_input_config()?;
    Ok(InputConfig {
        format: supported.sample_format(),
        stream: supported.config(),
    })
}
