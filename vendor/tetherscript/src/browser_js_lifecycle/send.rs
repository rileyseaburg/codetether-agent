//! Shared window event sender for navigation lifecycle events.

use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use super::super::*;

pub(super) fn dispatch_with_this(
    event_type: &str,
    this_value: JsValue,
    attrs: HashMap<String, JsValue>,
) -> Result<(), String> {
    let event = normalize_event(
        JsValue::Object(Rc::new(RefCell::new(attrs))),
        event_type,
        this_value.clone(),
        this_value.clone(),
    );
    let (listeners, handler) = super::listeners(event_type);
    for listener in listeners {
        call_dom_listener(listener, this_value.clone(), event.clone())?;
    }
    if let Some(handler) = handler {
        call_dom_listener(handler, this_value, event)?;
    }
    Ok(())
}
