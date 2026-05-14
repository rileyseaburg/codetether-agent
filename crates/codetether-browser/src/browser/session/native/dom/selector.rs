use crate::browser::BrowserError;
use tetherscript::browser_agent::Locator;

pub(super) fn css(selector: String) -> Locator {
    Locator::css(selector).relaxed()
}

pub(super) fn quote(value: &str) -> String {
    serde_json::to_string(value).unwrap_or_else(|_| "\"\"".into())
}

pub(super) fn js_error(error: String) -> BrowserError {
    super::super::eval::js_error(error)
}
