//! WAV encoder — converts i16 PCM samples to a WAV byte buffer.

use anyhow::Result;

/// Encode mono 16kHz 16-bit PCM samples into a WAV byte buffer.
pub fn encode_wav(samples: &[i16]) -> Result<Vec<u8>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut buf = Vec::new();
    let mut writer = hound::WavWriter::new(&mut buf, spec)?;
    for &s in samples {
        writer.write_sample(s)?;
    }
    writer.finalize()?;
    Ok(buf)
}
