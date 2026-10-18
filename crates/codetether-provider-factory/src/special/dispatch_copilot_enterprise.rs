//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_copilot_enterprise<H: crate::Host>(
    key: &str,
    secrets: &ProviderSecrets,
) -> Option<Arc<dyn Provider>> {
    let ent_url = secrets
        .extra
        .get("enterpriseUrl")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("enterprise_url").and_then(|v| v.as_str()));
    let result = if let Some(url) = &secrets.base_url {
        H::copilot_with_base_url(key.into(), url.clone(), "github-copilot-enterprise")
    } else if let Some(u) = ent_url {
        H::copilot_enterprise(key.into(), u.into())
    } else {
        H::copilot_with_base_url(
            key.into(),
            "https://api.githubcopilot.com".into(),
            "github-copilot-enterprise",
        )
    };
    match result {
        Ok(p) => Some(p),
        Err(e) => {
            tracing::warn!("Failed to init github-copilot-enterprise: {e}");
            None
        }
    }
}
