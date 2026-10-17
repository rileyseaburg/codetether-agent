//! MCP server catalog read_resource operations.
use super::*;
impl ServerCatalog {
    /// Handle read resource request
    pub async fn handle_read_resource(&self, params: Option<Value>) -> Result<Value, JsonRpcError> {
        let params: ReadResourceParams = if let Some(p) = params {
            serde_json::from_value(p).map_err(|e| JsonRpcError::invalid_params(e.to_string()))?
        } else {
            return Err(JsonRpcError::invalid_params("Missing params"));
        };

        let resources = self.resources.read().await;
        let handler = resources
            .get(&params.uri)
            .ok_or_else(|| JsonRpcError::method_not_found(&params.uri))?;

        match handler(params.uri.clone()) {
            Ok(result) => serde_json::to_value(result)
                .map_err(|e| JsonRpcError::internal_error(e.to_string())),
            Err(e) => Err(JsonRpcError::internal_error(e.to_string())),
        }
    }
}
