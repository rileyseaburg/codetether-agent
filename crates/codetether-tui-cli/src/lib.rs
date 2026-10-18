//! TUI CLI option precedence, peer defaults and native launch coordination.
//!
//! Implement [Host] for the application's TUI runtime and call [execute]. An
//! explicit TUI subcommand uses its own options; no subcommand preserves the
//! CLI's default peer-enabled launch behavior.
mod explicit;
mod host;
mod launch;
pub use host::Host;
pub use launch::execute;
