//! Secrets manager kubernetes.

use crate::KubernetesAuthConfig;
use crate::SecretsManager;
use anyhow::Result;
use parking_lot::RwLock as ClientRwLock;
use std::{collections::HashMap, sync::Arc};
use tokio::sync::RwLock;
impl SecretsManager {
    /// Authenticate to Vault using the pod's Kubernetes service account JWT.
    ///
    /// Reads the SA JWT from the standard Kubernetes mount path (overridable via
    /// `VAULT_K8S_SA_JWT_PATH`) then calls the Vault `auth/kubernetes/login`
    /// endpoint.  The returned manager holds the short-lived token that Vault
    /// issued — no `VAULT_TOKEN` environment variable is required.
    ///
    /// # Errors
    /// Returns an error if the service-account JWT cannot be read or Vault authentication fails.
    ///
    /// # Examples
    /// ```no_run
    /// use codetether_secrets::{SecretsManager};
    /// # tokio::runtime::Runtime::new().unwrap().block_on(async {
    /// let manager = SecretsManager::from_k8s_auth("https://vault.example", "worker",
    ///     "kubernetes", Some("secret"), Some("providers")).await.unwrap();
    /// assert!(manager.is_connected());
    /// # });
    /// ```
    pub async fn from_k8s_auth(
        address: &str,
        role: &str,
        mount: &str,
        kv_mount: Option<&str>,
        kv_path: Option<&str>,
    ) -> Result<Self> {
        let jwt_path = std::env::var("VAULT_K8S_SA_JWT_PATH")
            .unwrap_or_else(|_| "/var/run/secrets/kubernetes.io/serviceaccount/token".to_string());

        let auth = KubernetesAuthConfig {
            address: address.to_string(),
            role: role.to_string(),
            auth_mount: mount.to_string(),
            jwt_path,
        };

        let client = Self::login_with_kubernetes(&auth).await?;

        Ok(Self {
            client: Arc::new(ClientRwLock::new(Some(client))),
            cache: Arc::new(RwLock::new(HashMap::new())),
            mount: kv_mount.unwrap_or("secret").to_string(),
            path: kv_path.unwrap_or("codetether/providers").to_string(),
            k8s_auth: Some(Arc::new(auth)),
            renewal: Arc::default(),
        }
        .with_renewal())
    }
}
