//! Shared Vault initialization used by native TUI provider loading.
/// Initialize a configured manager only when one is not already installed.
/// # Arguments
/// Reads existing process-wide secrets configuration.
/// # Returns
/// Completes after installation or diagnostic logging.
/// # Examples
/// ```no_run
/// async fn prepare() { codetether_tui_startup::secrets::init().await; }
/// ```
pub async fn init() {
    if codetether_secrets::secrets_manager().is_some() {
        return;
    }
    match codetether_secrets::SecretsManager::from_env().await {
        Ok(secrets_manager) => {
            if secrets_manager.is_connected() {
                tracing::info!("Connected to HashiCorp Vault for secrets management");
            }
            if let Err(err) = codetether_secrets::init_from_manager(secrets_manager) {
                tracing::debug!(error = %err, "Secrets manager already initialized");
            }
        }
        Err(err) => {
            tracing::warn!(error = %err, "Vault not configured for TUI startup");
            tracing::warn!("Set VAULT_ADDR and VAULT_TOKEN environment variables to connect");
        }
    }
}
