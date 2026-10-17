//! Existing shallow JSON argument validation for requested tools.
use super::json_type::json_type_name;
use serde_json::Value;

/// Validate tool arguments against the tool's JSON Schema.
///
/// # Arguments
/// * `schema` - Declared tool-parameter schema.
/// * `args` - Candidate argument value.
///
/// # Returns
/// Unit if required fields and supported field types match.
///
/// # Errors
/// Returns an error string for a missing required field or a mismatched supported type.
///
/// # Examples
/// ```
/// # use serde_json::Value;
/// # fn example(schema: &Value, args: &Value) {
/// let _result = codetether_cognition_core::execution::requests::validate_tool_args(schema, args);
/// # }
/// ```
pub fn validate_tool_args(schema: &Value, args: &Value) -> Result<(), String> {
    // Validate required fields
    if let Some(required) = schema.get("required").and_then(|r| r.as_array()) {
        for field in required {
            if let Some(field_name) = field.as_str()
                && args.get(field_name).is_none()
            {
                return Err(format!("Missing required field: {}", field_name));
            }
        }
    }

    // Validate field types
    if let Some(properties) = schema.get("properties").and_then(|p| p.as_object())
        && let Some(args_obj) = args.as_object()
    {
        for (key, value) in args_obj {
            if let Some(prop_schema) = properties.get(key)
                && let Some(expected_type) = prop_schema.get("type").and_then(|t| t.as_str())
            {
                let type_ok = match expected_type {
                    "string" => value.is_string(),
                    "number" | "integer" => value.is_number(),
                    "boolean" => value.is_boolean(),
                    "array" => value.is_array(),
                    "object" => value.is_object(),
                    _ => true,
                };
                if !type_ok {
                    return Err(format!(
                        "Field '{}' has wrong type: expected {}, got {}",
                        key,
                        expected_type,
                        json_type_name(value)
                    ));
                }
            }
        }
    }

    Ok(())
}
