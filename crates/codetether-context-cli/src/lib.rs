//! CLI commands for resetting context and browsing durable session history.
//!
//! [execute] dispatches [ContextArgs] using a session implementing [ResetSession]
//! and [codetether_context_browse::BrowseSource]. The host owns persistence;
//! this crate owns command options, marker insertion order and presentation.
mod args;
mod browse;
mod browse_args;
mod execute;
mod reset;
mod reset_args;
mod session;
pub use args::{ContextArgs, ContextCommand};
pub use browse_args::{ContextBrowseArgs, ContextBrowseCommand, ContextShowTurnArgs};
pub use execute::execute;
pub use reset_args::ContextResetArgs;
pub use session::ResetSession;
