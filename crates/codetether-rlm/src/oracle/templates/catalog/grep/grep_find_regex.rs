//! Catalog entry grep_find_regex.

use crate::oracle::templates::{Param, QueryTemplate, TemplateKind};
pub(crate) const VALUE: QueryTemplate = QueryTemplate {
    id: "grep_find_regex",
    description: "Find all lines matching a regex pattern in a file",
    template: "Find all lines matching {REGEX} in {FILE}",
    params: &[
        Param {
            name: "REGEX",
            description: "Regular expression pattern",
            example: r"\bpub\s+fn\b",
        },
        Param {
            name: "FILE",
            description: "File path to search in",
            example: "src/lib.rs",
        },
    ],
    kind: TemplateKind::Grep,
    example: &[r"\bpub\s+fn\b", "src/lib.rs"],
};
