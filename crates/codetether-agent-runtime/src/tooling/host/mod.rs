//! Shared integration with tool preparation and dispatch.
use crate::{Agent, Host};
use codetether_config::PermissionAction;
use codetether_provider_core::Message;
use codetether_tool_core::{Tool, ToolResult};
use serde_json::Value;
use std::{path::Path, sync::Arc};
mod registry;
mod session;
