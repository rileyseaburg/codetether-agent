use super::*;

pub(super) fn matches(layout: &browser::LayoutBox, x: i64, y: i64) -> bool {
    layout.width > 0
        && layout.height > 0
        && pointer_enabled(layout)
        && x >= layout.x
        && y >= layout.y
        && x < layout.x + layout.width
        && y < layout.y + layout.height
}

pub(super) fn z_index(layout: &browser::LayoutBox) -> i64 {
    px(layout.styles.get("z-index")).unwrap_or(0)
}

fn pointer_enabled(layout: &browser::LayoutBox) -> bool {
    visible(layout)
        && !layout
            .styles
            .get("pointer-events")
            .is_some_and(|value| value.eq_ignore_ascii_case("none"))
}

fn visible(layout: &browser::LayoutBox) -> bool {
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

fn px(value: Option<&String>) -> Option<i64> {
    let value = value?.trim();
    value
        .strip_suffix("px")
        .unwrap_or(value)
        .trim()
        .parse()
        .ok()
}
