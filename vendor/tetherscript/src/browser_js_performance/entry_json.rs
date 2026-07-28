//! Performance entry JSON snapshots.

use super::*;

pub(super) fn object(entry: PerformanceEntry) -> JsValue {
    let mut map = fields(&entry);
    let snapshot = entry.clone();
    map.insert(
        "toJSON".into(),
        native("PerformanceEntry.toJSON", Some(0), move |_| {
            Ok(snapshot_value(&snapshot))
        }),
    );
    JsValue::Object(Rc::new(RefCell::new(map)))
}

fn snapshot_value(entry: &PerformanceEntry) -> JsValue {
    JsValue::Object(Rc::new(RefCell::new(fields(entry))))
}

fn fields(entry: &PerformanceEntry) -> HashMap<String, JsValue> {
    HashMap::from([
        ("name".into(), JsValue::String(entry.name.clone())),
        (
            "entryType".into(),
            JsValue::String(entry.entry_type.clone()),
        ),
        ("startTime".into(), JsValue::Number(entry.start_time)),
        ("duration".into(), JsValue::Number(entry.duration)),
    ])
}
