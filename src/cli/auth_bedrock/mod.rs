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
mod sso;
mod sso_default;
mod sso_parse;
mod validate;

use crate::provider::bedrock::{AwsCredentials, DEFAULT_REGION, token_gen};
use anyhow::Result;
pub use args::BedrockAuthArgs;
pub use login::LoginMode;

/// Resolve credentials, mint, validate, and emit/save the bearer token.
pub async fn execute(args: BedrockAuthArgs) -> Result<()> {
    let login_mode = mode::select(&args)?;
    let selected = select_profile(&args, login_mode)?;
    let profile = selected
        .as_ref()
        .map(|s| s.profile.as_str())
        .or(args.profile.as_deref());
    let region = select_region(&args, selected.as_ref());
    let aws = creds::resolve(profile, login_mode).await?;
    let expires = args.expires_secs.min(token_gen::DEFAULT_EXPIRES_SECS);
    let token = token_gen::generate_bearer_token(&aws, &region, expires);
    if !args.no_validate {
        validate::token(&region, &token).await?;
        eprintln!("Validated Bedrock API key against AWS in region {region}.");
    }
    output::emit(&args, &region, &token).await
}

fn select_profile(args: &BedrockAuthArgs, mode: LoginMode) -> Result<Option<sso::SsoProfile>> {
    if let Some(url) = args.sso.as_deref() {
        return sso::resolve(url).map(Some);
    }
    if args.profile.is_none() && matches!(mode, LoginMode::Browser | LoginMode::DeviceCode) {
        return sso_default::resolve().map(Some);
    }
    Ok(None)
}

fn select_region(args: &BedrockAuthArgs, selected: Option<&sso::SsoProfile>) -> String {
    args.region
        .clone()
        .or_else(|| selected.and_then(|s| s.region.clone()))
        .or_else(AwsCredentials::detect_region)
        .unwrap_or_else(|| DEFAULT_REGION.to_string())
}
