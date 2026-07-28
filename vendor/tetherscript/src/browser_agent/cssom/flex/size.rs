//! Flex grow/shrink sizing algorithm.

use super::{resolve, types::FlexItem};

/// Distribute free space (grow) or reclaim space (shrink) along the main axis.
pub fn size_line(items: &mut [FlexItem], row: bool, container_main: i64) {
    let mut base = Vec::with_capacity(items.len());
    let mut total = 0;
    for it in items.iter_mut() {
        let b = it
            .flex_basis
            .unwrap_or_else(|| resolve::main_size(it, row))
            .max(it.min_size)
            .min(it.max_size);
        resolve::set_main(it, row, b);
        base.push(b);
        total += b;
    }
    let free = container_main - total;
    if free > 0 {
        let grow: f64 = items.iter().map(|i| i.flex_grow).sum();
        if grow > 0.0 {
            for it in items {
                let add = ((free as f64) * it.flex_grow / grow).round() as i64;
                resolve::set_main(
                    it,
                    row,
                    (resolve::main_size(it, row) + add).min(it.max_size),
                );
            }
        }
    } else if free < 0 {
        let scaled: f64 = items
            .iter()
            .zip(base.iter())
            .map(|(i, b)| i.flex_shrink * (*b as f64))
            .sum();
        if scaled > 0.0 {
            for (it, b) in items.iter_mut().zip(base) {
                let cut = ((-free as f64) * it.flex_shrink * (b as f64) / scaled).round() as i64;
                resolve::set_main(it, row, (b - cut).max(it.min_size));
            }
        }
    }
}
