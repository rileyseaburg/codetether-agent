//! Observation and wait mappings for browserctl.

use crate::value::Value;

use super::call::BrowserCall;

#[path = "map_extra_wait.rs"]
mod wait;

pub(crate) fn prepare(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    match method {
        "snapshot" | "page_snapshot" | "dom_snapshot" => no_arg_action("snapshot", args),
        "text" | "html" => selector_optional(method, args),
        "eval" => one("eval", args, "expression"),
        "wait_for_selector" => wait::selector(args),
        "wait_for_text" => wait::text(args),
        "wait_for_network_idle" => wait::idle(args),
        _ => unreachable!(),
    }
}

pub(crate) fn screenshot(args: &[Value]) -> Result<BrowserCall, String> {
    let mut entries = Vec::new();
    if let Some(v) = args.first() {
        entries.push(("path", v.clone()));
    }
    Ok(call("screenshot", entries))
}

fn no_arg_action(action: &str, args: &[Value]) -> Result<BrowserCall, String> {
    super::args::no_args(action, args)?;
    Ok(call(action, Vec::new()))
}

fn selector_optional(action: &str, args: &[Value]) -> Result<BrowserCall, String> {
    let entries = match args.first() {
        Some(v) => vec![("selector", v.clone())],
        None => Vec::new(),
    };
    Ok(call(action, entries))
}

fn one(action: &str, args: &[Value], key: &'static str) -> Result<BrowserCall, String> {
    Ok(call(
        action,
        vec![(
            key,
            super::value::str_value(super::args::expect_str(action, args, 0)?),
        )],
    ))
}

pub(crate) fn call(action: &str, mut entries: Vec<(&str, Value)>) -> BrowserCall {
    entries.insert(0, ("action", super::value::str_value(action)));
    BrowserCall::new(
        action,
        super::actions::scope_for_action(action).unwrap(),
        super::value::map_value(entries),
    )
}
