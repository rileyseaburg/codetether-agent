//! Search for the deepest layout box at a given point.

use super::point::point_in_box;
use super::result::HitResult;
use crate::browser::LayoutBox;

/// Find the deepest LayoutBox at the given coordinates.
pub fn hit_test_layout(layout: &LayoutBox, x: i64, y: i64) -> Option<HitResult> {
    let mut path = Vec::new();
    hit_inner(layout, x, y, &mut path)
}

fn hit_inner(layout: &LayoutBox, x: i64, y: i64, path: &mut Vec<usize>) -> Option<HitResult> {
    if !point_in_box(x, y, layout.x, layout.y, layout.width, layout.height) {
        return None;
    }
    for index in (0..layout.children.len()).rev() {
        path.push(index);
        if let Some(hit) = hit_inner(&layout.children[index], x, y, path) {
            return Some(hit);
        }
        path.pop();
    }
    Some(HitResult {
        x,
        y,
        tag: layout.tag.clone(),
        text: layout.text.clone(),
        path: path.clone(),
    })
}
