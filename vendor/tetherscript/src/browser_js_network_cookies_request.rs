//! Cookie request-header generation.

use crate::browser_cookie;

use super::state;

pub(crate) fn append_request_header(
    headers: &mut Vec<(String, String)>,
    url: &str,
    initiator_url: &str,
) {
    if headers
        .iter()
        .any(|(name, _)| name.eq_ignore_ascii_case("cookie"))
    {
        return;
    }
    let value =
        state::with_jar(|jar| browser_cookie::request_cookie_header(jar, url, initiator_url));
    if !value.is_empty() {
        headers.push(("cookie".into(), value));
    }
}
