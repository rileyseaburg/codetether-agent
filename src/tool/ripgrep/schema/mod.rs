//! JSON Schema for the `rg` tool's parameters.

mod matching;
mod output;

use serde_json::{Value, json};

/// Parameter schema exposed to the model.
///
/// Merges the matching-behavior and output-shaping property groups into one
/// object schema.
pub fn parameters() -> Value {
    let mut properties = serde_json::Map::new();
    for group in [matching::properties(), output::properties()] {
        if let Value::Object(map) = group {
            properties.extend(map);
        }
    }
    json!({
        "type": "object",
        "properties": Value::Object(properties),
        "required": ["pattern"],
        "example": {
            "pattern": "public_funnel_route_aliases",
            "glob": ["!api/src/db/drizzle/**"]
        }
    })
}
