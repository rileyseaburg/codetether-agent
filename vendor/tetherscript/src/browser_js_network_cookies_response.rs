//! Cookie mutation from document and network responses.

use crate::browser_cookie;

use super::state;

pub(crate) fn apply_document_cookie(raw: &str) {
    let url = state::document_url();
    state::with_jar(|jar| {
        browser_cookie::apply_document_cookies(jar, vec![raw.into()], &url);
    });
}

pub(crate) fn apply_response_headers(url: &str, headers: &[(String, String)]) {
    state::with_jar(|jar| {
        let mut changed = false;
        for (_, value) in headers
            .iter()
            .filter(|(name, _)| name.eq_ignore_ascii_case("set-cookie"))
        {
            changed |= browser_cookie::set_server_cookie(jar, value, url).is_ok();
        }
        if changed {
            state::sync_document_projection(jar);
        }
    });
}
