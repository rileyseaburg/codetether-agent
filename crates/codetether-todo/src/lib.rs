//! Session-isolated todo storage and executable read/write tools.
//!
//! [TodoReadTool] filters and renders persisted items; [TodoWriteTool] mutates
//! them. [storage] selects a session-specific path, with checkout fallback.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::path::PathBuf;
mod add;
mod clear;
mod delete;
mod filter;
mod params;
mod priority;
mod read;
mod read_construct;
mod read_tool;
mod render;
pub mod storage;
mod todoitem;
mod todoreadtool;
mod todostatus;
mod todowritetool;
mod update;
mod write;
mod write_construct;
mod write_tool;
use params::{ReadParams, WriteParams};
pub use priority::Priority;
use storage as todo_path;
pub use todoitem::TodoItem;
pub use todoreadtool::TodoReadTool;
pub use todostatus::TodoStatus;
pub use todowritetool::TodoWriteTool;

#[cfg(test)]
mod native_path_tests;
