//! WASAPI audio playback for TTS output on Windows.

// WASAPI imports will be activated when the render loop is wired up:
// use windows::Win32::Media::Audio::*;

/// Plays 16-bit PCM WAV bytes through the default audio endpoint.
///
/// # Errors
///
/// Returns an error if WASAPI playback init or buffer write fails.
///
/// # Examples
///
/// ```rust,ignore
/// let wav = std::fs::read("voice_abc.wav")?;
/// codetether_agent::platform::windows::voice::WavPlayer::play(&wav)?;
/// ```
pub struct WavPlayer;

impl WavPlayer {
    pub fn play(wav_bytes: &[u8]) -> anyhow::Result<()> {
        unsafe { play_inner(wav_bytes) }
    }
}

unsafe fn play_inner(wav_bytes: &[u8]) -> anyhow::Result<()> {
    // Parse WAV to extract raw PCM data.
    let pcm = strip_wav_header(wav_bytes)?;

    // WASAPI render loop placeholder — full implementation needs COM init,
    // IAudioClient activation for eRender, and buffer service loop.
    let _ = pcm;

    // Fallback: write to temp file and open with default player
    let path = std::env::temp_dir().join(format!("ct_play_{}.wav", uuid::Uuid::new_v4()));
    std::fs::write(&path, wav_bytes)?;
    open::that(&path)?;

    Ok(())
}

/// Strip the 44-byte WAV header, returning raw PCM samples.
fn strip_wav_header(wav: &[u8]) -> anyhow::Result<&[u8]> {
    if wav.len() < 44 {
        anyhow::bail!("WAV data too short: {} bytes", wav.len());
    }
    if &wav[0..4] != b"RIFF" || &wav[8..12] != b"WAVE" {
        anyhow::bail!("not a valid WAV file");
    }
    // Find the "data" chunk (may not be at offset 36 if extra chunks exist)
    let mut offset = 12usize;
    while offset + 8 <= wav.len() {
        let id = &wav[offset..offset + 4];
        let size = u32::from_le_bytes(wav[offset + 4..offset + 8].try_into()?) as usize;
        if id == b"data" {
            let end = (offset + 8 + size).min(wav.len());
            return Ok(&wav[offset + 8..end]);
        }
        offset += 8 + size;
    }
    anyhow::bail!("no data chunk found in WAV")
}
