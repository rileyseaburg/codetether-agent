//! Serializable CLI projection of workflow results.
use super::Outcome;
use serde::Serialize;

#[derive(Serialize)]
pub(super) struct JsonOutcome<'a> {
    plan: &'a str,
    response: &'a str,
    session_id: &'a str,
    worker_model: &'a str,
    lsp_review_rounds: usize,
    unresolved_diagnostics: usize,
}

pub(super) fn json(outcome: &Outcome) -> JsonOutcome<'_> {
    JsonOutcome {
        plan: &outcome.plan,
        response: &outcome.final_text,
        session_id: &outcome.session_id,
        worker_model: &outcome.worker_model,
        lsp_review_rounds: outcome.review_rounds,
        unresolved_diagnostics: outcome.unresolved_diagnostics,
    }
}
