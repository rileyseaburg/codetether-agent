//! Approved task execution through PRD generation and the Ralph loop.
//!
//! Implement [Host] for an application runtime and call [execute_go_ralph].
//! [GoRalphResult] and [StoryResult] describe the terminal state, while
//! [format_go_ralph_result] renders it for callers.

mod apply_outcome;
mod audit_completed;
mod audit_generated;
mod config;
mod evidence;
mod execute;
mod finish;
mod format;
mod host;
mod metrics;
mod next_steps;
mod outcomes;
mod persist;
mod result;
mod start;
mod stories;
mod story;
mod terminal;
pub use execute::execute_go_ralph;
pub use format::format_go_ralph_result;
pub use host::Host;
pub use result::GoRalphResult;
pub use story::StoryResult;
/// Pipeline binding to an application provider registry and Ralph runtime.
pub mod application;
/// Ralph runtime construction and service attachment.
pub mod runtime;
#[cfg(test)]
mod tests;
