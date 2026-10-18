//! Text formatting for the mux CLI listing.
use super::list_entry::ListedSession;

pub(super) fn print_item(item: &ListedSession) {
    let status = if item.reachable { "up" } else { "stale" };
    println!(
        "{}: {} windows (active {}) [{}] pid={} {} {}",
        item.name, item.windows, item.active, status, item.pid, item.address, item.workspace
    );
}
