//! Microphone recorder using cpal — captures 16kHz mono i16 samples.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use anyhow::Result;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

/// Capture audio from the default microphone at 16kHz mono.
///
/// Records until `stop_flag` is set or `max_duration_secs` elapses.
/// Returns raw i16 PCM samples.
pub fn record(max_duration_secs: u64, stop_flag: Arc<AtomicBool>) -> Result<Vec<i16>> {
    let host = cpal::default_host();
    let device = host
        .default_input_device()
        .ok_or_else(|| anyhow::anyhow!("No audio input device found. Connect a microphone."))?;

    let config = cpal::StreamConfig {
        channels: 1,
        sample_rate: cpal::SampleRate(16000),
        buffer_size: cpal::BufferSize::Default,
    };

    let samples: Arc<Mutex<Vec<i16>>> = Arc::new(Mutex::new(Vec::new()));
    let sc = samples.clone();

    let stream = device.build_input_stream(
        &config,
        move |data: &[f32], _: &cpal::InputCallbackInfo| {
            let mut buf = sc.lock().unwrap();
            for &s in data {
                buf.push((s * 32767.0).clamp(-32768.0, 32767.0) as i16);
            }
        },
        |err| tracing::error!("Audio capture error: {err}"),
        None,
    )?;

    stream.play()?;
    tracing::info!("Recording started (max {max_duration_secs}s)");

    let deadline = Instant::now() + Duration::from_secs(max_duration_secs);
    while !stop_flag.load(Ordering::Relaxed) && Instant::now() < deadline {
        std::thread::sleep(Duration::from_millis(100));
    }

    drop(stream);
    tracing::info!("Recording stopped");

    Ok(Arc::try_unwrap(samples).unwrap().into_inner().unwrap())
}
