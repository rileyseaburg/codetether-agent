//! Network request and replay mappings.

use crate::value::Value;

use super::super::authority::BrowserAuthority;
use super::super::call::BrowserCall;

pub(crate) fn request(
    auth: &BrowserAuthority,
    action: &str,
    args: &[Value],
) -> Result<BrowserCall, String> {
    let url = super::super::args::expect_str(action, args, 0)?;
    auth.require_origin_url(&url)?;
    let mut entries = vec![("url", super::super::value::str_value(url))];
    if let Some(v) = args.get(1) {
        entries.push(("method", v.clone()));
    }
    if let Some(v) = args.get(2) {
        entries.push(("body", v.clone()));
    }
    Ok(super::call(action, entries))
}

pub(crate) fn replay(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    let mut entries = vec![(
        "url_contains",
        super::super::args::expect_value(method, args, 0)?,
    )];
    if let Some(v) = args.get(1) {
        entries.push(("body_patch", v.clone()));
    }
    Ok(super::call("replay", entries))
}

pub(crate) fn wait_network(method: &str, args: &[Value]) -> Result<BrowserCall, String> {
    Ok(super::call(
        "network_log",
        vec![
            (
                "url_contains",
                super::super::value::str_value(super::super::args::expect_str(method, args, 0)?),
            ),
            ("limit", Value::Int(1)),
        ],
    ))
}
