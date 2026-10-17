//! Average one interleaved frame into a mono i16 sample.
use cpal::Sample;

pub(crate) fn mono_i16<T>(frame: &[T]) -> i16
where
    T: Sample,
    i16: cpal::FromSample<T>,
{
    let total: i32 = frame
        .iter()
        .map(|sample| i32::from(sample.to_sample::<i16>()))
        .sum();
    (total / frame.len().max(1) as i32) as i16
}
