//! Exchange a provisioned SecretID without exposing response bodies.
use super::AppRoleAuth;
use anyhow::{Result, anyhow, ensure};
use serde::Deserialize;
use std::sync::Arc;
use vaultrs::client::{VaultClient, VaultClientSettingsBuilder};

#[derive(Deserialize)]
struct Reply {
    auth: Auth,
}
#[derive(Deserialize)]
struct Auth {
    client_token: String,
}

impl AppRoleAuth {
    pub(crate) async fn login(&self) -> Result<Arc<VaultClient>> {
        let secret = tokio::fs::read_to_string(&self.secret_id_file)
            .await
            .map_err(|_| anyhow!("Could not read AppRole secret ID file"))?;
        ensure!(!secret.trim().is_empty(), "AppRole secret ID file is empty");
        let reply: Reply = crate::login::http::post(
            &self.address,
            &self.login_path,
            &serde_json::json!({"role_id": self.role_id, "secret_id": secret.trim()}),
        )
        .await
        .map_err(|_| {
            anyhow!(
                "Vault AppRole login failed; check the provisioned identity and SecretID lifetime"
            )
        })?;
        ensure!(
            !reply.auth.client_token.is_empty(),
            "Vault AppRole login omitted token"
        );
        let settings = VaultClientSettingsBuilder::default()
            .address(&self.address)
            .token(reply.auth.client_token)
            .build()
            .map_err(|_| anyhow!("Could not configure AppRole Vault client"))?;
        Ok(Arc::new(VaultClient::new(settings).map_err(|_| {
            anyhow!("Could not create AppRole Vault client")
        })?))
    }
}
