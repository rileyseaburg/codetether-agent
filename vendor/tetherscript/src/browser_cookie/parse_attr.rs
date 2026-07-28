//! Cookie flag attribute parsing.

use super::model::Cookie;
use super::parse::CookieSource;
use super::parse_pair::apply_pair;

pub(crate) fn apply_attr(
    cookie: &mut Cookie,
    part: &str,
    host: &str,
    source: &CookieSource,
    max_age: &mut Option<i64>,
) -> Result<(), String> {
    if part.eq_ignore_ascii_case("secure") {
        cookie.secure = true;
    } else if part.eq_ignore_ascii_case("httponly") {
        cookie.http_only = matches!(source, CookieSource::Server);
    } else if let Some((name, value)) = part.split_once('=') {
        apply_pair(cookie, name.trim(), value.trim(), host, max_age)?;
    }
    Ok(())
}
