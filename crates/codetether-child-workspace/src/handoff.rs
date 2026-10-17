//! Identity of a retained child checkout and its integration base.
use serde::Serialize;
use std::path::PathBuf;
/// Checkout identity returned by [crate::allocate].
/// # Examples
/// ```rust
/// use codetether_child_workspace::Handoff;
/// let checkout = Handoff { workspace: "/child/project".into(), worktree: "/child".into(),
///     branch: "child/task".into(), parent_workspace: "/parent/project".into(),
///     base_commit: "example-commit".into() };
/// assert!(checkout.guidance().contains("Nothing is auto-merged"));
/// ```

#[derive(Serialize)]
pub struct Handoff {
    /// Working directory inside the allocated checkout.
    pub workspace: PathBuf,
    /// Root directory of the retained child checkout.
    pub worktree: PathBuf,
    /// Child branch created by the worktree manager.
    pub branch: String,
    /// Canonical requested parent working directory.
    pub parent_workspace: PathBuf,
    /// Committed parent HEAD used as the child base.
    pub base_commit: String,
}
