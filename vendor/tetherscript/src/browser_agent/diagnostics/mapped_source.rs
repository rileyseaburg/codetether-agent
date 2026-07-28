//! Generated JavaScript source lookup for diagnostics.

use crate::browser_agent::page::resources::ResourceKind;
use crate::browser_agent::page::BrowserPage;

pub fn script<'a>(page: &'a BrowserPage, script_url: &str) -> Option<&'a str> {
    page.resources.text(ResourceKind::Script, script_url)
}
