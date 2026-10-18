//! Native prompt, worktree and completion-notice operations.
use super::{SessionNotice, prompt_result::PromptRunResult};
use crate::tui::app::input::worktree_result::{handle_worktree_result, run_prompt};
use crate::{
    provider::ProviderRegistry,
    session::{ImageAttachment, Session, SessionEvent},
};
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};
use tokio::sync::mpsc;
pub(super) struct Native;
impl codetether_session_runtime::execute::Host for Native {
    type Session = Session;
    type Registry = ProviderRegistry;
    type Image = ImageAttachment;
    type Worktree = crate::tui::app::input::worktree::WorktreeState;
    type Event = SessionEvent;
    type Notice = SessionNotice;
    type Error = anyhow::Error;
    async fn create_worktree(cwd: &Path, session: &mut Session) -> Option<Self::Worktree> {
        crate::tui::app::input::worktree::create_worktree(cwd, session).await
    }
    fn restore_directory(session: &mut Session, directory: Option<PathBuf>) {
        session.metadata.directory = directory;
    }
    async fn prompt(
        session: &mut Session,
        prompt: &str,
        images: Vec<ImageAttachment>,
        events: mpsc::Sender<SessionEvent>,
        registry: Arc<ProviderRegistry>,
        original_dir: Option<PathBuf>,
    ) -> anyhow::Result<()> {
        run_prompt(session, prompt, images, events, registry, original_dir).await
    }
    fn notice(result: PromptRunResult, session: Session) -> SessionNotice {
        super::prompt_result::notice(result, session)
    }
    fn finished(notice: &SessionNotice) -> bool {
        matches!(notice, SessionNotice::Finished(_))
    }
    async fn worktree_result(
        success: bool,
        worktree: Option<Self::Worktree>,
        prompt: Option<&str>,
    ) {
        handle_worktree_result(success, worktree, prompt).await;
    }
}
