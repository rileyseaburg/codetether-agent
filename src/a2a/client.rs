//! A2A Client - for connecting to other A2A agents

use super::types::*;
use anyhow::Result;
use reqwest::Client;

/// A2A Client for interacting with A2A agents
pub struct A2AClient {
    client: Client,
    base_url: String,
    token: Option<String>,
}

#[allow(dead_code)]
impl A2AClient {
    /// Create a new A2A client
    pub fn new(base_url: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            base_url: base_url.into().trim_end_matches('/').to_string(),
            token: None,
        }
    }

    /// Set authentication token
    pub fn with_token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    /// Get the agent card
    pub async fn get_agent_card(&self) -> Result<AgentCard> {
        let url = format!("{}/.well-known/agent.json", self.base_url);
        let res = self.client.get(&url).send().await?;
        let card: AgentCard = res.json().await?;
        Ok(card)
    }

    /// Send a message to the agent
    pub async fn send_message(&self, params: MessageSendParams) -> Result<Task> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            method: "message/send".to_string(),
            params: serde_json::to_value(&params)?,
        };

        let response = self.call_rpc(request).await?;

        if let Some(error) = response.error {
            anyhow::bail!("RPC error {}: {}", error.code, error.message);
        }

        let task: Task = serde_json::from_value(
            response
                .result
                .ok_or_else(|| anyhow::anyhow!("No result"))?,
        )?;
        Ok(task)
    }

    /// Get task status
    #[allow(dead_code)]
    pub async fn get_task(&self, id: &str, history_length: Option<usize>) -> Result<Task> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            method: "tasks/get".to_string(),
            params: serde_json::to_value(TaskQueryParams {
                id: id.to_string(),
                history_length,
            })?,
        };

        let response = self.call_rpc(request).await?;

        if let Some(error) = response.error {
            anyhow::bail!("RPC error {}: {}", error.code, error.message);
        }

        let task: Task = serde_json::from_value(
            response
                .result
                .ok_or_else(|| anyhow::anyhow!("No result"))?,
        )?;
        Ok(task)
    }

    /// Cancel a task
    #[allow(dead_code)]
    pub async fn cancel_task(&self, id: &str) -> Result<Task> {
        let request = JsonRpcRequest {
            jsonrpc: "2.0".to_string(),
            id: serde_json::json!(1),
            method: "tasks/cancel".to_string(),
            params: serde_json::json!({ "id": id }),
        };

        let response = self.call_rpc(request).await?;

        if let Some(error) = response.error {
            anyhow::bail!("RPC error {}: {}", error.code, error.message);
        }

        let task: Task = serde_json::from_value(
            response
                .result
                .ok_or_else(|| anyhow::anyhow!("No result"))?,
        )?;
        Ok(task)
    }

    /// Make a JSON-RPC call
    pub async fn call_rpc(&self, request: JsonRpcRequest) -> Result<JsonRpcResponse> {
        let mut req = self.client.post(&self.base_url);

        if let Some(ref token) = self.token {
            req = req.bearer_auth(token);
        }

        let res = req
            .header("Content-Type", "application/json")
            .json(&request)
            .send()
            .await?;

        let response: JsonRpcResponse = res.json().await?;
        Ok(response)
    }
}
