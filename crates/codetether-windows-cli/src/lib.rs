//! Model-free Windows readiness diagnostics and desktop-worker startup.
//!
//! [`run`] dispatches parsed Windows commands. [`worker::run`] serves the
//! computer-use protocol without provider initialization or stdout logging.
mod readiness;
pub mod worker;
pub use codetether_cli_options::{WindowsArgs, WindowsCommand};
pub use readiness::run;

#[cfg(test)]
mod tests;
