//! Supported media-query condition evaluation.

use crate::browser_agent::{ColorScheme, ForcedColors, MediaEmulation, ReducedMotion};

pub(crate) fn matches(query: &str, width: i64, media: MediaEmulation) -> bool {
    query.split(',').any(|part| {
        part.split("and")
            .all(|item| matches_one(item, width, media))
    })
}

fn matches_one(raw: &str, width: i64, media: MediaEmulation) -> bool {
    let item = raw
        .trim()
        .trim_start_matches('(')
        .trim_end_matches(')')
        .trim();
    if matches!(item, "" | "all" | "screen") {
        return true;
    }
    let Some((name, value)) = item.split_once(':') else {
        return false;
    };
    match name.trim() {
        "min-width" => px(value).is_some_and(|limit| width >= limit),
        "max-width" => px(value).is_some_and(|limit| width <= limit),
        "prefers-color-scheme" => color_scheme(value, media.color_scheme),
        "prefers-reduced-motion" => reduced_motion(value, media.reduced_motion),
        "forced-colors" => forced_colors(value, media.forced_colors),
        _ => false,
    }
}

fn px(value: &str) -> Option<i64> {
    value.trim().trim_end_matches("px").trim().parse().ok()
}

fn color_scheme(value: &str, current: ColorScheme) -> bool {
    matches!(
        (value.trim(), current),
        ("dark", ColorScheme::Dark) | ("light", ColorScheme::Light)
    )
}

fn reduced_motion(value: &str, current: ReducedMotion) -> bool {
    matches!(
        (value.trim(), current),
        ("reduce", ReducedMotion::Reduce) | ("no-preference", ReducedMotion::NoPreference)
    )
}

fn forced_colors(value: &str, current: ForcedColors) -> bool {
    matches!(
        (value.trim(), current),
        ("active", ForcedColors::Active) | ("none", ForcedColors::None)
    )
}
