//! Query and cancel remote A2A tasks.
use super::*;
impl A2AClient {
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
}
