//! Catalog entry grep_find_traits.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_traits",
    description: "Find all trait definitions",
    template: "Find all traits in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Grep,
    example: &["src/main.rs"],
};
