//! Catalog entry ast_error_handling_count.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_error_handling_count",
    description: "Count error handling patterns",
    template: "Count the error handling patterns (Result, ?, unwrap, expect) in {FILE}",
    params: &[Param {
        name: "FILE",
        description: "File path",
        example: "src/main.rs",
    }],
    kind: TemplateKind::Ast,
    example: &["src/main.rs"],
};
