//! Refactor line-budget and displacement checks with optional native hooks.
//!
//! [evaluate] compares source files to Git HEAD, applies configured budgets,
//! and returns a continuation [Report]. [Hook] supplies the optional native
//! extension; serializable [GuardReport] metrics preserve its input contract.
mod budget;
mod config;
mod config_defaults;
mod config_file;
mod constructors;
mod displacement;
mod displacement_similar;
mod evaluate;
mod git;
mod hook;
mod lines;
mod plugin_decision;
mod render;
mod report;
mod rules;
mod scan;
mod similarity;
mod types;
mod wrapper;
pub use evaluate::evaluate;
pub use hook::Hook;
pub use plugin_decision::read as read_decision;
pub use report::Report;
pub use types::{FileStatus, GuardFile, GuardReport, GuardViolation};

/// Optional repository plugin invocation and decision handling.
pub mod plugin;
#[cfg(test)]
mod tests;
