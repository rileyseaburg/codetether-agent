//! Security headers for the companion web shell.
use axum::http::{HeaderMap, HeaderName, HeaderValue};

const CSP: &str = "default-src 'none'; script-src 'self'; style-src 'self'; connect-src 'self'; img-src 'self' data: blob:; media-src 'self' blob:; worker-src 'self'; manifest-src 'self'; base-uri 'none'; form-action 'self'; frame-ancestors 'none'";

fn kind(file: &str) -> &'static str {
    match file.rsplit('.').next() {
        Some("html") => "text/html; charset=utf-8",
        Some("js") => "text/javascript",
        Some("css") => "text/css",
        Some("webmanifest") => "application/manifest+json",
        Some("svg") => "image/svg+xml",
        _ => "application/octet-stream",
    }
}
/// Headers matching the TypeScript relay's asset responses.
pub(crate) fn headers(file: &str) -> HeaderMap {
    let mut map = HeaderMap::new();
    for (name, value) in [
        ("content-type", kind(file)),
        ("cache-control", "no-cache"),
        ("x-content-type-options", "nosniff"),
        ("referrer-policy", "no-referrer"),
        ("cross-origin-resource-policy", "same-origin"),
        (
            "permissions-policy",
            "camera=(), microphone=(), display-capture=(self)",
        ),
        ("content-security-policy", CSP),
        ("service-worker-allowed", "/companion/"),
    ] {
        map.insert(
            HeaderName::from_static(name),
            HeaderValue::from_static(value),
        );
    }
    map
}
