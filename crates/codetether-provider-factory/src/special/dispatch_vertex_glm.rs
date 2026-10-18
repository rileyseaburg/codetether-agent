//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_vertex_glm(secrets: &ProviderSecrets) -> Option<Arc<dyn Provider>> {
    use crate::vertex_glm;
    let sa = secrets
        .extra
        .get("service_account_json")
        .and_then(|v| v.as_str())?;
    let pid = secrets
        .extra
        .get("project_id")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("projectId").and_then(|v| v.as_str()))
        .map(String::from);
    match vertex_glm::VertexGlmProvider::new(sa, pid) {
        Ok(p) => Some(Arc::new(p)),
        Err(e) => {
            tracing::warn!("Failed to init vertex-glm: {e}");
            None
        }
    }
}
