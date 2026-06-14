//! Output/persistence for `codetether auth bedrock` tokens.

use super::{BedrockAuthArgs, token::OutputToken, vault_save};
use anyhow::Result;
use chrono::{DateTime, Utc};

/// Print the gated token and optionally persist it to Vault.
pub(super) async fn emit(
    args: &BedrockAuthArgs,
    region: &str,
    token: &OutputToken,
    profile: Option<&str>,
    expires_secs: u64,
    cred_expiration: Option<DateTime<Utc>>,
) -> Result<()> {
    if args.save_only {
        // Secret intentionally not printed for scriptable Vault saves.
    } else if args.raw {
        println!("{}", token.expose());
    } else {
        println!("Short-term Bedrock API key (region {region}, <= {expires_secs}s):");
        println!();
        println!("  export AWS_BEARER_TOKEN_BEDROCK={}", token.expose());
        println!();
        println!(
            "Note: tokens signed with SSO/STS session credentials expire when the session does."
        );
    }
    if args.save {
        vault_save::save(args, region, token.expose(), profile, cred_expiration).await?;
    }
    Ok(())
}
