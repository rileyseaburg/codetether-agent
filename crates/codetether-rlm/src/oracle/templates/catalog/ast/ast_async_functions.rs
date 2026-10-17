//! Catalog entry ast_async_functions.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_async_functions",
    description: "List async function signatures",
    template: "List all async function signatures in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Ast,
    example: &["src/main.rs"],
};
