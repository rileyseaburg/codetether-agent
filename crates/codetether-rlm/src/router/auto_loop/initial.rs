//! Initial mutable loop outcome.

use crate::router::types::LoopOutcome;
pub(super) fn initial() -> LoopOutcome {
    LoopOutcome {
        final_answer: None,
        iterations: 0,
        subcalls: 0,
        aborted: false,
        last_error: None,
    }
}
