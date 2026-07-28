//! Source-mapped page error construction.

use crate::browser_agent::events::PageErrorEvent;
use crate::browser_agent::page::BrowserPage;

use super::mapped_types::SourceMappedPageError;

pub fn collect(page: &BrowserPage, errors: &[PageErrorEvent]) -> Vec<SourceMappedPageError> {
    errors
        .iter()
        .filter_map(|error| mapped(page, error))
        .collect()
}

fn mapped(page: &BrowserPage, error: &PageErrorEvent) -> Option<SourceMappedPageError> {
    let generated = super::mapped_error_locate::location(page, error)?;
    let original = super::source_maps::map_location(page, &generated);
    Some(SourceMappedPageError {
        action: error.action.clone(),
        message: error.message.clone(),
        stack: super::mapped_stack::frames(page, &generated),
        generated,
        original,
    })
}
