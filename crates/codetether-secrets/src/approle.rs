//! Noninteractive Vault login with an explicitly provisioned AppRole identity.
//!
//! Secret IDs remain in an operator-managed file, re-read on each login.
mod config;
mod login;
mod manager;
pub(crate) use config::AppRoleAuth;

pub(crate) fn configured() -> anyhow::Result<Option<AppRoleAuth>> {
    if std::env::var("VAULT_AUTH_METHOD").as_deref() != Ok("approle") {
        return Ok(None);
    }
    use anyhow::Context;
    Ok(Some(AppRoleAuth::new(
        &std::env::var("VAULT_ADDR").context("AppRole requires VAULT_ADDR")?,
        &std::env::var("VAULT_ROLE_ID").context("AppRole requires VAULT_ROLE_ID")?,
        &std::env::var("VAULT_AUTH_MOUNT").unwrap_or_else(|_| "approle".into()),
        std::env::var_os("VAULT_SECRET_ID_FILE")
            .context("AppRole requires VAULT_SECRET_ID_FILE")?
            .into(),
    )?))
}
