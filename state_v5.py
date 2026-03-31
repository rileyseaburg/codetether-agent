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

#[derive(Debug, Clone, Default)]
pub struct SubTaskUpdateData {
    pub task_id: Option<String>,
    pub id: Option<String>,
    pub name: Option<String>,
    pub agent_name: Option<String>,
    pub status: Option<SubTaskStatus>,
    pub is_completed: Option<bool>,
    pub is_failed: Option<bool>,
    pub stage: Option<usize>,
    pub dependencies: Option<Vec<String>>,
    pub current_tool: Option<String>,
    pub active_tool: Option<String>,
    pub steps: Option<usize>,
    pub max_steps: Option<usize>,
    pub message_count: Option<usize>,
    pub output: Option<String>,
    pub error: Option<String>,
    pub subtask: Option<SubTaskInfo>,
}


#[derive(Debug, Clone)]
pub enum SwarmEvent {
    Started { task: String, total_subtasks: usize },
    Decomposed { subtasks: Vec<crate::swarm::subtask::SubTask> },
    Error(String),
    StageComplete { stage: usize, completed: usize, failed: usize },
    SubTaskUpdate(SubTaskUpdateData),
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
    AgentStarted { task_id: Option<String>, subtask_id: Option<String>, agent_name: Option<String>, specialty: Option<String> },
    AgentError { task_id: Option<String>, subtask_id: Option<String>, error: Option<String> },
    AgentOutput { task_id: Option<String>, subtask_id: Option<String>, output: Option<String> },
    AgentComplete { task_id: Option<String>, subtask_id: Option<String>, success: Option<bool>, steps: Option<usize> },
    AgentMessage { task_id: Option<String>, subtask_id: Option<String>, message: Option<AgentMessageEntry>, entry: Option<AgentMessageEntry> },
    AgentToolCall { task_id: Option<String>, subtask_id: Option<String>, tool: Option<String>, tool_name: Option<String>, input: Option<String> },
    AgentToolCallDetail { task_id: Option<String>, subtask_id: Option<String>, tool_call: Option<AgentToolCallDetail>, detail: Option<AgentToolCallDetail> },
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
    StoryQualityCheck { id: String, story_id: String, check_type: String, command: String, passed: Option<bool> },
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
    pub fn new() -> Self {
        Self { state: AppState::default() }
    }
}

impl Default for App {
    fn default() -> Self {
        Self::new()
    }
}

pub enum SessionEvent {}

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
"""

with open("src/tui/app/state.rs", "w") as f:
    f.write(content)
