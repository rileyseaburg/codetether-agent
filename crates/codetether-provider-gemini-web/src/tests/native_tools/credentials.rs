//! Explicit live-test credentials; never print Vault output or cookie values.
use crate::GeminiWebProvider;
use anyhow::{Context, Result, ensure};

pub(super) fn load() -> Result<String> {
    let output = std::process::Command::new("vault")
        .args([
            "kv",
            "get",
            "-format=json",
            "secret/codetether/providers/gemini-web",
        ])
        .output()
        .context("could not start Vault CLI")?;
    ensure!(output.status.success(), "Vault cookie lookup failed");
    let value: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|_| anyhow::anyhow!("Vault returned invalid JSON"))?;
    Ok(value["data"]["data"]["cookies"]
        .as_str()
        .filter(|cookies| !cookies.trim().is_empty())
        .context("Vault cookie record is missing")?
        .to_owned())
}

pub(super) async fn redact(error: &anyhow::Error, provider: &GeminiWebProvider) -> String {
    let mut text = format!("{error:#}");
    let header = provider.cookie_header();
    let mut secrets = vec![provider.cookies.clone(), header.clone()];
    secrets.extend(
        header
            .split(';')
            .filter_map(|pair| pair.trim().split_once('='))
            .map(|(_, value)| value.to_owned()),
    );
    if let Some((tokens, _)) = provider.token_cache.lock().await.as_ref() {
        secrets.extend([tokens.at_token.clone(), tokens.f_sid.clone()]);
    }
    for secret in secrets.iter().filter(|secret| !secret.is_empty()) {
        text = text.replace(secret, "[REDACTED]");
    }
    text
}
