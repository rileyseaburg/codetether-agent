//! Configuration loading and native context/sandbox hooks for invocation checks.
//!
//! [evaluate] selects workspace configuration from arguments; [for_workspace]
//! uses an explicit workspace. Both call [with_config] with native [Host] hooks.
mod configured;
mod host;
mod load;
pub use configured::with_config;
pub use host::Host;
pub use load::{evaluate, for_workspace};
