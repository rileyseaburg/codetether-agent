//! Positioned element methods.

use super::{PositionType, PositionedElement, Rect};

impl<E> PositionedElement<E> {
    pub fn rect(&self) -> Rect {
        Rect {
            x: self.computed_x,
            y: self.computed_y,
            width: self.width,
            height: self.height,
        }
    }

    pub fn is_positioned(&self) -> bool {
        self.position != PositionType::Static
    }

    pub fn effective_z_index(&self) -> i32 {
        self.z_index.unwrap_or(0)
    }
}
