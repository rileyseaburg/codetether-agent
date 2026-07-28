//! Main positioned-layout algorithm.
use super::absolute::AbsolutePositioner;
use super::containing::{ContainingBlock, ContainingBlockResolver};
use super::fixed::FixedPositioner;
use super::relative::RelativePositioner;
use super::sticky::StickyOffsetResolver;
use super::types::{PositionType, PositionedElement};
use super::z_index::{PaintRecord, ZIndexResolver};

#[derive(Clone, Debug)]
pub struct PositionedLayout<E = usize> {
    pub elements: Vec<PositionedElement<E>>,
    pub viewport: ContainingBlock,
    pub scroll_x: f32,
    pub scroll_y: f32,
}

impl<E> PositionedLayout<E> {
    pub fn new(
        elements: Vec<PositionedElement<E>>,
        viewport_width: f32,
        viewport_height: f32,
    ) -> Self {
        Self {
            elements,
            viewport: ContainingBlock::viewport(viewport_width, viewport_height),
            scroll_x: 0.0,
            scroll_y: 0.0,
        }
    }

    pub fn compute(&mut self) {
        for el in &mut self.elements {
            el.computed_x = el.normal_x;
            el.computed_y = el.normal_y;
        }
        for i in 0..self.elements.len() {
            let cb = ContainingBlockResolver::resolve(&self.elements, i, self.viewport);
            match self.elements[i].position {
                PositionType::Static => {}
                PositionType::Relative => RelativePositioner::compute(&mut self.elements[i]),
                PositionType::Absolute => AbsolutePositioner::compute(&mut self.elements[i], cb),
                PositionType::Fixed => {
                    FixedPositioner::compute(&mut self.elements[i], self.viewport)
                }
                PositionType::Sticky => StickyOffsetResolver::compute(
                    &mut self.elements[i],
                    cb,
                    self.viewport,
                    self.scroll_x,
                    self.scroll_y,
                ),
            }
        }
    }

    pub fn paint_order(&self) -> Vec<PaintRecord> {
        ZIndexResolver::paint_order(&self.elements)
    }
}
