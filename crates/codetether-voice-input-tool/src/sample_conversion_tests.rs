//! Exercise packed 24-bit formats through the production PCM conversion.
use super::resampler::Resampler;

fn resampler() -> Resampler {
    Resampler::new(&cpal::StreamConfig {
        channels: 1,
        sample_rate: 16_000,
        buffer_size: cpal::BufferSize::Default,
    })
}

#[test]
fn signed_24_bit_samples_convert_to_pcm16() {
    let mut samples = resampler();
    samples.push(&[
        cpal::I24::new(-8_388_608).unwrap(),
        cpal::I24::new(0).unwrap(),
        cpal::I24::new(8_388_607).unwrap(),
    ]);
    assert_eq!(samples.take(), [i16::MIN, 0, i16::MAX]);
}

#[test]
fn unsigned_24_bit_samples_convert_to_pcm16() {
    let mut samples = resampler();
    samples.push(&[
        cpal::U24::new(0).unwrap(),
        cpal::U24::new(8_388_608).unwrap(),
        cpal::U24::new(16_777_215).unwrap(),
    ]);
    assert_eq!(samples.take(), [i16::MIN, 0, i16::MAX]);
}
