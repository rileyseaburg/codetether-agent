//! Catalog entry ast_function_signatures.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_function_signatures",
    description: "Get function signatures (name, params, return type)",
    template: "List all function signatures in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Ast,
    example: &["src/main.rs"],
};
