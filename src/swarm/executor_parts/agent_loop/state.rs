//! Mutable conversation state for a sub-agent loop.

use crate::bus::AgentBus;
use crate::provider::{Message, Provider, ToolDefinition};
use crate::tool::ToolRegistry;
use crate::tui::swarm_view::SwarmEvent;
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Instant;
use tokio::sync::mpsc;

pub(super) struct State {
    pub provider: Arc<dyn Provider>,
    pub model: String,
    pub tools: Vec<ToolDefinition>,
    pub registry: Arc<ToolRegistry>,
    pub max_steps: usize,
    pub timeout_secs: u64,
    pub events: Option<mpsc::Sender<SwarmEvent>>,
    pub subtask_id: String,
    pub bus: Option<Arc<AgentBus>>,
    pub working_dir: Option<PathBuf>,
    pub messages: Vec<Message>,
    pub steps: usize,
    pub tool_calls: usize,
    pub output: String,
    pub deadline: Instant,
    pub temperature: Option<f32>,
}

pub(super) struct ToolCall {
    pub id: String,
    pub name: String,
    pub arguments: String,
}
