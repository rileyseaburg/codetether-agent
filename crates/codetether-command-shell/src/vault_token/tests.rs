//! Tests for exporting the rotated Vault token to shell commands.

use super::apply_from;
use std::ffi::OsStr;
use tokio::process::Command;

/// Returns the `VAULT_TOKEN` override recorded on `cmd`, if any.
fn token_override(cmd: &Command) -> Option<String> {
    cmd.as_std()
        .get_envs()
        .find(|(key, _)| *key == OsStr::new("VAULT_TOKEN"))
        .and_then(|(_, value)| value.map(|v| v.to_string_lossy().into_owned()))
}

#[test]
fn exports_trimmed_token_from_file() {
    let dir = tempfile::tempdir().unwrap();
    let path = dir.path().join("token");
    std::fs::write(&path, "live-token\n").unwrap();
    let mut cmd = Command::new("example");
    apply_from(&mut cmd, Some(path.as_os_str()));
    assert_eq!(token_override(&cmd).as_deref(), Some("live-token"));
}

#[test]
fn leaves_environment_alone_without_a_token_file() {
    let mut cmd = Command::new("example");
    apply_from(&mut cmd, None);
    apply_from(&mut cmd, Some(OsStr::new("")));
    assert_eq!(token_override(&cmd), None);
}

#[test]
fn leaves_environment_alone_when_file_is_missing_or_blank() {
    let dir = tempfile::tempdir().unwrap();
    let blank = dir.path().join("blank");
    std::fs::write(&blank, "  \n").unwrap();
    let mut cmd = Command::new("example");
    apply_from(&mut cmd, Some(dir.path().join("missing").as_os_str()));
    apply_from(&mut cmd, Some(blank.as_os_str()));
    assert_eq!(token_override(&cmd), None);
}
