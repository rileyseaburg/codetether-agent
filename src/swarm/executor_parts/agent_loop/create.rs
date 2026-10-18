//! Initialization of sub-agent conversation state.

use super::state::State;
use crate::bus::AgentBus;
use crate::provider::{Provider, ToolDefinition};
use crate::tool::ToolRegistry;
use codetether_swarm_core::{
    SwarmEvent,
    agent_loop::{AgentState, Conversation},
};
use std::{path::PathBuf, sync::Arc};
use tokio::sync::mpsc;

pub(super) fn state(
    provider: Arc<dyn Provider>,
    model: &str,
    system: &str,
    user: &str,
    tools: Vec<ToolDefinition>,
    registry: Arc<ToolRegistry>,
    max_steps: usize,
    timeout_secs: u64,
    events: Option<mpsc::Sender<SwarmEvent>>,
    subtask_id: String,
    bus: Option<Arc<AgentBus>>,
    working_dir: Option<PathBuf>,
) -> State {
    let model = model.to_string();
    let mut conversation = Conversation::new(system, user, timeout_secs);
    conversation.temperature = codetether_provider_policy::temperature_for(&model);
    State {
        inner: AgentState {
            provider,
            model,
            tools,
            registry,
            max_steps,
            timeout_secs,
            events,
            subtask_id,
            bus,
            working_dir,
            conversation,
            control: crate::session::runtime::ControlHandle::current(),
        },
    }
}
