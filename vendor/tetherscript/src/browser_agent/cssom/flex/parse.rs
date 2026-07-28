//! Parse flex properties from CSS style maps.

use super::types::*;
mod value;
use std::collections::HashMap;
use value::val;

pub fn direction(s: &HashMap<String, String>) -> FlexDirection {
    match val(s, "flex-direction").as_deref() {
        Some("row-reverse") => FlexDirection::RowReverse,
        Some("column") => FlexDirection::Column,
        Some("column-reverse") => FlexDirection::ColumnReverse,
        _ => FlexDirection::Row,
    }
}

pub fn justify(s: &HashMap<String, String>) -> JustifyContent {
    match val(s, "justify-content").as_deref() {
        Some("flex-end") => JustifyContent::FlexEnd,
        Some("center") => JustifyContent::Center,
        Some("space-between") => JustifyContent::SpaceBetween,
        Some("space-around") => JustifyContent::SpaceAround,
        Some("space-evenly") => JustifyContent::SpaceEvenly,
        _ => JustifyContent::FlexStart,
    }
}

pub fn align_items(s: &HashMap<String, String>) -> AlignItems {
    match val(s, "align-items").as_deref() {
        Some("flex-start") => AlignItems::FlexStart,
        Some("flex-end") => AlignItems::FlexEnd,
        Some("center") => AlignItems::Center,
        Some("baseline") => AlignItems::Baseline,
        _ => AlignItems::Stretch,
    }
}

pub fn wrap(s: &HashMap<String, String>) -> FlexWrap {
    match val(s, "flex-wrap").as_deref() {
        Some("wrap") => FlexWrap::Wrap,
        Some("wrap-reverse") => FlexWrap::WrapReverse,
        _ => FlexWrap::NoWrap,
    }
}

pub fn num(s: &HashMap<String, String>, key: &str, default: f64) -> f64 {
    val(s, key)
        .and_then(|v| v.parse::<f64>().ok())
        .unwrap_or(default)
}

pub fn len(s: &HashMap<String, String>, key: &str) -> Option<i64> {
    val(s, key).and_then(|v| value::parse_len(&v))
}
