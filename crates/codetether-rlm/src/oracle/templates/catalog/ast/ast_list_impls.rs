//! Catalog entry ast_list_impls.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_list_impls",
    description: "List all impl blocks for a type",
    template: "List all impl blocks for {TYPE} in {FILE}",
    params: &[
        Param {
            name: "TYPE",
            description: "Type name to find impls for",
            example: "MyStruct",
        },
        Param {
            name: "FILE",
            description: "File path",
            example: "src/main.rs",
        },
    ],
    kind: TemplateKind::Ast,
    example: &["MyStruct", "src/main.rs"],
};
