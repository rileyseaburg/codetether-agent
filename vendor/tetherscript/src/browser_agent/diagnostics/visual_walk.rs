//! DOM walk that joins element, computed style, and layout evidence.

use crate::browser::{self, Document, Element, LayoutBox, Node};

use super::{visual_bounds, visual_selectors, visual_text, visual_types::VisualElementEvidence};

pub fn collect_node(
    document: &Document,
    node: &Node,
    path: &[usize],
    layout: &LayoutBox,
    css: &str,
    out: &mut Vec<VisualElementEvidence>,
    max: usize,
) {
    if out.len() >= max {
        return;
    }
    let Node::Element(element) = node else { return };
    if is_visual_tag(&element.tag) {
        if let Some(item) = build(document, element, path, layout, css) {
            out.push(item);
        }
    }
    for (index, child) in element.children.iter().enumerate() {
        let mut child_path = path.to_vec();
        child_path.push(index);
        collect_node(document, child, &child_path, layout, css, out, max);
    }
}

fn build(
    document: &Document,
    element: &Element,
    path: &[usize],
    root: &LayoutBox,
    css: &str,
) -> Option<VisualElementEvidence> {
    let layout = browser::find_layout_box_at_path(root, path)?;
    let style = crate::browser_agent::page::cssom::computed_style_at_path(document, css, path)?;
    let bounds = visual_bounds::bounds_for(layout);
    let visible = visual_bounds::visible(layout, &style.properties, bounds);
    Some(VisualElementEvidence {
        selector_candidates: visual_selectors::candidates(element, path),
        tag: element.tag.clone(),
        text: visual_text::element_text(element),
        bounds,
        visible,
        computed_styles: style.properties,
    })
}

fn is_visual_tag(tag: &str) -> bool {
    !matches!(tag, "script" | "style" | "link" | "meta" | "title")
}
