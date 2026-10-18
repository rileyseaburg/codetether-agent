//! Frontend controls for discovering and interacting with running mux sessions.
//!
//! [list_sessions] returns [MuxSessionSummary] values. Window operations and
//! [stop_session] act through authenticated clients; [report_runtime] publishes
//! inherited TUI state and [subscribe_live_output] follows retained PTY output.
mod list;
mod live;
mod mutate;
mod runtime;
mod stop;
mod summary;
mod summary_build;
mod summary_window;

pub use list::list_sessions;
pub use live::{MuxLiveOutput, subscribe_live_output};
pub use mutate::{close_window, create_window, select_window};
pub use runtime::report_runtime;
pub use stop::stop_session;
pub use summary::{MuxSessionSummary, MuxWindowSummary};

mod summary_create;

mod runtime_reporter;
/// Change-driven reporting for an inherited mux owner.
pub use runtime_reporter::RuntimeReporter;

mod agents;
pub use agents::{agent_sessions, interact_agent, is_agent_route, send_agent_message, watch_agent};
