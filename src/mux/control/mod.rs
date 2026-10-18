//! Reusable mux lifecycle controls for CLI and TUI front ends.

mod lifecycle;
mod lifecycle_launch;
mod start;
pub(in crate::mux) mod start_join;

pub(crate) use lifecycle::start_managed_session;
pub(crate) use start::start_session;
pub(in crate::mux) use start::start_target;

pub(crate) use codetether_mux_control::{
    MuxSessionSummary, close_window, create_window, list_sessions, report_runtime, select_window,
    stop_session, subscribe_live_output,
};
