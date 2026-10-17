//! Hook-checked, attributed publication of worker task changes.
//!
//! [run] stages, commits and pushes changes. Hosts receive progress through
//! [PublicationCheckpoint], without introducing a worker runtime dependency.
//! [require_installed] exposes the same hook precondition independently.
use super::git::run_git_command_at;
use anyhow::Result;
use std::path::Path;
mod command;
mod commit;
mod hooks;
mod provenance;
mod push;
mod run;
mod status;
use command::git;
use commit::commit;
pub use hooks::require_installed;
use push::push;
pub use run::run;
use status::status;

/// Progress points emitted while publishing repository changes.
///
/// Variants identify staging, commit creation, push start, and push completion.
/// # Examples
/// ```rust
/// use codetether_worktree::repository::publication::PublicationCheckpoint;
/// let event = PublicationCheckpoint::Created;
/// let label = match event {
///     PublicationCheckpoint::Staging => "staging",
///     PublicationCheckpoint::Created => "committed",
///     PublicationCheckpoint::Pushing => "pushing",
///     PublicationCheckpoint::Pushed => "pushed",
/// };
/// assert_eq!(label, "committed");
/// ```
pub enum PublicationCheckpoint {
    /// Changes are about to be staged.
    Staging,
    /// A commit has been created.
    Created,
    /// The commit is about to be pushed.
    Pushing,
    /// The push succeeded.
    Pushed,
}
