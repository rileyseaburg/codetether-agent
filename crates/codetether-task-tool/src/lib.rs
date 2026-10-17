//! Track in-memory tasks, their status and completion results.
//!
//! [TaskTool] shares one process-local registry and counter across instances.
//! It exposes tracking operations through the shared tool contract.
use anyhow::{Context, Result};
use async_trait::async_trait;
use codetether_tool_core::{Tool, ToolResult};
use parking_lot::RwLock;
use serde::Deserialize;
use serde_json::{Value, json};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
mod cancel;
mod complete;
mod construct;
mod create;
mod execute;
mod list;
mod params;
mod schema;
mod state;
mod status;
mod storage;
mod tool;
use params::Params;
use state::{TaskInfo, ToolTaskStatus};
use storage::{TASK_COUNTER, TASK_STORE};
/// Process-local task tracking tool.
///
/// # Examples
/// ```rust
/// let _tool = codetether_task_tool::TaskTool::new();
/// ```
pub struct TaskTool;
