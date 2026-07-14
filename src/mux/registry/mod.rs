//! On-disk discovery records for named mux servers.

mod entry;
mod io;
mod path;
mod permissions;
mod scan;

pub(super) use entry::MuxRecord;
pub(super) use io::{load, remove, store};
pub(super) use path::validate_name;
pub(super) use scan::list;
