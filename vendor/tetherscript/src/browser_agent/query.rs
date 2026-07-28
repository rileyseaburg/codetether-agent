//! DOM query helpers for locators.

use crate::browser::{Document, Element, Node};
use crate::browser_agent::{locator::Locator, locator_filters::matches_locator, selector_ext};

#[derive(Debug, Clone)]
pub(crate) struct DomMatch {
    pub path: Vec<usize>,
    pub element: Element,
}

pub(crate) fn locate(document: &Document, locator: &Locator) -> Vec<DomMatch> {
    let mut out = Vec::new();
    for (index, child) in document.children.iter().enumerate() {
        visit(document, child, &[index], &[], locator, &mut out);
    }
    selector_ext::apply_order(locator, out)
}

fn visit(
    document: &Document,
    node: &Node,
    path: &[usize],
    ancestors: &[Element],
    locator: &Locator,
    out: &mut Vec<DomMatch>,
) {
    let Node::Element(element) = node else { return };
    if matches_locator(document, node, element, ancestors, locator) {
        out.push(DomMatch {
            path: path.to_vec(),
            element: element.clone(),
        });
    }
    let mut next_ancestors = ancestors.to_vec();
    next_ancestors.push(element.clone());
    for (index, child) in element.children.iter().enumerate() {
        let mut child_path = path.to_vec();
        child_path.push(index);
        visit(document, child, &child_path, &next_ancestors, locator, out);
    }
}
