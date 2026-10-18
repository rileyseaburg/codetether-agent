//! Model-provider dispatch that does not require an API key.
use codetether_provider_core::Provider;
use codetether_secrets::ProviderSecrets;
use std::sync::Arc;
pub(super) fn dispatch<H: crate::Host>(
    id: &str,
    secrets: &ProviderSecrets,
) -> Option<Option<Arc<dyn Provider>>> {
    Some(match id {
        "bonsai" => crate::bonsai::vault(secrets),
        "local-cuda" | "local_cuda" | "localcuda" => {
            crate::special::dispatch_local_cuda::<H>(secrets)
        }
        "huggingface" => crate::special::dispatch_huggingface(secrets),
        _ => return None,
    })
}
