//! Reset-policy context derivation over read-only session inputs.
//!
//! [derive_reset] delegates preparation/finalization to shared context policy,
//! preserves the active task tail, and summarizes older messages. [Source]
//! exposes native subcall settings while reusing the legacy source contract.
mod reset;
mod reset_finish;
mod reset_rebuild;
mod reset_summary;
mod source;
pub use reset::derive_reset;
pub use source::Source;

mod prepare;
