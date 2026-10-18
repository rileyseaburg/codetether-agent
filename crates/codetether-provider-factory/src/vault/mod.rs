//! Vault loading, concurrent secret fetches and permitted fallback registration.
use crate::{Registry, fallback_policy};
use anyhow::Result;
use codetether_bedrock_auth::AwsCredentials;
mod register;
/// Populate a registry from Vault before considering environment fallbacks.
///
/// # Arguments
/// * `registry` — Provider collection receiving Vault providers first.
///
/// # Returns
/// Success after discovery and allowed fallbacks; individual fetch/constructor
/// failures are logged and skipped.
///
/// # Errors
/// Returns Vault provider-list discovery failures.
///
/// # Examples
/// ```no_run
/// # async fn load<H: codetether_provider_factory::Host>(registry: &mut impl codetether_provider_factory::Registry) -> anyhow::Result<()> {
/// codetether_provider_factory::vault::populate_from_vault::<H>(registry).await
/// # }
/// ```
pub async fn populate_from_vault<H: crate::Host>(registry: &mut impl Registry) -> Result<()> {
    let disable_env = fallback_policy::env_fallback_disabled();

    if let Some(mgr) = codetether_secrets::secrets_manager() {
        // Proactively assess the stored Bedrock key: if it is expired or
        // near expiry and SSO refresh metadata exists, silently re-mint
        // and re-save it before any provider is built. No-op otherwise.
        H::ensure_bedrock_fresh().await;

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
            register::provider::<H>(registry, pid, secrets);
        }
    } else {
        tracing::warn!("Vault not configured, no providers loaded from Vault");
    }

    // Bedrock auto-detect from local AWS creds if Vault didn't register it
    if !registry.contains("bedrock") && !disable_env {
        if let Some(creds) = AwsCredentials::from_environment() {
            let region = AwsCredentials::detect_region().unwrap_or_else(|| "us-east-1".into());
            match H::bedrock_with_credentials(creds, region) {
                Ok(p) => {
                    tracing::info!("Registered Bedrock from local AWS credentials");
                    registry.register(p);
                }
                Err(e) => tracing::warn!("Failed to init bedrock: {e}"),
            }
        }
    }

    if !disable_env {
        crate::environment::register_env_fallbacks::<H>(registry);
    } else {
        tracing::info!(
            env = fallback_policy::DISABLE_ENV_FALLBACK,
            "Env/AWS fallback disabled"
        );
    }

    tracing::info!(
        mode = fallback_policy::registry_mode_label(disable_env),
        "Registered {} providers",
        registry.len(),
    );
    Ok(())
}
