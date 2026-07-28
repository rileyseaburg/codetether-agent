//! Shared network cookie jar state.

use std::cell::RefCell;

use crate::browser_cookie::{self, Cookie};

thread_local! {
    static COOKIE_JAR: RefCell<Vec<Cookie>> = const { RefCell::new(Vec::new()) };
    static DOCUMENT_URL: RefCell<String> = const { RefCell::new(String::new()) };
}

pub(crate) fn reset() {
    COOKIE_JAR.with(|jar| jar.borrow_mut().clear());
    DOCUMENT_URL.with(|url| url.borrow_mut().clear());
}

pub(crate) fn seed(cookies: Vec<Cookie>, url: String) {
    COOKIE_JAR.with(|jar| *jar.borrow_mut() = cookies);
    set_document_url(&url);
}

pub(crate) fn jar() -> Vec<Cookie> {
    COOKIE_JAR.with(|jar| jar.borrow().clone())
}

pub(crate) fn set_document_url(url: &str) {
    DOCUMENT_URL.with(|stored| *stored.borrow_mut() = url.to_string());
}

pub(super) fn document_url() -> String {
    DOCUMENT_URL.with(|stored| stored.borrow().clone())
}

pub(super) fn with_jar<R>(f: impl FnOnce(&mut Vec<Cookie>) -> R) -> R {
    COOKIE_JAR.with(|stored| {
        let mut jar = stored.borrow_mut();
        f(&mut jar)
    })
}

pub(super) fn sync_document_projection(jar: &[Cookie]) {
    super::super::cookie_host::seed(browser_cookie::document_cookie_pairs(jar, &document_url()));
}
