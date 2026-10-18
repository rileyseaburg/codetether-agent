//! Commands that bypass normal environment and agent-service initialization.
//!
//! [`dispatch`] handles Windows diagnostics and direct Bonsai execution. The
//! process lifecycle calls it before initializing logging, Vault and sessions.
use codetether_cli_command::Command;

/// Dispatch an optional command that can run before full application startup.
/// # Arguments
/// * `command` - The selected root command, borrowed from the parsed invocation.
/// # Returns
/// `Some` with the execution result for Windows or Bonsai; `None` for every
/// other command, allowing normal startup to continue.
/// # Errors
/// The returned result propagates the selected command's execution failure.
/// # Examples
/// ```no_run
/// # async fn example() {
/// assert!(codetether_cli_startup::native::dispatch(&None).await.is_none());
/// # }
/// ```
pub async fn dispatch(command: &Option<Command>) -> Option<anyhow::Result<()>> {
    match command {
        Some(Command::Windows(args)) => Some(codetether_windows_cli::run(args.clone()).await),
        Some(Command::Bonsai(args)) => Some(codetether_bonsai_cli::run(args.clone()).await),
        _ => None,
    }
}
