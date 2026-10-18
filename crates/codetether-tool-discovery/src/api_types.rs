//! Request and response bodies for dynamic tool registration.
use crate::RegisteredToolView;
use serde::{Deserialize, Serialize};

/// Request to register a tool
#[derive(Debug, Deserialize)]
pub struct RegisterToolRequest {
    pub id: String,
    pub name: String,
    pub description: String,
    pub version: String,
    pub endpoint: String,
    pub capabilities: Vec<String>,
    pub parameters: serde_json::Value,
}

/// Response from registering a tool
#[derive(Serialize)]
pub struct RegisterToolResponse {
    pub(crate) tool: RegisteredToolView,
    pub(crate) message: String,
}
