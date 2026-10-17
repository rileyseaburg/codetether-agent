//! Worker model availability policy.
use super::*;

pub fn ensure_providers(providers: &[&str]) -> Result<()> {
    if !providers.is_empty() {
        return Ok(());
    }
    anyhow::bail!(
        "No LLM providers available (0 providers loaded). Configure API keys in HashiCorp Vault or set environment variables. Vault address: {}",
        std::env::var("VAULT_ADDR").unwrap_or_else(|_| "(not set)".into())
    )
}
