//! Provider-specific secret parsing and construction policy.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;

pub(crate) fn dispatch_local_cuda<H: crate::Host>(
    secrets: &ProviderSecrets,
) -> Option<Arc<dyn Provider>> {
    let name = secrets
        .extra
        .get("model_name")
        .and_then(|v| v.as_str())
        .or_else(|| secrets.extra.get("model").and_then(|v| v.as_str()))
        .unwrap_or("qwen3.5-9b")
        .to_string();
    let path = secrets
        .extra
        .get("model_path")
        .and_then(|v| v.as_str())
        .map(String::from);
    let tok_path = secrets
        .extra
        .get("tokenizer_path")
        .and_then(|v| v.as_str())
        .map(String::from);
    let arch = secrets
        .extra
        .get("architecture")
        .and_then(|v| v.as_str())
        .map(String::from);
    let result = if let Some(p) = path {
        H::local_cuda_with_paths(name.clone(), p, tok_path, arch)
    } else {
        H::local_cuda_new(name.clone())
    };
    match result {
        Ok(p) => {
            tracing::info!(model = %name, "Registered Local CUDA from Vault");
            Some(p)
        }
        Err(e) => {
            tracing::warn!("Failed to init local-cuda: {e}");
            None
        }
    }
}
