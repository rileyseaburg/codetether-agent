//! Performance entry conversion helpers.

use super::*;

#[derive(Clone)]
pub(super) struct PerformanceEntry {
    pub(super) name: String,
    pub(super) entry_type: String,
    pub(super) start_time: f64,
    pub(super) duration: f64,
}

impl PerformanceEntry {
    pub(super) fn mark(name: String, start_time: f64) -> Self {
        Self {
            name,
            entry_type: "mark".into(),
            start_time,
            duration: 0.0,
        }
    }

    pub(super) fn measure(name: String, start_time: f64, end_time: f64) -> Self {
        Self {
            name,
            entry_type: "measure".into(),
            start_time,
            duration: (end_time - start_time).max(0.0),
        }
    }

    pub(super) fn to_js(&self) -> JsValue {
        entry_json::object(self.clone())
    }
}

pub(super) fn array(entries: Vec<PerformanceEntry>) -> JsValue {
    JsValue::Array(Rc::new(RefCell::new(
        entries.into_iter().map(|entry| entry.to_js()).collect(),
    )))
}
