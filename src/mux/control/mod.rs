//! Reusable mux lifecycle controls for CLI and TUI front ends.

mod agent_interact;
mod agent_message;
mod agent_read;
mod agent_sessions;
mod agent_target;
mod agent_watch;
mod lifecycle;
mod lifecycle_launch;
mod lifecycle_restart;
mod list;
mod live;
mod mutate;
mod runtime;
mod start;
mod stop;
mod summary;

pub(crate) use agent_interact::interact_agent;
pub(crate) use agent_message::send_agent_message;
pub(crate) use agent_read::read_agent_output;
pub(crate) use agent_sessions::{agent_sessions, is_agent_route};
pub(crate) use agent_watch::watch_agent;
pub(crate) use lifecycle::{restart_session, start_managed_session};
pub(crate) use list::list_sessions;
pub(crate) use live::subscribe_live_output;
pub(crate) use mutate::{close_window, create_window, select_window};
pub(crate) use runtime::report_runtime;
pub(in crate::mux) use start::start_record;
pub(crate) use start::start_session;
pub(crate) use stop::stop_session;
pub(crate) use summary::MuxSessionSummary;
