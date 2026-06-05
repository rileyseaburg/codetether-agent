pub(super) async fn init() {
    if crate::secrets::secrets_manager().is_some() {
        return;
    }
    match crate::secrets::SecretsManager::from_env().await {
        Ok(secrets_manager) => {
            if secrets_manager.is_connected() {
                tracing::info!("Connected to HashiCorp Vault for secrets management");
            }
            if let Err(err) = crate::secrets::init_from_manager(secrets_manager) {
                tracing::debug!(error = %err, "Secrets manager already initialized");
            }
        }
        Err(err) => {
            tracing::warn!(error = %err, "Vault not configured for TUI startup");
            tracing::warn!("Set VAULT_ADDR and VAULT_TOKEN environment variables to connect");
        }
    }
}
