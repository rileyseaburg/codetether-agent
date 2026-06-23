//! Native (profile-free) credential resolution for `auth bedrock`.
//!
//! When the user supplies `--sso` + `--account-id` + `--role-name`, we can
//! log in via the native device-code flow without any `~/.aws/config`.

use anyhow::Result;

use super::{args::BedrockAuthArgs, exported::Exported, native_sso, select};

/// If native SSO inputs are present, resolve credentials without the CLI.
///
/// Returns `Ok(Some(exported))` when the native flow ran, `Ok(None)` when the
/// caller should fall back to the profile/CLI-based path.
pub(super) async fn try_resolve(args: &BedrockAuthArgs) -> Result<Option<Exported>> {
    let (Some(start_url), Some(account_id), Some(role_name)) = (
        args.sso.as_deref(),
        args.account_id.as_deref(),
        args.role_name.as_deref(),
    ) else {
        return Ok(None);
    };
    let region = select::region(args, None);
    let exported = native_sso::login(native_sso::NativeSsoArgs {
        start_url,
        region: &region,
        account_id,
        role_name,
    })
    .await?;
    Ok(Some(exported))
}
