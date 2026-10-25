//! Resolve Gemini cookies without exposing values or modifying the source.
use anyhow::{Result, bail};

pub(super) async fn load() -> Result<String> {
    let hardened = std::env::var("CODETETHER_DISABLE_ENV_FALLBACK").as_deref() == Ok("1");
    if !hardened && let Some(path) = std::env::var_os("GEMINI_WEB_COOKIE_FILE") {
        let bytes = tokio::fs::read(path)
            .await
            .map_err(|_| anyhow::anyhow!("Gemini cookie file could not be read"))?;
        return super::cookie_file::decode(&bytes);
    }
    let Some(secrets) = codetether_secrets::get_provider_secrets("gemini-web").await else {
        bail!(
            "Gemini Web cookies unavailable: configure Vault provider gemini-web or GEMINI_WEB_COOKIE_FILE"
        );
    };
    secrets
        .extra
        .get("cookies")
        .and_then(serde_json::Value::as_str)
        .filter(|value| !value.trim().is_empty())
        .map(str::to_owned)
        .ok_or_else(|| anyhow::anyhow!("Vault gemini-web provider has no cookies export"))
}
