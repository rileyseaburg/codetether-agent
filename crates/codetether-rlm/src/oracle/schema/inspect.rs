//! Payload capability and source-path inspection.

use super::FinalPayload;
impl FinalPayload {
    /// Check if this payload is verifiable by an oracle.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{FinalPayload};
    /// assert!(!FinalPayload::parse("not JSON").is_verifiable());
    /// ```
    pub fn is_verifiable(&self) -> bool {
        matches!(self, FinalPayload::Grep(_) | FinalPayload::Ast(_))
    }

    /// Get the file path this payload references (if any).
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{FinalPayload};
    /// assert_eq!(FinalPayload::parse("not JSON").file(), None);
    /// ```
    pub fn file(&self) -> Option<&str> {
        match self {
            FinalPayload::Grep(p) => Some(&p.file),
            FinalPayload::Ast(p) => Some(&p.file),
            FinalPayload::Semantic(p) => Some(&p.file),
            FinalPayload::Malformed { .. } => None,
        }
    }
}
