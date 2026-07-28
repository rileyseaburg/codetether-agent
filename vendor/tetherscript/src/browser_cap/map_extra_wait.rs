//! Wait-related argument mappings for browserctl.

use crate::value::Value;

use crate::browser_cap::call::BrowserCall;

pub(crate) fn selector(args: &[Value]) -> Result<BrowserCall, String> {
    let selector = crate::browser_cap::args::expect_str("wait_for_selector", args, 0)?;
    let mut entries = vec![
        ("selector", crate::browser_cap::value::str_value(selector)),
        ("state", crate::browser_cap::value::str_value("visible")),
    ];
    push_timeout(&mut entries, args, 1)?;
    Ok(call("wait", entries))
}

pub(crate) fn text(args: &[Value]) -> Result<BrowserCall, String> {
    let text = crate::browser_cap::args::expect_str("wait_for_text", args, 0)?;
    let mut entries = vec![("text", crate::browser_cap::value::str_value(text))];
    push_timeout(&mut entries, args, 1)?;
    Ok(call("wait", entries))
}

pub(crate) fn idle(args: &[Value]) -> Result<BrowserCall, String> {
    crate::browser_cap::args::no_args("wait_for_network_idle", args)?;
    Err(
        "browser.wait_for_network_idle: browserctl backend does not support network idle waits"
            .into(),
    )
}

fn push_timeout(
    entries: &mut Vec<(&'static str, Value)>,
    args: &[Value],
    index: usize,
) -> Result<(), String> {
    entries.push((
        "timeout_ms",
        Value::Int(crate::browser_cap::args::optional_int(args, index, 30_000)?),
    ));
    Ok(())
}

fn call(action: &str, mut entries: Vec<(&str, Value)>) -> BrowserCall {
    entries.insert(0, ("action", crate::browser_cap::value::str_value(action)));
    BrowserCall::new(
        action,
        crate::browser_cap::actions::scope_for_action(action).unwrap(),
        crate::browser_cap::value::map_value(entries),
    )
}
