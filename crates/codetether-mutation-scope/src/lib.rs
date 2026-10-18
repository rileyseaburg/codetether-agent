//! Mutation target classification and bounded workspace lease scopes.
//!
//! [paths::mutation_paths] maps tool inputs to affected files or directories.
//! [scope::resolve] rebases targets to their shared repository; [scope::scoped_only]
//! removes whole-workspace claims using the same [workspace_claim] check as mux.
mod batch;
mod command_path;
pub mod patch;
pub mod paths;
pub mod scope;
pub mod shell;
mod structured;
mod workspace;
pub use workspace::workspace_claim;
#[cfg(test)]
mod command_path_tests;
#[cfg(test)]
mod patch_tests;
#[cfg(test)]
mod scope_tests;
#[cfg(test)]
mod tests;
#[cfg(test)]
mod tool_tests;

#[cfg(test)]
mod batch_tests;
