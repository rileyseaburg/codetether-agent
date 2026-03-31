import sys

content = """use std::time::Instant;
use serde::{Deserialize, Serialize};
use crate::tui::models::{InputMode, ViewMode};
use crate::tui::chat::message::ChatMessage;
use crate::tui::swarm::AgentMessageEntry;
use crate::tui::swarm::SubTaskInfo;
use crate::tui::swarm::AgentToolCallDetail;
use crate::swarm::subtask::{SubTask, SubTaskStatus};
use crate::swarm::SwarmStats;
use crate::tui::ralph::{RalphStoryInfo, RalphStoryStatus};

#[derive(Debug, Clone)]
pub enum SwarmEvent {
    Started { task: String, total_subtasks: usize },
    Decomposed { subtasks: Vec<crate::swarm::subtask::SubTask> },
    Error(String),
    StageComplete { stage: usize },
    SubTaskUpdate {
        id: String,
        task_id: String,
        name: String,
        agent_name: Option<String>,
        status: SubTaskStatus,
        is_completed: bool,
        is_failed: bool,
        stage: usize,
        dependencies: Vec<String>,
        current_tool: Option<String>,
        active_tool: Option<String>,
        steps: usize,
        max_steps: usize,
        message_count: usize,
        output: Option<String>,
        error: Option<String>,
    },
    MessageAdded {
        task_id: String,
        role: String,
        content: String,
        is_tool_call: bool,
    },
    ToolCallStarted {
        task_id: String,
        tool_name: String,
        input: String,
        input_preview: String,
    },
    ToolCallCompleted {
        task_id: String,
        tool_name: String,
        output: Option<String>,
        output_preview: Option<String>,
        success: Option<bool>,
    },
    ToolCallError {
        task_id: String,
        tool_name: String,
        error: String,
    },
    AgentStarted { task_id: String, agent_name: String },
    AgentError { task_id: String, error: String },
    AgentOutput { task_id: String, output: String },
    AgentComplete { task_id: String, success: bool },
    AgentMessage { task_id: String, message: AgentMessageEntry },
    AgentToolCall { task_id: String, tool: String, input: String },
    AgentToolCallDetail { task_id: String, tool_call: AgentToolCallDetail },
    Complete {
        success: bool,
        final_summary: String,
        total_tasks: usize,
        completed_tasks: usize,
        failed_tasks: usize,
    },
    Completed {
        success: bool,
        final_summary: String,
        total_tasks: usize,
        completed_tasks: usize,
        failed_tasks: usize,
    },
    AgentTypingStarted {
        task_id: String,
    },
    AgentTypingStopped {
        task_id: String,
    },
    AgentReassigned {
        task_id: String,
        new_agent: String,
        reason: String,
    },
}

#[derive(Debug, Clone)]
pub enum RalphEvent {
    LoopStarted { prd_id: String },
    StoryStarted { id: String, story_id: String, title: String },
    ToolCallStarted { id: String, story_id: String, tool_name: String, input_preview: String },
    ToolCallCompleted { id: String, story_id: String, tool_name: String, output: Option<String>, success: bool },
    MessageAdded { id: String, story_id: String, role: String, content: String, is_tool_call: bool },
    StoryComplete { id: String, story_id: String, success: bool, output: String },
    StoryCompleted { id: String, story_id: String, success: bool, output: String },
    StoryError { id: String, story_id: String, error: String },
    StoryToolCall { id: String, story_id: String, tool_name: String, input_preview: String },
    StoryQualityCheck { id: String, story_id: String, check_type: String, command: String },
    QualityCheckStarted { id: String, story_id: String, check_type: String, command: String },
    QualityCheckCompleted { id: String, story_id: String, check_type: String, command: String, success: bool, output: String },
    MergeStarted { id: String, story_id: String },
    MergeCompleted { id: String, story_id: String, success: bool, merge_summary: String },
    LoopCompleted { total: usize, success: usize, failed: usize, final_summary: String },
    SubTaskUpdate {
        id: String,
        task_id: String,
        name: String,
        status: SubTaskStatus,
        depends_on: Vec<String>,
        priority: usize,
        description: String,
        pass: bool,
    }
}

pub struct App {
    pub state: AppState,
}
impl App {
    pub fn default() -> Self {
        Self { state: AppState::new() }
    }
}

pub enum SessionEvent {}

pub struct AppState {
    pub view_mode: ViewMode,
    pub input_mode: InputMode,
    pub messages: Vec<ChatMessage>,
    pub input: String,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            view_mode: ViewMode::Chat,
            input_mode: InputMode::Normal,
            messages: vec![],
            input: String::new(),
        }
    }
}
"""

with open("src/tui/app/state.rs", "w") as f:
    f.write(content)
