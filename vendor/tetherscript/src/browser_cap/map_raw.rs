//! Explicit raw browserctl action escape hatch.

use crate::value::Value;

use super::authority::BrowserAuthority;
use super::call::BrowserCall;

pub(crate) fn prepare(auth: &BrowserAuthority, args: &[Value]) -> Result<BrowserCall, String> {
    let action = super::args::expect_str("raw", args, 0)?;
    let scope = super::actions::scope_for_action(&action)
        .ok_or_else(|| format!("browser.raw: unknown action `{}`", action))?;
    let params = args.get(1).cloned().unwrap_or(Value::Nil);
    let payload = super::value::with_action(&action, &params)?;
    if let Some(url) = raw_url(&payload) {
        auth.require_origin_url(&url)?;
    }
    Ok(BrowserCall::new(&action, scope, payload))
}

fn raw_url(payload: &Value) -> Option<String> {
    match payload {
        Value::Map(m) => match m.borrow().get("url") {
            Some(Value::Str(s)) => Some((**s).clone()),
            _ => None,
        },
        _ => None,
    }
}
