//! Map activity counts to an orphan-aware lifecycle state.

use crate::codetether_agent::state::SubAgentRunState;

pub(super) fn classify_counts(
    messages: usize,
    tools: usize,
    tokens: u64,
    has_heartbeat: bool,
) -> SubAgentRunState {
    if messages <= 1 && tools == 0 && tokens == 0 && !has_heartbeat {
        SubAgentRunState::Orphaned
    } else if tools > 0 || tokens > 0 || has_heartbeat {
        SubAgentRunState::Running
    } else {
        SubAgentRunState::Created
    }
}

pub(super) fn reason_for(state: SubAgentRunState) -> &'static str {
    match state {
        SubAgentRunState::Orphaned => {
            "child session was created but no worker heartbeat, tool use, token usage, or output was recorded"
        }
        SubAgentRunState::Running => "child session recorded worker activity",
        _ => "child session exists but has not reached a terminal state",
    }
}
