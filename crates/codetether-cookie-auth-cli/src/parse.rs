//! Cookie import parse responsibilities.
use crate::row::CookieRow;

pub(super) fn parse_netscape_cookie_file(raw: &str) -> Vec<CookieRow> {
    raw.lines().filter_map(parse_netscape_cookie_line).collect()
}

pub(super) fn parse_netscape_cookie_line(line: &str) -> Option<CookieRow> {
    let trimmed = line.trim();
    if trimmed.is_empty() || (trimmed.starts_with('#') && !trimmed.starts_with("#HttpOnly_")) {
        return None;
    }

    let (http_only, normalized) = if let Some(rest) = trimmed.strip_prefix("#HttpOnly_") {
        (true, rest)
    } else {
        (false, trimmed)
    };
    let parts: Vec<&str> = normalized.split('\t').collect();
    if parts.len() < 7 {
        return None;
    }

    Some(CookieRow {
        domain: parts[0].trim().to_string(),
        include_subdomains: parts[1].trim().eq_ignore_ascii_case("TRUE"),
        path: parts[2].trim().to_string(),
        secure: parts[3].trim().eq_ignore_ascii_case("TRUE"),
        expires_epoch: parts[4].trim().parse::<i64>().unwrap_or(0),
        name: parts[5].trim().to_string(),
        value: parts[6].trim().to_string(),
        http_only,
    })
}
