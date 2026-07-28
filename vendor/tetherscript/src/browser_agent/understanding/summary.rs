//! Structured page summary generation.

mod types;

use super::{
    actions::detect_actionable, forms::classify_form, links::classify_link,
    regions::detect_regions, ElementSummary, InputSummary,
};
pub use types::{FormInfo, LinkInfo, PageSummary};

pub fn summarize_page(
    elements: &[ElementSummary],
    forms: &[Vec<InputSummary>],
    links: &[(String, String, String)],
) -> PageSummary {
    PageSummary {
        regions: detect_regions(elements),
        forms: forms
            .iter()
            .map(|f| FormInfo::new(classify_form(f), f.len()))
            .collect(),
        links: links
            .iter()
            .map(|(href, text, ctx)| LinkInfo::new(href, text, classify_link(href, text, ctx)))
            .collect(),
        actions: detect_actionable(elements),
    }
}
