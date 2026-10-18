//! Root-scoped live-agent tree listing and target resolution.
//!
//! Implement [Entry] and [Host] for the application's live registry snapshots.
//! [list] renders sorted scoped paths and statuses; [resolve] and [canonical]
//! preserve the same identity/path lookup rules for collaboration operations.
mod entry;
mod path;
mod prefix;
mod query;
mod status;
mod target;
pub use entry::{Entry, Host};
pub use query::{ListedAgent, list};
pub use target::{canonical, resolve};

mod hierarchy_depth;
mod hierarchy_entry;
mod hierarchy_node;
mod hierarchy_order;
pub use hierarchy_depth::{child_depth, validate_spawn};
pub use hierarchy_entry::HierarchyEntry;
pub use hierarchy_node::{AgentNode, MAX_AGENT_DEPTH};
pub use hierarchy_order::dfs_order;
