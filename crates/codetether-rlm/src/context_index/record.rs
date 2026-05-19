//! Constructors for evidence records.

use super::{EvidenceKind, EvidenceRecord};

impl EvidenceRecord {
    /// Construct an unscored evidence record.
    pub fn new(
        id: usize,
        source: &str,
        kind: EvidenceKind,
        span: (usize, usize),
        text: String,
        symbols: Vec<String>,
    ) -> Self {
        Self {
            id,
            source: source.into(),
            kind,
            span,
            symbols,
            score: 0.0,
            reason: String::new(),
            text,
        }
    }
}
