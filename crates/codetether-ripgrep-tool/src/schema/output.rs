//! Output-shaping properties of the `rg` schema.

use serde_json::{Value, json};

/// Properties controlling how results are reported.
pub(super) fn properties() -> Value {
    json!({
        "files_with_matches": {
            "type": "boolean",
            "description": "List only matching file paths (rg -l). Default false."
        },
        "context_lines": {
            "type": "integer",
            "description": "Lines of context around each match (rg -C)"
        },
        "max_count": {
            "type": "integer",
            "description": "Stop after this many matches per file (rg -m)"
        },
        "limit": {
            "type": "integer",
            "description": "Max output lines to return (default 200)"
        },
        "timeout_secs": {
            "type": "integer",
            "description": "Kill the search after this many seconds (default 30)"
        }
    })
}
