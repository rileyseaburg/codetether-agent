//! Initialization of sub-agent conversation state.

use super::state::State;
use crate::bus::AgentBus;
use crate::provider::{ContentPart, Message, Provider, Role, ToolDefinition};
use crate::tool::ToolRegistry;
use crate::tui::swarm_view::SwarmEvent;
use std::{path::PathBuf, sync::Arc, time::Instant};
use tokio::{sync::mpsc, time::Duration};

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
    let text = |role, text: &str| Message {
        role,
        content: vec![ContentPart::Text {
            text: text.to_string(),
        }],
    };
    State {
        provider,
        model: model.into(),
        tools,
        registry,
        max_steps,
        timeout_secs,
        events,
        subtask_id,
        bus,
        working_dir,
        messages: vec![text(Role::System, system), text(Role::User, user)],
        steps: 0,
        tool_calls: 0,
        output: String::new(),
        deadline: Instant::now() + Duration::from_secs(timeout_secs),
        temperature: crate::session::helper::request_state::settings::temperature_for(model),
    }
}
