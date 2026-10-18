//! Compatibility import; turn state is owned by the shared session runtime.

pub(super) use codetether_session_runtime::ActiveTurn;

#[cfg(test)]
#[path = "active_turn_cancel_tests.rs"]
mod tests;
