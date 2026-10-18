//! Read-only local-child and discovered-peer views for agent UIs.
//!
//! [Host] supplies existing session snapshots. [list] and [find] project local
//! and observed peer identities; [live_trace] and [transcript] preserve the same
//! owner-scoped fallback. [legacy_list] renders the agent tool's JSON roster.
mod host;
mod listing;
mod live;
mod origin;
mod origin_methods;
mod snapshot;
mod snapshots;
mod transcript;
pub use host::Host;
pub use listing::legacy_list;
pub use live::for_parent as live_trace;
pub use origin::AgentOrigin;
pub use snapshot::AgentSnapshot;
pub use snapshots::{all as list, find};
pub use transcript::for_parent as transcript;
mod local;
mod remote;
