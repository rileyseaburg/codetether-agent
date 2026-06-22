//! Local default-browser launcher for the verification URL.
//!
//! Runs on the machine invoking `codetether connect` (typically Windows),
//! so the SSO approval happens in the user's personal browser.

use anyhow::{Context, Result};

/// Open `url` in the local default browser.
///
/// Uses `cmd /c start` on Windows, `open` on macOS, and `xdg-open`
/// elsewhere. The URL is passed as a single argument to avoid shell
/// interpretation of query-string characters.
pub fn open(url: &str) -> Result<()> {
    let mut cmd = browser_command(url);
    cmd.spawn()
        .with_context(|| format!("failed to launch browser for {url}"))?;
    Ok(())
}

#[cfg(target_os = "windows")]
fn browser_command(url: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("cmd");
    // Empty title arg guards against URLs being parsed as the window title.
    cmd.args(["/C", "start", "", url]);
    cmd
}

#[cfg(target_os = "macos")]
fn browser_command(url: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("open");
    cmd.arg(url);
    cmd
}

#[cfg(all(not(target_os = "windows"), not(target_os = "macos")))]
fn browser_command(url: &str) -> std::process::Command {
    let mut cmd = std::process::Command::new("xdg-open");
    cmd.arg(url);
    cmd
}
