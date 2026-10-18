//! Concrete native services for the shared Agent state owner.
//!
//! Tool registries and sessions remain application types; their orchestration
//! and public Agent API live in codetether-agent-runtime.
use crate::{
    agent::ToolUse,
    provider::{Message, Usage},
    session::Session,
    tool::{Tool, ToolRegistry, ToolResult},
};
use serde_json::Value;
use std::{path::Path, sync::Arc};
mod context;
mod registry;
mod view;
mod write;
/// Native registry and session binding exposed by the public Agent alias.
pub struct Native;
