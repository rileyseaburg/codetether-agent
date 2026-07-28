//! User-timing timeline state facade.

use super::*;

#[path = "state/query.rs"]
mod query;
#[path = "state/store.rs"]
mod store;
#[path = "state/timing.rs"]
mod timing;

pub(super) fn reset() {
    store::reset();
}

pub(super) fn now() -> f64 {
    store::now()
}

pub(super) fn add_mark(name: String) -> PerformanceEntry {
    timing::add_mark(name)
}

pub(super) fn add_measure(
    name: String,
    start: Option<String>,
    end: Option<String>,
) -> Result<PerformanceEntry, String> {
    timing::add_measure(name, start, end)
}

pub(super) fn entries() -> Vec<PerformanceEntry> {
    query::entries()
}

pub(super) fn by_type(entry_type: &str) -> Vec<PerformanceEntry> {
    query::by_type(entry_type)
}

pub(super) fn by_name(name: &str, entry_type: Option<&str>) -> Vec<PerformanceEntry> {
    query::by_name(name, entry_type)
}

pub(super) fn clear(entry_type: &str, name: Option<&str>) {
    query::clear(entry_type, name);
}
