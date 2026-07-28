#![allow(dead_code)]
//! CSS positioning support for the tetherscript browser.

pub mod absolute;
pub mod compute;
pub mod containing;
pub mod fixed;
pub mod relative;
pub mod sticky;
pub mod types;
pub mod z_index;

#[cfg(test)]
mod tests;

pub use absolute::AbsolutePositioner;
pub use compute::PositionedLayout;
pub use containing::{ContainingBlock, ContainingBlockResolver};
pub use fixed::FixedPositioner;
pub use relative::RelativePositioner;
pub use sticky::StickyOffsetResolver;
pub use types::{BoxEdges, Edges, PositionType, PositionedElement, Rect, Size};
pub use z_index::{PaintPhase, PaintRecord, ZIndexResolver};
