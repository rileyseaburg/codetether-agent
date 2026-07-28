//! Main-axis positioning using justify-content.

use super::{resolve, types::*};

/// Position items along the main axis according to justify-content.
pub fn position_line(
    items: &mut [FlexItem],
    row: bool,
    container_main: i64,
    jc: JustifyContent,
    reverse: bool,
) {
    let used: i64 = items.iter().map(|i| resolve::main_size(i, row)).sum();
    let free = (container_main - used).max(0);
    let n = items.len() as i64;
    if n == 0 {
        return;
    }
    let (mut pos, gap) = match jc {
        JustifyContent::FlexEnd => (free, 0),
        JustifyContent::Center => (free / 2, 0),
        JustifyContent::SpaceBetween if n > 1 => (0, free / (n - 1)),
        JustifyContent::SpaceAround => (free / (2 * n), free / n),
        JustifyContent::SpaceEvenly => (free / (n + 1), free / (n + 1)),
        _ => (0, 0),
    };
    let mut order: Vec<usize> = (0..items.len()).collect();
    if reverse {
        order.reverse();
    }
    for idx in order {
        let sz = resolve::main_size(&items[idx], row);
        resolve::set_main_pos(&mut items[idx], row, pos);
        pos += sz + gap;
    }
}
