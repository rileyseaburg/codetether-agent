//! In-memory tool host.
use crate::router::{HostToolResult, RouterHost};
use crate::traits::ToolDefinition;
#[derive(Default)]
pub struct Host {
    pub calls: Vec<String>,
}
impl RouterHost for Host {
    fn tool_definitions(&self) -> Vec<ToolDefinition> {
        vec![ToolDefinition {
            name: "inspect".into(),
            description: "fixture".into(),
            parameters: serde_json::json!({}),
        }]
    }
    fn dispatch(&mut self, name: &str, _arguments: &str) -> Option<HostToolResult> {
        self.calls.push(name.into());
        match name {
            "inspect" => Some(HostToolResult::Output("tool output".into())),
            "FINAL" => Some(HostToolResult::Final("tool final".into())),
            _ => None,
        }
    }
}
