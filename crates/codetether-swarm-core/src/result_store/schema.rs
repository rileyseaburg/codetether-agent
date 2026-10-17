use super::*;

/// Schema information for a shared result
#[derive(Debug, Clone, Serialize, Deserialize)]
///
/// # Examples
/// ```
/// let schema = codetether_swarm_core::result_store::ResultSchema::from_value(&serde_json::json!({"count": 1}));
/// assert_eq!(schema.type_name, "object");
/// ```
pub struct ResultSchema {
    /// Type name (e.g., "string", "number", "object", "array")
    pub type_name: String,

    /// Optional description of the result
    pub description: Option<String>,

    /// For objects, the expected field types
    pub fields: Option<HashMap<String, String>>,
}

impl ResultSchema {
    /// Create a schema from a serde_json::Value
    pub fn from_value(value: &Value) -> Self {
        let type_name = match value {
            Value::Null => "null".to_string(),
            Value::Bool(_) => "boolean".to_string(),
            Value::Number(n) => {
                if n.is_i64() || n.is_u64() {
                    "integer".to_string()
                } else {
                    "number".to_string()
                }
            }
            Value::String(_) => "string".to_string(),
            Value::Array(_) => "array".to_string(),
            Value::Object(_) => "object".to_string(),
        };

        let fields = if let Value::Object(obj) = value {
            let mut field_types = HashMap::new();
            for (key, val) in obj {
                field_types.insert(key.clone(), Self::from_value(val).type_name);
            }
            Some(field_types)
        } else {
            None
        };

        Self {
            type_name,
            description: None,
            fields,
        }
    }
}
