//! Core typed evidence structures.

use serde::{Deserialize, Serialize};

/// Cached index for one source blob.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContextIndex {
    pub source: String,
    pub hash: u64,
    pub symbols: Vec<String>,
    pub records: Vec<EvidenceRecord>,
}

/// Kind of evidence represented by a record.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EvidenceKind {
    Symbol,
    Import,
    Error,
    Text,
}

/// High-level retrieval intent.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlanIntent {
    Debug,
    Symbol,
    Summary,
}

/// Auditable retrieval unit passed to synthesis.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EvidenceRecord {
    pub id: usize,
    pub source: String,
    pub kind: EvidenceKind,
    pub span: (usize, usize),
    pub symbols: Vec<String>,
    pub score: f32,
    pub reason: String,
    pub text: String,
}

/// Query plan used by the retriever.
#[derive(Debug, Clone)]
pub struct RetrievalPlan {
    pub terms: Vec<String>,
    pub budget_tokens: usize,
    pub intent: PlanIntent,
}
