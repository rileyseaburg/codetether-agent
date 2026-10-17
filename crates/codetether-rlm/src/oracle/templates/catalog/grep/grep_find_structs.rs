//! Catalog entry grep_find_structs.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_structs",
    description: "Find all struct definitions",
    template: "Find all structs in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Grep,
    example: &["src/main.rs"],
};
