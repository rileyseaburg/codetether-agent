//! Durable answer-review decisions and interactive feedback sequencing.
//!
//! Implement [Host] for review storage and UI effects, then call [handle].
//! Review decisions are recorded before input, status, or selection is reset.
mod host;
mod passthrough;
mod run;
pub use host::Host;
pub use run::handle;
