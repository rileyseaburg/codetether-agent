//! Reauthenticate workload clients after rejection without interactive prompts.

use crate::SecretsManager;
use anyhow::Result;
use std::sync::Arc;
use vaultrs::{client::VaultClient, error::ClientError};

impl SecretsManager {
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
        let client = if let Some(auth) = self.approle_auth.as_deref() {
            auth.login().await?
        } else if let Some(auth) = self.k8s_auth.as_deref() {
            Self::login_with_kubernetes(auth).await?
        } else {
            return Ok(self.client());
        };
        *self.client.write() = Some(client.clone());
        self.renewal.start(Arc::downgrade(&self.client));
        self.clear_cache().await;
        tracing::info!(
            auth_method = if self.approle_auth.is_some() {
                "approle"
            } else {
                "kubernetes"
            },
            "Refreshed Vault workload token"
        );
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
