//! `codetether auth bedrock --refresh` — silently re-mint the Bedrock key.
//!
//! Delegates to the shared provider-layer silent-refresh machinery
//! ([`crate::provider::bedrock::sso_refresh`]), which runs a browser-free
//! OIDC `refresh_token` grant + `GetRoleCredentials`, re-mints the bearer
//! token, and (when saving) writes it back preserving stored SSO metadata.

use crate::provider::bedrock::{sso_refresh, token_gen};
use anyhow::Result;

use super::{args::BedrockAuthArgs, refresh_emit, validate};

/// Refresh the stored Bedrock key without any browser interaction.
pub(super) async fn run(args: &BedrockAuthArgs) -> Result<()> {
    let save = args.save || args.save_only;
    let refreshed = sso_refresh::refresh_now(save).await?;
    let token = validate::output_token(refreshed.token, &refreshed.region, args.no_validate).await?;
    let expires = args.expires_secs.min(token_gen::DEFAULT_EXPIRES_SECS);
    refresh_emit::print(args, &refreshed.region, &token, expires);
    Ok(())
}
