//! PerformanceObserver registry storage.

use super::*;

#[path = "store/lifecycle.rs"]
mod lifecycle;
#[path = "store/record.rs"]
mod record;

thread_local! {
    static OBSERVERS: RefCell<ObserverStore> = RefCell::new(ObserverStore::default());
}

#[derive(Clone)]
pub(super) struct Delivery {
    pub(super) id: u64,
    pub(super) callback: JsValue,
    pub(super) object: JsValue,
}

#[derive(Default)]
pub(super) struct ObserverStore {
    pub(super) next_id: u64,
    pub(super) observers: HashMap<u64, ObserverState>,
}

pub(super) struct ObserverState {
    pub(super) callback: JsValue,
    pub(super) object: JsValue,
    pub(super) types: Vec<String>,
    pub(super) records: Vec<PerformanceEntry>,
}

pub(super) fn with_mut<R>(f: impl FnOnce(&mut ObserverStore) -> R) -> R {
    OBSERVERS.with(|store| f(&mut store.borrow_mut()))
}

pub(super) fn reset() {
    lifecycle::reset();
}

pub(super) fn insert(callback: JsValue, object: JsValue) -> u64 {
    lifecycle::insert(callback, object)
}

pub(super) fn observe(id: u64, types: Vec<String>) {
    lifecycle::observe(id, types);
}

pub(super) fn disconnect(id: u64) {
    lifecycle::disconnect(id);
}

pub(super) fn take(id: u64) -> Vec<PerformanceEntry> {
    lifecycle::take(id)
}

pub(super) fn record(entry: PerformanceEntry) -> Vec<Delivery> {
    record::record(entry)
}
