//! Accessibility-aware focus-order collection.

#[path = "focus_native.rs"]
mod native;
#[path = "focus_walk.rs"]
mod walk;

use crate::browser::{Document, Element};

#[derive(Clone)]
pub(super) struct FocusEntry {
    pub path: Vec<usize>,
    pub selector: String,
}

pub(super) fn order(document: &Document) -> Vec<FocusEntry> {
    let mut items = walk::collect(document);
    items.sort_by_key(|(dom, tab, _, _)| sort_key(*dom, *tab));
    items.into_iter().map(entry).collect()
}

pub(super) fn focusable(element: &Element) -> bool {
    if native::disabled(element) {
        return false;
    }
    match tab_index_attr(element) {
        Some(index) => index >= 0,
        None => native::focusable(element),
    }
}

pub(super) fn tab_index(element: &Element) -> i32 {
    tab_index_attr(element).unwrap_or(0)
}

fn tab_index_attr(element: &Element) -> Option<i32> {
    element
        .attrs
        .get("tabindex")
        .and_then(|value| value.parse().ok())
}

fn sort_key(dom: usize, tab: i32) -> (i32, i32, usize) {
    if tab > 0 {
        (0, tab, dom)
    } else {
        (1, 0, dom)
    }
}

fn entry(raw: (usize, i32, Vec<usize>, String)) -> FocusEntry {
    FocusEntry {
        path: raw.2,
        selector: raw.3,
    }
}
