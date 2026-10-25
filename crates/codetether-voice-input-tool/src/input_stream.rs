//! Construct audio streams for each supported sample format.
use super::{capture_errors::CaptureErrors, input_builder};
use super::{input_config, resampler::Resampler};
use anyhow::{Result, anyhow};
use cpal::SampleFormat;
use std::sync::{Arc, Mutex};

pub type SampleBuffer = Arc<Mutex<Resampler>>;

pub fn build(device: &cpal::Device) -> Result<(cpal::Stream, SampleBuffer, CaptureErrors)> {
    let config = input_config::select(device)?;
    let samples = Arc::new(Mutex::new(Resampler::new(&config.stream)));
    let errors = CaptureErrors::default();
    let stream = build_stream(
        device, &config.stream, config.format, samples.clone(), errors.clone(),
    )?;
    Ok((stream, samples, errors))
}

fn build_stream(
    device: &cpal::Device,
    stream: &cpal::StreamConfig,
    format: SampleFormat,
    samples: SampleBuffer,
    errors: CaptureErrors,
) -> Result<cpal::Stream> {
    match format {
        SampleFormat::I8 => input_builder::typed::<i8>(device, stream, samples, errors),
        SampleFormat::I16 => input_builder::typed::<i16>(device, stream, samples, errors),
        SampleFormat::I24 => input_builder::typed::<cpal::I24>(device, stream, samples, errors),
        SampleFormat::I32 => input_builder::typed::<i32>(device, stream, samples, errors),
        SampleFormat::I64 => input_builder::typed::<i64>(device, stream, samples, errors),
        SampleFormat::U8 => input_builder::typed::<u8>(device, stream, samples, errors),
        SampleFormat::U16 => input_builder::typed::<u16>(device, stream, samples, errors),
        SampleFormat::U24 => input_builder::typed::<cpal::U24>(device, stream, samples, errors),
        SampleFormat::U32 => input_builder::typed::<u32>(device, stream, samples, errors),
        SampleFormat::U64 => input_builder::typed::<u64>(device, stream, samples, errors),
        SampleFormat::F32 => input_builder::typed::<f32>(device, stream, samples, errors),
        SampleFormat::F64 => input_builder::typed::<f64>(device, stream, samples, errors),
        other => Err(anyhow!("Unsupported input sample format: {other:?}")),
    }
}