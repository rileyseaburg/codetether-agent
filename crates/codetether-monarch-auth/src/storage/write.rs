//! Persist or clear only the Monarch fields in the configured secret record.
use super::{PROVIDER_ID, Store, record};
use crate::oauth::Credentials;
use anyhow::{Result, anyhow};
use codetether_secrets::ProviderSecrets;

impl Store {
    pub(super) async fn persist(&self, record: &ProviderSecrets) -> Result<()> {
        self.manager
            .set_provider_secrets(PROVIDER_ID, record)
            .await
            .map_err(|_| {
                anyhow!("Could not persist Monarch credentials to the configured secret backend")
            })
    }

    pub(crate) async fn save(&self, credentials: &Credentials) -> Result<()> {
        let current = self.record().await?.unwrap_or_default();
        self.persist(&record::updated(current, Some(credentials))?)
            .await
    }

    pub(crate) async fn erase(&self) -> Result<()> {
        if let Some(current) = self.record().await? {
            self.persist(&record::updated(current, None)?).await?;
        }
        Ok(())
    }
}
