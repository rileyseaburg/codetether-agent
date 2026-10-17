//! Readable payload descriptions.

use super::FinalPayload;
use std::fmt;
impl FinalPayload {
    /// Convert to a debuggable string representation.
    ///
    /// # Panics
    /// Panics when a semantic answer exceeds 50 bytes and byte 50 splits a UTF-8 character.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{FinalPayload};
    /// assert!(FinalPayload::parse("not JSON").summary().starts_with("Malformed("));
    /// ```
    pub fn summary(&self) -> String {
        match self {
            FinalPayload::Grep(p) => {
                format!(
                    "Grep(file={}, pattern={}, {} matches)",
                    p.file,
                    p.pattern,
                    p.matches.len()
                )
            }
            FinalPayload::Ast(p) => {
                format!(
                    "Ast(file={}, query={}, {} results)",
                    p.file,
                    p.query,
                    p.results.len()
                )
            }
            FinalPayload::Semantic(p) => {
                let preview = if p.answer.len() > 50 {
                    format!("{}...", &p.answer[..50])
                } else {
                    p.answer.clone()
                };
                format!("Semantic(file={}, answer={})", p.file, preview)
            }
            FinalPayload::Malformed { error, .. } => {
                format!("Malformed({})", error)
            }
        }
    }
}

impl fmt::Display for FinalPayload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.summary())
    }
}
