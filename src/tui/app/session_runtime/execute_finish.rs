//! Session-policy-aware publication after one prompt finishes.

use crate::session::Session;
use crate::tui::app::input::worktree::WorktreeState;
use crate::tui::app::input::worktree_result::handle_worktree_result;

use super::super::{SessionNotice, prompt_result::PromptRunResult};

pub(super) async fn notice(
    result: PromptRunResult,
    session: Session,
    worktree: Option<WorktreeState>,
    prompt: &str,
) -> SessionNotice {
    let network_allowed = session.metadata.allow_network;
    let notice = super::super::prompt_result::notice(result, session);
    handle_worktree_result(
        matches!(notice, SessionNotice::Finished(_)),
        worktree,
        Some(prompt),
        network_allowed,
    )
    .await;
    notice
}
