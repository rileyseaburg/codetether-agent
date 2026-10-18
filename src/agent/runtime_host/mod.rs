//! Concrete native services for the shared Agent state owner.
//!
//! Tool registries and sessions remain application types; their orchestration
//! and public Agent API live in codetether-agent-runtime.
use crate::{
    agent::ToolUse,
    provider::{Message, Usage},
    session::Session,
    tool::ToolResult,
};
use serde_json::Value;
use std::path::Path;
mod context;
mod view;
mod write;
/// Native registry and session binding exposed by the public Agent alias.
pub struct SessionServices;
/// Native registry/context services with application session bookkeeping.
pub type Native =
    codetether_native_agent::Host<SessionServices, crate::tool::registry_lifecycle::Native>;
