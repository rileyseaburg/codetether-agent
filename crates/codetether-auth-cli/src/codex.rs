//! Authentication codex responsibilities.
use crate::{CodexAuthArgs, authenticate_codex_device_code, store_codex_credentials};
use anyhow::{Context, Result};
use codetether_secrets as secrets;

pub(super) async fn authenticate_codex(
    args: CodexAuthArgs,
    application_version: &str,
) -> Result<()> {
    if secrets::secrets_manager().is_none() {
        anyhow::bail!(
            "HashiCorp Vault is not configured. Set VAULT_ADDR and VAULT_TOKEN before running `codetether auth codex`."
        );
    }
    secrets::verify_reachable().await.context(
        "HashiCorp Vault is configured but unreachable. Start Vault, correct VAULT_ADDR/VAULT_TOKEN, or unset stale Vault env before running `codetether auth codex`.",
    )?;

    if args.device_code {
        let credentials = authenticate_codex_device_code(application_version).await?;
        return store_codex_credentials(credentials).await;
    }

    crate::codex_browser::run(application_version).await
}
