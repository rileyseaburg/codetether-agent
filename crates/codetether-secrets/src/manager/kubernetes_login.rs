//! Secrets manager kubernetes login.

use crate::KubernetesAuthConfig;
use crate::SecretsManager;
use anyhow::{Context, Result};
use std::sync::Arc;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};
impl SecretsManager {
    pub(crate) async fn login_with_kubernetes(
        auth: &KubernetesAuthConfig,
    ) -> Result<Arc<VaultClient>> {
        let jwt = tokio::fs::read_to_string(&auth.jwt_path)
            .await
            .with_context(|| {
                format!(
                    "Failed to read Kubernetes service account token from {}",
                    auth.jwt_path
                )
            })?;
        let jwt = jwt.trim().to_string();

        // Bootstrap client with an empty token — only used for the one-shot
        // auth call; the real authenticated client is built from the result.
        let bootstrap_settings = VaultClientSettingsBuilder::default()
            .address(&auth.address)
            .token("")
            .build()
            .context("Failed to build bootstrap Vault client settings")?;
        let bootstrap_client = VaultClient::new(bootstrap_settings)
            .context("Failed to create bootstrap Vault client")?;

        let auth_info =
            vaultrs::auth::kubernetes::login(&bootstrap_client, &auth.auth_mount, &auth.role, &jwt)
                .await
                .context("Vault Kubernetes auth login failed")?;

        let settings = VaultClientSettingsBuilder::default()
            .address(&auth.address)
            .token(&auth_info.client_token)
            .build()
            .context("Failed to build authenticated Vault client settings")?;
        let client =
            VaultClient::new(settings).context("Failed to create authenticated Vault client")?;

        Ok(Arc::new(client))
    }
}
