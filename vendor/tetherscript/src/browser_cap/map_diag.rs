//! Diagnostic and framework method mapping.

use crate::value::Value;

use super::call::BrowserCall;

pub(crate) fn prepare(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    let mut entries = vec![("action", super::value::str_value("diagnose"))];
    if let Some(arg) = args.first() {
        entries.push(("query", arg.clone()));
    }
    Ok(BrowserCall::new(
        "diagnose",
        scope(method),
        super::value::map_value(entries),
    ))
}

fn scope(method: &str) -> &'static str {
    match method {
        "console_logs"
        | "console_errors"
        | "unhandled_rejections"
        | "runtime_exceptions"
        | "source_mapped_stack_traces" => "browser.inspect.console",
        _ => "browser.inspect.react",
    }
}
