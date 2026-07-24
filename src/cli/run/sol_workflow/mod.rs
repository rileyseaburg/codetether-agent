//! Tool-free Sol planning around a separate tool-using code worker.

mod diagnostics;
mod model;
mod output;
mod planner;
mod prompts;
mod worker;
mod workflow;

use anyhow::Result;

use super::RunArgs;

/// Run the Sol planner/reviewer workflow selected by `--sol-planner`.
pub(super) async fn execute(args: RunArgs) -> Result<()> {
    let outcome = workflow::run(&args).await?;
    output::render(&args.format, &outcome)
}
