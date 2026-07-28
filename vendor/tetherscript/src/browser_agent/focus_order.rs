//! Tab-order collection for focusable DOM elements.

use super::path;
use super::FocusTarget;
use crate::browser::{collect_focusable, Document, Element};

pub(crate) fn collect(document: &Document) -> Vec<FocusTarget> {
    let mut items = collect_focusable(document)
        .into_iter()
        .enumerate()
        .filter(|(_, (_, element))| !element.attrs.contains_key("disabled"))
        .map(|(index, (path, element))| (index, tab_index(&element), path, element))
        .collect::<Vec<_>>();
    items.sort_by_key(|(index, tab, _, _)| {
        if *tab > 0 {
            (0, *tab, *index)
        } else {
            (1, 0, *index)
        }
    });
    items
        .into_iter()
        .map(|(_, _, path, element)| target(path, element))
        .collect()
}

fn target(path: Vec<usize>, element: Element) -> FocusTarget {
    FocusTarget {
        selector: path::selector_for(&path, &element),
        tag: element.tag,
        path,
    }
}

fn tab_index(element: &Element) -> i32 {
    element
        .attrs
        .get("tabindex")
        .and_then(|value| value.parse::<i32>().ok())
        .unwrap_or(0)
}
