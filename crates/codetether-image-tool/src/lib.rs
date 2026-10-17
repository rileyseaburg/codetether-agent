//! Load local or remote images as data URLs for vision-capable models.
//!
//! [ImageTool] accepts [ImageToolInput], reads the requested source and returns
//! image metadata through the shared tool contract.
use anyhow::Result;
use async_trait::async_trait;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use codetether_tool_core::{Tool, ToolResult};
use serde::{Deserialize, Serialize};
use serde_json::Value;
mod construct;
mod encode;
mod execute;
mod input;
mod mime;
mod schema;
mod tool;
pub use input::ImageToolInput;
/// Load an image from a local file or URL.
///
/// # Examples
/// ```rust
/// let _tool = codetether_image_tool::ImageTool::new();
/// ```
pub struct ImageTool;
