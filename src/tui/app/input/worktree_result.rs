//! Post-prompt worktree cleanup and result handling.
//!
//! Runs the prompt against the provider, then delegates
//! merge/PR logic to [`super::merge`] and cleans up.

use std::path::PathBuf;
use std::sync::Arc;

use tokio::sync::mpsc;

use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};

/// Run the prompt against the provider and restore the
/// original directory on completion.
pub(crate) async fn run_prompt(
    session: &mut Session,
    prompt: &str,
    images: Vec<ImageAttachment>,
    event_tx: mpsc::Sender<SessionEvent>,
    registry: Arc<ProviderRegistry>,
    original_dir: Option<PathBuf>,
) -> anyhow::Result<()> {
    codetether_question::runtime::scope(
        session.id.clone(),
        session.prompt_with_events_and_images(prompt, images, event_tx, registry),
    )
    .await?;
    session.metadata.directory = original_dir;
    Ok(())
}
pub(crate) use codetether_github_pr::worktree::handle_worktree_result;
