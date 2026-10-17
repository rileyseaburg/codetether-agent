//! Catalog entry ast_struct_fields.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_struct_fields",
    description: "Get struct field names and types",
    template: "What fields does {STRUCT} have in {FILE}?",
    params: &[
        Param {
            name: "STRUCT",
            description: "Struct name",
            example: "Config",
        },
        Param {
            name: "FILE",
            description: "File path",
            example: "src/main.rs",
        },
    ],
    kind: TemplateKind::Ast,
    example: &["Config", "src/main.rs"],
};
