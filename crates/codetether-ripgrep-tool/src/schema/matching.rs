//! Match-behavior properties of the `rg` schema.

use serde_json::{Value, json};

/// Properties controlling what counts as a match.
pub(super) fn properties() -> Value {
    json!({
        "pattern": {
            "type": "string",
            "description": "Ripgrep regex pattern. Full Rust-regex syntax including alternation (a|b), anchors, and classes."
        },
        "paths": {
            "type": "array",
            "items": {"type": "string"},
            "description": "Files or directories to search (default: workspace root)"
        },
        "glob": {
            "type": "array",
            "items": {"type": "string"},
            "description": "Ripgrep --glob filters. Prefix with ! to exclude, e.g. '!api/src/db/drizzle/**'"
        },
        "fixed_strings": {
            "type": "boolean",
            "description": "Treat pattern as a literal string (rg -F). Default false."
        },
        "case_insensitive": {
            "type": "boolean",
            "description": "Case-insensitive match (rg -i). Default false."
        },
        "hidden": {
            "type": "boolean",
            "description": "Search hidden files (rg --hidden). Default false."
        },
        "no_ignore": {
            "type": "boolean",
            "description": "Ignore .gitignore rules (rg --no-ignore). Default false."
        }
    })
}
