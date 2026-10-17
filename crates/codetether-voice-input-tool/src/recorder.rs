//! Microphone recorder using cpal — captures 16kHz mono i16 samples.

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use cpal::traits::StreamTrait;

use super::stderr_guard::silence_alsa;
use super::{device, input_stream};

/// Capture audio from the default microphone as 16kHz mono PCM samples.
///
/// # Arguments
/// * `max_duration_secs` - Maximum capture duration.
/// * `stop_flag` - Shared flag allowing the caller to stop capture early.
/// # Returns
/// Captured mono 16kHz PCM samples.
/// # Errors
/// Returns an error if device selection, configuration, playback, or buffer access fails.
/// # Panics
/// May panic if the duration cannot be represented by the platform clock.
/// # Examples
/// ```rust,no_run
/// use std::sync::{Arc, atomic::AtomicBool};
/// let stop = Arc::new(AtomicBool::new(false));
/// let samples = codetether_voice_input_tool::recorder::record(5, stop)?;
/// assert!(!samples.is_empty());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn record(max_duration_secs: u64, stop_flag: Arc<AtomicBool>) -> Result<Vec<i16>> {
    record_from(None, max_duration_secs, stop_flag)
}

/// Capture audio from a selected microphone as 16kHz mono PCM samples.
///
/// # Arguments
/// * `requested_device` - Optional device-name substring; otherwise use environment or host default.
/// * `max_duration_secs` - Maximum capture duration.
/// * `stop_flag` - Shared flag allowing the caller to stop capture early.
/// # Returns
/// Captured mono 16kHz PCM samples.
/// # Errors
/// Returns an error if device selection, configuration, playback, or buffer access fails.
/// # Panics
/// May panic if the duration cannot be represented by the platform clock.
/// # Examples
/// ```rust,no_run
/// use std::sync::{Arc, atomic::AtomicBool};
/// let stop = Arc::new(AtomicBool::new(false));
/// let samples = codetether_voice_input_tool::recorder::record_from(None, 5, stop)?;
/// assert!(!samples.is_empty());
/// # Ok::<(), anyhow::Error>(())
/// ```
pub fn record_from(
    requested_device: Option<&str>,
    max_duration_secs: u64,
    stop_flag: Arc<AtomicBool>,
) -> Result<Vec<i16>> {
    let host = cpal::default_host();
    let device = silence_alsa(|| device::select(&host, requested_device))?;
    let (stream, samples) = silence_alsa(|| input_stream::build(&device))?;
    silence_alsa(|| stream.play())?;
    tracing::info!(target: "codetether_agent::tool::voice_input::recorder", max_duration_secs, "Recording started");
    wait_for_stop(max_duration_secs, &stop_flag);
    drop(stream);
    tracing::info!(target: "codetether_agent::tool::voice_input::recorder", "Recording stopped");
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
