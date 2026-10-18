//! Native bypass and environment/service initialization order.
use super::Host;
use anyhow::Result;
use codetether_cli_command::Command;

/// Initialize the CLI process and dispatch its selected command.
///
/// # Arguments
/// The `H` type supplies native parser, early-command, and dispatch operations.
/// # Returns
/// The early-command result or the final dispatched command result.
/// # Errors
/// Propagates early-command, environment initialization, and dispatch failures.
/// # Examples
/// ```no_run
/// # async fn example<H: codetether_cli_startup::lifecycle::Host>() -> anyhow::Result<()> {
/// codetether_cli_startup::lifecycle::run::<H>().await
/// # }
/// ```
pub async fn run<H: Host>() -> Result<()> {
    if let Some(result) = H::early().await {
        return result;
    }
    crate::environment::initialize()?;
    let invocation = H::parse();
    if let Some(result) = H::native(&invocation).await {
        return result;
    }
    let command = H::command(&invocation);
    let is_tui = matches!(command, Some(Command::Tui(_)) | None);
    let is_git_credential_helper = matches!(command, Some(Command::GitCredentialHelper(_)));
    crate::logging::initialize(is_tui, is_git_credential_helper);
    H::initialize_crash(is_tui).await;
    crate::vault::initialize(command, is_tui, is_git_credential_helper).await;
    H::dispatch(invocation).await
}
