//! Tests for line-level diff classification.

use super::line_diff::{LineKind, classify};

#[test]
fn pure_addition_is_insert() {
    let lines = classify("", "new line\n");
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].kind, LineKind::Insert);
    assert_eq!(lines[0].text, "new line");
}

#[test]
fn pure_deletion_is_delete() {
    let lines = classify("gone\n", "");
    assert_eq!(lines.len(), 1);
    assert_eq!(lines[0].kind, LineKind::Delete);
}

#[test]
fn replacement_pairs_as_change() {
    let lines = classify("let x = 1;\n", "let x = 2;\n");
    let kinds: Vec<_> = lines.iter().map(|l| l.kind).collect();
    assert!(kinds.contains(&LineKind::ChangeOld));
    assert!(kinds.contains(&LineKind::ChangeNew));
}

#[test]
fn surplus_insert_after_pairing_is_insert() {
    let lines = classify("a\n", "b\nc\n");
    let kinds: Vec<_> = lines.iter().map(|l| l.kind).collect();
    assert!(kinds.contains(&LineKind::ChangeOld));
    assert!(kinds.contains(&LineKind::ChangeNew));
    assert!(kinds.contains(&LineKind::Insert));
}
