//! Shared helpers for navigation action envelopes.

use crate::value::Value;

use super::super::call::BrowserCall;

pub(crate) fn simple(action: &str, args: &[Value]) -> Result<BrowserCall, String> {
    super::super::args::no_args(action, args)?;
    Ok(call(action, Vec::new()))
}

pub(crate) fn call(action: &str, entries: Vec<(&str, Value)>) -> BrowserCall {
    BrowserCall::new(
        action,
        super::super::actions::scope_for_action(action).unwrap(),
        envelope(action, entries),
    )
}

fn envelope(action: &str, mut entries: Vec<(&str, Value)>) -> Value {
    entries.insert(0, ("action", super::super::value::str_value(action)));
    super::super::value::map_value(entries)
}
