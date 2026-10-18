//! TUI startup entry point.
//!
//! This module keeps the public `run` facade stable while splitting startup
//! responsibilities into small private modules.

mod channels;
mod config;
mod driver;
mod host;
mod hydrate;
mod hydrate_host;
mod hydrate_initial;
mod initial_frame;
mod loop_run;
mod peer;
mod provider;
mod secrets;
mod session_outcome;
mod session_resolve;
mod session_resolve_host;
mod session_scan;
#[cfg(test)]
mod session_start_tests;
mod session_status;
mod startup;
mod tui_log;
pub use tui_log::open as open_tui_log;

pub use driver::run;
