//! Flex item collection from style maps.

use super::super::{parse, types::*};
use super::is_row;
use std::collections::HashMap;

pub fn collect(
    styles: &[HashMap<String, String>],
    sizes: &[(i64, i64)],
    dir: FlexDirection,
) -> Vec<FlexItem> {
    let row = is_row(dir);
    styles
        .iter()
        .enumerate()
        .map(|(i, st)| item(st, sizes.get(i).copied().unwrap_or((0, 0)), row))
        .collect()
}

fn item(st: &HashMap<String, String>, size: (i64, i64), row: bool) -> FlexItem {
    let (w, h) = size;
    let basis = parse::len(st, "flex-basis").or_else(|| {
        if row {
            parse::len(st, "width")
        } else {
            parse::len(st, "height")
        }
    });
    FlexItem {
        flex_grow: parse::num(st, "flex-grow", 0.0),
        flex_shrink: parse::num(st, "flex-shrink", 1.0),
        flex_basis: basis,
        min_size: parse::len(st, if row { "min-width" } else { "min-height" }).unwrap_or(0),
        max_size: parse::len(st, if row { "max-width" } else { "max-height" })
            .unwrap_or(i64::MAX / 4),
        x: 0,
        y: 0,
        width: w,
        height: h,
    }
}
