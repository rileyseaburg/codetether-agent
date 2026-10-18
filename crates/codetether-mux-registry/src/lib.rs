//! Owner-private discovery records for workspace-scoped mux servers.
//!
//! [store] publishes one server record atomically; [load] resolves a session
//! to its server. [for_workspace] and [validate_name] preserve stable registry
//! keys and safe names. Legacy record and snapshot formats remain readable.
mod entry;
mod io;
mod key;
mod path;
mod permissions;
mod scan;
mod target;
pub use entry::MuxRecord;
pub use io::{load_key, remove_key, store};
pub use key::for_workspace;
pub use path::validate_name;
pub use scan::{find_session, find_workspace, list, load};
pub use target::SessionTarget;
