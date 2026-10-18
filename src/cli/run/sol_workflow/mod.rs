//! Native bindings for the crate-owned Sol planning workflow.
mod diagnostics;
mod host;
mod planner;
mod worker;
/// Run the Sol planner/reviewer workflow selected by `--sol-planner`.
pub(super) async fn execute(args: super::RunArgs) -> anyhow::Result<()> {
    codetether_sol_workflow::execute::<host::NativeHost>(args).await
}
