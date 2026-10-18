//! Autonomous agent tool filtering and read-only task classification.
//!
//! [restrict_registry] removes interactive/mutating tools through a native
//! [Registry]. [definitions] applies the same advertised-tool policy.
mod definitions;
mod registry;
pub use definitions::{definitions, is_read_only_task};
pub use registry::{Registry, restrict_registry};
#[cfg(test)]
mod tests;
#[cfg(test)]
mod worktree_tests;
