//! Non-secret effective login status; workload initialization remains separate.
use crate::secrets::login;
use anyhow::Result;
pub(super) async fn status() -> Result<()> {
    let env_only = std::env::var("CODETETHER_VAULT_SOURCE").as_deref() == Ok("env");
    let workload = std::env::var("VAULT_ROLE").is_ok_and(|role| !role.trim().is_empty());
    if workload {
        println!(
            "{}",
            serde_json::json!({"credential_source":"kubernetes", "authenticated":null, "note":"Workload authentication is initialized by the runtime, not this status command"})
        );
        return Ok(());
    }
    let profile = if env_only {
        login::environment()?
    } else {
        login::current()?
    };
    let facts = match &profile.token {
        Some(token) => Some(login::validate::token(&profile.address, token).await?),
        None => None,
    };
    println!(
        "{}",
        serde_json::to_string_pretty(&serde_json::json!({
            "address": profile.address,
            "credential_present": profile.token.is_some(),
            "authenticated": facts.is_some(),
            "facts": facts,
            "saved_profile_selected": !env_only && login::load()?.is_some(),
            "environment_override": std::env::var("CODETETHER_VAULT_SOURCE").as_deref() == Ok("env")
        }))?
    );
    Ok(())
}
