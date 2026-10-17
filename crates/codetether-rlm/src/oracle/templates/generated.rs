//! Rendered query metadata.

use super::{QueryTemplate, TemplateKind};

/// A pre-generated query ready for use.
///
/// # Examples
/// ```
/// use codetether_rlm::{GeneratedQuery, QueryTemplate};
/// let template = QueryTemplate::find("grep_find_occurrences").unwrap();
/// let query = GeneratedQuery::render(template, &["fn", "main.rs"]);
/// assert_eq!(query.file, "main.rs");
/// ```
#[derive(Debug, Clone)]
pub struct GeneratedQuery {
    /// The rendered query text
    pub query: String,
    /// Template ID used
    pub template_id: &'static str,
    /// Expected oracle kind
    pub kind: TemplateKind,
    /// File the query targets
    pub file: String,
}

impl GeneratedQuery {
    /// Render a template with custom values.
    ///
    /// # Examples
    /// ```
    /// use codetether_rlm::{GeneratedQuery, QueryTemplate};
    /// let template = QueryTemplate::find("grep_find_occurrences").unwrap();
    /// let query = GeneratedQuery::render(template, &["fn", "main.rs"]);
    /// assert_eq!(query.file, "main.rs");
    /// ```
    pub fn render(template: &'static QueryTemplate, values: &[&str]) -> Self {
        let file = values.last().unwrap_or(&"").to_string();

        Self {
            query: template.render(values),
            template_id: template.id,
            kind: template.kind,
            file,
        }
    }
}
