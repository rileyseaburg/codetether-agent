//! Provider task spawning with optional worktree isolation.
//!
//! Creates an isolated git worktree (when enabled), clones
//! the session, and spawns a Tokio task via
//! [`super::chat_spawn_task::run_spawned_task`].

use std::path::Path;
use std::sync::Arc;

use tokio::sync::mpsc;

use super::chat_spawn_task::run_spawned_task;
use super::worktree::create_worktree;
use crate::provider::ProviderRegistry;
use crate::session::{ImageAttachment, Session, SessionEvent};
use crate::tui::app::state::App;

/// Spawn the provider task with optional worktree isolation.
///
/// Sets up the worktree (if `use_worktree` is enabled),
/// clones the session, and spawns a background task.
pub(super) async fn spawn_provider_task(
    app: &mut App,
    cwd: &Path,
    session: &mut Session,
    registry: &Arc<ProviderRegistry>,
    prompt: &str,
    pending_images: Vec<ImageAttachment>,
    event_tx: &mpsc::Sender<SessionEvent>,
    result_tx: &mpsc::Sender<anyhow::Result<Session>>,
) {
    session.metadata.auto_apply_edits = app.state.auto_apply_edits;
    let mut session_for_task = session.clone();
    let event_tx = event_tx.clone();
    let result_tx = result_tx.clone();
    let registry = Arc::clone(registry);
    let prompt = prompt.to_string();
    let use_worktree = app.state.use_worktree;

    let worktree_state = if use_worktree {
        create_worktree(app, cwd, &mut session_for_task).await
    } else {
        None
    };

    let original_dir = session.metadata.directory.clone();
    let prompt_for_pr = prompt.clone();

    // Register a cancel handle so a mid-turn user steering message can
    // interrupt the provider stream instead of merely queueing.
    let cancel = Arc::new(tokio::sync::Notify::new());
    app.state.current_turn_cancel = Some(Arc::clone(&cancel));

    tokio::spawn(run_spawned_task(
        session_for_task,
        prompt,
        pending_images,
        event_tx,
        registry,
        original_dir,
        result_tx,
        worktree_state,
        prompt_for_pr,
        cancel,
    ));
}
