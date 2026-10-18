//! Native command-line parsing and standalone command services.
//!
//! [Cli] and [Command] retain application help/version metadata. Standalone
//! auth, Vault, browser, model and search handlers use their owning service
//! crates. Session-running commands are bound by the embedding application.
mod cli;
pub use cli::*;

/// Native Sol planner and stateful coding-worker integration.
pub mod sol;
