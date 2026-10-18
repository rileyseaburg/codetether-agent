//! OKR Approval Gate for /go command
//!
//! Provides the interactive OKR approval flow:
//! /go <task> drafts an OKR via the model, shows it for approval (A/D keys),
//! then starts autochat execution on approval.

use std::sync::Arc;

use codetether_relay_okr::{PlannedOkrDraft, default_relay_okr_template, okr_from_planned_draft};
use uuid::Uuid;

use crate::okr::{Okr, OkrRepository, OkrRun};
use crate::tui::constants::{GO_SWAP_MODEL_GLM, GO_SWAP_MODEL_MINIMAX};
use crate::tui::utils::helpers::truncate_with_ellipsis;

/// Pending OKR approval gate state for PRD-gated relay commands.
pub struct PendingOkrApproval {
    /// The OKR being proposed
    pub okr: Okr,
    /// The OKR run being proposed
    pub run: OkrRun,
    /// Optional note when we had to fall back to a template draft
    pub draft_note: Option<String>,
    /// Original task that triggered the OKR
    pub task: String,
    /// Agent count for the relay
    pub agent_count: usize,
    /// Model to use
    pub model: String,
}

impl PendingOkrApproval {
    /// Create a new pending approval from a task using the default template.
    pub fn new(task: String, agent_count: usize, model: String) -> Self {
        let okr_id = Uuid::new_v4();
        let okr = default_relay_okr_template(okr_id, &task);

        let mut run = OkrRun::new(
            okr_id,
            format!("Run {}", chrono::Local::now().format("%Y-%m-%d %H:%M")),
        );
        let _ = run.submit_for_approval();

        Self {
            okr,
            run,
            draft_note: None,
            task,
            agent_count,
            model,
        }
    }

    /// Create a new pending approval by asking the configured model to draft the OKR.
    /// Falls back to a safe template if providers are unavailable or the response can't be parsed.
    pub async fn propose(task: String, agent_count: usize, model: String) -> Self {
        let mut pending = Self::new(task, agent_count, model);
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

    /// Get the approval prompt text
    pub fn approval_prompt(&self) -> String {
        let krs: Vec<String> = self
            .okr
            .key_results
            .iter()
            .map(|kr| format!("  • {} (target: {} {})", kr.title, kr.target_value, kr.unit))
            .collect();

        let note_line = self
            .draft_note
            .as_deref()
            .map(|note| format!("{note}\n"))
            .unwrap_or_default();

        format!(
            "⚠️  Relay OKR Draft\n\n\
             Task: {task}\n\
             Agents: {agents} | Model: {model}\n\n\
             {note_line}\
             Objective: {objective}\n\n\
             Key Results:\n{key_results}\n\n\
             Press [A] to approve or [D] to deny",
            task = truncate_with_ellipsis(&self.task, 100),
            agents = self.agent_count,
            model = self.model,
            note_line = note_line,
            objective = self.okr.title,
            key_results = krs.join("\n"),
        )
    }
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

/// Check whether the input is a `/go` or `/team` easy command.
pub fn is_easy_go_command(input: &str) -> bool {
    let command = input
        .split_whitespace()
        .next()
        .unwrap_or("")
        .to_ascii_lowercase();

    matches!(command.as_str(), "/go" | "/team")
}

fn is_glm5_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    matches!(
        normalized.as_str(),
        "zai/glm-5"
            | "z-ai/glm-5"
            | "openrouter/z-ai/glm-5"
            | "glm5/glm-5-fp8"
            | "glm5/glm-5"
            | "glm5:glm-5-fp8"
            | "glm5:glm-5"
    )
}

fn is_minimax_model(model: &str) -> bool {
    let normalized = model.trim().to_ascii_lowercase();
    normalized.contains("minimax")
}

/// Rotate between available models for the /go command.
/// Alternates between MiniMax and GLM-5 based on the current model.
pub fn next_go_model(current_model: Option<&str>) -> String {
    match current_model {
        Some(model) if is_glm5_model(model) => GO_SWAP_MODEL_MINIMAX.to_string(),
        Some(model) if is_minimax_model(model) => GO_SWAP_MODEL_GLM.to_string(),
        _ => GO_SWAP_MODEL_MINIMAX.to_string(),
    }
}

/// Initialize the OKR repository on AppState if not already present.
pub async fn ensure_okr_repository(repo: &mut Option<Arc<OkrRepository>>) {
    if repo.is_none() {
        if let Ok(new_repo) = OkrRepository::from_config().await {
            *repo = Some(Arc::new(new_repo));
        }
    }
}
