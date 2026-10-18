//! ESLint request preparation and normalized diagnostic rendering.
//!
//! [request] selects supported extensions and builds the command payload.
//! [render] decodes JSON stdout. Native callers own authorization and execution.
mod message;
mod request;
mod types;
/// Prepare an ESLint request for a workspace-relative source file.
/// # Arguments
/// Receives workspace root and source path.
/// # Returns
/// Command/cwd JSON for supported extensions, otherwise None.
/// # Examples
/// ```
/// use std::path::Path;
/// assert!(codetether_validation::eslint::request(Path::new("/workspace"), Path::new("/workspace/a.ts")).is_some());
/// ```
pub fn request(
    workspace_dir: &std::path::Path,
    path: &std::path::Path,
) -> Option<serde_json::Value> {
    request::request(workspace_dir, path)
}
/// Decode ESLint stdout into the validation pipeline's diagnostic strings.
/// # Arguments
/// Receives raw stdout from the native sandbox command.
/// # Returns
/// Rendered messages in source order, or an empty vector for invalid JSON.
/// # Examples
/// ```
/// assert!(codetether_validation::eslint::render("[]").is_empty());
/// ```
pub fn render(output: &str) -> Vec<String> {
    let Ok(reports) = serde_json::from_str::<Vec<types::FileReport>>(output) else {
        return Vec::new();
    };
    reports
        .into_iter()
        .flat_map(types::FileReport::render)
        .collect()
}
