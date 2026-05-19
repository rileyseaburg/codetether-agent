use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct EvidenceRecord {
    pub kind: String,
    pub level: String,
    pub value: String,
}

impl EvidenceRecord {
    pub(crate) fn new(kind: &str, level: &str, value: impl Into<String>) -> Self {
        Self {
            kind: kind.to_string(),
            level: level.to_string(),
            value: value.into(),
        }
    }
}
