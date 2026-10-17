//! Initialization request construction with explicit client identity.

use crate::{ClientInfo, InitializeParams, LspConfig};

pub(super) fn build(config: &LspConfig, client_version: &str) -> InitializeParams {
    let root_uri = config.root_uri.clone();

    let params = InitializeParams {
        process_id: Some(std::process::id() as i64),
        client_info: ClientInfo {
            name: "codetether".to_string(),
            version: client_version.to_string(),
        },
        locale: None,
        root_path: None,
        root_uri: root_uri.clone(),
        initialization_options: config.initialization_options.clone(),
        capabilities: crate::client_capabilities::build(),
        trace: None,
        workspace_folders: crate::workspace_folder::derive(root_uri),
    };

    params
}
