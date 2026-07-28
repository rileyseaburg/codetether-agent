//! Agent-side CSSOM inspection for computed style snapshots.
//!
//! This module layers small agent style-inspection APIs over the
//! deterministic browser style engine. It keeps viewport/media filtering local
//! to agent pages and returns stable property maps for agent assertions.

mod active_css;
mod conditions;
mod defaults;
pub mod flex;
pub mod inline;
mod lookup;
mod model;
mod page;

use crate::browser::Document;
use crate::browser_agent::MediaEmulation;

pub use model::ComputedStyle;

pub(crate) fn active_css_for(source: &str, width: i64, media: MediaEmulation) -> String {
    active_css::active_css(source, width, media)
}

pub(crate) fn computed_style_at_path(
    document: &Document,
    css: &str,
    path: &[usize],
) -> Option<ComputedStyle> {
    lookup::at_path(document, css, path)
}

#[cfg(test)]
mod cascade_tests;
#[cfg(test)]
mod media_tests;
