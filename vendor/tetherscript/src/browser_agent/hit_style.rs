//! Style helpers for action hit testing.

use crate::browser::LayoutBox;
use crate::browser_agent::action::BoundingBox;

pub(crate) fn bounds_for(layout: &LayoutBox) -> BoundingBox {
    BoundingBox {
        x: layout.x,
        y: layout.y,
        width: layout.width,
        height: layout.height,
    }
}

pub(crate) fn pointer_enabled(layout: &LayoutBox) -> bool {
    visible(layout)
        && !layout
            .styles
            .get("pointer-events")
            .is_some_and(|value| value.eq_ignore_ascii_case("none"))
}

fn visible(layout: &LayoutBox) -> bool {
    !layout
        .styles
        .get("visibility")
        .is_some_and(|value| matches_hidden(value))
}

fn matches_hidden(value: &str) -> bool {
    matches!(
        value.trim().to_ascii_lowercase().as_str(),
        "hidden" | "collapse"
    )
}

pub(crate) fn z_index(layout: &LayoutBox) -> i64 {
    px(layout.styles.get("z-index")).unwrap_or(0)
}

fn px(value: Option<&String>) -> Option<i64> {
    value?
        .trim()
        .strip_suffix("px")
        .unwrap_or(value?.trim())
        .parse()
        .ok()
}
