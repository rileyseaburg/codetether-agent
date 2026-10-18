//! Shared integration with context-aware completion.
use crate::{Agent, AgentInfo, Host, ToolUse};
use codetether_provider_core::{Message, Provider, ToolDefinition, Usage};
use std::sync::Arc;
mod completion;
mod state;
