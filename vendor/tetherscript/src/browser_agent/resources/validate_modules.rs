//! Module graph validation for deterministic page resources.

use crate::browser_agent::page::BrowserPage;

use super::{script_module, script_module_refs};

pub(crate) fn missing(page: &BrowserPage) -> Vec<String> {
    script_module_refs::collect(&page.session.document)
        .into_iter()
        .filter_map(|reference| {
            script_module::source(&page.resources, &page.session.url, &reference)
                .map(|_| ())
                .err()
        })
        .collect()
}
