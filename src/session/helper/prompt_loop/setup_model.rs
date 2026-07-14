//! Resolution of the initial provider and model runtime.

use super::model::ModelState;
use crate::provider::ProviderRegistry;
use crate::session::Session;
use anyhow::Result;
use std::sync::Arc;

/// Resolves the initial provider, model, and tool configuration.
///
/// # Errors
///
/// Returns an error when no provider is available or the selection is invalid.
pub(super) fn resolve(session: &Session, registry: &Arc<ProviderRegistry>) -> Result<ModelState> {
    let providers = registry.list();
    if providers.is_empty() {
        anyhow::bail!(
            "No providers available. Configure provider credentials in HashiCorp Vault (for ChatGPT subscription Codex use `codetether auth codex`; for Copilot use `codetether auth copilot`)."
        );
    }
    let (requested, requested_model) = super::selector::parse(session, &providers);
    let name = super::super::provider::resolve_provider_for_session_request(
        &providers,
        requested.as_deref(),
    )?
    .to_string();
    let provider = registry
        .get(&name)
        .ok_or_else(|| anyhow::anyhow!("Provider {name} not found"))?;
    let model_id = if requested_model.is_empty() {
        super::super::defaults::default_model_for_provider(&name)
    } else {
        requested_model
    };
    let step = super::super::request_state::build_provider_step_state(
        provider.clone(),
        &name,
        &model_id,
        session,
    );
    Ok(ModelState::new(name, model_id, provider, step))
}
