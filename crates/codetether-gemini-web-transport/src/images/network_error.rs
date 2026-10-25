//! Allowlisted diagnostics, excluding request URLs, headers, and body contents.
use std::error::Error;

pub(super) fn classify(error: &reqwest::Error) -> String {
    let category = if error.is_timeout() {
        "timeout"
    } else if error.is_connect() {
        "connect"
    } else if error.is_builder() {
        "request_builder"
    } else if error.is_decode() {
        "decode"
    } else if error.is_body() {
        "body"
    } else {
        "request"
    };
    let mut markers = Vec::new();
    let mut source = error.source();
    while let Some(cause) = source {
        let description = cause.to_string().to_ascii_lowercase();
        for marker in [
            "dns",
            "certificate",
            "tls",
            "http2",
            "http/2",
            "protocol",
            "reset",
            "refused",
            "closed",
            "broken pipe",
        ] {
            if description.contains(marker) && !markers.contains(&marker) {
                markers.push(marker);
            }
        }
        source = cause.source();
    }
    format!("{category}:{}", markers.join(","))
}
