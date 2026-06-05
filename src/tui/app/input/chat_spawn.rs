//! Provider task submission with optional worktree isolation.
//!
//! Creates an isolated git worktree when enabled, then moves
//! the checked-out session into the TUI session runtime.
//!
//! This module is responsible for handing an idle TUI session to the
//! asynchronous session runtime. It records the current UI execution settings
//! on the session metadata, prepares the prompt request, and restores the
//! session if the runtime cannot accept the work.
//!
//! The provider call itself is not performed here. Model execution, worktree
//! creation, event streaming, and session persistence are handled by the
//! runtime after it receives the [`PromptRequest`].

use std::path::Path;
use std::sync::Arc;

use crate::provider::ProviderRegistry;
use crate::session::ImageAttachment;
use crate::tui::app::session_runtime::{PromptRequest, SessionSlot, TuiSessionHandle};
use crate::tui::app::state::App;

/// Spawn the provider task with optional worktree isolation.
///
/// Sets up the worktree (if `use_worktree` is enabled),
/// clones the session, and spawns a background task.
///
/// The active session is taken from `slot` and submitted to `runtime` as a
/// [`PromptRequest`]. While the runtime owns the session, the TUI treats the
/// slot as busy. If submission fails, the session is restored immediately so
/// the user can retry without losing conversation state.
///
/// # Parameters
///
/// * `app` - TUI application state. The function reads `auto_apply_edits` and
///   `use_worktree` from `app.state`, and writes a status message when the
///   session cannot be submitted.
/// * `cwd` - Current working directory used as the worktree root when worktree
///   isolation is enabled.
/// * `slot` - Holder for the active session. The session must be idle and
///   available to be moved into the runtime.
/// * `registry` - Shared provider registry used by the runtime to resolve the
///   selected model provider.
/// * `prompt` - Prompt text entered by the user.
/// * `pending_images` - Image attachments to send with the prompt.
/// * `runtime` - Session runtime handle that accepts prompt requests for
///   background execution.
///
/// # Side Effects
///
/// Updates session metadata, may remove the session from `slot`, submits work
/// to the runtime, and updates `app.state.status` when the session is already
/// running or the runtime is unavailable.
///
/// # Errors
///
/// This function does not return an error. Runtime submission failure is handled
/// by restoring the session into `slot` and setting a user-visible status
/// message.
///
/// # Preconditions
///
/// `slot` should contain an idle session. If the slot cannot be borrowed, the
/// function assumes a prompt is already running and returns without submitting
/// new work.
pub(super) async fn spawn_provider_task(
    app: &mut App,
    cwd: &Path,
    slot: &mut SessionSlot,
    registry: &Arc<ProviderRegistry>,
    prompt: &str,
    pending_images: Vec<ImageAttachment>,
    runtime: &TuiSessionHandle,
) {
    let Some(original_dir) = slot.borrow().map(|s| s.metadata.directory.clone()) else {
        app.state.status = "Session is already running".to_string();
        return;
    };
    let use_worktree = app.state.use_worktree;
    if let Some(session) = slot.borrow_mut() {
        session.metadata.auto_apply_edits = app.state.auto_apply_edits;
        session.metadata.use_worktree = use_worktree;
    }
    let worktree_root = use_worktree.then(|| cwd.to_path_buf());
    let Some(session) = slot.take_for_prompt() else {
        return;
    };
    let request = PromptRequest::new(
        session,
        prompt.to_string(),
        pending_images,
        Arc::clone(registry),
        original_dir,
        worktree_root,
    );
    if let Err(request) = runtime.submit(request).await {
        slot.restore(request.session);
        app.state.status = "Session runtime is unavailable".to_string();
    }
}
