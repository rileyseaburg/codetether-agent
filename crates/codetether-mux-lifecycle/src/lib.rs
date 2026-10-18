//! Mux daemon startup, session joining and verified shutdown.
//!
//! [start_session] starts or joins a workspace server. [start_managed_session]
//! also launches a semantic TUI. [kill] and [kill_all] close sessions/servers,
//! retaining the original process-identity checks before forced termination.
pub mod backoff;
pub mod kill;
pub mod kill_all;
mod kill_server;
mod lifecycle;
mod lifecycle_launch;
mod shutdown;
#[cfg(test)]
mod shutdown_tests;
pub mod spawn;
mod start;
pub mod start_join;
pub mod startup;
mod terminate;
mod terminate_identity;

pub use codetether_mux_control::{MuxSessionSummary, list_sessions};
pub use lifecycle::start_managed_session;
pub use start::{start_session, start_target};

mod lifecycle_wait;
