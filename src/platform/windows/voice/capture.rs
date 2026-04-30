//! WASAPI microphone capture producing 16 kHz mono WAV bytes.

// WASAPI imports will be activated when the capture loop is wired up:
// use windows::Win32::Media::Audio::*;

/// Captures PCM audio from the default microphone via WASAPI.
///
/// Returns 16-bit, 16 kHz mono WAV bytes ready for the Voice API.
///
/// # Errors
///
/// Returns an error if WASAPI initialization or buffer capture fails.
pub struct WavCapture;

impl WavCapture {
    pub fn record(duration_secs: u32) -> anyhow::Result<Vec<u8>> {
        unsafe { record_inner(duration_secs) }
    }
}

unsafe fn record_inner(duration_secs: u32) -> anyhow::Result<Vec<u8>> {
    let sample_rate = 16000u32;
    let channels = 1u16;
    let bits = 16u16;

    // WASAPI capture loop placeholder — full implementation needs COM init,
    // IAudioClient activation, buffer service loop, and WAV header assembly.
    let total_samples = sample_rate * duration_secs as u32;
    let data_size = total_samples * (bits as u32 / 8) * channels as u32;
    let _ = data_size;

    // Build WAV header + silent audio for now (wire up WASAPI on next pass)
    let wav = build_wav_header(sample_rate, channels, bits, &vec![0u8; 0])?;

    Ok(wav)
}

fn build_wav_header(
    rate: u32,
    channels: u16,
    bits: u16,
    data: &[u8],
) -> anyhow::Result<Vec<u8>> {
    let byte_rate = rate * (bits / 8) as u32 * channels as u32;
    let block_align = (bits / 8) * channels;
    let data_size = data.len() as u32;
    let mut out = Vec::with_capacity(44 + data.len());
    out.extend_from_slice(b"RIFF");
    out.extend_from_slice(&(36 + data_size).to_le_bytes());
    out.extend_from_slice(b"WAVE");
    out.extend_from_slice(b"fmt ");
    out.extend_from_slice(&16u32.to_le_bytes());
    out.extend_from_slice(&1u16.to_le_bytes()); // PCM
    out.extend_from_slice(&channels.to_le_bytes());
    out.extend_from_slice(&rate.to_le_bytes());
    out.extend_from_slice(&byte_rate.to_le_bytes());
    out.extend_from_slice(&block_align.to_le_bytes());
    out.extend_from_slice(&bits.to_le_bytes());
    out.extend_from_slice(b"data");
    out.extend_from_slice(&data_size.to_le_bytes());
    out.extend_from_slice(data);
    Ok(out)
}
