//! Catalog entry grep_find_string_literals.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_string_literals",
    description: "Find string literals",
    template: "Find all string literals in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Grep,
    example: &["src/main.rs"],
};
