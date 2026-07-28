//! Intersection ratio computation.

use super::intersection_types::IntersectionEntry;

pub type Rect = (f64, f64, f64, f64);

/// Compute the intersection ratio between a root and target rect.
pub fn intersection_ratio(root: Rect, target: Rect) -> f64 {
    let (x1, y1) = (root.0.max(target.0), root.1.max(target.1));
    let (x2, y2) = (
        (root.0 + root.2).min(target.0 + target.2),
        (root.1 + root.3).min(target.1 + target.3),
    );
    let iw = (x2 - x1).max(0.0);
    let ih = (y2 - y1).max(0.0);
    let area = target.2.max(0.0) * target.3.max(0.0);
    if area <= 0.0 {
        0.0
    } else {
        (iw * ih / area).clamp(0.0, 1.0)
    }
}

/// Build an intersection entry for a single target.
pub fn compute_entry(id: u64, root: Rect, target: Rect) -> IntersectionEntry {
    let ratio = intersection_ratio(root, target);
    IntersectionEntry {
        target_node_id: id,
        intersection_ratio: ratio,
        is_intersecting: ratio > 0.0,
        bounds: target,
    }
}
