//! Catalog entry ast_trait_impls.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_trait_impls",
    description: "Find trait implementations",
    template: "What traits does {TYPE} implement in {FILE}?",
    params: &[
        Param {
            name: "TYPE",
            description: "Type name",
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
