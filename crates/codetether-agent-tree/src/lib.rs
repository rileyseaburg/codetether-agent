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
