use std::{
    any::Any,
    future::Future,
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
/// Native operations used by the owned prompt executor.
/// # Implementors
/// Bind frontend session, worktree and event types without copying the session.
pub trait Host {
    /// Session moved into execution and returned in a notice.
    type Session: Send;
    /// Shared provider registry used by prompt execution.
    type Registry: Send + Sync;
    /// One input attachment.
    type Image: Send;
    /// Optional worktree cleanup state.
    type Worktree: Send;
    /// Streamed event payload.
    type Event: Send;
    /// Completion notice carrying the returned session.
    type Notice: Send;
    /// Application error preserved by notice conversion.
    type Error: Send;
    /// Create optional worktree isolation before polling cancellation.
    fn create_worktree(
        cwd: &Path,
        session: &mut Self::Session,
    ) -> impl Future<Output = Option<Self::Worktree>> + Send;
    /// Restore session directory after cooperative cancellation.
    fn restore_directory(session: &mut Self::Session, directory: Option<PathBuf>);
    /// Execute the prompt, including the native success-directory restoration.
    /// # Errors
    /// Returns native prompt execution failures for notice conversion.
    fn prompt(
        session: &mut Self::Session,
        prompt: &str,
        images: Vec<Self::Image>,
        events: mpsc::Sender<Self::Event>,
        registry: Arc<Self::Registry>,
        original_dir: Option<PathBuf>,
    ) -> impl Future<Output = Result<(), Self::Error>> + Send;
    /// Convert success, failure or caught panic into an owned notice.
    fn notice(
        result: Result<Result<(), Self::Error>, Box<dyn Any + Send>>,
        session: Self::Session,
    ) -> Self::Notice;
    /// Whether the notice represents a successful or cancelled prompt.
    fn finished(notice: &Self::Notice) -> bool;
    /// Apply native worktree merge/cleanup before publishing the notice.
    fn worktree_result(
        success: bool,
        worktree: Option<Self::Worktree>,
        prompt: Option<&str>,
    ) -> impl Future<Output = ()> + Send;
}
