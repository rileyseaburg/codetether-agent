//! TetherScript values exported from a browser trace.

use std::cell::RefCell;
use std::rc::Rc;

use crate::value::Value;

use super::export::{now_ms, trace_events};
use super::BrowserTrace;

impl BrowserTrace {
    pub(crate) fn value(&self) -> Value {
        super::super::value::map_value(vec![
            ("actions", trace_events(&self.actions)),
            ("observations", trace_events(&self.observations)),
            ("console_events", list(self.console_events.clone())),
            ("network_events", list(self.network_events.clone())),
            ("screenshots", trace_events(&self.screenshots)),
            ("dom_snapshots", list(self.dom_snapshots.clone())),
            ("storage_changes", trace_events(&self.storage_changes)),
            ("errors", trace_events(&self.errors)),
            (
                "timestamps",
                super::super::value::map_value(vec![("exported_at_ms", Value::Int(now_ms()))]),
            ),
        ])
    }

    pub(crate) fn har(&self) -> Value {
        super::super::value::map_value(vec![("log", {
            super::super::value::map_value(vec![
                ("version", super::super::value::str_value("1.2")),
                ("entries", list(self.network_events.clone())),
            ])
        })])
    }
}

fn list(values: Vec<Value>) -> Value {
    Value::List(Rc::new(RefCell::new(values)))
}
