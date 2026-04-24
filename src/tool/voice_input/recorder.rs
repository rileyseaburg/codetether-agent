//! Microphone recorder using cpal — captures 16kHz mono i16 samples.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::{Result, anyhow};
use cpal::traits::{HostTrait, StreamTrait};

use super::input_stream;

/// Capture audio from the default microphone at 16kHz mono.
///
/// Records until `stop_flag` is set or `max_duration_secs` elapses.
/// Returns raw i16 PCM samples.
pub fn record(max_duration_secs: u64, stop_flag: Arc<AtomicBool>) -> Result<Vec<i16>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No audio input device found. Connect a microphone."))?;

    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let stream = input_stream::build(&device, samples.clone())?;

    stream.play()?;
    tracing::info!("Recording started (max {max_duration_secs}s)");

    let deadline = Instant::now() + Duration::from_secs(max_duration_secs);
    while !stop_flag.load(Ordering::Relaxed) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
    }

    drop(stream);
    tracing::info!("Recording stopped");

    let mut samples = samples
        .lock()
        .map_err(|_| anyhow!("Recorded audio buffer lock poisoned"))?;
    Ok(std::mem::take(&mut *samples))
}
