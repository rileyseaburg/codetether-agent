//! Build a [`ProviderRegistry`](super::ProviderRegistry) from HashiCorp Vault.
//!
//! Iterates all providers configured in Vault, delegates each to
//! [`super::init_dispatch::dispatch`], then adds env-var / AWS auto-detection
//! unless [`CODETETHER_DISABLE_ENV_FALLBACK`](super::fallback_policy::DISABLE_ENV_FALLBACK)
//! is set.

use super::bedrock;
use super::fallback_policy;
use super::init_dispatch;
use super::init_env;
use super::registry::ProviderRegistry;
use anyhow::Result;
use std::sync::Arc;

impl ProviderRegistry {
    /// Initialize providers from HashiCorp Vault with optional env/AWS fallback.
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
        let disable_env = fallback_policy::env_fallback_disabled();

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
                env = fallback_policy::DISABLE_ENV_FALLBACK,
                "Environment variable fallback disabled"
            );
        }

        tracing::info!(
            mode = fallback_policy::registry_mode_label(disable_env),
            "Registered {} providers",
            registry.providers.len(),
        );
        Ok(registry)
    }

    /// Process-wide cached [`from_vault`](Self::from_vault) registry.
    ///
    /// `from_vault` performs vault fetches plus env-var / AWS probing on
    /// every call. Compression paths (e.g. RLM model resolution inside
    /// [`enforce_on_messages`](crate::session::helper::compression::enforce_on_messages))
    /// invoke it once per keep-last attempt per turn, which can add up
    /// to several Vault round-trips of unnecessary latency in the hot
    /// loop. This accessor lazily builds the registry exactly once and
    /// hands out `Arc` clones thereafter.
    ///
    /// The cache is process-global. Restart the binary to pick up
    /// re-keyed providers.
    ///
    /// # Errors
    ///
    /// Propagates the underlying [`from_vault`](Self::from_vault) error
    /// on the first call. Subsequent calls reuse the cached value and
    /// cannot fail.
    pub async fn shared_from_vault() -> Result<Arc<Self>> {
        use tokio::sync::OnceCell;
        static CACHED: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();
        CACHED
            .get_or_try_init(|| async { Self::from_vault().await.map(Arc::new) })
            .await
            .cloned()
    }
}
