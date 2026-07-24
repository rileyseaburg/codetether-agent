//! Rendering for the Sol planner workflow's command-line result.

use anyhow::Result;
use serde::Serialize;

use super::workflow::Outcome;

#[derive(Serialize)]
struct JsonOutcome<'a> {
    plan: &'a str,
    response: &'a str,
    session_id: &'a str,
    worker_model: &'a str,
    lsp_review_rounds: usize,
    unresolved_diagnostics: usize,
}

/// Render a workflow result using the regular `run` output formats.
pub(super) fn render(format: &str, outcome: &Outcome) -> Result<()> {
    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&json(outcome))?),
        "jsonl" => super::super::jsonl::write_completed(
            Some(&outcome.session_id),
            &outcome.final_text,
            None,
        )?,
        _ => print_default(outcome),
    }
    Ok(())
}

fn json(outcome: &Outcome) -> JsonOutcome<'_> {
    JsonOutcome {
        plan: &outcome.plan,
        response: &outcome.final_text,
        session_id: &outcome.session_id,
        worker_model: &outcome.worker_model,
        lsp_review_rounds: outcome.review_rounds,
        unresolved_diagnostics: outcome.unresolved_diagnostics,
    }
}

fn print_default(outcome: &Outcome) {
    println!(
        "Sol plan:\n{}\n\nWorker result:\n{}",
        outcome.plan, outcome.final_text
    );
    eprintln!(
        "\n[Session: {} | Worker: {} | Tool-free Sol LSP reviews: {} | Remaining diagnostics: {}]",
        outcome.session_id,
        outcome.worker_model,
        outcome.review_rounds,
        outcome.unresolved_diagnostics
    );
}
