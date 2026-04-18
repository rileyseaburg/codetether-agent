use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum ViewMode {
    Chat,
    Sessions,
    Swarm,
    Ralph,
    Bus,
    Model,
    Settings,
    Lsp,
    Rlm,
    Latency,
    Protocol,
    FilePicker,
    Inspector,
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

pub use crate::tui::utils::workspace::{WorkspaceEntry, WorkspaceEntryKind, WorkspaceSnapshot};
