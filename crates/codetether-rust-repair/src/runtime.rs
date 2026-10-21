//! Stdin/stdout adapter using configured native providers, never agent sessions.
use crate::{MAX_REQUEST_BYTES, RustRepairArgs, parse_request, propose};
use anyhow::{Context, Result, ensure};
use codetether_native_providers::ProviderRegistry;
use std::io::Write;
use tokio::io::AsyncReadExt;

/// Read one bounded JSON request, generate a proposal, and emit JSON only.
/// # Arguments
/// * `args` - Optional model override; otherwise use the configured default model.
/// # Returns
/// Unit after writing a validated proposal to stdout.
/// # Errors
/// Reports invalid input, configuration/authentication, model resolution and provider errors.
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// codetether_rust_repair::run(codetether_rust_repair::RustRepairArgs { model: None }).await?;
/// # Ok(()) }
/// ```
pub async fn run(args: RustRepairArgs) -> Result<()> {
    let mut bytes = Vec::new();
    tokio::io::stdin()
        .take((MAX_REQUEST_BYTES + 1) as u64)
        .read_to_end(&mut bytes)
        .await?;
    let snapshot = parse_request(&bytes)?;
    let config = codetether_config::Config::load().await?;
    let requested = args
        .model
        .or(config.default_model)
        .context("no repair model configured; set a default model or --model")?;
    ensure!(!requested.trim().is_empty(), "repair model cannot be empty");
    initialize_secrets().await;
    let registry = ProviderRegistry::from_vault().await?;
    let (provider, model) = registry
        .resolve_model(&requested)
        .with_context(|| format!("could not resolve requested repair model {requested}"))?;
    let proposal = propose(provider.as_ref(), model, &snapshot).await?;
    let mut stdout = std::io::stdout().lock();
    serde_json::to_writer(&mut stdout, &proposal)?;
    writeln!(stdout)?;
    Ok(())
}

async fn initialize_secrets() {
    if codetether_secrets::secrets_manager().is_some() {
        return;
    }
    match codetether_secrets::SecretsManager::from_env().await {
        Ok(manager) => {
            let _ = codetether_secrets::init_from_manager(manager);
        }
        Err(error) => {
            tracing::warn!(error = %error, "Vault initialization unavailable; native provider credential policy remains authoritative")
        }
    }
}
