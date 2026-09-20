//! Audio transport selection for local, Windows, and SSH sessions.
//!
//! Full-duplex voice needs a capture device. Over SSH there is no local
//! audio device on the remote host, so the session must forward frames
//! from the client instead of opening ALSA/WASAPI on the server.

/// Where voice frames enter and leave the process.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioTransport {
    /// Native OS capture: WASAPI on Windows, ALSA/PulseAudio on Linux.
    LocalDevice,
    /// No server-side device; frames are relayed from the connecting client.
    ForwardedFromClient,
}

/// Session facts needed to pick a transport.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SessionEnv {
    /// True when the process was started through an SSH login.
    pub is_ssh: bool,
    /// True when a local audio device was detected.
    pub has_audio_device: bool,
}

/// Choose a transport for the current session.
///
/// SSH always forwards, even if the remote host happens to expose a
/// device, because that device is not where the operator is listening.
///
/// # Examples
///
/// ```rust
/// use codetether_agent::voice::transport::{AudioTransport, SessionEnv, select};
///
/// // Windows workstation, local mic.
/// let local = SessionEnv { is_ssh: false, has_audio_device: true };
/// assert_eq!(select(local), AudioTransport::LocalDevice);
///
/// // Windows -> SSH -> remote Ubuntu: forward from the client.
/// let remote = SessionEnv { is_ssh: true, has_audio_device: true };
/// assert_eq!(select(remote), AudioTransport::ForwardedFromClient);
/// ```
pub fn select(env: SessionEnv) -> AudioTransport {
    if env.is_ssh || !env.has_audio_device {
        return AudioTransport::ForwardedFromClient;
    }
    AudioTransport::LocalDevice
}

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

#[cfg(test)]
#[path = "transport_tests.rs"]
mod tests;
