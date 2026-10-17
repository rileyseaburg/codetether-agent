//! Scripted completion queues and captured message history.
use crate::traits::{LlmMessage, LlmProvider, LlmResponse, ToolDefinition};
use std::{collections::VecDeque, sync::Mutex};
pub struct Provider {
    pub queue: Mutex<VecDeque<Result<LlmResponse, String>>>,
    pub calls: Mutex<Vec<serde_json::Value>>,
}
impl Provider {
    pub fn new(answers: Vec<Result<LlmResponse, String>>) -> Self {
        Self {
            queue: Mutex::new(answers.into()),
            calls: Mutex::new(Vec::new()),
        }
    }
}
#[async_trait::async_trait]
impl LlmProvider for Provider {
    async fn complete(
        &self,
        messages: Vec<LlmMessage>,
        tools: Vec<ToolDefinition>,
        model: &str,
        temperature: Option<f32>,
    ) -> anyhow::Result<LlmResponse> {
        self.calls.lock().unwrap().push(serde_json::json!({"model":model,"temperature":temperature,"tools":tools.len(),
            "messages":messages.iter().map(|m|serde_json::json!({"role":m.role,"text":m.text,"tool_calls":m.tool_calls.len(),"tool_call_id":m.tool_call_id})).collect::<Vec<_>>()}));
        self.queue
            .lock()
            .unwrap()
            .pop_front()
            .expect("unexpected completion")
            .map_err(anyhow::Error::msg)
    }
}
pub fn text(value: &str) -> Result<LlmResponse, String> {
    Ok(LlmResponse {
        text: value.into(),
        tool_calls: vec![],
        finish_reason: None,
        input_tokens: 1,
        output_tokens: 2,
    })
}
