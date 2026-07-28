//! Cross-axis alignment using align-items and align-self.

use super::{parse, resolve, types::*};
use std::collections::HashMap;

/// Align items on the cross axis within a single flex line.
pub fn align_line(
    items: &mut [FlexItem],
    styles: &[HashMap<String, String>],
    row: bool,
    cross: i64,
    default: AlignItems,
    line_off: i64,
) {
    for (i, it) in items.iter_mut().enumerate() {
        let a = styles
            .get(i)
            .and_then(|s| s.get("align-self"))
            .map(|v| match v.as_str() {
                "flex-start" => AlignItems::FlexStart,
                "flex-end" => AlignItems::FlexEnd,
                "center" => AlignItems::Center,
                "baseline" => AlignItems::Baseline,
                "stretch" => AlignItems::Stretch,
                _ => default,
            })
            .unwrap_or(default);
        let sz = resolve::cross_size(it, row);
        let pos = match a {
            AlignItems::FlexEnd => cross - sz,
            AlignItems::Center => (cross - sz) / 2,
            AlignItems::Stretch => {
                if sz == 0 {
                    resolve::set_cross(it, row, cross);
                }
                0
            }
            _ => 0,
        };
        resolve::set_cross_pos(it, row, line_off + pos.max(0));
    }
}

/// Resolve the container cross-axis size from styles.
pub fn container_cross(styles: &HashMap<String, String>, row: bool, fallback: i64) -> i64 {
    parse::len(styles, if row { "height" } else { "width" }).unwrap_or(fallback)
}
