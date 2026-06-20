//! Line-level classification for edit-diff rendering.
//!
//! Compares an `old_string`/`new_string` pair line by line using
//! [`similar::TextDiff`] and classifies each output line as a deletion,
//! addition, or a *change* (a deletion paired with an insertion in the same
//! hunk). The change class lets the renderer use a distinct color so modified
//! lines read differently from pure adds/removes.

use similar::{ChangeTag, TextDiff};

/// How a single rendered diff line should be styled.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(super) enum LineKind {
    /// Line removed from the old text (no paired insertion).
    Delete,
    /// Line added in the new text (no paired deletion).
    Insert,
    /// Line replaced: deletion side of a paired modification.
    ChangeOld,
    /// Line replaced: insertion side of a paired modification.
    ChangeNew,
}

/// A classified diff line: its kind plus the text content.
pub(super) struct DiffLine {
    pub kind: LineKind,
    pub text: String,
}

/// Produce classified diff lines for an `old` → `new` change.
///
/// Within each hunk, deletions and insertions are paired up to the smaller
/// count and emitted as `ChangeOld`/`ChangeNew`; any surplus stays a pure
/// `Delete` or `Insert`.
pub(super) fn classify(old: &str, new: &str) -> Vec<DiffLine> {
    let diff = TextDiff::from_lines(old, new);
    let mut deletes: Vec<String> = Vec::new();
    let mut inserts: Vec<String> = Vec::new();
    let mut out: Vec<DiffLine> = Vec::new();
    for change in diff.iter_all_changes() {
        let text = change.value().trim_end_matches('\n').to_string();
        match change.tag() {
            ChangeTag::Delete => deletes.push(text),
            ChangeTag::Insert => inserts.push(text),
            ChangeTag::Equal => flush(&mut deletes, &mut inserts, &mut out),
        }
    }
    flush(&mut deletes, &mut inserts, &mut out);
    out
}

fn flush(deletes: &mut Vec<String>, inserts: &mut Vec<String>, out: &mut Vec<DiffLine>) {
    let paired = deletes.len().min(inserts.len());
    for (i, text) in deletes.drain(..).enumerate() {
        let kind = if i < paired {
            LineKind::ChangeOld
        } else {
            LineKind::Delete
        };
        out.push(DiffLine { kind, text });
    }
    for (i, text) in inserts.drain(..).enumerate() {
        let kind = if i < paired {
            LineKind::ChangeNew
        } else {
            LineKind::Insert
        };
        out.push(DiffLine { kind, text });
    }
}
