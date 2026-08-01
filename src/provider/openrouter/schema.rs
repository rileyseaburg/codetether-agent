//! Normalization of tool schemas for heterogeneous OpenRouter endpoints.
//!
//! OpenRouter may route one model across providers with different JSON Schema
//! subsets. Runtime validation remains authoritative, so removing unsupported
//! advisory constraints is safer than having the provider reject the request.

use serde_json::Value;

#[path = "schema_required.rs"]
mod required;

/// Returns a provider-portable copy of a JSON Schema.
pub(super) fn sanitize(mut schema: Value) -> Value {
    visit(&mut schema);
    schema
}

fn visit(schema: &mut Value) {
    let Some(object) = schema.as_object_mut() else {
        return;
    };
    object.remove("$schema");
    object.remove("pattern");
    object.remove("patternProperties");
    required::normalize(object);
    for keyword in [
        "items",
        "additionalProperties",
        "contains",
        "not",
        "if",
        "then",
        "else",
    ] {
        if let Some(child) = object.get_mut(keyword) {
            visit(child);
        }
    }
    for keyword in ["properties", "$defs", "definitions"] {
        if let Some(children) = object.get_mut(keyword).and_then(Value::as_object_mut) {
            for child in children.values_mut() {
                visit(child);
            }
        }
    }
    for keyword in ["anyOf", "oneOf", "allOf", "prefixItems"] {
        if let Some(children) = object.get_mut(keyword).and_then(Value::as_array_mut) {
            children.iter_mut().for_each(visit);
        }
    }
}

#[cfg(test)]
#[path = "schema_tests.rs"]
mod tests;
