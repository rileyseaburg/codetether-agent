//! Per-turn suppression of tool requests the user explicitly declined.
//!
//! Create a fresh [DeclineGuard] for each user turn. Record approval denials
//! after tool execution, then query the guard before repeating an invocation.
//! Approval transport fields are ignored when comparing requests.
mod guard;
mod normalize;
pub use guard::DeclineGuard;
#[cfg(test)]
mod tests;
