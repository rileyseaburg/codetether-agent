//! Trace export helpers for browser capability calls.

use std::cell::RefCell;
use std::rc::Rc;
use std::time::{SystemTime, UNIX_EPOCH};

use crate::value::Value;

use super::{BrowserTrace, TraceEvent};

pub(crate) fn now_ms() -> i64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as i64
}

pub(crate) fn trace_events(events: &[TraceEvent]) -> Value {
    Value::List(Rc::new(RefCell::new(
        events.iter().map(|e| e.value()).collect(),
    )))
}

impl BrowserTrace {
    pub(crate) fn summary(&self) -> Value {
        super::super::value::map_value(vec![
            ("action_count", Value::Int(self.actions.len() as i64)),
            (
                "observation_count",
                Value::Int(self.observations.len() as i64),
            ),
            (
                "console_event_count",
                Value::Int(self.console_events.len() as i64),
            ),
            (
                "network_event_count",
                Value::Int(self.network_events.len() as i64),
            ),
            ("error_count", Value::Int(self.errors.len() as i64)),
        ])
    }
}
