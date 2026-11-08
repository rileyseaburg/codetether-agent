//! Construct a manager retaining its configured noninteractive login identity.
use super::AppRoleAuth;
use crate::SecretsManager;
use anyhow::Result;
use std::sync::Arc;

impl SecretsManager {
    pub(crate) async fn from_approle(
        auth: AppRoleAuth,
        mount: Option<String>,
        path: Option<String>,
    ) -> Result<Self> {
        let client = auth.login().await?;
        let manager = Self {
            approle_auth: Some(Arc::new(auth)),
            mount: mount.unwrap_or_else(|| "secret".into()),
            path: path.unwrap_or_else(|| "codetether/providers".into()),
            ..Self::default()
        };
        *manager.client.write() = Some(client);
        Ok(manager.with_renewal())
    }
}
