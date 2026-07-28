//! Deterministic page navigation commits.

use crate::browser_agent::page::BrowserPage;

pub(crate) fn document(page: &mut BrowserPage, url: String, action: &str) {
    crate::browser_agent::navigation::commit_document(page, url, action);
}
