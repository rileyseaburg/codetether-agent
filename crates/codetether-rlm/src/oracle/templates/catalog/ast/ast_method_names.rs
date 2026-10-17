//! Catalog entry ast_method_names.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_method_names",
    description: "List all method names in impl blocks",
    template: "List all methods defined in impl blocks in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Ast,
    example: &["src/main.rs"],
};
