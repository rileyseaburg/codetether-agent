//! PerformanceObserver entry-list objects.

use super::*;

pub(super) fn object(records: Vec<PerformanceEntry>) -> JsValue {
    let records = Rc::new(RefCell::new(records));
    let mut map = HashMap::new();
    let all_records = records.clone();
    map.insert(
        "getEntries".into(),
        native(
            "PerformanceObserverEntryList.getEntries",
            Some(0),
            move |_| Ok(entry::array(all_records.borrow().clone())),
        ),
    );
    let type_records = records.clone();
    map.insert(
        "getEntriesByType".into(),
        native(
            "PerformanceObserverEntryList.getEntriesByType",
            Some(1),
            move |args| {
                Ok(entry::array(by_type(
                    &type_records.borrow(),
                    &args[0].display(),
                )))
            },
        ),
    );
    JsValue::Object(Rc::new(RefCell::new(map)))
}

fn by_type(records: &[PerformanceEntry], kind: &str) -> Vec<PerformanceEntry> {
    records
        .iter()
        .filter(|entry| entry.entry_type == kind)
        .cloned()
        .collect()
}
