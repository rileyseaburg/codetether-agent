//! Flex basis and constraint resolution.

use super::container::FlexContainer;
use super::item::{FlexBasis, FlexItem};

pub fn main_size(c: &FlexContainer, w: f32, h: f32) -> f32 {
    if c.is_row() {
        w
    } else {
        h
    }
}

pub fn cross_size(c: &FlexContainer, w: f32, h: f32) -> f32 {
    if c.is_row() {
        h
    } else {
        w
    }
}

pub fn base_size(c: &FlexContainer, item: &FlexItem) -> f32 {
    match item.flex_basis {
        FlexBasis::Auto => main_size(c, item.desired_size.width, item.desired_size.height).max(
            main_size(c, item.content_size.width, item.content_size.height),
        ),
        FlexBasis::Zero => 0.0,
        FlexBasis::Points(v) => v.max(0.0),
    }
}

pub fn item_cross_size(c: &FlexContainer, item: &FlexItem) -> f32 {
    cross_size(c, item.desired_size.width, item.desired_size.height).max(cross_size(
        c,
        item.content_size.width,
        item.content_size.height,
    ))
}

pub fn clamp_main(c: &FlexContainer, item: &FlexItem, v: f32) -> f32 {
    if c.is_row() {
        clamp(v, item.min_width, item.max_width)
    } else {
        clamp(v, item.min_height, item.max_height)
    }
}

pub fn clamp_cross(c: &FlexContainer, item: &FlexItem, v: f32) -> f32 {
    if c.is_row() {
        clamp(v, item.min_height, item.max_height)
    } else {
        clamp(v, item.min_width, item.max_width)
    }
}

fn clamp(v: f32, min: Option<f32>, max: Option<f32>) -> f32 {
    let v = min.map_or(v, |m| v.max(m));
    max.map_or(v, |m| v.min(m))
}
