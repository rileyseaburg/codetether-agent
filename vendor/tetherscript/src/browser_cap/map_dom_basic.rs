//! Basic DOM action mappings.

use crate::value::Value;

use super::super::call::BrowserCall;

pub(crate) fn selector_action(action: &str, args: &[Value]) -> Result<BrowserCall, String> {
    one(action, args, "selector")
}

pub(crate) fn pair(
    action: &str,
    args: &[Value],
    a: &'static str,
    b: &'static str,
) -> Result<BrowserCall, String> {
    Ok(call(
        action,
        vec![
            (
                a,
                super::super::value::str_value(super::super::args::expect_str(action, args, 0)?),
            ),
            (
                b,
                super::super::value::str_value(super::super::args::expect_str(action, args, 1)?),
            ),
        ],
    ))
}

pub(crate) fn one(action: &str, args: &[Value], key: &'static str) -> Result<BrowserCall, String> {
    Ok(call(
        action,
        vec![(
            key,
            super::super::value::str_value(super::super::args::expect_str(action, args, 0)?),
        )],
    ))
}

pub(crate) fn call(action: &str, mut entries: Vec<(&str, Value)>) -> BrowserCall {
    entries.insert(0, ("action", super::super::value::str_value(action)));
    BrowserCall::new(
        action,
        super::super::actions::scope_for_action(action).unwrap(),
        super::super::value::map_value(entries),
    )
}
