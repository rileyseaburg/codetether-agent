//! Selector extension matching.

use crate::browser::{element_matches, Document, Element, Node};

use super::types::SelectorFilter;
use super::{parse, state, text, visible};

pub(crate) fn matches(
    _document: &Document,
    node: &Node,
    element: &Element,
    ancestors: &[Element],
    selector: &str,
) -> bool {
    let plan = parse::parse(selector);
    element_matches(element, ancestors, &plan.base)
        && plan
            .filters
            .iter()
            .all(|filter| matches_filter(node, element, ancestors, filter))
}

fn matches_filter(
    node: &Node,
    element: &Element,
    ancestors: &[Element],
    filter: &SelectorFilter,
) -> bool {
    match filter {
        SelectorFilter::Visible => visible::visible(element, ancestors),
        SelectorFilter::Enabled => state::enabled(element),
        SelectorFilter::Disabled => state::disabled(element),
        SelectorFilter::Checked => state::checked(element),
        SelectorFilter::HasText(text) => text::has_text(node, text),
        SelectorFilter::Nth(_) => true,
        SelectorFilter::Invalid => false,
    }
}
