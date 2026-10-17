//! Pure header analysis for explain_cors. No I/O.

use std::collections::BTreeMap;

pub(super) fn header_reasons(
    headers: &BTreeMap<String, String>,
    origin: &str,
    method: &str,
) -> Vec<String> {
    let mut reasons = Vec::new();
    let allow_origin = headers
        .get("access-control-allow-origin")
        .map(String::as_str)
        .unwrap_or("");
    if allow_origin != "*" && allow_origin != origin {
        reasons.push(format!(
            "Access-Control-Allow-Origin = '{allow_origin}', does not match '{origin}'"
        ));
    }
    let allow_methods = headers
        .get("access-control-allow-methods")
        .map(String::as_str)
        .unwrap_or("");
    if !allow_methods
        .split(',')
        .any(|m| m.trim().eq_ignore_ascii_case(method))
    {
        reasons.push(format!(
            "method {method} not in Access-Control-Allow-Methods = '{allow_methods}'"
        ));
    }
    reasons
}
