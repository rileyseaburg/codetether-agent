//! TUI startup entry point.
//!
//! This module keeps the public `run` facade stable while splitting startup
//! responsibilities into small private modules.

mod bus;
mod channels;
mod config;
mod driver;
mod full_auto;
mod hydrate;
mod hydrate_initial;
mod loop_run;
mod network_env;
mod peer;
mod policy_notice;
mod project;
mod provider;
mod secrets;
mod session_banner;
mod session_outcome;
mod session_resolve;
mod session_resolve_helpers;
mod session_scan;
#[cfg(test)]
mod session_start_tests;
mod session_status;
mod startup;
mod terminal;
mod worker_attach;
mod workspace;

pub use driver::run;
