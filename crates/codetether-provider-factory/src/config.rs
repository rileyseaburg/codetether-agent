//! Provider population from TOML configuration and permitted fallbacks.
use crate::{Registry, fallback_policy};
use anyhow::Result;
use codetether_bedrock_auth::{AwsCredentials, DEFAULT_REGION};
mod compatible;
mod primary;
#[cfg(test)]
mod tests;
/// Populate a registry from application configuration, preserving provider precedence.
///
/// # Arguments
/// * `registry` — Provider collection receiving configured providers.
/// * `config` — Application provider configuration.
///
/// # Returns
/// Success after configured providers and permitted fallbacks are registered.
///
/// # Errors
/// Returns the first configured provider constructor failure.
///
/// # Examples
/// ```no_run
/// # async fn load<H: codetether_provider_factory::Host>(registry: &mut impl codetether_provider_factory::Registry, config: &codetether_config::Config) -> anyhow::Result<()> {
/// codetether_provider_factory::config::populate_from_config::<H>(registry, config).await
/// # }
/// ```
pub async fn populate_from_config<H: crate::Host>(
    registry: &mut impl Registry,
    config: &codetether_config::Config,
) -> Result<()> {
    let disable_env = fallback_policy::env_fallback_disabled();

    primary::register(registry, config, disable_env)?;
    compatible::register(registry, config)?;

    // ---- Bedrock (auto-detect from env / ~/.aws) ----
    if !disable_env && let Some(creds) = AwsCredentials::from_environment() {
        let region = AwsCredentials::detect_region().unwrap_or_else(|| DEFAULT_REGION.to_string());
        match H::bedrock_with_credentials(creds, region) {
            Ok(p) => registry.register(p),
            Err(e) => tracing::warn!("Failed to init bedrock from AWS credentials: {}", e),
        }
    }

    // ---- Env-var fallback ----
    if !disable_env {
        crate::environment::register_env_fallbacks::<H>(registry);
    } else {
        tracing::info!(
            env = fallback_policy::DISABLE_ENV_FALLBACK,
            "Env/AWS fallback disabled"
        );
    }

    Ok(())
}
