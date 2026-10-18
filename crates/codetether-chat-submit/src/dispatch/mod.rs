//! Prepare accepted chat input for provider dispatch.
//!
//! [dispatch_prompt] applies the held-goal gate, resets prompt recovery state,
//! and chooses provider handoff or the missing-provider path. [Host] retains
//! native UI/session services; [State] supplies the disjoint mutable fields.
mod host;
mod run;
mod state;
pub use host::Host;
pub use run::dispatch_prompt;
pub use state::State;
