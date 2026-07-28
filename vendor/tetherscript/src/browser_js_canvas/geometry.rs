//! Canvas coordinate helpers.

use super::*;

pub(super) fn rect(args: &[JsValue]) -> (i64, i64, i64, i64) {
    (
        i64_value(args.first()),
        i64_value(args.get(1)),
        i64_value(args.get(2)),
        i64_value(args.get(3)),
    )
}

pub(super) fn clip(
    (x, y, w, h): (i64, i64, i64, i64),
    width: u32,
    height: u32,
) -> Option<(usize, usize, usize, usize)> {
    if w <= 0 || h <= 0 {
        return None;
    }
    let x0 = x.max(0).min(width as i64) as usize;
    let y0 = y.max(0).min(height as i64) as usize;
    let x1 = (x + w).max(0).min(width as i64) as usize;
    let y1 = (y + h).max(0).min(height as i64) as usize;
    (x1 > x0 && y1 > y0).then_some((x0, y0, x1, y1))
}

fn i64_value(value: Option<&JsValue>) -> i64 {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() => *n as i64,
        other => other.display().parse().unwrap_or(0),
    }
}
