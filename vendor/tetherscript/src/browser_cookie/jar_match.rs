//! Cookie visibility filtering.

use super::date::now_seconds;
use super::model::Cookie;
use super::path::{path_for_url, path_matches};
use super::scope::{domain_visible, same_site_visible};
use super::url::is_https;

pub(crate) fn visible_pairs(jar: &[Cookie], url: &str, http: bool, source: Option<&str>) -> String {
    let mut cookies = matching(jar, url, http, source);
    cookies.sort_by(|left, right| right.path.len().cmp(&left.path.len()));
    cookies
        .into_iter()
        .map(|cookie| format!("{}={}", cookie.name, cookie.value))
        .collect::<Vec<_>>()
        .join("; ")
}

pub(crate) fn matching<'a>(
    jar: &'a [Cookie],
    url: &str,
    include_http_only: bool,
    source: Option<&str>,
) -> Vec<&'a Cookie> {
    let now = now_seconds();
    jar.iter()
        .filter(|cookie| cookie_visible(cookie, url, include_http_only, source, now))
        .collect()
}

fn cookie_visible(
    cookie: &Cookie,
    url: &str,
    include_http_only: bool,
    source: Option<&str>,
    now: i64,
) -> bool {
    !cookie.is_expired(now)
        && (include_http_only || !cookie.http_only)
        && (!cookie.secure || is_https(url))
        && domain_visible(cookie, url)
        && path_matches(&cookie.path, &path_for_url(url))
        && same_site_visible(cookie, url, source)
}
