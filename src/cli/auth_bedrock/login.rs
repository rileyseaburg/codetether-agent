//! SSO auto-login for `codetether auth bedrock`.
//!
//! Runs `aws sso login` with stdio inherited so the GFE browser flow or
//! the device-code prompt is visible. `Auto` only logs in when export fails;
//! explicit browser/device-code modes force a fresh login before export.

use anyhow::{Context, Result, bail};

/// How to perform `aws sso login` when requested or required.
#[derive(Clone, Copy, Debug, PartialEq, Eq, clap::ValueEnum)]
pub enum LoginMode {
    /// Browser when a GUI is detected, else device-code.
    Auto,
    /// Force the interactive browser redirect flow (GFE).
    Browser,
    /// Force the device-code flow (personal laptop / headless / SSH).
    DeviceCode,
    /// Never auto-login; fail with a hint instead.
    Off,
}

/// Run `aws sso login` for `profile` using the resolved mode.
pub(super) async fn run(profile: &str, mode: LoginMode) -> Result<()> {
    let use_device_code = match mode {
        LoginMode::Off => bail!("SSO session expired. Run `aws sso login --profile {profile}`."),
        LoginMode::DeviceCode => true,
        LoginMode::Browser => false,
        LoginMode::Auto => !gui_available(),
    };
    eprintln!(
        "Logging in to SSO profile '{profile}' ({})...",
        if use_device_code {
            "device code"
        } else {
            "browser"
        }
    );
    let mut cmd = tokio::process::Command::new("aws");
    cmd.args(["sso", "login", "--profile", profile]);
    if use_device_code {
        cmd.arg("--use-device-code");
    }
    let status = cmd
        .status()
        .await
        .context("Failed to spawn `aws sso login`")?;
    if !status.success() {
        bail!("`aws sso login` failed for profile '{profile}'");
    }
    Ok(())
}

/// Heuristic: a GUI browser is reachable for the redirect flow.
fn gui_available() -> bool {
    if std::env::var_os("SSH_CONNECTION").is_some() {
        return false;
    }
    cfg!(target_os = "macos")
        || cfg!(target_os = "windows")
        || std::env::var_os("DISPLAY").is_some()
        || std::env::var_os("WAYLAND_DISPLAY").is_some()
}
