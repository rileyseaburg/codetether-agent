//! Retry decisions and corrective messages for provider responses.
//!
//! [BuildContext] borrows the current turn's inputs. [crate::LoopState] methods
//! update retry counters while callers retain transcript insertion and effects.
mod build;
mod codesearch;
mod context;
mod exhaustion;
mod guidance;
mod native;
mod thinking;
pub use context::BuildContext;
pub use guidance::message as corrective_message;
pub use thinking::thinking_only_nudge;
