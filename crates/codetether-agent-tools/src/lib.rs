//! Agent tool input preparation, policy gating and dispatch.
//!
//! [execute_for_session] parses and enriches serialized input before
//! [execute_value] dispatches a registered tool or the invalid-tool fallback.
//! [Host] binds the native registry, policy evaluator and session enrichment.
mod dispatch;
mod host;
mod images;
mod input;
mod invalid;
use codetether_config::PermissionAction;
use codetether_provider_core::Message;
use codetether_tool_core::{Tool, ToolResult};
pub use dispatch::execute_value;
pub use host::{Host, Registry, SessionView};
pub use images::enrich;
pub use input::{execute_for_session, prepare};
use serde_json::Value;
use std::{path::Path, sync::Arc};
