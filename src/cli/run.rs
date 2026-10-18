//! Native host bindings for crate-owned non-interactive execution.
mod host;
mod sol_workflow;
use super::RunArgs;
codetether_native_cli::impl_run_session!(
    crate::session::Session,
    crate::session::import_codex_session_by_id,
    crate::session::DEFAULT_MAX_STEPS,
);
/// Execute a CLI run using the native application hosts.
/// # Errors
/// Returns shared workflow preparation, execution or output errors.
pub async fn execute(args: RunArgs) -> anyhow::Result<()> {
    codetether_run_cli::execute::<host::NativeHost>(args).await
}
