//! Managed-agent focus, removal and listing command behavior.
//!
//! Implement [Host] using the application's session registry and UI state, then
//! dispatch [handle], [kill] or [list]. [roster] formats legacy and managed agent
//! rows; the host owns actual registry operations and presentation.
mod focus;
mod host;
mod listing;
mod remove;
pub mod roster;
mod swarm;
pub use focus::handle;
pub use host::{Host, Listing, Registry, State};
pub use listing::list;
pub use remove::kill;
pub use swarm::focus as focus_swarm;

mod chat_host;
pub use chat_host::ChatHost;
pub mod chat;
pub mod presentation;
pub mod request;
