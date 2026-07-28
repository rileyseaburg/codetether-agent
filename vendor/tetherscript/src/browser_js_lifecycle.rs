//! Navigation lifecycle helpers for the JavaScript host.

#[path = "browser_js_lifecycle/attrs.rs"]
mod attrs;
#[path = "browser_js_lifecycle/dispatch.rs"]
mod dispatch;
#[path = "browser_js_lifecycle/runtime.rs"]
mod runtime;
#[path = "browser_js_lifecycle/send.rs"]
mod send;

use super::*;

pub(super) fn dispatch_hashchange_event(old_url: &str, new_url: &str) -> Result<(), String> {
    dispatch::dispatch_hashchange_event(old_url, new_url)
}

pub(super) fn dispatch_popstate_event(state: crate::js::JsValue) -> Result<(), String> {
    dispatch::dispatch_popstate_event(state)
}

fn listeners(event_type: &str) -> (Vec<JsValue>, Option<JsValue>) {
    EVENT_REGISTRY.with(|registry| {
        registry
            .borrow()
            .get("window")
            .map(|entry| {
                (
                    entry
                        .listeners
                        .get(event_type)
                        .cloned()
                        .unwrap_or_default()
                        .into_iter()
                        .map(|listener| listener.callback)
                        .collect(),
                    entry.handlers.get(&format!("on{}", event_type)).cloned(),
                )
            })
            .unwrap_or_default()
    })
}

#[cfg(test)]
#[path = "browser_js_lifecycle/tests.rs"]
mod tests;
