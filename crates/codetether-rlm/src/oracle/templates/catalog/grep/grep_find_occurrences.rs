//! Catalog entry grep_find_occurrences.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_occurrences",
    description: "Find all occurrences of a string pattern in a file",
    template: "Find all occurrences of {PATTERN} in {FILE}",
    params: &[
        Param {
            name: "PATTERN",
            description: "String or regex pattern to search for",
            example: "async fn",
        },
        Param {
            name: "FILE",
            description: "File path to search in",
            example: "src/main.rs",
        },
    ],
    kind: TemplateKind::Grep,
    example: &["async fn", "src/main.rs"],
};
