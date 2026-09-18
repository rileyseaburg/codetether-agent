//! Saved interactive settings override stale shell values, not workload auth.

use super::Profile;
use anyhow::{Context, Result};

pub(crate) fn configured() -> Result<Option<super::super::VaultConfig>> {
    if std::env::var("CODETETHER_VAULT_SOURCE").as_deref() == Ok("env")
        || std::env::var("VAULT_ROLE").is_ok_and(|role| !role.trim().is_empty())
    {
        return Ok(None);
    }
    let Some(profile) = super::load()? else {
        return Ok(None);
    };
    let token = profile
        .token
        .context("Vault login is required; run codetether vault login")?;
    Ok(Some(super::super::VaultConfig {
        address: profile.address,
        token,
        mount: std::env::var("VAULT_MOUNT").ok(),
        path: std::env::var("VAULT_SECRETS_PATH").ok(),
    }))
}

pub(crate) fn environment() -> Result<Profile> {
    let address =
        std::env::var("VAULT_ADDR").unwrap_or_else(|_| "https://vault.spotlessbinco.com".into());
    Ok(Profile {
        address: super::normalize(&address)?,
        token: std::env::var("VAULT_TOKEN")
            .ok()
            .filter(|value| !value.trim().is_empty()),
    })
}

pub(crate) fn current() -> Result<Profile> {
    if let Some(profile) = super::load()? {
        return Ok(profile);
    }
    environment()
}
