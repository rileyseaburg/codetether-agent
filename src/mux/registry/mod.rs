//! On-disk discovery records for workspace-bound mux servers.
//!
//! One record per server process, keyed by workspace. Session names are
//! globally unique, so [`load`] resolves a name to the server hosting it.

mod entry;
mod io;
mod key;
mod path;
mod permissions;
mod scan;
mod target;

pub(super) use entry::MuxRecord;
pub(super) use io::{load_key, remove_key, store};
pub(super) use key::for_workspace;
pub(super) use path::validate_name;
pub(super) use scan::{find_session, find_workspace, list, load};
pub(super) use target::SessionTarget;
