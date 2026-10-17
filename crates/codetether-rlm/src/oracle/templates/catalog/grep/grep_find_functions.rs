//! Catalog entry grep_find_functions.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_functions",
    description: "Find all function definitions matching a pattern",
    template: "Find all {VISIBILITY} functions matching {PATTERN} in {FILE}",
    params: &[
        Param {
            name: "VISIBILITY",
            description: "public, private, or all (leave empty)",
            example: "public",
        },
        Param {
            name: "PATTERN",
            description: "Pattern in function definition",
            example: "async fn",
        },
        Param {
            name: "FILE",
            description: "File path",
            example: "src/main.rs",
        },
    ],
    kind: TemplateKind::Grep,
    example: &["public", "async fn", "src/main.rs"],
};
