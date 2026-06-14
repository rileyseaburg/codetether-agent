//! Output/persistence for `codetether auth bedrock` tokens.

use super::{BedrockAuthArgs, vault_save};
use anyhow::Result;

/// Print the minted token and optionally persist it to Vault.
pub(super) async fn emit(
    args: &BedrockAuthArgs,
    region: &str,
    token: &str,
    profile: Option<&str>,
) -> Result<()> {
    if args.raw {
        println!("{token}");
    } else {
        println!(
            "Short-term Bedrock API key (region {region}, <= {}s):",
            args.expires_secs
        );
        println!();
        println!("  export AWS_BEARER_TOKEN_BEDROCK={token}");
        println!();
        println!(
            "Note: tokens signed with SSO/STS session credentials expire when the session does."
        );
    }
    if args.save {
        vault_save::save(args, region, token, profile).await?;
    }
    Ok(())
}
