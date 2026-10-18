//! Tests for task-list checkboxes and depth-aware bullets.

use super::tasks::{bullet_for_depth, task_item};
use super::{Block, classify};

#[test]
fn detects_unchecked_and_checked_tasks() {
    assert_eq!(task_item("- [ ] do it"), Some((false, "do it".to_string())));
    assert_eq!(task_item("- [x] done"), Some((true, "done".to_string())));
    assert_eq!(task_item("- [X] done"), Some((true, "done".to_string())));
    assert_eq!(task_item("- plain"), None);
}

#[test]
fn bullets_vary_by_depth() {
    assert_eq!(bullet_for_depth(0), "•");
    assert_eq!(bullet_for_depth(2), "◦");
    assert_eq!(bullet_for_depth(4), "▪");
}

#[test]
fn classify_renders_checked_task_styled() {
    match classify("- [x] shipped") {
        Block::Styled(spans) => {
            let joined: String = spans.iter().map(|s| s.content.as_ref()).collect();
            assert!(joined.contains('☑') && joined.contains("shipped"));
        }
        _ => panic!("expected styled checked task"),
    }
}

#[test]
fn classify_nested_bullet_uses_depth_glyph() {
    match classify("  - nested") {
        Block::Prefixed { prefix, .. } => {
            let p: String = prefix.iter().map(|s| s.content.as_ref()).collect();
            assert!(p.contains('◦'));
        }
        _ => panic!("expected prefixed list item"),
    }
}
