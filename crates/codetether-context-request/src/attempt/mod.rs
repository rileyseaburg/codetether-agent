//! Construction of requests from a prepared prompt-loop attempt.
//!
//! [build] combines system instructions, derived history and optional context,
//! removes incompatible reasoning checkpoints and applies the request budget.
//! [Host] supplies the native preparation step and [Target] describes its model.
mod build;
mod host;
mod reasoning;
mod target;
pub use build::build;
pub use host::Host;
pub use target::Target;
