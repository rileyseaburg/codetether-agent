//! Reusable mux lifecycle controls for CLI and TUI front ends.

mod list;
mod live;
mod mutate;
mod start;
mod stop;
mod summary;

pub(crate) use list::list_sessions;
pub(crate) use live::subscribe_live_output;
pub(crate) use mutate::{close_window, create_window, select_window};
pub(in crate::mux) use start::start_record;
pub(crate) use start::start_session;
pub(crate) use stop::stop_session;
pub(crate) use summary::MuxSessionSummary;
