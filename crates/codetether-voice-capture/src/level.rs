//! Captured-audio sanity checks that explain silent or empty recordings.

/// Platform hint shown when the microphone produced no usable audio.
#[cfg(windows)]
pub(crate) const MIC_HINT: &str = "Check Settings > Privacy & security > Microphone (enable 'Let desktop apps access your microphone') and the default input device in Sound settings, or set CODETETHER_VOICE_INPUT_DEVICE.";
/// Platform hint shown when the microphone produced no usable audio.
#[cfg(not(windows))]
pub(crate) const MIC_HINT: &str =
    "Check the default input device and its volume, or set CODETETHER_VOICE_INPUT_DEVICE.";

/// Peak amplitude below which a recording is treated as silence.
const SILENCE_PEAK: u16 = 64;
/// Shortest recording (16 kHz samples) worth sending: 0.25 s.
const MIN_SAMPLES: usize = 4_000;

/// Largest absolute sample value in the recording.
pub(crate) fn peak(samples: &[i16]) -> u16 {
    samples.iter().map(|s| s.unsigned_abs()).max().unwrap_or(0)
}

/// Reject empty, too-short, or all-silent recordings with a user-facing reason.
pub(crate) fn check(samples: &[i16]) -> Result<(), String> {
    if samples.is_empty() {
        return Err(format!("No audio captured from the microphone. {MIC_HINT}"));
    }
    if samples.len() < MIN_SAMPLES {
        return Err("Recording too short — hold for at least a second before Ctrl+R".into());
    }
    if peak(samples) < SILENCE_PEAK {
        return Err(format!("Microphone recorded only silence. {MIC_HINT}"));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::check;

    #[test]
    fn empty_audio_is_rejected() {
        assert!(check(&[]).unwrap_err().contains("No audio"));
    }

    #[test]
    fn silent_audio_is_rejected() {
        assert!(check(&[0; 16_000]).unwrap_err().contains("silence"));
    }

    #[test]
    fn short_audio_is_rejected() {
        assert!(check(&[2_000; 100]).unwrap_err().contains("too short"));
    }

    #[test]
    fn audible_audio_is_accepted() {
        let samples: Vec<i16> = (0..16_000).map(|i| ((i % 200) * 50) as i16).collect();
        assert!(check(&samples).is_ok());
    }
}
