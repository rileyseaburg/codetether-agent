//! Source-map status extraction and generated-location remapping.

mod mapper;
mod parsed;
mod segments;
mod status;
mod vlq;

use crate::browser_agent::page::BrowserPage;

use super::mapped_types::{GeneratedSourceLocation, OriginalSourceLocation};
use super::types::SourceMapStatus;

pub fn statuses(page: &BrowserPage) -> Vec<SourceMapStatus> {
    status::statuses(page)
}

pub fn map_location(
    page: &BrowserPage,
    generated: &GeneratedSourceLocation,
) -> Option<OriginalSourceLocation> {
    mapper::map_location(page, generated)
}
