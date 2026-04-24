use super::input_config;
use anyhow::{Result, anyhow};
use cpal::traits::DeviceTrait;
use cpal::{Sample, SampleFormat, SizedSample};
use std::sync::{Arc, Mutex};

pub fn build(device: &cpal::Device, samples: Arc<Mutex<Vec<i16>>>) -> Result<cpal::Stream> {
    let config = input_config::select(device)?;
    match config.format {
        SampleFormat::I8 => typed::<i8>(device, &config.stream, samples),
        SampleFormat::I16 => typed::<i16>(device, &config.stream, samples),
        SampleFormat::I32 => typed::<i32>(device, &config.stream, samples),
        SampleFormat::I64 => typed::<i64>(device, &config.stream, samples),
        SampleFormat::U8 => typed::<u8>(device, &config.stream, samples),
        SampleFormat::U16 => typed::<u16>(device, &config.stream, samples),
        SampleFormat::U32 => typed::<u32>(device, &config.stream, samples),
        SampleFormat::U64 => typed::<u64>(device, &config.stream, samples),
        SampleFormat::F32 => typed::<f32>(device, &config.stream, samples),
        SampleFormat::F64 => typed::<f64>(device, &config.stream, samples),
        other => Err(anyhow!("Unsupported input sample format: {other:?}")),
    }
}

fn typed<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: Arc<Mutex<Vec<i16>>>,
) -> Result<cpal::Stream>
where
    T: Sample + SizedSample + Send + 'static,
    i16: cpal::FromSample<T>,
{
    Ok(device.build_input_stream(
        config,
        move |data: &[T], _: &cpal::InputCallbackInfo| push(data, &samples),
        |err| tracing::error!("Audio capture error: {err}"),
        None,
    )?)
}

fn push<T>(data: &[T], samples: &Arc<Mutex<Vec<i16>>>)
where
    T: Sample,
    i16: cpal::FromSample<T>,
{
    if let Ok(mut buf) = samples.lock() {
        buf.extend(data.iter().map(|sample| sample.to_sample::<i16>()));
    }
}
