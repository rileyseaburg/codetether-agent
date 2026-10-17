//! Catalog entry grep_find_comments.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_comments",
    description: "Find comments in code",
    template: "Find all comments in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Grep,
    example: &["src/main.rs"],
};
