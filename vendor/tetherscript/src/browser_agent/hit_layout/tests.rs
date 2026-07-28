//! Hit testing unit tests.

use super::{hit_test_layout, point_in_box};
use crate::browser::LayoutBox;
use std::collections::HashMap;

#[test]
fn point_inside_box() {
    assert!(point_in_box(5, 5, 0, 0, 10, 10));
    assert!(point_in_box(0, 0, 0, 0, 10, 10));
    assert!(!point_in_box(10, 5, 0, 0, 10, 10));
}

#[test]
fn nested_layout_hit_returns_deepest() {
    let child = LayoutBox {
        kind: "block".into(),
        tag: Some("button".into()),
        text: Some("Click".into()),
        x: 10,
        y: 10,
        width: 50,
        height: 30,
        styles: HashMap::new(),
        children: Vec::new(),
    };
    let root = LayoutBox {
        kind: "viewport".into(),
        tag: Some("body".into()),
        text: None,
        x: 0,
        y: 0,
        width: 100,
        height: 100,
        styles: HashMap::new(),
        children: vec![child],
    };
    let hit = hit_test_layout(&root, 20, 20).unwrap();
    assert_eq!(hit.tag.as_deref(), Some("button"));
    assert_eq!(hit.text.as_deref(), Some("Click"));
    assert_eq!(hit.path, vec![0]);
}
