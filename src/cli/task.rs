use super::TaskArgs;
use crate::cli::auth::load_saved_credentials;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Deserialize)]
struct WorkspaceSummary {
    id: String,
    path: Option<String>,
}

#[derive(Debug, Serialize)]
struct TriggerRequest {
    prompt: String,
    agent: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    model: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    files: Vec<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    worker_personality: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    notify_email: Option<String>,
    #[serde(default)]
    metadata: serde_json::Map<String, serde_json::Value>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaskTriggerResponse {
    pub success: bool,
    pub session_id: Option<String>,
    pub message: Option<String>,
    pub workspace_id: Option<String>,
    pub agent: Option<String>,
    pub knative: Option<bool>,
    pub error: Option<String>,
    pub routing: Option<serde_json::Value>,
}

pub async fn execute(
    args: TaskArgs,
    default_server: Option<String>,
    default_token: Option<String>,
    global_project: Option<PathBuf>,
) -> Result<()> {
    let (server, token) = resolve_server_and_token(default_server, default_token)?;
    let client = crate::provider::shared_http::shared_client().clone();

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

    let url = format!(
        "{}/v1/agent/workspaces/{}/trigger",
        server,
        urlencoding::encode(&workspace_id)
    );
    let mut req = client.post(url).json(&request);
    if let Some(token) = token.as_deref() {
        req = req.bearer_auth(token);
    }

    let response = req
        .send()
        .await
        .context("Failed to reach CodeTether server")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Task trigger failed ({}): {}", status, body);
    }

    let payload: TaskTriggerResponse = response
        .json()
        .await
        .context("Failed to parse task trigger response")?;

    if args.format == "json" {
        println!("{}", serde_json::to_string_pretty(&payload)?);
    } else {
        println!(
            "Queued task for workspace {}{}",
            workspace_id,
            if payload.knative.unwrap_or(false) {
                " via Knative"
            } else {
                ""
            }
        );
        if let Some(session_id) = payload.session_id.as_deref() {
            println!("Session ID: {}", session_id);
        }
        if let Some(message) = payload.message.as_deref() {
            println!("{}", message);
        }
    }

    Ok(())
}

fn resolve_server_and_token(
    default_server: Option<String>,
    default_token: Option<String>,
) -> Result<(String, Option<String>)> {
    let saved = load_saved_credentials();

    let server = default_server
        .or_else(|| saved.as_ref().map(|creds| creds.server.clone()))
        .map(|value| value.trim().trim_end_matches('/').to_string())
        .filter(|value| !value.is_empty())
        .ok_or_else(|| {
            anyhow::anyhow!(
                "No CodeTether server configured. Pass --server or run `codetether auth login`."
            )
        })?;

    let token = match default_token {
        Some(token) => Some(token),
        None => saved.and_then(|creds| {
            let saved_server = creds.server.trim().trim_end_matches('/');
            if saved_server == server {
                Some(creds.access_token)
            } else {
                None
            }
        }),
    };

    Ok((server, token))
}

fn resolve_workspace_root(
    explicit_workspace: Option<&PathBuf>,
    global_project: Option<&PathBuf>,
) -> Result<PathBuf> {
    let base = explicit_workspace
        .cloned()
        .or_else(|| global_project.cloned())
        .unwrap_or(std::env::current_dir().context("Failed to resolve current directory")?);

    normalize_local_path(&base)
}

fn normalize_local_path(path: &Path) -> Result<PathBuf> {
    let expanded = if path.is_absolute() {
        path.to_path_buf()
    } else {
        std::env::current_dir()
            .context("Failed to resolve current directory")?
            .join(path)
    };

    Ok(expanded)
}

async fn resolve_workspace_id(
    client: &Client,
    server: &str,
    token: Option<&str>,
    workspace_root: &Path,
) -> Result<String> {
    let workspace_root = workspace_root.to_string_lossy().to_string();
    let mut req = client.get(format!("{}/v1/agent/workspaces", server));
    if let Some(token) = token {
        req = req.bearer_auth(token);
    }

    let response = req
        .send()
        .await
        .context("Failed to list workspaces from CodeTether server")?;

    if !response.status().is_success() {
        let status = response.status();
        let body = response.text().await.unwrap_or_default();
        anyhow::bail!("Workspace lookup failed ({}): {}", status, body);
    }

    let workspaces: Vec<WorkspaceSummary> = response
        .json()
        .await
        .context("Failed to parse workspace list")?;

    match best_workspace_match(&workspace_root, &workspaces) {
        Some(workspace) => Ok(workspace.id.clone()),
        None => anyhow::bail!(
            "No registered workspace matched '{}'. Register this workspace first or pass --workspace-id.",
            workspace_root
        ),
    }
}

fn best_workspace_match<'a>(
    workspace_root: &str,
    workspaces: &'a [WorkspaceSummary],
) -> Option<&'a WorkspaceSummary> {
    let direct = workspaces
        .iter()
        .filter_map(|workspace| {
            let path = workspace.path.as_deref()?;
            if workspace_root == path || workspace_root.starts_with(&format!("{}/", path)) {
                Some((path.len(), workspace))
            } else {
                None
            }
        })
        .max_by_key(|(path_len, _)| *path_len)
        .map(|(_, workspace)| workspace);

    if direct.is_some() {
        return direct;
    }

    let mut scored: Vec<(usize, &WorkspaceSummary)> = workspaces
        .iter()
        .filter_map(|workspace| {
            let path = workspace.path.as_deref()?;
            let score = shared_path_suffix_score(workspace_root, path);
            (score > 0).then_some((score, workspace))
        })
        .collect();

    scored.sort_by(|left, right| right.0.cmp(&left.0));

    match scored.as_slice() {
        [] => None,
        [(score, workspace), ..] => {
            let is_unique_best = scored
                .get(1)
                .map(|(next_score, _)| next_score < score)
                .unwrap_or(true);
            if is_unique_best {
                Some(*workspace)
            } else {
                None
            }
        }
    }
}

fn shared_path_suffix_score(left: &str, right: &str) -> usize {
    let left_parts: Vec<&str> = left.split('/').filter(|part| !part.is_empty()).collect();
    let right_parts: Vec<&str> = right.split('/').filter(|part| !part.is_empty()).collect();

    let mut score = 0usize;
    for (left_part, right_part) in left_parts.iter().rev().zip(right_parts.iter().rev()) {
        if left_part == right_part {
            score += 1;
        } else {
            break;
        }
    }

    score
}

fn relativize_files(workspace_root: &Path, files: &[PathBuf]) -> Result<Vec<String>> {
    let mut relative = Vec::with_capacity(files.len());
    for file in files {
        let normalized = normalize_local_path(file)?;
        let path = normalized.strip_prefix(workspace_root).map_err(|_| {
            anyhow::anyhow!(
                "Attached file '{}' is not inside workspace '{}'.",
                normalized.display(),
                workspace_root.display()
            )
        })?;
        relative.push(path.to_string_lossy().to_string());
    }
    Ok(relative)
}

fn summarize_title(prompt: &str) -> String {
    const LIMIT: usize = 80;
    let trimmed = prompt.trim();
    if trimmed.chars().count() <= LIMIT {
        trimmed.to_string()
    } else {
        let mut title: String = trimmed.chars().take(LIMIT).collect();
        title.push_str("...");
        title
    }
}

#[cfg(test)]
mod tests {
    use super::{
        WorkspaceSummary, best_workspace_match, shared_path_suffix_score, summarize_title,
    };

    #[test]
    fn picks_longest_workspace_prefix_match() {
        let workspaces = vec![
            WorkspaceSummary {
                id: "root".to_string(),
                path: Some("/home/riley".to_string()),
            },
            WorkspaceSummary {
                id: "repo".to_string(),
                path: Some("/home/riley/A2A-Server-MCP".to_string()),
            },
        ];

        let matched = best_workspace_match("/home/riley/A2A-Server-MCP/subdir", &workspaces)
            .expect("expected workspace match");
        assert_eq!(matched.id, "repo");
    }

    #[test]
    fn summarize_title_truncates_long_prompts() {
        let prompt = "x".repeat(120);
        let title = summarize_title(&prompt);
        assert!(title.ends_with("..."));
        assert_eq!(title.chars().count(), 83);
    }

    #[test]
    fn shared_path_suffix_score_handles_container_prefix_remap() {
        assert_eq!(
            shared_path_suffix_score(
                "/home/riley/A2A-Server-MCP",
                "/app/home/riley/A2A-Server-MCP"
            ),
            3
        );
    }

    #[test]
    fn falls_back_to_unique_suffix_match() {
        let workspaces = vec![
            WorkspaceSummary {
                id: "repo".to_string(),
                path: Some("/app/home/riley/A2A-Server-MCP".to_string()),
            },
            WorkspaceSummary {
                id: "other".to_string(),
                path: Some("/srv/spotlessbinco".to_string()),
            },
        ];

        let matched = best_workspace_match("/home/riley/A2A-Server-MCP", &workspaces)
            .expect("expected suffix workspace match");
        assert_eq!(matched.id, "repo");
    }
}
