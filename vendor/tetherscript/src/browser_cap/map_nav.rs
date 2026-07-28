//! Navigation and tab browserctl action mapping.

use crate::value::Value;

use super::authority::BrowserAuthority;
use super::call::BrowserCall;

#[path = "map_nav_call.rs"]
mod nav_call;

#[rustfmt::skip]
pub(crate) fn prepare(auth: &BrowserAuthority, method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    match method {
        "health" | "detect" | "start" | "stop" | "reload" | "back" | "tabs" => {
            nav_call::simple(method, args)
        }
        "forward" => Err("browser.forward: browserctl backend does not support forward".into()),
        "goto" => {
            let url = super::args::expect_str(method, args, 0)?;
            auth.require_origin_url(&url)?;
            Ok(nav_call::call(
                "goto",
                vec![("url", super::value::str_value(url))],
            ))
        }
        "tabs_new" => {
            let url = super::args::expect_str(method, args, 0)?;
            auth.require_origin_url(&url)?;
            Ok(nav_call::call(
                "tabs_new",
                vec![("url", super::value::str_value(url))],
            ))
        }
        "tabs_select" | "tabs_close" => {
            let index = super::args::expect_int(method, args, 0)?;
            Ok(nav_call::call(method, vec![("index", Value::Int(index))]))
        }
        "wait_for_url" => Ok(nav_call::call(
            "wait",
            vec![
                (
                    "url_contains",
                    super::value::str_value(super::args::expect_str(method, args, 0)?),
                ),
                (
                    "timeout_ms",
                    Value::Int(super::args::optional_int(args, 1, 30_000)?),
                ),
            ],
        )),
        _ => unreachable!(),
    }
}
