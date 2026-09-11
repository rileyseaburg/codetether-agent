//! Shared startup options for mux session-creating subcommands.

use clap::Args;

/// Options that control how a new mux server is started.
///
/// # Examples
///
/// ```
/// use codetether_agent::cli::command::mux_args::MuxStartOptions;
///
/// let options = MuxStartOptions {
///     detached: true,
///     no_worktree: true,
/// };
/// assert!(options.no_worktree);
/// ```
#[derive(Args, Debug, PartialEq, Eq)]
pub struct MuxStartOptions {
    /// Leave the server detached instead of opening its client.
    #[arg(short = 'd', long)]
    pub detached: bool,
    /// Reuse the requested checkout instead of a managed worktree.
    ///
    /// Shares the checkout only: if a mux server already owns that directory
    /// the new session joins it as an isolated runtime, otherwise one starts.
    #[arg(long = "no-worktree")]
    pub no_worktree: bool,
}
