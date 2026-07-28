//! Trace recording operations for browser capability calls.

use std::rc::Rc;

use crate::value::Value;

use super::{BrowserTrace, TraceEvent};

impl BrowserTrace {
    pub(crate) fn action(&mut self, method: &str, data: Value) {
        self.actions.push(TraceEvent::new("action", method, data));
    }

    pub(crate) fn observe(&mut self, method: &str, value: &Value) {
        self.observations
            .push(TraceEvent::new("observation", method, value.clone()));
        match method {
            "console_logs" | "console_errors" | "unhandled_rejections" | "runtime_exceptions" => {
                self.console_events.push(value.clone())
            }
            "network_log" | "failed_requests" | "request" | "response" => {
                self.network_events.push(value.clone())
            }
            "screenshot" | "screenshot_element" => {
                self.screenshots
                    .push(TraceEvent::new("screenshot", method, value.clone()))
            }
            "dom_snapshot" | "page_snapshot" | "snapshot" => self.dom_snapshots.push(value.clone()),
            _ => {}
        }
    }

    pub(crate) fn error(&mut self, method: &str, message: String) {
        self.errors.push(TraceEvent::new(
            "error",
            method,
            Value::Str(Rc::new(message)),
        ));
    }
}
