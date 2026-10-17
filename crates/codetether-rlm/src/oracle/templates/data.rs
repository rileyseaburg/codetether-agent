//! Template parameters and immutable template metadata.

use super::TemplateKind;
use serde::{Deserialize, Serialize};

/// A query template for oracle-friendly trace generation.
///
/// # Examples
/// ```
/// use codetether_rlm::{QueryTemplate};
/// assert!(QueryTemplate::find("grep_find_occurrences").is_some());
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QueryTemplate {
    /// Unique template identifier
    pub id: &'static str,
    /// Human-readable template description
    pub description: &'static str,
    /// The template string with {PLACEHOLDERS}
    pub template: &'static str,
    /// Parameters this template expects
    #[serde(skip)]
    pub params: &'static [Param],
    /// Expected oracle kind
    pub kind: TemplateKind,
    /// Example parameter values
    #[serde(skip)]
    pub example: &'static [&'static str],
}

/// A parameter in a query template.
///
/// # Examples
/// ```
/// use codetether_rlm::{QueryTemplate};
/// let template = QueryTemplate::find("grep_find_occurrences").unwrap();
/// assert_eq!(template.params[0].name, "PATTERN");
/// ```
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Param {
    /// Parameter name (used as placeholder)
    pub name: &'static str,
    /// Description of what value to provide
    pub description: &'static str,
    /// Example value
    pub example: &'static str,
}
