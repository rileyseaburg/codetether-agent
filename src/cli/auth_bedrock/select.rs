//! Profile and region selection for Bedrock auth.

use super::{BedrockAuthArgs, LoginMode, sso, sso_default};
use crate::provider::bedrock::DEFAULT_REGION;
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
///
/// Bedrock API keys are region-scoped. Only an explicit `--region` overrides
/// the default; we deliberately do **not** inherit the general AWS CLI/SSO
/// profile region (e.g. `us-west-2`), because that produces a token rejected
/// by the primary `us-east-1` endpoint the provider targets.
pub(super) fn region(args: &BedrockAuthArgs, _selected: Option<&sso::SsoProfile>) -> String {
    args.region.clone().unwrap_or_else(|| DEFAULT_REGION.to_string())
}
