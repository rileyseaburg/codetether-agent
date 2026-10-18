//! Dispatch context commands using caller-provided session persistence.
use crate::{ContextArgs, ContextCommand, ResetSession};
use anyhow::Result;
use codetether_context_browse::BrowseSource;

/// Execute a context command with the supplied session implementation.
/// # Arguments
/// * `args` - Reset or browse options parsed by the application.
/// # Returns
/// Unit after the operation succeeds and its output has been printed.
/// # Errors
/// Returns directory lookup, persistence, browse or serialization errors.
/// # Examples
/// ```no_run
/// # async fn example<S: codetether_context_cli::ResetSession + codetether_context_browse::BrowseSource>(args: codetether_context_cli::ContextArgs) -> anyhow::Result<()> {
/// codetether_context_cli::execute::<S>(args).await
/// # }
/// ```
pub async fn execute<S: ResetSession + BrowseSource>(args: ContextArgs) -> Result<()> {
    match args.command {
        ContextCommand::Reset(args) => crate::reset::reset::<S>(args).await,
        ContextCommand::Browse(args) => crate::browse::browse::<S>(args).await,
    }
}
