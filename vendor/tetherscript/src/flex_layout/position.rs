//! Main-axis positioning via justify-content.

use super::container::JustifyContent;
use super::wrap::FlexLine;

pub fn main_positions(
    line: &FlexLine,
    available_main: f32,
    gap: f32,
    justify: JustifyContent,
) -> Vec<f32> {
    let n = line.items.len();
    if n == 0 {
        return vec![];
    }

    let used_items: f32 = line.items.iter().map(|i| i.target).sum();
    let normal_gaps = gap * n.saturating_sub(1) as f32;
    let free = (available_main - used_items - normal_gaps).max(0.0);

    let (start, between) = match justify {
        JustifyContent::FlexStart => (0.0, gap),
        JustifyContent::FlexEnd => (free, gap),
        JustifyContent::Center => (free / 2.0, gap),
        JustifyContent::SpaceBetween if n > 1 => (0.0, gap + free / (n - 1) as f32),
        JustifyContent::SpaceAround => {
            let s = free / n as f32;
            (s / 2.0, gap + s)
        }
        JustifyContent::SpaceEvenly => {
            let s = free / (n + 1) as f32;
            (s, gap + s)
        }
        _ => (0.0, gap),
    };

    let mut x = start;
    line.items
        .iter()
        .map(|i| {
            let p = x;
            x += i.target + between;
            p
        })
        .collect()
}
