//! Parse standard HTTP dates and Google's hyphen-separated cookie dates.
use std::time::UNIX_EPOCH;

pub(super) fn parse(value: &str) -> Option<u64> {
    // Google uses e.g. `Sun, 24-Oct-2027 04:20:54 GMT`, which is neither
    // IMF-fixdate nor the two-digit-year obsolete RFC 850 HTTP-date form.
    httpdate::parse_http_date(value)
        .or_else(|_| httpdate::parse_http_date(&value.replace('-', " ")))
        .ok()?
        .duration_since(UNIX_EPOCH)
        .ok()
        .map(|duration| duration.as_secs())
}
