//! Cross-axis item and line alignment.

use super::container::{AlignContent, AlignItems, FlexContainer};
use super::item::FlexItem;
use super::resolve::{clamp_cross, item_cross_size};
use super::wrap::FlexLine;

pub fn line_cross_positions(
    lines: &[FlexLine],
    available_cross: f32,
    gap: f32,
    align: AlignContent,
) -> Vec<f32> {
    let n = lines.len();
    let used: f32 =
        lines.iter().map(|l| l.cross_size).sum::<f32>() + gap * n.saturating_sub(1) as f32;
    let free = (available_cross - used).max(0.0);

    let (start, between) = match align {
        AlignContent::FlexEnd => (free, gap),
        AlignContent::Center => (free / 2.0, gap),
        AlignContent::SpaceBetween if n > 1 => (0.0, gap + free / (n - 1) as f32),
        AlignContent::SpaceAround => {
            let s = free / n.max(1) as f32;
            (s / 2.0, gap + s)
        }
        AlignContent::SpaceEvenly => {
            let s = free / (n + 1) as f32;
            (s, gap + s)
        }
        _ => (0.0, gap),
    };

    let mut p = start;
    lines
        .iter()
        .map(|l| {
            let out = p;
            p += l.cross_size + between;
            out
        })
        .collect()
}

pub fn align_item(c: &FlexContainer, item: &FlexItem, line_cross: f32) -> (f32, f32) {
    let align = item.align_self.unwrap_or(c.align_items);
    let intrinsic = clamp_cross(c, item, item_cross_size(c, item));

    match align {
        AlignItems::Stretch => (0.0, line_cross),
        AlignItems::FlexEnd => (line_cross - intrinsic, intrinsic),
        AlignItems::Center => ((line_cross - intrinsic) / 2.0, intrinsic),
        AlignItems::Baseline | AlignItems::FlexStart => (0.0, intrinsic),
    }
}
