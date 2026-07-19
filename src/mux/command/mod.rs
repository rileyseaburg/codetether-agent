//! CLI lifecycle operations for mux servers.

mod attach;
mod dispatch;
mod kill;
pub(in crate::mux) mod kill_all;
mod list;
mod new_session;
mod serve;
mod shutdown;
pub(in crate::mux) mod spawn;
pub(in crate::mux) mod startup;
mod terminate;
mod terminate_identity;

#[cfg(test)]
mod shutdown_tests;

pub(super) use dispatch::execute;
