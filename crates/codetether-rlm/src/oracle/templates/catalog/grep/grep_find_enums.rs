//! Catalog entry grep_find_enums.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_enums",
    description: "Find all enum definitions",
    template: "Find all enums in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Grep,
    example: &["src/main.rs"],
};
