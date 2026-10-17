//! Build provider-facing tool messages from execution status and image metadata.
//!
//! [tool_result_with_status] creates a text result; [tool_result_with_metadata]
//! also attaches structured images without putting image bytes in feedback text.
use codetether_provider_core::{ContentPart, Message, Role};
use serde_json::Value;
use std::collections::HashMap;
mod metadata;
mod status;
pub use metadata::tool_result_with_metadata;
pub use status::tool_result_with_status;
#[cfg(test)]
mod image_tests;
#[cfg(test)]
mod retention_tests;

mod raw;
pub use raw::raw_tool_result;
