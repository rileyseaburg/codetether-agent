//! Storage methods implemented through browserctl eval.

use crate::value::Value;

use super::call::BrowserCall;

#[path = "map_storage_js.rs"]
mod js;
#[path = "map_storage_write.rs"]
mod write;

pub(crate) fn prepare(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    match method {
        "cookies" => eval_no_args(method, args, "document.cookie", "browser.inspect.storage"),
        "local_storage" => eval_no_args(
            method,
            args,
            js::storage_js("localStorage"),
            "browser.inspect.storage",
        ),
        "session_storage" => eval_no_args(
            method,
            args,
            js::storage_js("sessionStorage"),
            "browser.inspect.storage",
        ),
        "indexed_db_summary" => eval_no_args(
            method,
            args,
            "({unsupported:'indexed_db_summary'})",
            "browser.inspect.storage",
        ),
        "set_cookie" => write::set_cookie(args),
        "set_local_storage" => write::set_local(args),
        "clear_storage" => eval_no_args(
            method,
            args,
            "localStorage.clear();sessionStorage.clear();true",
            "browser.mutate.storage",
        ),
        _ => unreachable!(),
    }
}

fn eval_no_args(
    method: &str,
    args: &[Value],
    expression: impl Into<String>,
    scope: &'static str,
) -> Result<BrowserCall, String> {
    super::args::no_args(method, args)?;
    Ok(js::eval(expression.into(), scope))
}
