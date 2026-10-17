//! Decode tagged payloads and retain schema errors.

use super::FinalPayload;
impl FinalPayload {
    pub(super) fn from_value(value: serde_json::Value, trimmed: &str) -> Self {
        // Try to deserialize into our enum
        match serde_json::from_value::<FinalPayload>(value.clone()) {
            Ok(payload) => payload,
            Err(e) => {
                // JSON is valid but doesn't match our schema
                // Check if it has a "kind" field we can use
                if let Some(kind) = value.get("kind").and_then(|k| k.as_str()) {
                    match kind {
                        "grep" => serde_json::from_value(value).unwrap_or_else(|e2| {
                            FinalPayload::Malformed {
                                raw: trimmed.to_string(),
                                error: format!("GrepPayload parse error: {}", e2),
                            }
                        }),
                        "ast" => serde_json::from_value(value).unwrap_or_else(|e2| {
                            FinalPayload::Malformed {
                                raw: trimmed.to_string(),
                                error: format!("AstPayload parse error: {}", e2),
                            }
                        }),
                        "semantic" => serde_json::from_value(value).unwrap_or_else(|e2| {
                            FinalPayload::Malformed {
                                raw: trimmed.to_string(),
                                error: format!("SemanticPayload parse error: {}", e2),
                            }
                        }),
                        _ => FinalPayload::Malformed {
                            raw: trimmed.to_string(),
                            error: format!("Unknown kind: {}", kind),
                        },
                    }
                } else {
                    FinalPayload::Malformed {
                        raw: trimmed.to_string(),
                        error: format!("Missing 'kind' field: {}", e),
                    }
                }
            }
        }
    }
}
