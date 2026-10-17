//! Application prompt composition and shared message extraction.

use crate::session::Session;
pub(super) use codetether_agent_core::execution::messages::{collect_tool_calls, response_text};

/// Compose the system prompt by appending the session's goal-governance
/// block (if any) to the agent's base persona prompt.
///
/// Reads `<sessions_dir>/<session-id>.tasks.jsonl` synchronously; the
/// file is small (a few KB at most) and this is used from the sync
/// `complete_with_context` prompt-composition path without a tokio handle.
pub(super) fn compose_system_prompt(base: &str, session: &Session) -> String {
    let prompt = crate::session::tasks::runtime::compose(base, &session.id);
    crate::a2a::peer_prompt::append(prompt)
}
