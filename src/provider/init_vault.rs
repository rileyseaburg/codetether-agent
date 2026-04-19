//! Build a [`ProviderRegistry`](super::ProviderRegistry) from HashiCorp Vault.
//!
//! Iterates all providers configured in Vault, delegates each to
//! [`super::init_dispatch::dispatch`], then falls back to env-var / AWS
//! auto-detection when `CODETETHER_DISABLE_ENV_FALLBACK` is not set.

use super::bedrock;
use super::init_dispatch;
use super::init_env;
use super::registry::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;

impl ProviderRegistry {
    /// Initialize providers from HashiCorp Vault with env-var fallback.
    ///
    /// See [module-level docs](super) for the security model and fallback order.
    ///
    /// # Examples
    ///
    /// ```rust,no_run
    /// use codetether_agent::provider::ProviderRegistry;
    /// # async fn demo() {
    /// let registry = ProviderRegistry::from_vault().await.unwrap();
    /// # }
    /// ```
    pub async fn from_vault() -> Result<Self> {
        let mut registry = Self::new();
        let disable_env = std::env::var("CODETETHER_DISABLE_ENV_FALLBACK")
            .map(|v| v == "1" || v.eq_ignore_ascii_case("true"))
            .unwrap_or(false);

        if let Some(mgr) = crate::secrets::secrets_manager() {
            let providers = mgr.list_configured_providers().await?;
            tracing::info!("Found {} providers configured in Vault", providers.len());

            // Fetch every provider's secrets concurrently; each fetch is an
            // independent Vault HTTP round-trip so there's no reason to
            // serialize them. With ~10 providers this turns ~10 * RTT
            // latency into ~1 * RTT.
            let fetches = providers.into_iter().map(|pid| async move {
                let secrets = mgr.get_provider_secrets(&pid).await;
                (pid, secrets)
            });
            let results = futures::future::join_all(fetches).await;

            for (pid, secrets) in results {
                let secrets = match secrets {
                    Ok(Some(s)) => s,
                    Ok(None) => continue,
                    Err(err) => {
                        tracing::warn!(provider = %pid, %err, "vault fetch failed; skipping");
                        continue;
                    }
                };
                if let Some(provider) = init_dispatch::dispatch(&pid, &secrets) {
                    registry.register(provider);
                }
            }
        } else {
            tracing::warn!("Vault not configured, no providers loaded from Vault");
        }

        // Bedrock auto-detect from local AWS creds if Vault didn't register it
        if !registry.providers.contains_key("bedrock") && !disable_env {
            if let Some(creds) = bedrock::AwsCredentials::from_environment() {
                let region =
                    bedrock::AwsCredentials::detect_region().unwrap_or_else(|| "us-east-1".into());
                match bedrock::BedrockProvider::with_credentials(creds, region) {
                    Ok(p) => {
                        tracing::info!("Registered Bedrock from local AWS credentials");
                        registry.register(Arc::new(p));
                    }
                    Err(e) => tracing::warn!("Failed to init bedrock: {e}"),
                }
            }
        }

        if !disable_env {
            init_env::register_env_fallbacks(&mut registry);
        } else {
            tracing::info!(
                "Environment variable fallback disabled (CODETETHER_DISABLE_ENV_FALLBACK=1)"
            );
        }

        tracing::info!(
            "Registered {} providers{}",
            registry.providers.len(),
            if disable_env {
                " (Vault only)"
            } else {
                " (Vault + env fallback)"
            }
        );
        Ok(registry)
    }
}
