use anyhow::{Result, anyhow};
use cpal::traits::{DeviceTrait, HostTrait};

const DEVICE_ENV: &str = "CODETETHER_VOICE_INPUT_DEVICE";

pub fn select(host: &cpal::Host, requested: Option<&str>) -> Result<cpal::Device> {
    match requested {
        Some(name) if !name.is_empty() => named(host, name),
        _ => select_env_or_default(host),
    }
}

fn select_env_or_default(host: &cpal::Host) -> Result<cpal::Device> {
    match env_name() {
        Some(name) => named(host, &name),
        None => host
            .default_input_device()
            .ok_or_else(|| anyhow!("No audio input device found. Connect a microphone.")),
    }
}

fn env_name() -> Option<String> {
    std::env::var(DEVICE_ENV)
        .ok()
        .filter(|name| !name.is_empty())
}

fn named(host: &cpal::Host, requested: &str) -> Result<cpal::Device> {
    let wanted = requested.to_lowercase();
    for device in host.input_devices()? {
        let name = device.name().unwrap_or_default();
        if name.to_lowercase().contains(&wanted) {
            return Ok(device);
        }
    }
    Err(anyhow!(
        "Input device '{requested}' not found. Available: {}",
        names(host)?.join(", ")
    ))
}

fn names(host: &cpal::Host) -> Result<Vec<String>> {
    Ok(host.input_devices()?.map(device_name).collect())
}

fn device_name(device: cpal::Device) -> String {
    device.name().unwrap_or_else(|_| "<unknown>".into())
}
