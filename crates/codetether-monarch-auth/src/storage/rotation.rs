//! Reject stale refresh writers before updating the shared credential record.
use super::{Store, record};
use crate::oauth::Credentials;
use anyhow::{Result, anyhow, ensure};

impl Store {
    pub(crate) async fn save_rotated(&self, old: &Credentials, new: &Credentials) -> Result<()> {
        let current = self
            .record()
            .await?
            .ok_or_else(|| anyhow!("Monarch authorization was removed during refresh"))?;
        let stored = record::decode(Some(current.clone()))?
            .ok_or_else(|| anyhow!("Monarch authorization was removed during refresh"))?;
        ensure!(
            stored.client_id == old.client_id
                && stored.refresh_token == old.refresh_token
                && stored.access_token == old.access_token
                && stored.expires_at == old.expires_at,
            "Monarch credentials changed during refresh; retry with the current backend record"
        );
        // The host lock serializes local writers. This comparison detects an
        // observed peer rotation; it is not a distributed compare-and-swap lease.
        self.persist(&record::updated(current, Some(new))?).await
    }
}
