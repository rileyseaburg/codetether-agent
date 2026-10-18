//! CLI lifecycle operations for mux servers.

mod attach;
mod dispatch;
mod kill;
pub(in crate::mux) mod kill_all;
mod list;
mod new_session;
mod resume;
mod serve;
pub(in crate::mux) mod spawn;
pub(in crate::mux) mod startup;

pub(super) use dispatch::execute;
