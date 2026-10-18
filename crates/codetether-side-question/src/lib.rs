//! Ephemeral tool-free side questions over read-only conversation context.
//!
//! Implement [`Registry`] and [`View`], then call [`run_ask`] with existing
//! conversation messages. This API has no session mutation or persistence hook.
mod build;
mod execute;
mod extract;
mod host;
mod run;
pub use host::{Registry, View};
pub use run::run_ask;

/// Interactive ask argument and usage handling.
pub mod command;
