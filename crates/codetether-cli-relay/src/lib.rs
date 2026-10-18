//! Protocol-first CLI relay runtime.
//!
//! [run_protocol_first_relay] owns planning, correlated handoffs, refinement,
//! dynamic spawning, progress recording and final reporting. [Host] supplies
//! native provider loading and sink setup; [Session] supplies coding sessions.
/// Cached application providers and configured bus archival for relay runs.
pub mod application;
mod convergence;
mod execute;
mod host;
pub use execute::run_protocol_first_relay;
pub use host::{Host, Session};
mod accept_spawn;
mod assemble;
mod attach_spawn;
mod bootstrap;
mod drive;
mod finish;
mod initial;
mod profiles;
mod prompt;
mod publish_context;
mod receive;
mod record_convergence;
mod record_progress;
mod refine;
mod source;
mod spawn;
mod state;
mod team;
mod team_add;
mod team_build;
mod team_registration;
