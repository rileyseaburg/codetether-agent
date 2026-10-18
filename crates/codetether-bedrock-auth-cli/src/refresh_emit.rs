//! Console output for refreshed Bedrock tokens (no Vault side effects).

use super::{BedrockAuthArgs, mask::mask_token, token::OutputToken};

/// Print the refreshed token honoring `--raw` / `--save-only` flags.
pub(super) fn print(args: &BedrockAuthArgs, region: &str, token: &OutputToken, expires_secs: u64) {
    if args.save_only {
        // Secret intentionally not printed for scriptable Vault saves.
    } else if args.raw {
        println!("{}", token.expose());
    } else {
        println!("Refreshed Bedrock API key (region {region}, <= {expires_secs}s):");
        println!("  {}", mask_token(token.expose()));
        println!("  (use --raw for the full token, or --save to store in Vault)");
    }
}
