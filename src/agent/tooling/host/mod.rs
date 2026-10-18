//! Native registry, invocation policy and session adapters for shared tools.
//!
//! These bindings retain concrete native tools and session enrichment helpers.
use crate::{
    agent::Agent,
    config::PermissionAction,
    provider::Message,
    session::Session,
    tool::{Tool, ToolResult},
};
use serde_json::Value;
use std::{path::Path, sync::Arc};
mod registry;
mod session;
