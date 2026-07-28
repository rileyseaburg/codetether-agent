//! Flex line creation.

use super::container::{FlexContainer, FlexWrap};
use super::item::FlexItem;
use super::resolve::{base_size, clamp_main, item_cross_size};

#[derive(Clone, Debug)]
pub struct LineItem {
    pub index: usize,
    pub base: f32,
    pub target: f32,
}

#[derive(Clone, Debug, Default)]
pub struct FlexLine {
    pub items: Vec<LineItem>,
    pub main_size: f32,
    pub cross_size: f32,
}

pub fn create_lines(
    c: &FlexContainer,
    items: &[FlexItem],
    available_main: f32,
    main_gap: f32,
) -> Vec<FlexLine> {
    let mut ordered: Vec<_> = items.iter().enumerate().collect();
    ordered.sort_by_key(|(_, i)| i.order);

    let mut lines = vec![FlexLine::default()];
    for (idx, item) in ordered {
        let b = clamp_main(c, item, base_size(c, item));
        let add_gap = if lines.last().unwrap().items.is_empty() {
            0.0
        } else {
            main_gap
        };
        let would = lines.last().unwrap().main_size + add_gap + b;
        let should_wrap = c.wrap != FlexWrap::NoWrap
            && !lines.last().unwrap().items.is_empty()
            && would > available_main;

        if should_wrap {
            lines.push(FlexLine::default());
        }

        let line = lines.last_mut().unwrap();
        if !line.items.is_empty() {
            line.main_size += main_gap;
        }
        line.main_size += b;
        line.cross_size = line.cross_size.max(item_cross_size(c, item));
        line.items.push(LineItem {
            index: idx,
            base: b,
            target: b,
        });
    }

    if matches!(c.wrap, FlexWrap::WrapReverse) {
        lines.reverse();
    }
    lines
}
