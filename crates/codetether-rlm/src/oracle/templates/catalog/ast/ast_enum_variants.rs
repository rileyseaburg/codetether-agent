//! Catalog entry ast_enum_variants.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "ast_enum_variants",
    description: "Get enum variant names",
    template: "What variants does {ENUM} have in {FILE}?",
    params: &[
        Param {
            name: "ENUM",
            description: "Enum name",
            example: "Status",
        },
        Param {
            name: "FILE",
            description: "File path",
            example: "src/main.rs",
        },
    ],
    kind: TemplateKind::Ast,
    example: &["Status", "src/main.rs"],
};
