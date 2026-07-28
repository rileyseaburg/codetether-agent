//! PerformanceObserver lifecycle operations.

use super::*;

pub(super) fn reset() {
    with_mut(|store| *store = ObserverStore::default());
}

pub(super) fn insert(callback: JsValue, object: JsValue) -> u64 {
    with_mut(|store| {
        store.next_id += 1;
        let id = store.next_id;
        store.observers.insert(
            id,
            ObserverState {
                callback,
                object,
                types: Vec::new(),
                records: Vec::new(),
            },
        );
        id
    })
}

pub(super) fn observe(id: u64, types: Vec<String>) {
    with_mut(|store| {
        if let Some(observer) = store.observers.get_mut(&id) {
            observer.types = types;
            observer.records.clear();
        }
    });
}

pub(super) fn disconnect(id: u64) {
    with_mut(|store| {
        store.observers.remove(&id);
    });
}

pub(super) fn take(id: u64) -> Vec<PerformanceEntry> {
    with_mut(|store| {
        store
            .observers
            .get_mut(&id)
            .map(|observer| observer.records.drain(..).collect())
            .unwrap_or_default()
    })
}
