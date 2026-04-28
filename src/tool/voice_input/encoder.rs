//! WAV encoder — converts i16 PCM samples to a WAV byte buffer.

use anyhow::Result;
use std::io::Cursor;

/// Encode mono 16kHz 16-bit PCM samples into a WAV byte buffer.
pub fn encode_wav(samples: &[i16]) -> Result<Vec<u8>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate: 16000,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
        for &sample in samples {
            writer.write_sample(sample)?;
        }
        writer.finalize()?;
    }
    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::encode_wav;

    #[test]
    fn encode_wav_returns_valid_wave_bytes() {
        let bytes = encode_wav(&[0, 1024, -1024]).unwrap();
        assert!(bytes.len() >= 44);
        assert_eq!(&bytes[..4], b"RIFF");
        assert_eq!(&bytes[8..12], b"WAVE");
    }
}
