//! Oracle category associated with a template.

use serde::{Deserialize, Serialize};

/// The kind of oracle this template is designed for.
///
/// # Examples
/// ```
/// use codetether_rlm::{TemplateKind};
/// assert!(matches!(TemplateKind::Ast,TemplateKind::Ast));
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TemplateKind {
    /// Grep/pattern-match oracle
    Grep,
    /// Tree-sitter/AST oracle
    Ast,
    /// Semantic (unverifiable) - included for completeness
    Semantic,
}

impl TemplateKind {
    /// Convert to the FinalPayload kind string.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{TemplateKind};
    /// assert_eq!(TemplateKind::Grep.as_payload_kind(), "grep");
    /// ```
    pub fn as_payload_kind(&self) -> &'static str {
        match self {
            TemplateKind::Grep => "grep",
            TemplateKind::Ast => "ast",
            TemplateKind::Semantic => "semantic",
        }
    }
}
