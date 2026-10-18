//! OKR Approval Gate for /go command
//!
//! Provides the interactive OKR approval flow:
//! /go <task> drafts an OKR via the model, shows it for approval (A/D keys),
//! then starts autochat execution on approval.

use std::sync::Arc;

use codetether_relay_okr::{PlannedOkrDraft, default_relay_okr_template, okr_from_planned_draft};

use crate::okr::OkrRepository;
pub use codetether_relay_okr::PendingOkrApproval;
pub use codetether_relay_okr::{is_easy_go_command, next_go_model};

/// Create a new pending approval by asking the configured model to draft the OKR.
/// Falls back to a safe template if providers are unavailable or the response can't be parsed.
pub async fn propose(task: String, agent_count: usize, model: String) -> PendingOkrApproval {
    let mut pending = PendingOkrApproval::new(task, agent_count, model);
    let okr_id = pending.okr.id;
    let registry = crate::provider::ProviderRegistry::shared_from_vault()
        .await
        .ok();

    let task = pending.task.clone();
    let agent_count = pending.agent_count;
    let model = pending.model.clone();

    let (okr, draft_note) = if let Some(registry) = &registry {
        match plan_okr_draft_with_registry(&task, &model, agent_count, registry).await {
            Some(planned) => (okr_from_planned_draft(okr_id, &task, planned), None),
            None => (
                default_relay_okr_template(okr_id, &task),
                Some("(OKR: fallback template — model draft parse failed)".to_string()),
            ),
        }
    } else {
        (
            default_relay_okr_template(okr_id, &task),
            Some("(OKR: fallback template — provider unavailable)".to_string()),
        )
    };

    pending.okr = okr;
    pending.draft_note = draft_note;
    pending
}

fn resolve_provider_for_model_autochat(
    registry: &Arc<crate::provider::ProviderRegistry>,
    model_ref: &str,
) -> Option<(Arc<dyn crate::provider::Provider>, String)> {
    crate::autochat::model_rotation::resolve_provider_for_model_autochat(registry, model_ref)
}
async fn plan_okr_draft_with_registry(
    task: &str,
    model_ref: &str,
    agent_count: usize,
    registry: &Arc<crate::provider::ProviderRegistry>,
) -> Option<PlannedOkrDraft> {
    let (provider, model_name) = resolve_provider_for_model_autochat(registry, model_ref)?;
    codetether_relay_okr::draft(task, model_name, agent_count, &provider).await
}

/// Initialize the OKR repository on AppState if not already present.
pub async fn ensure_okr_repository(repo: &mut Option<Arc<OkrRepository>>) {
    if repo.is_none() {
        if let Ok(new_repo) = OkrRepository::from_config().await {
            *repo = Some(Arc::new(new_repo));
        }
    }
}
