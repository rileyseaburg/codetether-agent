//! Text-window record extraction.

use super::{EvidenceKind, EvidenceRecord, text};

pub(super) fn collect(source: &str, lines: &[&str], records: &mut Vec<EvidenceRecord>) {
    for (start, end, body) in text::windows(lines, 40, 32) {
        records.push(EvidenceRecord::new(
            records.len(),
            source,
            EvidenceKind::Text,
            (start, end),
            body,
            vec![],
        ));
    }
}
