//! User-facing login-mode selection for Bedrock auth.

use super::{BedrockAuthArgs, LoginMode};
use anyhow::{Result, bail};

/// Convert clearer CLI booleans into the internal [`LoginMode`].
pub(super) fn select(args: &BedrockAuthArgs) -> Result<LoginMode> {
    let count = args.device_code as u8 + args.browser as u8 + args.no_login as u8;
    if count > 1 {
        bail!("Choose only one of --device-code, --browser, or --no-login");
    }
    if args.device_code {
        return Ok(LoginMode::DeviceCode);
    }
    if args.browser {
        return Ok(LoginMode::Browser);
    }
    if args.no_login {
        return Ok(LoginMode::Off);
    }
    Ok(args.login)
}
