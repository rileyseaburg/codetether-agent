//! HAR (HTTP Archive) format export.

use super::capture::{header, CapturedExchange};

/// Export captured exchanges as a HAR JSON string.
pub fn export_har(exchanges: &[CapturedExchange]) -> String {
    let entries: Vec<String> = exchanges.iter().map(|e| format!(
        r#"{{"startedDateTime":"{}","time":{},"request":{{"method":"{}","url":"{}"}},"response":{{"status":{},"statusText":"{}"}},"timings":{{"dns":{},"connect":{},"ssl":{},"wait":{},"receive":{}}}}}"#,
        e.timing.start_ms, e.timing.total_ms,
        esc(&e.request.method), esc(&e.request.url),
        e.response.as_ref().map(|r| r.status).unwrap_or(0),
        esc(&e.response.as_ref().map(|r| r.status_text.clone()).unwrap_or_default()),
        e.timing.dns_ms, e.timing.connect_ms, e.timing.tls_ms, e.timing.ttfb_ms, e.timing.download_ms
    )).collect();
    format!(r#"{{"log":{{"version":"1.2","creator":{{"name":"tetherscript","version":"1"}},"entries":[{}]}}}}"#, entries.join(","))
}

fn esc(s: &str) -> String {
    s.replace('\\', "\\\\").replace('"', "\\\"").replace('\n', "\\n").replace('\r', "\\r").replace('\t', "\\t")
}
