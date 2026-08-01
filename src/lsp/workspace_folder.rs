//! Automatic LSP workspace-folder derivation.
//!
//! Some servers (notably Ruff) ignore legacy `rootUri` and require
//! `workspaceFolders`. CodeTether previously sent none, producing
//! `No workspace(s) were provided` even inside a configured repository.
//!
//! The folder name is derived from `package.json` when available, then from the
//! directory containing `tsconfig.json`, and finally from the directory name.

use super::types::WorkspaceFolder;

/// Creates one workspace folder from a root URI.
///
/// # Examples
///
/// ```
/// use codetether_agent::lsp::workspace_folder::derive;
///
/// let folder = derive(Some("file:///tmp/example".to_string())).unwrap();
/// assert_eq!(folder.uri, "file:///tmp/example");
/// assert_eq!(folder.name, "example");
/// assert!(derive(None).is_none());
/// ```
pub fn derive(root_uri: Option<String>) -> Option<Vec<WorkspaceFolder>> {
    let uri = root_uri?;
    let path = super::uri_to_path(&uri);
    let name = package_name(&path)
        .or_else(|| manifest_root_name(&path))
        .or_else(|| path.file_name()?.to_str().map(str::to_string))?;
    Some(vec![WorkspaceFolder { uri, name }])
}

fn package_name(root: &std::path::Path) -> Option<String> {
    let text = std::fs::read_to_string(root.join("package.json")).ok()?;
    serde_json::from_str::<serde_json::Value>(&text)
        .ok()?
        .get("name")?
        .as_str()
        .map(str::to_string)
}

fn manifest_root_name(root: &std::path::Path) -> Option<String> {
    ["tsconfig.json", "tsconfig.base.json"]
        .iter()
        .any(|name| root.join(name).is_file())
        .then(|| root.file_name()?.to_str().map(str::to_string))?
}
