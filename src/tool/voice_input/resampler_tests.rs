use super::resampler::Resampler;

fn config(rate: u32, channels: u16) -> cpal::StreamConfig {
    cpal::StreamConfig {
        channels,
        sample_rate: cpal::SampleRate(rate),
        buffer_size: cpal::BufferSize::Default,
    }
}

#[test]
fn downmixes_stereo_to_mono() {
    let mut resampler = Resampler::new(&config(16_000, 2));
    resampler.push(&[1000_i16, -1000, 500, 500]);
    assert_eq!(resampler.take(), vec![0, 500]);
}

#[test]
fn downsamples_48khz_to_16khz() {
    let mut resampler = Resampler::new(&config(48_000, 1));
    resampler.push(&[1_i16, 2, 3, 4, 5, 6]);
    assert_eq!(resampler.take(), vec![1, 4]);
}

#[test]
fn upsamples_8khz_to_16khz() {
    let mut resampler = Resampler::new(&config(8_000, 1));
    resampler.push(&[7_i16, 9]);
    assert_eq!(resampler.take(), vec![7, 7, 9, 9]);
}
