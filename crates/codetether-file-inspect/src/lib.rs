//! Directory trees, file metadata and text previews as executable tools.
//!
//! [TreeTool] walks bounded directory trees, [FileInfoTool] reports metadata
//! and text statistics, and [HeadTailTool] previews selected file lines.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use codetether_util::workspace_scan::is_pruned_workspace_dir;
use serde_json::{Value, json};
use std::path::Path;
use tokio::fs;
mod info;
mod info_construct;
mod info_content;
mod info_metadata;
mod info_schema;
mod info_tool;
mod language;
mod preview;
mod preview_construct;
mod preview_schema;
mod preview_tool;
mod preview_view;
mod size;
mod tree;
mod tree_collect;
mod tree_construct;
mod tree_entry;
mod tree_line;
mod tree_schema;
mod tree_tool;
mod tree_visit;
pub use info_construct::FileInfoTool;
pub use preview_construct::HeadTailTool;
use size::format_size;
pub use tree_construct::TreeTool;
use tree_entry::TreeEntry;
use tree_visit::build_tree;
#[cfg(test)]
mod tests;
