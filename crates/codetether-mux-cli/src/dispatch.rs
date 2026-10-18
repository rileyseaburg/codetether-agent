//! Mux lifecycle command routing.

use anyhow::Result;

use crate::MuxCommand;

/// Execute a parsed mux CLI command with application-provided services.
///
/// # Arguments
/// * `command` - Parsed mux operation and its options.
/// * `host` - Native session workspace lookup and lazy clipboard access.
/// # Returns
/// Unit after the command completes or an attached client detaches.
/// # Errors
/// Returns validation, registry, process, network or terminal errors.
/// # Examples
/// ```no_run
/// # async fn example(host: &impl codetether_mux_cli::Host) -> anyhow::Result<()> {
/// codetether_mux_cli::execute(codetether_mux_cli::MuxCommand::List { json: true }, host).await
/// # }
/// ```
pub async fn execute(command: MuxCommand, host: &impl crate::Host) -> Result<()> {
    match command {
        MuxCommand::New {
            session,
            directory,
            start,
        } => super::new_session::run(session, directory, start, host).await,
        MuxCommand::Attach { target } => super::attach::run(&target, host).await,
        MuxCommand::Resume {
            session,
            name,
            start,
        } => super::resume::run(session, name, start, host).await,
        MuxCommand::List { json } => super::list::run(json).await,
        MuxCommand::Kill { target } => codetether_mux_lifecycle::kill::run(&target).await,
        MuxCommand::KillAll => codetether_mux_lifecycle::kill_all::run().await,
        MuxCommand::Serve {
            session,
            directory,
            bind,
            no_worktree,
        } => super::serve::run(session, directory, bind, no_worktree).await,
    }
}
