//! initialize operations for language server clients.

use super::*;

impl LspClient {
    /// Initialize the language server
    pub async fn initialize(&self) -> Result<()> {
        let root_uri = self.config.root_uri.clone();

        let params = InitializeParams {
            process_id: Some(std::process::id() as i64),
            client_info: ClientInfo {
                name: "codetether".to_string(),
                version: env!("CARGO_PKG_VERSION").to_string(),
            },
            locale: None,
            root_path: None,
            root_uri: root_uri.clone(),
            initialization_options: self.config.initialization_options.clone(),
            capabilities: super::super::client_capabilities::build(),
            trace: None,
            workspace_folders: super::super::workspace_folder::derive(root_uri),
        };

        let response = self
            .transport
            .request("initialize", Some(serde_json::to_value(params)?))
            .await?;

        if let Some(error) = response.error {
            return Err(anyhow::anyhow!("LSP initialize error: {}", error.message));
        }

        if let Some(result) = response.result {
            let init_result: InitializeResult = serde_json::from_value(result)?;
            *self.server_capabilities.write().await = Some(init_result.capabilities);
            info!(server_info = ?init_result.server_info, "LSP server initialized");
        }

        self.transport.notify("initialized", None).await?;
        self.transport.set_initialized(true);

        Ok(())
    }
}
