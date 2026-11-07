//! Reauthenticate rejected clients and restart lease maintenance.
//!
//! Kubernetes workloads log in again with their service-account JWT. Hosts
//! that set `VAULT_TOKEN_FILE` re-read the externally rotated token instead.
//! Without either source, the current client is returned unchanged.

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
    /// Returns the client to retry with once; callers fail if it is rejected
    /// again. Errors when the refresh source cannot produce a client.
    pub(super) async fn refresh_auth(&self) -> Result<Option<Arc<VaultClient>>> {
        if self.k8s_auth.is_some() {
            return self.refresh_kubernetes_auth().await;
        }
        if self.token_file.is_some() {
            return self.refresh_token_file_auth().await;
        }
        Ok(self.client())
    }

    async fn refresh_kubernetes_auth(&self) -> Result<Option<Arc<VaultClient>>> {
        let Some(auth) = self.k8s_auth.as_deref() else {
            return Ok(self.client());
        };
        tracing::warn!("Vault token was rejected; refreshing Kubernetes auth token");
        let client = Self::login_with_kubernetes(auth).await?;
        self.install_refreshed_client(client.clone()).await;
        tracing::info!(role = %auth.role, mount = %auth.auth_mount, "Refreshed Vault Kubernetes auth token");
        Ok(Some(client))
    }

    async fn refresh_token_file_auth(&self) -> Result<Option<Arc<VaultClient>>> {
        let Some(auth) = self.token_file.as_deref() else {
            return Ok(self.client());
        };
        tracing::warn!(token_file = %auth.path.display(), "Vault token was rejected; re-reading token file");
        let token = crate::token_file::read_token(&auth.path).await?;
        let client = Arc::new(token_client(&auth.address, &token)?);
        self.install_refreshed_client(client.clone()).await;
        tracing::info!(token_file = %auth.path.display(), "Reloaded Vault token from file");
        Ok(Some(client))
    }

    async fn install_refreshed_client(&self, client: Arc<VaultClient>) {
        *self.client.write() = Some(client);
        self.renewal.start(Arc::downgrade(&self.client));
        self.clear_cache().await;
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
