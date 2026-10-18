//! Shared output shape returned by native tool execution.
use serde_json::Value;
use std::collections::HashMap;
/// Output, success and optional tool metadata.
pub type ToolTuple = (String, bool, Option<HashMap<String, Value>>);
