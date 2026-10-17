//! Shared routing policy and native audio-device probing.

pub use codetether_voice_core::transport::*;

/// Select a transport from the live process environment.
///
/// Detects SSH via [`crate::voice::ssh_env`]; `has_audio_device` is
/// supplied by the caller's device probe.
pub fn select_for_process(has_audio_device: bool) -> AudioTransport {
    select(SessionEnv {
        is_ssh: crate::voice::ssh_env::is_ssh_session(),
        has_audio_device,
    })
}

/// Probe the host for a default input device via `cpal`.
///
/// Returns `false` when no host input device is present, which is the
/// normal case on a headless remote Ubuntu box.
pub fn has_default_input_device() -> bool {
    use cpal::traits::HostTrait;
    cpal::default_host().default_input_device().is_some()
}
