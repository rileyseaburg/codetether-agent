//! glob schema for core file tools.
use super::*;

pub(super) fn parameters() -> Value {
    json!({
        "type": "object",
        "properties": {
            "pattern": {
                "type": "string",
                "description": "The glob pattern to match files"
            },
            "limit": {
                "type": "integer",
                "description": "Maximum number of results to return"
            }
        },
        "required": ["pattern"],
        "example": {
            "pattern": "src/**/*.rs",
            "limit": 50
        }
    })
}
