//! Browser OIDC uses Vault's callback implementation, but CodeTether owns persistence.
//!
//! The subprocess receives no Vault token and cannot update the Vault CLI cache.
//! Its returned token stays private until validation permits atomic profile saving.

mod process;
mod stream;
use anyhow::{Result, ensure};
use serde::Deserialize;

#[derive(Deserialize)]
struct Reply {
    auth: Auth,
}
#[derive(Deserialize)]
struct Auth {
    client_token: String,
}

pub(super) async fn login(
    address: &str,
    mount: &str,
    role: &str,
    no_browser: bool,
) -> Result<String> {
    crate::secrets::login::http::auth_path(mount, role)?;
    eprintln!(
        "Sign in using Vault OIDC. Under SSH, forward localhost:8250 to this VM, or use device login."
    );
    let output = tokio::time::timeout(
        std::time::Duration::from_secs(900),
        process::run(address, mount, role, no_browser),
    )
    .await
    .map_err(|_| anyhow::anyhow!("Vault OIDC login timed out"))??;
    let reply: Reply = serde_json::from_str(&output)
        .map_err(|_| anyhow::anyhow!("Vault CLI did not return a credential"))?;
    ensure!(
        !reply.auth.client_token.is_empty(),
        "Vault OIDC response omitted a token"
    );
    Ok(reply.auth.client_token)
}
