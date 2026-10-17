//! Special task routing with explicit application effects.
//!
//! [dispatch_special_task] handles clone and forage routing, policy gates,
//! and result mapping through [SpecialTaskHost].
mod host;
mod run;
pub use host::SpecialTaskHost;
pub use run::dispatch_special_task;
