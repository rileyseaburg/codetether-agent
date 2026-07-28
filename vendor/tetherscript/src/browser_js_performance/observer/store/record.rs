//! PerformanceObserver record routing.

use super::*;

pub(super) fn record(entry: PerformanceEntry) -> Vec<Delivery> {
    with_mut(|store| {
        store
            .observers
            .iter_mut()
            .filter_map(|(id, observer)| record_one(*id, observer, &entry))
            .collect()
    })
}

fn record_one(id: u64, observer: &mut ObserverState, entry: &PerformanceEntry) -> Option<Delivery> {
    if !observer.types.iter().any(|kind| kind == &entry.entry_type) {
        return None;
    }
    observer.records.push(entry.clone());
    Some(Delivery {
        id,
        callback: observer.callback.clone(),
        object: observer.object.clone(),
    })
}
