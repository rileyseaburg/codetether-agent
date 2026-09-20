//! Tests for audio transport selection.

use super::{AudioTransport, SessionEnv, select};

#[test]
fn local_windows_or_linux_uses_device() {
    let env = SessionEnv {
        is_ssh: false,
        has_audio_device: true,
    };
    assert_eq!(select(env), AudioTransport::LocalDevice);
}

#[test]
fn ssh_always_forwards_even_with_remote_device() {
    // Remote Ubuntu may expose ALSA, but the operator hears the client.
    let env = SessionEnv {
        is_ssh: true,
        has_audio_device: true,
    };
    assert_eq!(select(env), AudioTransport::ForwardedFromClient);
}

#[test]
fn headless_host_without_device_forwards() {
    let env = SessionEnv {
        is_ssh: false,
        has_audio_device: false,
    };
    assert_eq!(select(env), AudioTransport::ForwardedFromClient);
}
