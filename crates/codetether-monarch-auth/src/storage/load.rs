//! Read current OAuth credentials through the configured secret manager.
use super::{PROVIDER_ID, Store, record};
use crate::oauth::Credentials;
use anyhow::{Result, anyhow};
use codetether_secrets::ProviderSecrets;

impl Store {
    pub(super) async fn record(&self) -> Result<Option<ProviderSecrets>> {
        self.manager
            .read_provider_secrets(PROVIDER_ID)
            .await
            .map_err(|_| {
                anyhow!("Could not read Monarch credentials from the configured secret backend")
            })
    }

    pub(crate) async fn load(&self) -> Result<Option<Credentials>> {
        record::decode(self.record().await?)
    }

    pub(crate) async fn preflight(&self) -> Result<()> {
        self.manager.verify_reachable().await.map_err(|_| {
            anyhow!(
                "Configured secret backend is unavailable; authenticate with codetether vault first"
            )
        })?;
        self.record().await?;
        Ok(())
    }
}
