//! Audio transport selection for local, Windows, and SSH sessions.
//!
//! Full-duplex voice needs a capture device. Over SSH there is no local
//! audio device on the remote host, so the session must forward frames
//! from the client instead of opening ALSA/WASAPI on the server.

/// Where voice frames enter and leave the process.
///
/// # Examples
/// ```
/// use codetether_voice_core::transport::{AudioTransport, SessionEnv, select};
/// match select(SessionEnv { is_ssh: true, has_audio_device: false }) {
///     AudioTransport::ForwardedFromClient => {},
///     AudioTransport::LocalDevice => panic!("SSH should forward"),
/// }
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AudioTransport {
    /// Native OS capture: WASAPI on Windows, ALSA/PulseAudio on Linux.
    LocalDevice,
    /// No server-side device; frames are relayed from the connecting client.
    ForwardedFromClient,
}

/// Session facts needed to pick a transport.
///
/// # Examples
/// ```
/// use codetether_voice_core::transport::SessionEnv;
/// let env = SessionEnv { is_ssh: false, has_audio_device: true };
/// assert!(env.has_audio_device);
/// ```
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
/// # Arguments
/// * `env` - SSH presence and local input-device availability.
///
/// # Returns
/// Client forwarding for SSH/headless sessions, otherwise local capture.
///
/// # Examples
///
/// ```rust
/// use codetether_voice_core::transport::{AudioTransport, SessionEnv, select};
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

#[cfg(test)]
#[path = "transport_tests.rs"]
mod tests;
