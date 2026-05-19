use crate::oracle::QueryType;

use super::TreeSitterOracle;

const STRUCTURAL_TERMS: &[&str] = &[
    "signature",
    "parameters",
    "return type",
    "fields of",
    "what fields",
    "struct definition",
    "enum variants",
    "implements",
    "methods",
];

impl TreeSitterOracle {
    pub(crate) fn classify_query(query: &str) -> QueryType {
        let lower = query.to_lowercase();
        if STRUCTURAL_TERMS.iter().any(|term| lower.contains(term)) {
            QueryType::Structural
        } else {
            QueryType::Semantic
        }
    }
}
