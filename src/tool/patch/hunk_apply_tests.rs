//! Empty-range insertions respect unified-diff positions and fail closed.

use super::{PatchHunk, apply};

fn insertion(at: usize) -> PatchHunk {
    PatchHunk {
        file: "file.rs".into(),
        start_line: at,
        old_lines: Vec::new(),
        new_lines: vec!["inserted".into()],
    }
}

#[test]
fn empty_range_can_insert_at_beginning_middle_and_end() {
    let content = "first\nsecond";
    assert_eq!(
        apply(content, &insertion(0)).unwrap(),
        "inserted\nfirst\nsecond"
    );
    assert_eq!(
        apply(content, &insertion(1)).unwrap(),
        "first\ninserted\nsecond"
    );
    assert_eq!(
        apply(content, &insertion(2)).unwrap(),
        "first\nsecond\ninserted"
    );
}

#[test]
fn empty_range_outside_file_is_rejected_not_prepended() {
    assert!(apply("first\nsecond", &insertion(3)).is_err());
    assert_eq!(apply("", &insertion(0)).unwrap(), "inserted");
}
