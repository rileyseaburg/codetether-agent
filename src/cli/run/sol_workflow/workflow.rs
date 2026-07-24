//! Bounded orchestration of planning, coding, diagnostics, and repair.

use anyhow::Result;
use std::path::PathBuf;

use super::super::RunArgs;
use super::{diagnostics, planner, prompts, worker::Worker};

const MAX_LSP_REVIEW_ROUNDS: usize = 2;

/// Final state needed by the CLI renderer.
pub(super) struct Outcome {
    pub(super) plan: String,
    pub(super) final_text: String,
    pub(super) session_id: String,
    pub(super) review_rounds: usize,
    pub(super) unresolved_diagnostics: usize,
    pub(super) worker_model: String,
}

/// Execute one initial implementation pass and at most two Sol-guided repairs.
pub(super) async fn run(args: &RunArgs) -> Result<Outcome> {
    let workspace = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
    let (system, user) = prompts::plan(&args.message);
    let plan = planner::complete(system, user).await?;
    let mut worker = Worker::start(args, workspace.clone()).await?;
    let worker_model = worker.session.metadata.model.clone().unwrap_or_default();
    let mut final_text = worker
        .execute(&prompts::implement(&args.message, &plan))
        .await?;
    let mut review_rounds = 0;
    while review_rounds < MAX_LSP_REVIEW_ROUNDS {
        let Some(report) = diagnostics::collect(&workspace, &worker.session).await? else {
            break;
        };
        let (system, user) = prompts::review(&args.message, &plan, &report.prompt);
        let review = planner::complete(system, user).await?;
        final_text = worker
            .execute(&prompts::repair(&review, &report.prompt))
            .await?;
        review_rounds += 1;
    }
    let unresolved_diagnostics = diagnostics::collect(&workspace, &worker.session)
        .await?
        .map_or(0, |report| report.issue_count);
    Ok(Outcome {
        plan,
        final_text,
        session_id: worker.session.id.clone(),
        review_rounds,
        unresolved_diagnostics,
        worker_model,
    })
}
