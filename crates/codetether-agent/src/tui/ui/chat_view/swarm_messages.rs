//! Application projection for the selected worker transcript.
use crate::tui::app::state::AppState;
use crate::tui::chat::message::ChatMessage;
pub(super) fn active(state: &AppState) -> Option<Vec<ChatMessage>> {
    codetether_swarm_view::active_messages(&state.swarm, state.agent_tree.active.as_deref())
}
