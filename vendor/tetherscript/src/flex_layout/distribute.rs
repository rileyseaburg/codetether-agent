//! Main-axis grow and shrink distribution.

use super::container::FlexContainer;
use super::item::FlexItem;
use super::resolve::clamp_main;
use super::wrap::FlexLine;

pub fn distribute_line(
    c: &FlexContainer,
    line: &mut FlexLine,
    items: &[FlexItem],
    available_main: f32,
    main_gap: f32,
) {
    let gaps = main_gap * line.items.len().saturating_sub(1) as f32;
    let base_sum: f32 = line.items.iter().map(|i| i.base).sum();
    let free = available_main - gaps - base_sum;

    if free > 0.0 {
        let total: f32 = line
            .items
            .iter()
            .map(|i| items[i.index].flex_grow.max(0.0))
            .sum();
        if total > 0.0 {
            for li in &mut line.items {
                let item = &items[li.index];
                li.target = clamp_main(c, item, li.base + free * item.flex_grow.max(0.0) / total);
            }
        }
    } else if free < 0.0 {
        let total: f32 = line
            .items
            .iter()
            .map(|i| items[i.index].flex_shrink.max(0.0) * i.base)
            .sum();
        if total > 0.0 {
            for li in &mut line.items {
                let item = &items[li.index];
                let w = item.flex_shrink.max(0.0) * li.base;
                li.target = clamp_main(c, item, li.base + free * w / total);
            }
        }
    }

    line.main_size = line.items.iter().map(|i| i.target).sum::<f32>() + gaps;
}
