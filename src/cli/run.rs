//! Native host bindings for crate-owned non-interactive execution.
mod host;
mod relay_host;
mod session_runtime;
mod session_source;
mod sol_workflow;
use super::RunArgs;
/// Execute a CLI run using the native application hosts.
/// # Errors
/// Returns shared workflow preparation, execution or output errors.
pub async fn execute(args: RunArgs) -> anyhow::Result<()> {
    codetether_run_cli::execute::<host::NativeHost>(args).await
}
