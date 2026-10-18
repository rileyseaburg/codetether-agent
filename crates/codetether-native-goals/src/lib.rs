//! Native session goal journals, user controls and continuation runtime.
//!
//! [`TaskLog`] resolves durable session journal paths. [`control`] applies
//! explicit user goal edits and [`runtime`] provides accounting, answer-review
//! holds, governance prompts and continuation against the same journals.
mod tasks;
pub use tasks::*;
/// Goal execution operations bound to durable native session journals.
pub mod runtime;
