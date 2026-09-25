use std::path::Path;
use std::process::{Command, Stdio};

#[path = "sandbox_bwrap_smoke_args.rs"]
mod smoke_args;
pub(super) use smoke_args::SMOKE_ARGS;

pub(super) fn version(path: &Path) -> Option<String> {
    let output = Command::new(path)
        .arg("--version")
        .stdin(Stdio::null())
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    parse_version(&output.stdout).or_else(|| parse_version(&output.stderr))
}

pub(super) fn smoke(path: &Path) -> bool {
    Command::new(path)
        .args(SMOKE_ARGS)
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .is_ok_and(|status| status.success())
}

pub(super) fn parse_version(bytes: &[u8]) -> Option<String> {
    String::from_utf8_lossy(bytes)
        .split_whitespace()
        .find(|part| part.starts_with(|ch: char| ch.is_ascii_digit()))
        .map(str::to_string)
}