//! Cookie jar operations.

use super::date::now_seconds;
use super::jar_apply::{apply_document_parsed, apply_server_parsed};
use super::jar_match::{matching, visible_pairs};
use super::model::Cookie;
use super::parse::{parse_cookie, CookieSource};

pub(crate) fn set_server_cookie(
    jar: &mut Vec<Cookie>,
    raw: &str,
    current_url: &str,
) -> Result<(), String> {
    let parsed = parse_cookie(raw, current_url, CookieSource::Server)?;
    apply_server_parsed(jar, parsed);
    Ok(())
}

pub(crate) fn apply_document_cookies(jar: &mut Vec<Cookie>, raws: Vec<String>, current_url: &str) {
    for raw in raws {
        if let Ok(parsed) = parse_cookie(&raw, current_url, CookieSource::Document) {
            apply_document_parsed(jar, parsed);
        }
    }
}

pub(crate) fn cookie_header(jar: &[Cookie], url: &str) -> String {
    visible_pairs(jar, url, true, None)
}

pub(crate) fn request_cookie_header(jar: &[Cookie], url: &str, initiator_url: &str) -> String {
    visible_pairs(jar, url, true, Some(initiator_url))
}

pub(crate) fn document_cookie_pairs(jar: &[Cookie], url: &str) -> Vec<(String, String)> {
    matching(jar, url, false, None)
        .into_iter()
        .map(|cookie| (cookie.name.clone(), cookie.value.clone()))
        .collect()
}

pub(crate) fn persistent_cookies(jar: &[Cookie]) -> Vec<Cookie> {
    let now = now_seconds();
    jar.iter()
        .filter(|cookie| !cookie.is_expired(now))
        .cloned()
        .collect()
}
