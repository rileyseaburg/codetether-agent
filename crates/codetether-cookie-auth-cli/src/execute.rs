//! Validate and execute a provider-cookie import.
use crate::CookieAuthArgs;
use anyhow::{Context, Result};
use codetether_secrets as secrets;

/// Import a Netscape cookie file into one provider's Vault record.
///
/// # Arguments
/// * `args` - Input file, provider identifier and filtering policy.
/// # Returns
/// Unit after saving, checking read-back and reporting the import.
/// # Errors
/// Returns unavailable-store, invalid-input, file-read or persistence errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_cookie_auth_cli::CookieAuthArgs) -> anyhow::Result<()> {
/// codetether_cookie_auth_cli::execute(args).await
/// # }
/// ```
pub async fn execute(args: CookieAuthArgs) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth cookies`."
        );
    }

    let provider_id = args.provider.trim().to_string();
    if provider_id.is_empty() {
        anyhow::bail!("--provider cannot be empty");
    }

    let raw = tokio::fs::read_to_string(&args.file)
        .await
        .with_context(|| format!("Failed to read cookie file {}", args.file.display()))?;
    let rows = crate::parse::parse_netscape_cookie_file(&raw);
    if rows.is_empty() {
        anyhow::bail!(
            "No valid cookie rows found in {} (expected Netscape format)",
            args.file.display()
        );
    }

    let (selected, dropped_expired, dropped_non_auth) =
        crate::select::select_cookie_rows(&rows, &provider_id, args.keep_all);
    if selected.is_empty() {
        anyhow::bail!("No usable cookies remained after filtering");
    }

    let (extra, earliest_expiry) =
        crate::metadata::build(&selected, args.keep_all, dropped_expired, dropped_non_auth);
    let can_read_back = crate::save::save(&provider_id, extra).await?;
    crate::report::print(
        selected.len(),
        &provider_id,
        dropped_expired,
        dropped_non_auth,
        earliest_expiry,
        can_read_back,
    );
    Ok(())
}
