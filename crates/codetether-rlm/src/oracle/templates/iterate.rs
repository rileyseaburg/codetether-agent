//! Stateful iteration across the ordered catalog.

use super::{QueryTemplate, TEMPLATES, TemplateKind};

/// Iterator over templates of a specific kind.
///
/// # Examples
/// ```
/// use codetether_rlm::{QueryTemplate};
/// assert_eq!(QueryTemplate::iter().count(), QueryTemplate::all().len());
/// ```
pub struct TemplateIter {
    pub(super) index: usize,
    pub(super) kind: Option<TemplateKind>,
}

impl Iterator for TemplateIter {
    type Item = &'static QueryTemplate;

    fn next(&mut self) -> Option<Self::Item> {
        while self.index < TEMPLATES.len() {
            let template = &TEMPLATES[self.index];
            self.index += 1;

            if let Some(kind) = self.kind {
                if template.kind == kind {
                    return Some(template);
                }
            } else {
                return Some(template);
            }
        }
        None
    }
}
