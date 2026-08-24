//! TetherScript-backed plugin tool.

pub mod convert;
mod errors;
mod execute;
mod execute_policy;
mod input;
mod join;
mod load;
mod partner;
mod result;
mod runner;
mod schema;
mod task;
mod tool;

#[cfg(test)]
#[path = "test_support.rs"]
mod test_support;
#[cfg(test)]
mod tests;

#[cfg(test)]
pub(crate) use test_support::require_sandbox;

pub use tool::TetherScriptPluginTool;

pub(crate) use partner::register;
