use super::input_builder;
use super::{input_config, resampler::Resampler};
use anyhow::{Result, anyhow};
use cpal::SampleFormat;
use std::sync::{Arc, Mutex};

pub type SampleBuffer = Arc<Mutex<Resampler>>;

pub fn build(device: &cpal::Device) -> Result<(cpal::Stream, SampleBuffer)> {
    let config = input_config::select(device)?;
    let samples = Arc::new(Mutex::new(Resampler::new(&config.stream)));
    let stream = build_stream(device, &config.stream, config.format, samples.clone())?;
    Ok((stream, samples))
}

fn build_stream(
    device: &cpal::Device,
    stream: &cpal::StreamConfig,
    format: SampleFormat,
    samples: SampleBuffer,
) -> Result<cpal::Stream> {
    match format {
        SampleFormat::I8 => input_builder::typed::<i8>(device, stream, samples),
        SampleFormat::I16 => input_builder::typed::<i16>(device, stream, samples),
        SampleFormat::I32 => input_builder::typed::<i32>(device, stream, samples),
        SampleFormat::I64 => input_builder::typed::<i64>(device, stream, samples),
        SampleFormat::U8 => input_builder::typed::<u8>(device, stream, samples),
        SampleFormat::U16 => input_builder::typed::<u16>(device, stream, samples),
        SampleFormat::U32 => input_builder::typed::<u32>(device, stream, samples),
        SampleFormat::U64 => input_builder::typed::<u64>(device, stream, samples),
        SampleFormat::F32 => input_builder::typed::<f32>(device, stream, samples),
        SampleFormat::F64 => input_builder::typed::<f64>(device, stream, samples),
        other => Err(anyhow!("Unsupported input sample format: {other:?}")),
    }
}
