//! Runtime exception taxonomy report construction.

use crate::browser_agent::events::PageErrorEvent;
use crate::browser_agent::page::BrowserPage;

use super::exception_types::RuntimeException;

pub fn collect(page: &BrowserPage, page_errors: &[PageErrorEvent]) -> Vec<RuntimeException> {
    let mut out = super::exception_page::collect(page_errors);
    out.extend(super::exception_network::collect(page));
    out.extend(super::exception_console::collect(page));
    out
}
