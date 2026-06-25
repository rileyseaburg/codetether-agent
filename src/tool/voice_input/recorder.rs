//! Microphone recorder using cpal — captures 16kHz mono i16 samples.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use cpal::traits::StreamTrait;

use super::stderr_guard::silence_alsa;
use super::{device, input_stream};

/// Capture audio from the default microphone as 16kHz mono PCM samples.
pub fn record(max_duration_secs: u64, stop_flag: Arc<AtomicBool>) -> Result<Vec<i16>> {
    record_from(None, max_duration_secs, stop_flag)
}

/// Capture audio from a selected microphone as 16kHz mono PCM samples.
pub fn record_from(
    requested_device: Option<&str>,
    max_duration_secs: u64,
    stop_flag: Arc<AtomicBool>,
) -> Result<Vec<i16>> {
    let host = cpal::default_host();
    let device = silence_alsa(|| device::select(&host, requested_device))?;
    let (stream, samples) = silence_alsa(|| input_stream::build(&device))?;
    silence_alsa(|| stream.play())?;
    tracing::info!(max_duration_secs, "Recording started");
    wait_for_stop(max_duration_secs, &stop_flag);
    drop(stream);
    tracing::info!("Recording stopped");
    samples
        .lock()
        .map_err(|_| anyhow!("Recorded audio buffer lock poisoned"))
        .map(|mut s| s.take())
}

fn wait_for_stop(max_duration_secs: u64, stop_flag: &AtomicBool) {
    let deadline = Instant::now() + Duration::from_secs(max_duration_secs);
    while !stop_flag.load(Ordering::Relaxed) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
    }
}
