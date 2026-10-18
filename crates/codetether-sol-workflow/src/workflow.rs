//! Bounded orchestration of planning, coding, diagnostics, and repair.

use anyhow::Result;
use std::path::PathBuf;

use super::{Host, Worker, outcome::Outcome, prompts};
use codetether_cli_options::RunArgs;

const MAX_LSP_REVIEW_ROUNDS: usize = 2;

/// Execute one initial implementation pass and at most two Sol-guided repairs.
pub(super) async fn run<H: Host>(args: &RunArgs) -> Result<Outcome> {
    let workspace = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let (system, user) = prompts::plan(&args.message);
    let plan = H::complete(system, user).await?;
    let mut worker = H::start(args, workspace.clone()).await?;
    let worker_model = worker.model();
    let mut final_text = worker
        .execute(&prompts::implement(&args.message, &plan))
        .await?;
    let mut review_rounds = 0;
    while review_rounds < MAX_LSP_REVIEW_ROUNDS {
        let Some(report) = H::collect(&workspace, &worker).await? else {
            break;
        };
        let (system, user) = prompts::review(&args.message, &plan, &report.prompt);
        let review = H::complete(system, user).await?;
        final_text = worker
            .execute(&prompts::repair(&review, &report.prompt))
            .await?;
        review_rounds += 1;
    }
    let unresolved_diagnostics = H::collect(&workspace, &worker)
        .await?
        .map_or(0, |report| report.issue_count);
    Ok(Outcome {
        plan,
        final_text,
        session_id: worker.session_id(),
        review_rounds,
        unresolved_diagnostics,
        worker_model,
    })
}
