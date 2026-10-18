use crate::{
    TaskArgs, credentials::resolve_server_and_token, files::relativize_files,
    paths::resolve_workspace_root, title::summarize_title, types::TriggerRequest,
    workspace::resolve_workspace_id,
};
use anyhow::Result;
use reqwest::Client;
use std::path::PathBuf;
/// Submit a remote task using explicit or saved server credentials.
///
/// # Arguments
/// * `args` - Task request and workspace/output options.
/// * `default_server` - Server override before saved credentials.
/// * `default_token` - Token override before matching saved credentials.
/// * `global_project` - Workspace fallback before the current directory.
/// # Returns
/// Completes after printing the task trigger response.
/// # Errors
/// Returns credential, path, workspace lookup, HTTP or response errors.
/// # Examples
/// ```no_run
/// # async fn example(args: codetether_task_cli::TaskArgs) -> anyhow::Result<()> {
/// codetether_task_cli::execute(args, None, None, None).await
/// # }
/// ```

pub async fn execute(
    args: TaskArgs,
    default_server: Option<String>,
    default_token: Option<String>,
    global_project: Option<PathBuf>,
) -> Result<()> {
    let (server, token) = resolve_server_and_token(default_server, default_token)?;
    let client = Client::new();

    let workspace_id = match args.workspace_id.clone() {
        Some(workspace_id) => workspace_id,
        None => {
            let workspace_root =
                resolve_workspace_root(args.workspace.as_ref(), global_project.as_ref())?;
            resolve_workspace_id(&client, &server, token.as_deref(), &workspace_root).await?
        }
    };

    let workspace_root = resolve_workspace_root(args.workspace.as_ref(), global_project.as_ref())?;
    let files = relativize_files(&workspace_root, &args.file)?;
    let title = args.title.unwrap_or_else(|| summarize_title(&args.prompt));

    let mut metadata = serde_json::Map::new();
    metadata.insert("title".to_string(), serde_json::Value::String(title));

    let request = TriggerRequest {
        prompt: args.prompt,
        agent: args.agent,
        model: args.model,
        files,
        worker_personality: args.worker_personality,
        notify_email: args.notify_email,
        metadata,
    };

    let payload =
        crate::submit::submit(&client, &server, token.as_deref(), &workspace_id, &request).await?;
    crate::output::print(&args.format, &workspace_id, &payload)?;
    Ok(())
}
