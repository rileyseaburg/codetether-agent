//! Reauthenticate rejected clients and restart lease maintenance.
//!
//! AppRole and Kubernetes workloads log in again without interactive prompts.
//! Hosts that set `VAULT_TOKEN_FILE` re-read the externally rotated token
//! instead. Without any source, the current client is returned unchanged.

use crate::SecretsManager;
use anyhow::Result;
use std::sync::Arc;
use vaultrs::{
    client::{VaultClient, VaultClientSettingsBuilder},
    error::ClientError,
};

impl SecretsManager {
    /// Replace a rejected client from the configured refresh source.
    ///
    /// Concurrent callers share one refresh: if another caller already
    /// replaced `rejected`, its client is returned. Callers retry once and fail
    /// if the replacement is rejected again.
    pub(super) async fn refresh_auth(
        &self,
        rejected: &Arc<VaultClient>,
    ) -> Result<Option<Arc<VaultClient>>> {
        let _lock = self.auth_refresh_lock.lock().await;
        if let Some(current) = self.client()
            && !Arc::ptr_eq(&current, rejected)
        {
            return Ok(Some(current));
        }
        let (client, method) = if let Some(auth) = self.approle_auth.as_deref() {
            (auth.login().await?, "approle")
        } else if let Some(auth) = self.k8s_auth.as_deref() {
            tracing::warn!("Vault token was rejected; refreshing Kubernetes auth token");
            (Self::login_with_kubernetes(auth).await?, "kubernetes")
        } else if let Some(auth) = self.token_file.as_deref() {
            tracing::warn!(token_file = %auth.path.display(), "Vault token was rejected; re-reading token file");
            let token = crate::token_file::read_token(&auth.path).await?;
            (Arc::new(token_client(&auth.address, &token)?), "token_file")
        } else {
            return Ok(self.client());
        };
        *self.client.write() = Some(client.clone());
        self.renewal.start(Arc::downgrade(&self.client));
        self.clear_cache().await;
        tracing::info!(auth_method = method, "Refreshed Vault token");
        Ok(Some(client))
    }

    pub(super) fn should_refresh_vault_token(err: &ClientError) -> bool {
        match err {
            ClientError::APIError { code, errors } => {
                *code == 403
                    || errors.iter().any(|msg| {
                        let msg = msg.to_ascii_lowercase();
                        msg.contains("invalid token") || msg.contains("permission denied")
                    })
            }
            _ => false,
        }
    }
}

fn token_client(address: &str, token: &str) -> Result<VaultClient> {
    use anyhow::Context;
    let settings = VaultClientSettingsBuilder::default()
        .address(address)
        .token(token)
        .build()
        .context("Failed to build Vault client settings")?;
    VaultClient::new(settings).context("Failed to create Vault client")
}
