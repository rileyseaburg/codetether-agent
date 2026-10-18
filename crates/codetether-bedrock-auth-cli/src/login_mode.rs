//! Selection of interactive AWS SSO login behavior.

/// How to perform AWS SSO login when requested or required.
///
/// # Examples
/// ```
/// let mode = codetether_bedrock_auth_cli::LoginMode::Off;
/// assert!(matches!(mode, codetether_bedrock_auth_cli::LoginMode::Off));
/// ```
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
