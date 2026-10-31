//! Autonomous agent tool filtering and read-only task classification.
//!
//! [restrict_registry] removes interactive/mutating tools through a native
//! [Registry]. [definitions] applies the same advertised-tool policy.
mod access;
mod definitions;
pub use access::TaskAccess;
mod registry;
pub use definitions::{definitions, is_read_only_task};
pub use registry::{Registry, restrict_registry};
#[cfg(test)]
mod refactoring_tests;
#[cfg(test)]
mod shared_prompt_tests;
#[cfg(test)]
mod shared_workspace_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod worktree_tests;
