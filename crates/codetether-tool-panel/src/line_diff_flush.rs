//! Pair changed lines and flush pending diff runs.
use super::line_diff::{DiffLine, LineKind};

pub(super) fn flush(deletes: &mut Vec<String>, inserts: &mut Vec<String>, out: &mut Vec<DiffLine>) {
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
