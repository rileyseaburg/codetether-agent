//! Monarch records in the application's configured secret backend.
use anyhow::{Result, anyhow, ensure};
use codetether_secrets::SecretsManager;
mod load;
mod lock;
mod record;
mod rotation;
#[cfg(test)]
mod tests;
mod write;
const PROVIDER_ID: &str = "monarch";

pub(crate) struct Store {
    manager: &'static SecretsManager,
}

impl Store {
    pub(crate) fn open() -> Result<Self> {
        let manager = codetether_secrets::secrets_manager().ok_or_else(|| {
            anyhow!(
                "Secret backend is not initialized; use codetether auth monarch after Vault setup"
            )
        })?;
        ensure!(
            manager.is_connected(),
            "Vault is not configured; authenticate with codetether vault first"
        );
        Ok(Self { manager })
    }
}
