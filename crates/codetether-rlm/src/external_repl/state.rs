//! Child process handle and runtime label.
use super::*;
use tokio::process::Child;

/// Own an external Bun/Node REPL process
///
/// # Examples
/// ```no_run
/// # async fn example() -> anyhow::Result<()> {
/// let mut repl = codetether_rlm::external_repl::ExternalRepl::spawn_bun("context").await?;
/// repl.destroy().await?;
/// # Ok(()) }
/// ```
pub struct ExternalRepl {
    pub(super) child: Child,
    #[allow(dead_code)]
    pub(super) runtime: ReplRuntime,
}
