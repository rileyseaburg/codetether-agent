//! MCP image-result wire format and compatibility checks.
use super::convert;
use crate::types::{CallToolResult, ToolContent};
use codetether_tool_core::{ToolResult, images as result_images};
use serde_json::json;
mod compatibility;
mod roundtrip;
