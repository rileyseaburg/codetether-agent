//! Model and access-policy fields refreshed during child reload.

use crate::ChildSession;
use codetether_agent_lifecycle::ResumeConfig;
use codetether_agent_registry::AgentEntry;

pub(super) fn apply<S: ChildSession>(entry: &mut AgentEntry<S>, config: &ResumeConfig) -> bool {
    let model_changed = apply_model(entry, config.model.as_ref());
    let policy_changed = apply_policy(entry, config.prior_context_allowed);
    model_changed || policy_changed
}

fn apply_model<S: ChildSession>(entry: &mut AgentEntry<S>, model: Option<&String>) -> bool {
    let Some(model) = model else { return false };
    if entry.session.settings().model.as_ref() == Some(model)
        && entry.model_id.as_ref() == Some(model)
    {
        return false;
    }
    *entry.session.settings().model = Some(model.clone());
    entry.model_id = Some(model.clone());
    true
}

fn apply_policy<S: ChildSession>(entry: &mut AgentEntry<S>, allowed: Option<bool>) -> bool {
    let Some(allowed) = allowed else { return false };
    if *entry.session.settings().inherited_prior_context_allowed == Some(allowed) {
        return false;
    }
    *entry.session.settings().inherited_prior_context_allowed = Some(allowed);
    true
}
