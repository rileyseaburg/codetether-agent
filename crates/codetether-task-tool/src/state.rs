//! state for in-memory task tracking.

#[derive(Debug, Clone)]
pub(super) struct TaskInfo {
    pub(super) id: String,
    pub(super) description: String,
    pub(super) status: ToolTaskStatus,
    pub(super) result: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub(super) enum ToolTaskStatus {
    Pending,
    #[allow(dead_code)]
    Running,
    Complete,
    Failed,
}
