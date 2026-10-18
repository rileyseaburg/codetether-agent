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
mod loop_run;
use codetether_tui_startup::peer;
mod provider;
use codetether_session_resume as session_outcome;
use codetether_tui_startup::secrets;
mod session_resolve;
mod session_resolve_host;
mod session_scan;
#[cfg(test)]
mod session_start_tests;
mod session_status;
mod startup;
use codetether_cli_startup::logging as tui_log;
pub use tui_log::open_tui_log;

pub use driver::{Cli, run};
