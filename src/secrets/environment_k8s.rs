//! Preserve the existing Kubernetes service-account preference and env fallback.

use super::SecretsManager;

pub(super) async fn authenticate(
    address: &str,
    mount: Option<&str>,
    path: Option<&str>,
) -> Option<SecretsManager> {
    let role = std::env::var("VAULT_ROLE").ok()?;
    let role = role.trim();
    if role.is_empty() {
        return None;
    }
    let auth_mount = std::env::var("VAULT_AUTH_MOUNT").unwrap_or_else(|_| "kubernetes".into());
    match SecretsManager::from_k8s_auth(address, role, &auth_mount, mount, path).await {
        Ok(manager) => {
            tracing::info!(role, mount = %auth_mount, "Authenticated to Vault via Kubernetes service account");
            Some(manager)
        }
        Err(error) => {
            tracing::warn!(error = %error, "Vault Kubernetes auth failed; falling back to VAULT_TOKEN");
            None
        }
    }
}
