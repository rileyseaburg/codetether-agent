//! Flex axis accessors.

use super::super::types::{FlexDirection, FlexItem};

pub fn is_row(d: FlexDirection) -> bool {
    matches!(d, FlexDirection::Row | FlexDirection::RowReverse)
}

pub fn is_reverse(d: FlexDirection) -> bool {
    matches!(d, FlexDirection::RowReverse | FlexDirection::ColumnReverse)
}

pub fn main_size(it: &FlexItem, row: bool) -> i64 {
    if row {
        it.width
    } else {
        it.height
    }
}

pub fn cross_size(it: &FlexItem, row: bool) -> i64 {
    if row {
        it.height
    } else {
        it.width
    }
}

pub fn set_main(it: &mut FlexItem, row: bool, v: i64) {
    if row {
        it.width = v
    } else {
        it.height = v
    }
}

pub fn set_cross(it: &mut FlexItem, row: bool, v: i64) {
    if row {
        it.height = v
    } else {
        it.width = v
    }
}

pub fn set_main_pos(it: &mut FlexItem, row: bool, v: i64) {
    if row {
        it.x = v
    } else {
        it.y = v
    }
}

pub fn set_cross_pos(it: &mut FlexItem, row: bool, v: i64) {
    if row {
        it.y = v
    } else {
        it.x = v
    }
}
