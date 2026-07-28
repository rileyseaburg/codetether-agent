//! Header scanners for static request parsing.

use super::request_body::MAX_BODY_BYTES;
use super::request_parse::ParsedHead;

/// Apply one header line to parsed request metadata.
pub(crate) fn apply_header(line: &str, parsed: &mut ParsedHead<'_>) {
    if let Some((name, value)) = line.split_once(':') {
        let value = value.trim();
        parsed.close |= name.eq_ignore_ascii_case("connection") && has_token(value, "close");
        parsed.keep_alive |=
            name.eq_ignore_ascii_case("connection") && has_token(value, "keep-alive");
        if name.eq_ignore_ascii_case("content-length") {
            parsed.content_length = value.parse().unwrap_or(MAX_BODY_BYTES + 1);
        }
    }
}

fn has_token(value: &str, token: &str) -> bool {
    value
        .split(',')
        .any(|part| part.trim().eq_ignore_ascii_case(token))
}
