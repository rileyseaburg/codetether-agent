//! Sol planning and bounded LSP review around an application coding worker.
//!
//! [execute] runs the initial plan, implementation and up to two repair rounds.
//! [Host] keeps native provider, session and diagnostics setup in the application.
mod host;
mod model;
mod outcome;
mod output;
mod prompts;
mod request;
mod workflow;
pub use host::{Host, Worker};
pub use model::{SOL_PLANNER_MODEL, worker_model};
pub use request::{request, text};
/// Execute the workflow and render the final CLI result.
/// # Arguments
/// * `args` - User task, worker configuration and output format.
/// # Returns
/// Unit once the workflow result is printed.
/// # Errors
/// Returns planning, worker, diagnostics or output errors.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_sol_workflow::Host>(args: codetether_cli_options::RunArgs) -> anyhow::Result<()> {
/// codetether_sol_workflow::execute::<H>(args).await?;
/// # Ok(()) }
/// ```
pub async fn execute<H: Host>(args: codetether_cli_options::RunArgs) -> anyhow::Result<()> {
    let outcome = workflow::run::<H>(&args).await?;
    output::render(&args.format, &outcome)
}
