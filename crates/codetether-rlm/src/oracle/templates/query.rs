//! Stable catalog lookup and filtering.

use super::{QueryTemplate, TEMPLATES, TemplateIter, TemplateKind};
impl QueryTemplate {
    /// Get all defined templates.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate};
    /// assert!(!QueryTemplate::all().is_empty());
    /// ```
    pub fn all() -> &'static [QueryTemplate] {
        TEMPLATES
    }

    /// Get templates by kind.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate, TemplateKind};
    /// assert!(QueryTemplate::by_kind(TemplateKind::Grep).all(|t| t.kind == TemplateKind::Grep));
    /// ```
    pub fn by_kind(kind: TemplateKind) -> impl Iterator<Item = &'static QueryTemplate> {
        TEMPLATES.iter().filter(move |t| t.kind == kind)
    }

    /// Find a template by ID.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate};
    /// assert!(QueryTemplate::find("grep_find_occurrences").is_some());
    /// ```
    pub fn find(id: &str) -> Option<&'static QueryTemplate> {
        TEMPLATES.iter().find(|t| t.id == id)
    }
}

impl QueryTemplate {
    /// Iterate over all templates.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate};
    /// assert_eq!(QueryTemplate::iter().count(), QueryTemplate::all().len());
    /// ```
    pub fn iter() -> TemplateIter {
        TemplateIter {
            index: 0,
            kind: None,
        }
    }

    /// Iterate over templates of a specific kind.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{QueryTemplate, TemplateKind};
    /// assert_eq!(QueryTemplate::iter_kind(TemplateKind::Ast).count(), QueryTemplate::by_kind(TemplateKind::Ast).count());
    /// ```
    pub fn iter_kind(kind: TemplateKind) -> TemplateIter {
        TemplateIter {
            index: 0,
            kind: Some(kind),
        }
    }
}
