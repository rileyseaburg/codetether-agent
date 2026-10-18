//! Prefer cached subcall models over caller settings for proactive work.
use codetether_proactive_runtime::Runtime;
use codetether_provider_core::Provider;
use std::sync::Arc;

pub(super) fn resolve<S: crate::Source>(
    session: &S,
    provider: Arc<dyn Provider>,
    model: &str,
) -> Runtime {
    let provider = session.subcall_provider().cloned().unwrap_or(provider);
    let model = session
        .subcall_model_name()
        .map(str::to_string)
        .unwrap_or_else(|| model.to_string());
    Runtime {
        provider,
        model,
        config: session.rlm_config().clone(),
    }
}
