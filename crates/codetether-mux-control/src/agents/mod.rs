//! Mux discovery projected into agent-roster entries.
//!
//! [agent_sessions] lists sessions and formats identity/runtime fields.
//! [is_agent_route] matches the stable route ID rather than the display name.
mod list;
mod project;
mod route;
pub use list::agent_sessions;
pub use route::is_agent_route;
mod interact;
mod message;
mod target;
mod watch;
pub use interact::interact_agent;
pub use message::send_agent_message;
pub use watch::watch_agent;
