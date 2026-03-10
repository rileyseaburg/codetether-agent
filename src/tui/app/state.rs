use std::time::Instant;
use serde::{Deserialize, Serialize};
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::chat::message::ChatMessage;
use crate::tui::swarm_view::AgentMessageEntry;
use crate::tui::swarm_view::SubTaskInfo;
use crate::tui::swarm_view::AgentToolCallDetail;
use crate::swarm::subtask::{SubTask, SubTaskStatus};
use crate::swarm::SwarmStats;
use crate::tui::ralph_view::{RalphStoryInfo, RalphStoryStatus};

pub use crate::tui::swarm_view::SwarmEvent;
pub use crate::tui::ralph_view::RalphEvent;
pub use crate::session::SessionEvent;

#[derive(Default)]
pub struct App {
    pub state: AppState,
}
pub struct AppState {
    pub view_mode: ViewMode,
    pub input_mode: InputMode,
    pub messages: Vec<ChatMessage>,
    pub input: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            view_mode: ViewMode::Chat,
            input_mode: InputMode::Normal,
            messages: vec![],
            input: String::new(),
        }
    }
}
