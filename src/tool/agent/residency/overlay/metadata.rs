//! Model and access-policy fields refreshed during child reload.

use super::super::ResumeConfig;
use crate::tool::agent::store::AgentEntry;

pub(super) fn apply(entry: &mut AgentEntry, config: &ResumeConfig) -> bool {
    let model_changed = apply_model(entry, config.model.as_ref());
    let policy_changed = apply_policy(entry, config.prior_context_allowed);
    model_changed || policy_changed
}

fn apply_model(entry: &mut AgentEntry, model: Option<&String>) -> bool {
    let Some(model) = model else { return false };
    if entry.session.metadata.model.as_ref() == Some(model)
        && entry.model_id.as_ref() == Some(model)
    {
        return false;
    }
    entry.session.metadata.model = Some(model.clone());
    entry.model_id = Some(model.clone());
    true
}

fn apply_policy(entry: &mut AgentEntry, allowed: Option<bool>) -> bool {
    let Some(allowed) = allowed else { return false };
    if entry.session.metadata.inherited_prior_context_allowed == Some(allowed) {
        return false;
    }
    entry.session.metadata.inherited_prior_context_allowed = Some(allowed);
    true
}
