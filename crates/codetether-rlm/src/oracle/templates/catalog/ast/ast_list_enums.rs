//! Catalog entry ast_list_enums.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_list_enums",
    description: "List all enums and their variants",
    template: "List all enums and variants in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Ast,
    example: &["src/main.rs"],
};
