//! URL assertion observations.

use crate::browser_agent::page::BrowserPage;

use super::retry;

pub(crate) fn contains(page: &mut BrowserPage, substring: &str) -> Result<(), String> {
    retry::run(page, "expect_url_contains", |page| {
        let actual = page.session.url.clone();
        if actual.contains(substring) {
            Ok(())
        } else {
            Err(format!(
                "expected URL to contain {substring:?}, last URL {actual:?}"
            ))
        }
    })
}
