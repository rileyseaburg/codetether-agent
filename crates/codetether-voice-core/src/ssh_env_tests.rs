//! Tests for SSH session detection.

use super::is_ssh_session_from;

#[test]
fn detects_ssh_connection_marker() {
    let lookup =
        |key: &str| (key == "SSH_CONNECTION").then(|| "10.0.0.4 51234 10.0.0.9 22".to_string());
    assert!(is_ssh_session_from(lookup));
}

#[test]
fn detects_pty_only_marker() {
    let lookup = |key: &str| (key == "SSH_TTY").then(|| "/dev/pts/3".to_string());
    assert!(is_ssh_session_from(lookup));
}

#[test]
fn local_session_has_no_markers() {
    assert!(!is_ssh_session_from(|_| None));
}

#[test]
fn blank_markers_are_not_ssh() {
    // Exported-but-empty vars must not force forwarding.
    assert!(!is_ssh_session_from(|_| Some("   ".to_string())));
}
