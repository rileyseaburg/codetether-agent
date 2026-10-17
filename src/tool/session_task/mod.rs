//! Application binding for session work-item and legacy goal actions.
#[cfg(test)]
mod dispatch;
#[cfg(test)]
mod handlers;
mod host;
#[cfg(test)]
mod schema;
/// Session task tool using the application journal path and goal verifier.
pub type SessionTaskTool = codetether_session_task::SessionTaskTool<host::ApplicationHost>;
