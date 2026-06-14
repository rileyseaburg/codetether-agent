//! `codetether auth bedrock` — mint a short-term Bedrock API key from
//! AWS credentials, including SSO/IdP profiles (`aws sso login`).
//!
//! AWS publishes this mechanism as the "Bedrock API key": a SigV4
//! presigned `CallWithBearerToken` URL usable as a plain bearer token
//! (`AWS_BEARER_TOKEN_BEDROCK`). We mirror the official generator.

mod args;
mod aws_cli;
mod creds;
mod exported;
mod login;
mod mode;
mod output;
mod select;
mod sso;
mod sso_cache;
mod sso_cache_files;
mod sso_default;
mod sso_parse;
mod sso_profile_meta;
#[cfg(test)]
mod sso_tests;
mod sso_vault_extra;
mod token;
mod validate;
mod vault_extra;
mod vault_save;

use crate::provider::bedrock::token_gen;
use anyhow::{Result, bail};
pub use args::BedrockAuthArgs;
pub use login::LoginMode;

/// Resolve credentials, mint, validate, and emit/save the bearer token.
pub async fn execute(args: BedrockAuthArgs) -> Result<()> {
    if args.save_only && !args.save {
        bail!("--save-only requires --save");
    }
    if args.save_only && args.raw {
        bail!("--save-only cannot be combined with --raw");
    }
    let login_mode = mode::select(&args)?;
    let selected = select::profile(&args, login_mode)?;
    let profile = selected
        .as_ref()
        .map(|s| s.profile.as_str())
        .or(args.profile.as_deref());
    let region = select::region(&args, selected.as_ref());
    let aws = creds::resolve(profile, login_mode).await?;
    let expires = args.expires_secs.min(token_gen::DEFAULT_EXPIRES_SECS);
    let token = token_gen::generate_bearer_token(&aws, &region, expires);
    let token = validate::output_token(token, args.no_validate).await?;
    output::emit(&args, &region, &token, profile, expires).await
}