//! Original design_review_is_read_only test.
use super::*;

#[test]
fn design_review_is_read_only() {
    assert_eq!(
        classify("chunk-design-review", "Design review", None),
        TaskKind::ReadOnly
    );
}
