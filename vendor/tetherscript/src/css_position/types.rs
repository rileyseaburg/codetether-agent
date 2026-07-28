//! Shared positioning data types.

mod element_methods;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PositionType {
    Static,
    Relative,
    Absolute,
    Fixed,
    Sticky,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Edges {
    pub top: Option<f32>,
    pub right: Option<f32>,
    pub bottom: Option<f32>,
    pub left: Option<f32>,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct BoxEdges {
    pub top: f32,
    pub right: f32,
    pub bottom: f32,
    pub left: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct Size {
    pub width: f32,
    pub height: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub struct PositionedElement<E = usize> {
    pub element: E,
    pub parent: Option<usize>,
    pub position: PositionType,
    pub offsets: Edges,
    pub z_index: Option<i32>,
    pub normal_x: f32,
    pub normal_y: f32,
    pub width: f32,
    pub height: f32,
    pub computed_x: f32,
    pub computed_y: f32,
    pub margin: BoxEdges,
    pub padding: BoxEdges,
    pub border: BoxEdges,
}
