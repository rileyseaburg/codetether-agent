//! Flex item style and output geometry.

use super::container::AlignItems;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum FlexBasis {
    Auto,
    Zero,
    Points(f32),
}

#[derive(Clone, Debug, PartialEq)]
pub struct FlexItem {
    pub flex_grow: f32,
    pub flex_shrink: f32,
    pub flex_basis: FlexBasis,
    pub align_self: Option<AlignItems>,
    pub min_width: Option<f32>,
    pub max_width: Option<f32>,
    pub min_height: Option<f32>,
    pub max_height: Option<f32>,
    pub order: i32,
    pub content_size: Size,
    pub desired_size: Size,
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct PositionedFlexItem {
    pub index: usize,
    pub rect: Rect,
}

impl Default for FlexItem {
    fn default() -> Self {
        Self {
            flex_grow: 0.0,
            flex_shrink: 1.0,
            flex_basis: FlexBasis::Auto,
            align_self: None,
            min_width: None,
            max_width: None,
            min_height: None,
            max_height: None,
            order: 0,
            content_size: Size::default(),
            desired_size: Size::default(),
        }
    }
}
