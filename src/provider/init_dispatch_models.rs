//! Model-provider dispatch that does not require an API key.
use super::Provider;
use crate::secrets::ProviderSecrets;
use std::sync::Arc;
pub(super) fn dispatch(id: &str, secrets: &ProviderSecrets) -> Option<Option<Arc<dyn Provider>>> {
    Some(match id {
        "bonsai" => super::bonsai::registration::vault(secrets),
        "local-cuda" | "local_cuda" | "localcuda" => {
            super::init_dispatch_impl::dispatch_local_cuda(secrets)
        }
        "huggingface" => super::init_dispatch_impl::dispatch_huggingface(secrets),
        _ => return None,
    })
}
