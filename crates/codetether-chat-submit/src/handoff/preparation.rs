use super::Transfer;
use std::path::PathBuf;
/// Native session metadata and request construction for one handoff.
/// # Implementors
/// UI adapters read current execution settings and move the checked-out session.
pub trait Preparation: Transfer {
    /// Image attachment moved into the request.
    type Image;
    /// Outer None means no idle session; inner None means no original directory.
    fn original_directory(&self) -> Option<Option<PathBuf>>;
    /// Whether the current prompt requests an isolated worktree.
    fn use_worktree(&self) -> bool;
    /// Apply current edit and worktree settings to an available session.
    fn configure(&mut self, use_worktree: bool);
    /// Take the session for an owned prompt request.
    fn take_session(&mut self) -> Option<Self::Session>;
    /// Construct the runtime request from the checked-out session and inputs.
    fn request(
        &self,
        session: Self::Session,
        prompt: String,
        images: Vec<Self::Image>,
        original_dir: Option<PathBuf>,
        worktree_root: Option<PathBuf>,
    ) -> Self::Request;
}
