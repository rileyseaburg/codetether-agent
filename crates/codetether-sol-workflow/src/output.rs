//! Rendering for the Sol planner workflow's command-line result.

use anyhow::Result;
#[path = "output_json.rs"]
mod json;
use json::json;

use super::outcome::Outcome;

/// Render a workflow result using the regular `run` output formats.
pub(super) fn render(format: &str, outcome: &Outcome) -> Result<()> {
    match format {
        "json" => println!("{}", serde_json::to_string_pretty(&json(outcome))?),
        "jsonl" => codetether_run_jsonl::write_completed(
            Some(&outcome.session_id),
            &outcome.final_text,
            None,
        )?,
        _ => print_default(outcome),
    }
    Ok(())
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
