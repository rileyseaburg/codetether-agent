//! Element similarity scoring.

mod helpers;
mod props;

use helpers::{class_overlap, opt_eq, text_eq};
pub use props::ElementProps;

/// Compute a similarity score between 0.0 and 1.0 for two elements.
pub fn element_similarity(a: &ElementProps, b: &ElementProps) -> f64 {
    let mut s: f64 = 0.0;
    if helpers::clean(&a.tag) == helpers::clean(&b.tag) {
        s += 0.20;
    }
    if opt_eq(&a.id, &b.id) {
        s += 0.30;
    }
    s += class_overlap(&a.classes, &b.classes) * 0.20;
    if text_eq(&a.text, &b.text) || text_eq(&a.label, &b.label) {
        s += 0.15;
    }
    if opt_eq(&a.role, &b.role) {
        s += 0.10;
    }
    if a.position_hint.is_some() && a.position_hint == b.position_hint {
        s += 0.05;
    }
    s.min(1.0)
}
