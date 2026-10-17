//! Windows-native voice via WASAPI.
//!
//! Replaces PowerShell-mediated audio with direct Windows Audio Session API
//! calls for lower latency and better reliability than shelling out.

mod capture;
mod player;

pub use capture::WavCapture;
pub use player::WavPlayer;

/// Record audio from the default microphone.
///
/// Wraps WASAPI capture to produce 16-bit, 16 kHz mono WAV bytes.
///
/// # Errors
///
/// Returns an error if the audio device is unavailable or capture fails.
pub fn record(duration_secs: u32) -> anyhow::Result<Vec<u8>> {
    capture::WavCapture::record(duration_secs)
}

/// Play WAV audio through the default speakers.
///
/// # Errors
///
/// Returns an error if the audio device is unavailable.
pub fn play(wav_bytes: &[u8]) -> anyhow::Result<()> {
    player::WavPlayer::play(wav_bytes)
}
