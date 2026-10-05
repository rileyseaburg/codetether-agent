//! Reusable mux lifecycle controls for CLI and TUI front ends.

mod lifecycle;
mod lifecycle_launch;
mod list;
mod live;
mod mutate;
mod runtime;
mod start;
pub(in crate::mux) mod start_join;
mod stop;
mod summary;
mod summary_build;
mod summary_window;

pub(crate) use lifecycle::start_managed_session;
pub(crate) use list::list_sessions;
pub(crate) use live::subscribe_live_output;
pub(crate) use mutate::{close_window, create_window, select_window};
pub(crate) use runtime::report_runtime;
pub(crate) use start::start_session;
pub(in crate::mux) use start::start_target;
pub(crate) use stop::stop_session;
pub(crate) use summary::MuxSessionSummary;
