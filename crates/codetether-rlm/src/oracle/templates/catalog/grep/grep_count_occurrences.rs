//! Catalog entry grep_count_occurrences.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_count_occurrences",
    description: "Count occurrences of a pattern in a file",
    template: "Count occurrences of {PATTERN} in {FILE}",
    params: &[
        Param {
            name: "PATTERN",
            description: "Pattern to count",
            example: "TODO",
        },
        Param {
            name: "FILE",
            description: "File path",
            example: "src/main.rs",
        },
    ],
    kind: TemplateKind::Grep,
    example: &["TODO", "src/main.rs"],
};
