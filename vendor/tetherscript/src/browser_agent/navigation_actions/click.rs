//! Click-result navigation dispatch.

use crate::browser_agent::page::BrowserPage;
use crate::browser_agent::resolve::Resolved;
use crate::js::JsValue;

pub(crate) fn after_click(
    page: &mut BrowserPage,
    resolved: &Resolved,
    value: &JsValue,
) -> Result<(), String> {
    if value == &JsValue::Bool(false) {
        return Ok(());
    }
    let action = navigation_target(page, &resolved.dom.path);
    if let Some((url, action)) = action {
        super::commit::document(page, url, action);
    }
    Ok(())
}

fn navigation_target(page: &BrowserPage, path: &[usize]) -> Option<(String, &'static str)> {
    let current = page.session.url.as_str();
    let document = &page.session.document;
    if let Some(href) = super::anchor::href(document, path) {
        return Some((super::url::resolve(current, &href), "anchor_click"));
    }
    super::form::submit_target(document, path, current).map(|url| (url, "form_submit"))
}
