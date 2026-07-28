//! Window-level navigation event dispatch wrappers.

use crate::js::JsValue;

pub(super) fn dispatch_plain_with_this(
    event_type: &str,
    this_value: JsValue,
) -> Result<(), String> {
    super::send::dispatch_with_this(event_type, this_value, Default::default())
}

pub(super) fn dispatch_hashchange_event(old_url: &str, new_url: &str) -> Result<(), String> {
    dispatch_hashchange_with_this(JsValue::Undefined, old_url, new_url)
}

pub(super) fn dispatch_hashchange_with_this(
    this_value: JsValue,
    old_url: &str,
    new_url: &str,
) -> Result<(), String> {
    super::send::dispatch_with_this(
        "hashchange",
        this_value,
        super::attrs::urls(old_url, new_url),
    )
}

pub(super) fn dispatch_popstate_event(state: JsValue) -> Result<(), String> {
    dispatch_popstate_with_this(JsValue::Undefined, state)
}

pub(super) fn dispatch_popstate_with_this(
    this_value: JsValue,
    state: JsValue,
) -> Result<(), String> {
    super::send::dispatch_with_this("popstate", this_value, super::attrs::state(state))
}
