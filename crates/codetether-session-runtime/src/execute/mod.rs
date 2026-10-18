//! Cancellation-aware execution of owned prompt requests.
//! [run] creates optional worktree state, catches prompt panics, applies
//! worktree outcomes and returns an owned notice after clearing active control.
//! [Host] supplies native prompt, worktree and notice operations.
mod guard;
mod host;
mod run;
pub use host::Host;
pub use run::run;
