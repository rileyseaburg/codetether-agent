//! WAV encoder — converts i16 PCM samples to a WAV byte buffer.

use anyhow::Result;
use std::io::Cursor;

/// Encode mono 16kHz 16-bit PCM samples into a WAV byte buffer.
///
/// # Arguments
/// * `samples` - Signed mono PCM samples already resampled to 16kHz.
/// # Returns
/// A complete in-memory WAV file.
/// # Errors
/// Returns an error if the WAV header, samples or finalization cannot be written.
/// # Examples
/// ```rust
/// let wav = codetether_voice_input_tool::encoder::encode_wav(&[0, 1024, -1024])?;
/// assert_eq!(&wav[..4], b"RIFF");
/// # Ok::<(), anyhow::Error>(())
/// ```
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
