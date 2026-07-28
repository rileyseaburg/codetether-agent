#![allow(dead_code)]
//! Flexbox layout engine for the tetherscript browser.

pub mod algorithm;
pub mod align;
pub mod container;
pub mod distribute;
pub mod item;
pub mod position;
pub mod resolve;
pub mod wrap;

pub use algorithm::{layout_flex, FlexConstraints};
pub use container::{
    AlignContent, AlignItems, FlexContainer, FlexDirection, FlexWrap, Gap, JustifyContent,
};
pub use item::{FlexBasis, FlexItem, PositionedFlexItem, Rect, Size};

#[cfg(test)]
mod tests;
