//! Parallel tool job input.

#[derive(Clone)]
pub(crate) struct Job {
    pub tool_id: String,
    pub tool_name: String,
    pub tool_input: serde_json::Value,
    pub exec_input: serde_json::Value,
}
