//! Numeric conversion helpers for WebGL calls.

use super::*;

pub(super) fn i64_quad(args: &[JsValue]) -> [i64; 4] {
    [
        i64_value(args.first()),
        i64_value(args.get(1)),
        i64_value(args.get(2)),
        i64_value(args.get(3)),
    ]
}

pub(super) fn f64_quad(args: &[JsValue]) -> [f64; 4] {
    [
        unit_value(args.first()),
        unit_value(args.get(1)),
        unit_value(args.get(2)),
        unit_value(args.get(3)),
    ]
}

pub(super) fn i64_value(value: Option<&JsValue>) -> i64 {
    match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() => *n as i64,
        other => other.display().parse().unwrap_or(0),
    }
}

fn unit_value(value: Option<&JsValue>) -> f64 {
    let raw = match value.unwrap_or(&JsValue::Undefined) {
        JsValue::Number(n) if n.is_finite() => *n,
        other => other.display().parse().unwrap_or(0.0),
    };
    raw.clamp(0.0, 1.0)
}
