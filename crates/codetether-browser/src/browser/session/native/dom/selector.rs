//! Selector conversion and JavaScript quoting helpers.

use crate::browser::BrowserError;
use tetherscript::browser_agent::Locator;

/// Convert a CSS selector string into a TetherScript locator.
pub(super) fn css(selector: String) -> Locator {
    Locator::css(selector).relaxed()
}

/// Quote a string for use inside JavaScript source.
pub(super) fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".into())
}

/// Convert a JavaScript error string into a browser error.
pub(super) fn js_error(error: String) -> BrowserError {
    super::super::eval::js_error(error)
}
