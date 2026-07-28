//! CORS and credentials policy helpers.
use std::collections::HashMap;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CorsMode {
    None,
    NoCors,
    Cors,
    SameOrigin,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CredentialsMode {
    Omit,
    SameOrigin,
    Include,
}

/// Minimal response-side CORS check.
pub fn check_cors(
    origin: &str,
    url: &str,
    mode: CorsMode,
    headers: &HashMap<String, String>,
) -> bool {
    if matches!(mode, CorsMode::None | CorsMode::NoCors) || same_origin(origin, url) {
        return true;
    }
    headers
        .get("access-control-allow-origin")
        .map(|v| v == "*" || v == origin)
        .unwrap_or(false)
}

pub fn same_origin(a: &str, b: &str) -> bool {
    origin_of(a) == origin_of(b)
}

pub(crate) fn origin_of(u: &str) -> String {
    let (scheme, rest) = u.split_once("://").unwrap_or(("", u));
    let host = rest.split('/').next().unwrap_or(rest);
    format!("{}://{}", scheme, host)
}
