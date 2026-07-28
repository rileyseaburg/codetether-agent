//! Layout bounds and visibility helpers for visual evidence.

use std::collections::BTreeMap;

use crate::browser::LayoutBox;
use crate::browser_agent::BoundingBox;

pub fn bounds_for(layout: &LayoutBox) -> BoundingBox {
    BoundingBox {
        x: layout.x,
        y: layout.y,
        width: layout.width,
        height: layout.height,
    }
}

pub fn visible(layout: &LayoutBox, styles: &BTreeMap<String, String>, bounds: BoundingBox) -> bool {
    bounds.width > 0
        && bounds.height > 0
        && layout.kind != "none"
        && !matches_value(styles, "display", "none")
        && !matches_hidden(styles)
}

fn matches_hidden(styles: &BTreeMap<String, String>) -> bool {
    styles
        .get("visibility")
        .is_some_and(|value| matches!(value.trim(), "hidden" | "collapse"))
}

fn matches_value(styles: &BTreeMap<String, String>, name: &str, expected: &str) -> bool {
    styles
        .get(name)
        .is_some_and(|value| value.eq_ignore_ascii_case(expected))
}
