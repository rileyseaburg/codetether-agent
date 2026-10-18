//! Scheduled failover retry and temporary-model restoration lifecycle.
//!
//! [execute] consumes ready retries, applies model changes and submits the exact
//! pending prompt. [Host] retains native persistence and runtime effects, while
//! [State] supplies retry and UI bookkeeping fields.
mod binding;
mod cooldown;
mod host;
mod restore;
mod run;
mod state;
pub use host::Host;
pub use run::execute;
pub use state::State;
