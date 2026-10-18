//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_gemini_web(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::gemini_web;
    let cookies = secrets.extra.get("cookies").and_then(|v| v.as_str())?;
    match gemini_web::GeminiWebProvider::new(cookies.into()) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init gemini-web: {e}");
            None
        }
    }
}
