//! Native state views and session-context completion for the shared loop.
//!
//! Prompt policy, history accounting and context completion retain their native methods.
use crate::{
    agent::{Agent, AgentInfo, ToolUse},
    provider::{Message, Provider, ToolDefinition, Usage},
    session::Session,
};
use std::sync::Arc;
mod completion;
mod state;
