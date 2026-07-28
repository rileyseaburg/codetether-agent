//! Visual evidence collection for production debug reports.

use crate::browser;
use crate::browser_agent::page::{cssom, BrowserPage};

use super::{visual_types::VisualElementEvidence, visual_walk};

const MAX_VISUAL_ELEMENTS: usize = 200;

pub fn collect(page: &BrowserPage) -> Vec<VisualElementEvidence> {
    let css = cssom::active_css_for(&page.session.css, page.viewport_width, page.media);
    let layout = browser::layout_document(&page.session.document, &css, page.viewport_width);
    let mut out = Vec::new();
    for (index, node) in page.session.document.children.iter().enumerate() {
        visual_walk::collect_node(
            &page.session.document,
            node,
            &[index],
            &layout,
            &css,
            &mut out,
            MAX_VISUAL_ELEMENTS,
        );
        if out.len() >= MAX_VISUAL_ELEMENTS {
            break;
        }
    }
    out
}
