//! Fallback evidence when extraction yields no records.

use super::{EvidenceKind, EvidenceRecord};

pub(super) fn record(source: &str, content: &str) -> EvidenceRecord {
    EvidenceRecord::new(
        0,
        source,
        EvidenceKind::Text,
        (1, 1),
        content.chars().take(8000).collect(),
        vec![],
    )
}
