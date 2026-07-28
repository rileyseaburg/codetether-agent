//! Element accessibility node construction.

use crate::browser::{Document, Element, Node};
use crate::browser_agent::{interact::focus::selector_for, names, roles};

use super::super::model::AccessibilityNode;
use super::super::state;
use super::FocusMap;

pub(super) fn entry(
    document: &Document,
    item: &Node,
    element: &Element,
    path: Vec<usize>,
    ancestors: &[Element],
    focus_map: &FocusMap,
    children: Vec<AccessibilityNode>,
) -> AccessibilityNode {
    let focus_index = focus_map.get(&path).copied();
    AccessibilityNode {
        role: roles::role_of(element),
        name: names::accessible_name(document, item, element, ancestors),
        tag: element.tag.clone(),
        selector: selector_for(&path, element),
        focusable: focus_index.is_some(),
        focus_index,
        states: state::of(element),
        path,
        children,
    }
}
