//! Acquire privately, validate least privilege, then atomically replace login state.

use super::args::Login;
use crate::secrets::login;
use anyhow::Result;

pub(super) async fn run(method: Login) -> Result<()> {
    let mut profile = login::current()?;
    eprintln!("Vault server: {}", profile.address);
    let candidate = match method {
        Login::Token { stdin } => {
            tokio::task::spawn_blocking(move || super::token_input::read(stdin)).await??
        }
        Login::Oidc {
            mount,
            role,
            no_browser,
        } => super::oidc::login(&profile.address, &mount, &role, no_browser).await?,
        Login::Device(args) => super::device::login(&profile.address, &args).await?,
    };
    let facts = login::validate::token(&profile.address, &candidate).await?;
    profile.token = Some(candidate);
    login::save(&profile)?;
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "authenticated": true,
            "address": profile.address,
            "facts": facts,
            "saved": true,
            "scope": "CodeTether user profile; existing processes and shell environment are unchanged"
        }))?
    );
    if std::env::var("CODETETHER_VAULT_SOURCE").as_deref() == Ok("env") {
        eprintln!(
            "CODETETHER_VAULT_SOURCE=env bypasses this saved profile; unset it to use the new login."
        );
    }
    Ok(())
}
