use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    Chat,
    Swarm,
    Ralph,
    Sessions,
    Help,
    Settings,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum InputMode {
    Normal,
    Editing,
    Command,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Theme {
    pub name: String,
    pub primary: (u8, u8, u8),
    pub border: (u8, u8, u8),
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            name: "Default".to_string(),
            primary: (0, 255, 255),
            border: (100, 100, 100),
        }
    }
}

pub struct WorkspaceEntry {
    pub name: String,
    pub kind: WorkspaceEntryKind,
}

pub struct WorkspaceSnapshot {
    pub root_display: String,
    pub git_branch: Option<String>,
    pub git_dirty_files: Vec<String>,
    pub entries: Vec<WorkspaceEntry>,
    pub captured_at: std::time::Instant,
}

pub enum WorkspaceEntryKind { File, Directory }

// Re-export specific events if they were previously in mod.rs
pub use crate::tui::app::state::{SessionEvent, SwarmEvent, RalphEvent};
