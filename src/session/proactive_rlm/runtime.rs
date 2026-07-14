//! Select the provider/model used for proactive RLM work.

use std::sync::Arc;

use crate::provider::Provider;
use crate::session::Session;

use super::types::Runtime;

pub(super) fn resolve(session: &Session, provider: Arc<dyn Provider>, model: &str) -> Runtime {
    let provider = session
        .metadata
        .subcall_provider
        .clone()
        .unwrap_or(provider);
    let model = session
        .metadata
        .subcall_model_name
        .clone()
        .unwrap_or_else(|| model.to_string());
    Runtime {
        provider,
        model,
        config: session.metadata.rlm.clone(),
    }
}
