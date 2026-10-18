//! Reusable mux lifecycle controls for CLI and TUI front ends.

mod agent_read;
pub(in crate::mux) mod start_join;
pub(crate) use agent_read::read_agent_output;

pub(crate) use codetether_mux_control::{
    MuxSessionSummary, agent_sessions, close_window, create_window, interact_agent, is_agent_route,
    list_sessions, report_runtime, select_window, send_agent_message, stop_session,
    subscribe_live_output, watch_agent,
};

pub(in crate::mux) use codetether_mux_lifecycle::start_target;
pub(crate) use codetether_mux_lifecycle::{start_managed_session, start_session};
