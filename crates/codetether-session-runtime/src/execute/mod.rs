//! Cancellation-aware execution of owned prompt requests.
//! [run] creates optional worktree state, catches prompt panics, applies
//! worktree outcomes and returns an owned notice after clearing active control.
//! [Host] supplies native prompt, worktree and notice operations.
mod frontend_binding;
mod frontend_prompt;
mod guard;
mod host;
mod run;
pub use host::Host;
pub use run::run;

/// Drop-safe active-ownership cleanup for prompt execution.
pub use guard::Guard as CompletionGuard;
