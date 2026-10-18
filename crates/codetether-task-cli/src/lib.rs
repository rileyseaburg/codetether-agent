//! Remote task submission with workspace resolution and result reporting.
//!
//! [execute] consumes [TaskArgs], resolves credentials and workspace identity,
//! submits the task and prints the server response. Argument parsing remains
//! the caller's responsibility; this crate does not install a CLI command.
mod args;
mod credentials;
mod execute;
mod files;
mod matching;
mod output;
mod paths;
mod submit;
mod title;
mod types;
mod workspace;
pub use args::TaskArgs;
pub use execute::execute;
pub use types::TaskTriggerResponse;
#[cfg(test)]
mod tests;
