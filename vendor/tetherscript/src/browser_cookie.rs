//! Cookie parsing and matching helpers for deterministic browser sessions.

#[path = "browser_cookie/date.rs"]
mod date;
#[path = "browser_cookie/date_civil.rs"]
mod date_civil;
#[path = "browser_cookie/date_parts.rs"]
mod date_parts;
#[path = "browser_cookie/jar.rs"]
mod jar;
#[path = "browser_cookie/jar_apply.rs"]
mod jar_apply;
#[path = "browser_cookie/jar_match.rs"]
mod jar_match;
#[path = "browser_cookie/model.rs"]
mod model;
#[path = "browser_cookie/parse.rs"]
mod parse;
#[path = "browser_cookie/parse_attr.rs"]
mod parse_attr;
#[path = "browser_cookie/parse_pair.rs"]
mod parse_pair;
#[path = "browser_cookie/parse_start.rs"]
mod parse_start;
#[path = "browser_cookie/path.rs"]
mod path;
#[path = "browser_cookie/scope.rs"]
mod scope;
#[path = "browser_cookie/url.rs"]
mod url;

#[allow(unused_imports)]
pub use model::{Cookie, SameSite};

#[allow(unused_imports)]
pub(crate) use jar::{
    apply_document_cookies, cookie_header, document_cookie_pairs, persistent_cookies,
    request_cookie_header, set_server_cookie,
};
#[allow(unused_imports)]
pub(crate) use url::storage_origin;

#[cfg(test)]
#[path = "browser_cookie/tests_document.rs"]
mod tests_document;
#[cfg(test)]
#[path = "browser_cookie/tests_expiry.rs"]
mod tests_expiry;
#[cfg(test)]
#[path = "browser_cookie/tests_scope.rs"]
mod tests_scope;
