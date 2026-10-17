//! File token measurements using the shared RLM estimator.
//!
//! [TokenCountTool] measures requested files; [describe] formats one in-memory
//! text value without reading a file.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::{Value, json};
mod execute;
mod schema;
mod tool;
use codetether_rlm::RlmChunker;
mod describe;
pub use describe::describe;
/// Reports estimated tokens, byte lengths and line counts for files.
///
/// # Examples
/// ```rust
/// let _tool = codetether_token_count::TokenCountTool;
/// ```

/// Reports estimated token count, bytes, and lines for one or more files.
pub struct TokenCountTool;
