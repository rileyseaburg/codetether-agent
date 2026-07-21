use super::auth::ImagesAuth;
use anyhow::{Result, bail};

pub(super) async fn resolve() -> Result<ImagesAuth> {
    if let Some(auth) = super::vault_credentials::resolve().await? {
        return Ok(auth);
    }
    let disabled = std::env::var("CODETETHER_DISABLE_ENV_FALLBACK")
        .is_ok_and(|value| value == "1" || value.eq_ignore_ascii_case("true"));
    if !disabled
        && let Ok(key) = std::env::var("OPENAI_API_KEY")
        && !key.trim().is_empty()
    {
        return Ok(ImagesAuth::openai(key));
    }
    bail!("image credentials unavailable in Vault or permitted environment fallbacks")
}
