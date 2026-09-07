//! Reauthenticate Kubernetes clients after rejection and restart lease maintenance.

use super::*;

impl SecretsManager {
    pub(super) async fn refresh_kubernetes_auth(&self) -> Result<Option<Arc<VaultClient>>> {
        let Some(auth) = self.k8s_auth.as_deref() else {
            return Ok(self.client());
        };
        tracing::warn!("Vault token was rejected; refreshing Kubernetes auth token");
        let client = Self::login_with_kubernetes(auth).await?;
        *self.client.write() = Some(client.clone());
        self.renewal.start(Arc::downgrade(&self.client));
        self.clear_cache().await;
        tracing::info!(role = %auth.role, mount = %auth.auth_mount, "Refreshed Vault Kubernetes auth token");
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
