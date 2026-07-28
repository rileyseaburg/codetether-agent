//! Cookie header parsing.

use super::date::now_seconds;
use super::model::Cookie;
use super::parse_attr::apply_attr;
use super::parse_start::{first_pair, new_cookie};
use super::url::host_for_url;

pub(crate) enum CookieSource {
    Server,
    Document,
}

pub(crate) struct ParsedCookie {
    pub cookie: Cookie,
    pub expired: bool,
}

pub(crate) fn parse_cookie(
    raw: &str,
    current_url: &str,
    source: CookieSource,
) -> Result<ParsedCookie, String> {
    let mut parts = raw.split(';').map(str::trim);
    let (name, value) = first_pair(parts.next())?;
    let host = host_for_url(current_url);
    let mut cookie = new_cookie(name, value, &host, current_url);
    let mut max_age: Option<i64> = None;
    for part in parts {
        apply_attr(&mut cookie, part, &host, &source, &mut max_age)?;
    }
    if let Some(age) = max_age {
        cookie.expires_at = Some(now_seconds().saturating_add(age));
    }
    Ok(ParsedCookie {
        expired: max_age.is_some_and(|age| age <= 0) || cookie.is_expired(now_seconds()),
        cookie,
    })
}
