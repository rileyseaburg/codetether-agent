//! Profile and region selection for Bedrock auth.

use super::{BedrockAuthArgs, LoginMode, sso, sso_default};
use crate::provider::bedrock::{AwsCredentials, DEFAULT_REGION};
use anyhow::Result;

/// Select the AWS SSO profile implied by the CLI arguments.
pub(super) fn profile(args: &BedrockAuthArgs, mode: LoginMode) -> Result<Option<sso::SsoProfile>> {
    if let Some(url) = args.sso.as_deref() {
        return sso::resolve(url).map(Some);
    }
    if args.profile.is_none() && matches!(mode, LoginMode::Browser | LoginMode::DeviceCode) {
        return sso_default::resolve().map(Some);
    }
    Ok(None)
}

/// Resolve the Bedrock signing region.
pub(super) fn region(args: &BedrockAuthArgs, selected: Option<&sso::SsoProfile>) -> String {
    args.region
        .clone()
        .or_else(|| selected.and_then(|s| s.region.clone()))
        .or_else(AwsCredentials::detect_region)
        .unwrap_or_else(|| DEFAULT_REGION.to_string())
}
