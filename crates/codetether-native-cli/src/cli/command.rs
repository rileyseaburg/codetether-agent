//! Shared command schema with native mux argument compatibility paths.
pub use codetether_cli_command::Command;
#[path = "mux_args.rs"]
pub mod mux_args;
