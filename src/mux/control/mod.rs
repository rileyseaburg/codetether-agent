//! Reusable mux lifecycle controls for CLI and TUI front ends.

pub(in crate::mux) mod start_join;

pub(crate) use codetether_mux_control::{
    MuxSessionSummary, close_window, create_window, list_sessions, report_runtime, select_window,
    stop_session, subscribe_live_output,
};

pub(in crate::mux) use codetether_mux_lifecycle::start_target;
pub(crate) use codetether_mux_lifecycle::{start_managed_session, start_session};
