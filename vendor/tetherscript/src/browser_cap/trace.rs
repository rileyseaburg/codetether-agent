//! Browser authority trace data structures.

use crate::value::Value;

#[path = "trace_export.rs"]
mod export;
#[path = "trace_ops.rs"]
mod ops;
#[path = "trace_view.rs"]
mod view;

#[derive(Clone, Debug, Default)]
pub(crate) struct BrowserTrace {
    pub(crate) actions: Vec<TraceEvent>,
    pub(crate) observations: Vec<TraceEvent>,
    pub(crate) console_events: Vec<Value>,
    pub(crate) network_events: Vec<Value>,
    pub(crate) screenshots: Vec<TraceEvent>,
    pub(crate) dom_snapshots: Vec<Value>,
    pub(crate) storage_changes: Vec<TraceEvent>,
    pub(crate) errors: Vec<TraceEvent>,
}

#[derive(Clone, Debug)]
pub(crate) struct TraceEvent {
    pub(crate) timestamp_ms: i64,
    pub(crate) kind: String,
    pub(crate) method: String,
    pub(crate) data: Value,
}

impl TraceEvent {
    pub(crate) fn new(kind: &str, method: &str, data: Value) -> Self {
        Self {
            timestamp_ms: export::now_ms(),
            kind: kind.into(),
            method: method.into(),
            data,
        }
    }

    pub(crate) fn value(&self) -> Value {
        super::value::map_value(vec![
            ("timestamp_ms", Value::Int(self.timestamp_ms)),
            ("kind", super::value::str_value(self.kind.clone())),
            ("method", super::value::str_value(self.method.clone())),
            ("data", self.data.clone()),
        ])
    }
}
