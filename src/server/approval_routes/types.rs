use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub(super) struct RequestBody {
    pub tool: String,
    pub action: String,
    pub resource: String,
    pub reason: Option<String>,
    pub tool_call_id: Option<String>,
}

#[derive(Debug, Deserialize)]
pub(super) struct DecisionBody {
    pub decision: String,
    pub actor: Option<String>,
    pub reason: Option<String>,
}
