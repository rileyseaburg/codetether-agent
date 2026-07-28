//! Cookie jar mutation rules.

use super::model::Cookie;
use super::parse::ParsedCookie;

pub(crate) fn apply_server_parsed(jar: &mut Vec<Cookie>, parsed: ParsedCookie) {
    jar.retain(|cookie| !cookie.same_key(&parsed.cookie));
    if !parsed.expired {
        jar.push(parsed.cookie);
    }
}

pub(crate) fn apply_document_parsed(jar: &mut Vec<Cookie>, parsed: ParsedCookie) {
    if jar
        .iter()
        .any(|cookie| cookie.http_only && cookie.same_key(&parsed.cookie))
    {
        return;
    }
    apply_server_parsed(jar, parsed);
}
