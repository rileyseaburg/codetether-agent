//! Output/persistence for `codetether auth bedrock` tokens.

use super::{BedrockAuthArgs, token::OutputToken, vault_save};
use anyhow::Result;

/// Print the gated token and optionally persist it to Vault.
pub(super) async fn emit(
    args: &BedrockAuthArgs,
    region: &str,
    token: &OutputToken,
    profile: Option<&str>,
    expires_secs: u64,
) -> Result<()> {
    if args.save_only {
        // Secret intentionally not printed for scriptable Vault saves.
    } else if args.raw {
        println!("{}", token.expose());
    } else {
        println!(
            "Short-term Bedrock API key (region {region}, <= {expires_secs}s):"
        );
        println!();
        println!("  export AWS_BEARER_TOKEN_BEDROCK={}", token.expose());
        println!();
        println!(
            "Note: tokens signed with SSO/STS session credentials expire when the session does."
        );
    }
    if args.save {
        vault_save::save(args, region, token.expose(), profile).await?;
    }
    Ok(())
}
