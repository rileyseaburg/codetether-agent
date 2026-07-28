//! Cookie name/value attribute parsing.

use super::date::parse_cookie_time;
use super::model::{Cookie, SameSite};
use super::url::domain_matches;

pub(crate) fn apply_pair(
    cookie: &mut Cookie,
    name: &str,
    value: &str,
    host: &str,
    max_age: &mut Option<i64>,
) -> Result<(), String> {
    if name.eq_ignore_ascii_case("domain") {
        apply_domain(cookie, value, host)?;
    } else if name.eq_ignore_ascii_case("path") {
        cookie.path = valid_path(value);
    } else if name.eq_ignore_ascii_case("samesite") {
        cookie.same_site = SameSite::parse(value);
    } else if name.eq_ignore_ascii_case("max-age") {
        *max_age = value.parse::<i64>().ok();
    } else if name.eq_ignore_ascii_case("expires") {
        cookie.expires_at = parse_cookie_time(value);
    }
    Ok(())
}

fn apply_domain(cookie: &mut Cookie, value: &str, host: &str) -> Result<(), String> {
    let domain = value.trim_start_matches('.').to_ascii_lowercase();
    if domain.is_empty() || !domain_matches(host, &domain) {
        return Err(format!("cookie domain {domain} does not match host {host}"));
    }
    cookie.domain = domain;
    cookie.host_only = false;
    Ok(())
}

fn valid_path(value: &str) -> String {
    if value.starts_with('/') {
        value.into()
    } else {
        "/".into()
    }
}
