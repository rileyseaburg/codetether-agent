use axum::{Json, Router, extract::State, routing::post};
use codetether_agent::tool::Tool;
use codetether_agent::tool::edit::EditTool;
use codetether_agent::tool::morph_backend;
use codetether_agent::tool::multiedit::MultiEditTool;
use serde_json::{Value, json};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
};
use tempfile::tempdir;
use tokio::net::TcpListener;

#[derive(Clone)]
struct MockState {
    output: String,
    requests: Arc<AtomicUsize>,
}

async fn mock_morph_handler(State(state): State<MockState>) -> Json<Value> {
    state.requests.fetch_add(1, Ordering::SeqCst);
    Json(json!({
        "choices": [{
            "message": {
                "content": state.output
            }
        }]
    }))
}

async fn spawn_mock_morph_server(
    output: String,
) -> anyhow::Result<(String, Arc<AtomicUsize>, tokio::task::JoinHandle<()>)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let requests = Arc::new(AtomicUsize::new(0));
    let app = Router::new()
        .route("/chat/completions", post(mock_morph_handler))
        .with_state(MockState {
            output,
            requests: requests.clone(),
        });
    let handle = tokio::spawn(async move {
        let _ = axum::serve(listener, app).await;
    });
    Ok((format!("http://{}", addr), requests, handle))
}

fn set_env_for_morph(
    base_url: &str,
) -> (
    Option<String>,
    Option<String>,
    Option<String>,
    Option<String>,
) {
    let prev_backend = std::env::var("CODETETHER_MORPH_TOOL_BACKEND").ok();
    let prev_key = std::env::var("OPENROUTER_API_KEY").ok();
    let prev_url = std::env::var("CODETETHER_OPENROUTER_BASE_URL").ok();
    let prev_model = std::env::var("CODETETHER_MORPH_TOOL_MODEL").ok();
    unsafe {
        std::env::set_var("CODETETHER_MORPH_TOOL_BACKEND", "1");
        std::env::set_var("OPENROUTER_API_KEY", "test-key");
        std::env::set_var("CODETETHER_OPENROUTER_BASE_URL", base_url);
        std::env::set_var("CODETETHER_MORPH_TOOL_MODEL", "morph/morph-v3-large");
    }
    (prev_backend, prev_key, prev_url, prev_model)
}

fn restore_env(
    prev: (
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ),
) {
    let (prev_backend, prev_key, prev_url, prev_model) = prev;
    unsafe {
        match prev_backend {
            Some(v) => std::env::set_var("CODETETHER_MORPH_TOOL_BACKEND", v),
            None => std::env::remove_var("CODETETHER_MORPH_TOOL_BACKEND"),
        }
        match prev_key {
            Some(v) => std::env::set_var("OPENROUTER_API_KEY", v),
            None => std::env::remove_var("OPENROUTER_API_KEY"),
        }
        match prev_url {
            Some(v) => std::env::set_var("CODETETHER_OPENROUTER_BASE_URL", v),
            None => std::env::remove_var("CODETETHER_OPENROUTER_BASE_URL"),
        }
        match prev_model {
            Some(v) => std::env::set_var("CODETETHER_MORPH_TOOL_MODEL", v),
            None => std::env::remove_var("CODETETHER_MORPH_TOOL_MODEL"),
        }
    }
}

fn restore_backend_env(prev_backend: Option<String>) {
    unsafe {
        match prev_backend {
            Some(v) => std::env::set_var("CODETETHER_MORPH_TOOL_BACKEND", v),
            None => std::env::remove_var("CODETETHER_MORPH_TOOL_BACKEND"),
        }
    }
}

/// RAII guard to restore environment variable on drop
struct EnvGuard {
    key: &'static str,
    prev_value: Option<String>,
}

impl EnvGuard {
    fn new(key: &'static str) -> Self {
        let prev_value = std::env::var(key).ok();
        Self { key, prev_value }
    }
}

impl Drop for EnvGuard {
    fn drop(&mut self) {
        unsafe {
            match &self.prev_value {
                Some(v) => std::env::set_var(self.key, v),
                None => std::env::remove_var(self.key),
            }
        }
    }
}

#[test]
fn morph_backend_is_opt_in() {
    let _guard = EnvGuard::new("CODETETHER_MORPH_TOOL_BACKEND");
    unsafe {
        std::env::remove_var("CODETETHER_MORPH_TOOL_BACKEND");
    }

    assert!(!morph_backend::should_use_morph_backend());
    // _guard restores env var on drop, even if assertion panics
}

#[tokio::test]
async fn morph_backed_edit_tool_flow() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("sample.txt");
    tokio::fs::write(&file_path, "line-1\nline-2\n").await?;

    let expected = "line-1\nline-2\nline-3\n".to_string();
    let (base_url, requests, handle) = spawn_mock_morph_server(expected.clone()).await?;
    let prev = set_env_for_morph(&base_url);

    let tool = EditTool::new();
    let result = tool
        .execute(json!({
            "path": file_path.to_string_lossy().to_string(),
            "instruction": "Append line-3",
            "update": "line-3"
        }))
        .await?;

    assert!(result.success, "{}", result.output);
    assert_eq!(
        result
            .metadata
            .get("backend")
            .and_then(|v| v.as_str())
            .unwrap_or_default(),
        "morph"
    );
    assert!(
        result
            .metadata
            .get("new_string")
            .and_then(|v| v.as_str())
            .unwrap_or_default()
            .contains("line-3")
    );
    assert_eq!(requests.load(Ordering::SeqCst), 1);

    restore_env(prev);
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn exact_replace_edit_skips_morph_even_when_enabled() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("exact-edit.txt");
    tokio::fs::write(&file_path, "alpha\nbeta\n").await?;

    let (base_url, requests, handle) =
        spawn_mock_morph_server("this should never be returned".to_string()).await?;
    let prev = set_env_for_morph(&base_url);

    let tool = EditTool::new();
    let result = tool
        .execute(json!({
            "path": file_path.to_string_lossy().to_string(),
            "old_string": "beta",
            "new_string": "gamma"
        }))
        .await?;

    assert!(result.success, "{}", result.output);
    assert!(result.metadata.get("backend").is_none());
    assert_eq!(requests.load(Ordering::SeqCst), 0);

    restore_env(prev);
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn morph_backed_multiedit_tool_flow() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("multi.txt");
    tokio::fs::write(&file_path, "a\n").await?;

    let expected = "a\nb\n".to_string();
    let (base_url, requests, handle) = spawn_mock_morph_server(expected.clone()).await?;
    let prev = set_env_for_morph(&base_url);

    let tool = MultiEditTool::new();
    let result = tool
        .execute(json!({
            "edits": [{
                "file": file_path.to_string_lossy().to_string(),
                "instruction": "Append b",
                "update": "b"
            }]
        }))
        .await?;

    assert!(result.success, "{}", result.output);
    let updated = tokio::fs::read_to_string(&file_path).await?;
    assert_eq!(updated, expected);
    assert_eq!(requests.load(Ordering::SeqCst), 1);

    restore_env(prev);
    handle.abort();
    Ok(())
}

#[tokio::test]
async fn exact_replace_multiedit_skips_morph_even_when_enabled() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("multi-exact.txt");
    tokio::fs::write(&file_path, "a\n").await?;

    let expected = "a\nb\n".to_string();
    let (base_url, requests, handle) =
        spawn_mock_morph_server("this should never be written".to_string()).await?;
    let prev = set_env_for_morph(&base_url);

    let tool = MultiEditTool::new();
    let result = tool
        .execute(json!({
            "edits": [{
                "file": file_path.to_string_lossy().to_string(),
                "old_string": "a\n",
                "new_string": "a\nb\n"
            }]
        }))
        .await?;

    assert!(result.success, "{}", result.output);
    let updated = tokio::fs::read_to_string(&file_path).await?;
    assert_eq!(updated, expected);
    assert_eq!(requests.load(Ordering::SeqCst), 0);

    restore_env(prev);
    handle.abort();
    Ok(())
}