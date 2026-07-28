//! Source-map remapping entrypoint.

use crate::browser_agent::page::BrowserPage;

use super::super::mapped_types::{GeneratedSourceLocation, OriginalSourceLocation};

pub fn map_location(
    page: &BrowserPage,
    generated: &GeneratedSourceLocation,
) -> Option<OriginalSourceLocation> {
    let text = super::status::source_map_text(page, &generated.script_url)?;
    let map = super::parsed::parse(text)?;
    super::segments::original(&map, generated.line, generated.column)
}
