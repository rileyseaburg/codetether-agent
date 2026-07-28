//! Initial blank-page lifecycle suppression.

use crate::browser_agent::page::BrowserPage;

pub(super) fn should_log(page: &BrowserPage, from: &str) -> bool {
    page.navigation.id != 0 || from != "about:blank"
}
