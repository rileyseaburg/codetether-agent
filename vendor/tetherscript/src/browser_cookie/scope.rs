//! Cookie domain and SameSite visibility.

use super::model::{Cookie, SameSite};
use super::url::{domain_matches, host_for_url, same_site};

pub(crate) fn domain_visible(cookie: &Cookie, url: &str) -> bool {
    let host = host_for_url(url);
    if cookie.host_only {
        host == cookie.domain
    } else {
        domain_matches(&host, &cookie.domain)
    }
}

pub(crate) fn same_site_visible(cookie: &Cookie, url: &str, source: Option<&str>) -> bool {
    match source {
        Some(source) => cookie.same_site == SameSite::None || same_site(source, url),
        None => true,
    }
}
