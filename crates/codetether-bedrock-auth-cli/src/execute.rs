//! Orchestration for `codetether auth bedrock`.
//!
//! Holds the multi-step credential→token→emit pipeline so [`super`] stays a
//! thin module table within the 50-line file budget.

use anyhow::{Result, bail};
use codetether_bedrock_auth::token_gen;

use super::{args::BedrockAuthArgs, creds, mode, native_resolve, output, select, validate};

/// Resolve credentials, mint, validate, and emit/save the bearer token.
///
/// # Arguments
/// * `args` - Credential selection, login, output and persistence options.
/// # Returns
/// Unit after the requested credential operation and output complete.
/// # Errors
/// Returns option, credential, login, validation or persistence errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// use clap::Parser;
/// let args = codetether_bedrock_auth_cli::BedrockAuthArgs::parse_from(["bedrock", "--no-login"]);
/// codetether_bedrock_auth_cli::execute(args).await
/// # }
/// ```
pub async fn execute(args: BedrockAuthArgs) -> Result<()> {
    if args.save_only && !args.save {
        bail!("--save-only requires --save");
    }
    if args.save_only && args.raw {
        bail!("--save-only cannot be combined with --raw");
    }
    if args.refresh {
        return super::refresh::run(&args).await;
    }
    if let Some(exported) = native_resolve::try_resolve(&args).await? {
        let region = select::region(&args, None);
        return finish(&args, exported, &region, None).await;
    }
    let login_mode = mode::select(&args)?;
    let selected = select::profile(&args, login_mode)?;
    let profile = selected
        .as_ref()
        .map(|s| s.profile.as_str())
        .or(args.profile.as_deref());
    let region = select::region(&args, selected.as_ref());
    let exported = creds::resolve(profile, login_mode).await?;
    finish(&args, exported, &region, profile).await
}

/// Mint, validate, and emit/save the bearer token from resolved credentials.
async fn finish(
    args: &BedrockAuthArgs,
    exported: super::exported::Exported,
    region: &str,
    profile: Option<&str>,
) -> Result<()> {
    let expires = args.expires_secs.min(token_gen::DEFAULT_EXPIRES_SECS);
    let token = token_gen::generate_bearer_token(&exported.creds, region, expires);
    let token = validate::output_token(token, region, args.no_validate).await?;
    output::emit(args, region, &token, profile, expires, exported.expiration).await
}
