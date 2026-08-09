//! # Ripgrep Tool
//!
//! Exposes the real `rg` binary as a tool, so agents get ripgrep's exact
//! semantics — full regex alternation, `--glob` include/exclude filters, and
//! its `.gitignore` handling — instead of a reimplementation.
//!
//! Prefer this over `grep` for regex work. The hand-rolled `grep` tool escapes
//! its pattern unless `is_regex: true` is passed, so `a|b` silently matches the
//! literal text `a|b` there; here alternation always works.
//!
//! ## Quick Start
//!
//! ```rust,no_run
//! # tokio::runtime::Runtime::new().unwrap().block_on(async {
//! use codetether_agent::tool::Tool;
//! use codetether_agent::tool::ripgrep::RipgrepTool;
//! use serde_json::json;
//!
//! let tool = RipgrepTool::new();
//! let result = tool.execute(json!({
//!     "pattern": "public_funnel_route_aliases",
//!     "glob": ["!api/src/db/drizzle/**"]
//! })).await.unwrap();
//! assert!(result.success);
//! # });
//! ```
//!
//! ## Architecture
//!
//! - [`schema`] — JSON Schema advertised to the model
//! - `args` — typed arguments plus limit/timeout clamping
//! - `command` — [`RgArgs`] to ripgrep flag vector
//! - `exec` — process spawn under a wall-clock budget
//! - `render` — ripgrep exit codes to [`ToolResult`](crate::tool::ToolResult)
//! - `tool_impl` — the [`Tool`](crate::tool::Tool) implementation

pub(crate) mod args;
pub(crate) mod command;
mod exec;
mod render;
pub mod schema;
mod tool_impl;

#[cfg(test)]
mod tests;

use args::RgArgs;
use std::path::{Path, PathBuf};

/// Tool wrapper around the `rg` binary.
pub struct RipgrepTool {
    root: PathBuf,
}

impl Default for RipgrepTool {
    fn default() -> Self {
        Self::new()
    }
}

impl RipgrepTool {
    /// Build a tool rooted at the current working directory.
    pub fn new() -> Self {
        Self {
            root: std::env::current_dir().unwrap_or_else(|_| PathBuf::from(".")),
        }
    }

    /// Build a tool rooted at `root`.
    pub fn with_root(root: PathBuf) -> Self {
        Self { root }
    }

    /// Directory searches are executed from.
    pub fn root(&self) -> &Path {
        &self.root
    }
}
