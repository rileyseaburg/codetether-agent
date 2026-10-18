//! # Approval Review
//!
//! An independent, read-only reviewer agent for tool approvals. When a
//! session in `ask` mode proposes a file mutation, the TUI can spawn a
//! reviewer that explores the codebase with real tools, judges the change
//! against the session goal, and returns a structured [`ReviewVerdict`].
//!
//! In `advise` mode (the only mode today) the verdict is rendered beside the
//! approval preview; the human still decides. The reviewer never receives a
//! mutating tool and never touches the approval store.
//!
//! ```rust
//! use codetether_agent::review::{ReviewOutcome, parse};
//!
//! let verdict = parse(r#"Looks fine. {"outcome":"approve","reason":"Matches the goal.","findings":[]}"#);
//! assert_eq!(verdict.outcome, ReviewOutcome::Approve);
//! ```

codetether_native_review::declare_review!(
    crate::tool::registry_lifecycle::Native,
    crate::swarm::executor::LoopHost,
);

#[cfg(test)]
mod tests;
