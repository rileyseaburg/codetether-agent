//! Assemble the final worker result after session execution.
use super::PolicySessionResult;
use crate::session_protocol::SessionStepsOutcome;
pub(super) fn assemble(text: SessionStepsOutcome, session_id: &str) -> PolicySessionResult {
    PolicySessionResult {
        result: codetether_session_events::SessionResult {
            text: text.text,
            session_id: session_id.to_owned(),
        },
        budget_exhausted: text.budget_exhausted,
        max_steps: text.max_steps,
    }
}
