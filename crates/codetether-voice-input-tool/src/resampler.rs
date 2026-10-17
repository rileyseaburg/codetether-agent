//! Convert interleaved input samples to 16kHz mono PCM.
use super::downmix::mono_i16;
use cpal::Sample;

const TARGET_RATE: u64 = 16_000;

pub struct Resampler {
    samples: Vec<i16>,
    channels: usize,
    source_rate: u64,
    frame_index: u64,
    out_index: u64,
}

impl Resampler {
    pub fn new(config: &cpal::StreamConfig) -> Self {
        Self {
            samples: Vec::new(),
            channels: usize::from(config.channels),
            source_rate: u64::from(config.sample_rate),
            frame_index: 0,
            out_index: 0,
        }
    }

    pub fn push<T>(&mut self, data: &[T])
    where
        T: Sample,
        i16: cpal::FromSample<T>,
    {
        for frame in data.chunks(self.channels) {
            self.push_frame(mono_i16(frame));
        }
    }

    pub fn take(&mut self) -> Vec<i16> {
        std::mem::take(&mut self.samples)
    }

    fn push_frame(&mut self, sample: i16) {
        self.frame_index += 1;
        while self.frame_index * TARGET_RATE > self.out_index * self.source_rate {
            self.samples.push(sample);
            self.out_index += 1;
        }
    }
}
