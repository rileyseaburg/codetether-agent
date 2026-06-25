use super::input_stream::SampleBuffer;
use anyhow::Result;
use cpal::traits::DeviceTrait;
use cpal::{Sample, SizedSample};

pub fn typed<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    samples: SampleBuffer,
) -> Result<cpal::Stream>
where
    T: Sample + SizedSample + Send + 'static,
    i16: cpal::FromSample<T>,
{
    Ok(device.build_input_stream(
        config,
        move |data: &[T], _| push(data, &samples),
        |err| tracing::error!(error = %err, "Audio capture error"),
        None,
    )?)
}

fn push<T>(data: &[T], samples: &SampleBuffer)
where
    T: Sample,
    i16: cpal::FromSample<T>,
{
    if let Ok(mut buf) = samples.lock() {
        buf.push(data);
    }
}
