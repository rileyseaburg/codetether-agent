//! Activation of an alternate provider/model after upstream failure.

use super::super::{Runner, model::ModelState};
use crate::session::Bucket;
use anyhow::Result;

/// Switches to a bandit-selected provider/model target when available.
///
/// # Errors
///
/// Returns an error when the selected provider is absent from the registry.
pub(super) fn apply(runner: &mut Runner<'_>, bucket: Bucket) -> Result<bool> {
    let Some((name, model_id)) = super::super::super::router::choose_router_target_bandit(
        &runner.registry,
        &runner.session.metadata.delegation,
        bucket,
        &runner.model.provider_name,
        &runner.model.model_id,
    ) else {
        return Ok(false);
    };
    tracing::info!(to_provider = %name, to_model = %model_id,
        "Failing over to alternate provider/model");
    let provider = runner
        .registry
        .get(&name)
        .ok_or_else(|| anyhow::anyhow!("Provider {name} not found"))?;
    let step = super::super::super::request_state::build_provider_step_state(
        provider.clone(),
        &name,
        &model_id,
        runner.session,
    );
    runner.model = ModelState::new(name, model_id, provider, step);
    Ok(true)
}
