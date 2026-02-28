use axum::{Json, Router, extract::State, routing::post};
use codetether_agent::tool::Tool;
use codetether_agent::tool::edit::EditTool;
use codetether_agent::tool::multiedit::MultiEditTool;
use serde_json::{Value, json};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use tempfile::tempdir;
use tokio::net::TcpListener;

#[derive(Clone)]
struct MockState {
    output: String,
    ready: Arc<AtomicBool>,
}

async fn mock_morph_handler(State(state): State<MockState>) -> Json<Value> {
    // Wait until the server is marked as ready
    while !state.ready.load(Ordering::SeqCst) {
        tokio::time::sleep(std::time::Duration::from_millis(10)).await;
    }
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
) -> anyhow::Result<(String, tokio::task::JoinHandle<()>)> {
    let listener = TcpListener::bind("127.0.0.1:0").await?;
    let addr = listener.local_addr()?;
    let ready = Arc::new(AtomicBool::new(false));
    let ready_clone = ready.clone();

    let app = Router::new()
        .route("/chat/completions", post(mock_morph_handler))
        .with_state(MockState {
            output,
            ready: ready_clone,
        });

    let handle = tokio::spawn(async move {
        // Server is now ready to accept connections
        ready.store(true, Ordering::SeqCst);
        let _ = axum::serve(listener, app).await;
    });

    // Wait for server to be fully ready
    tokio::time::sleep(std::time::Duration::from_millis(100)).await;

    Ok((format!("http://{}", addr), handle))
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

// Force tests to run serially to avoid port conflicts
#[tokio::test]
async fn morph_backed_edit_tool_flow() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("sample.txt");
    tokio::fs::write(&file_path, "line-1\nline-2\n").await?;

    let expected = "line-1\nline-2\nline-3\n".to_string();
    let (base_url, handle) = spawn_mock_morph_server(expected.clone()).await?;
    let prev = set_env_for_morph(&base_url);

    let tool = EditTool::new();
    let result = tool
        .execute(json!({
            "path": file_path.to_string_lossy().to_string(),
            "instruction": "Append line-3",
            "update": "line-3"
        }))
        .await?;

    eprintln!(
        "DEBUG edit result: success={}, output={}, metadata={:?}",
        result.success, result.output, result.metadata
    );
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

    restore_env(prev);
    handle.abort();
    // Ensure clean shutdown before next test
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    Ok(())
}

#[tokio::test]
async fn morph_backed_multiedit_tool_flow() -> anyhow::Result<()> {
    let dir = tempdir()?;
    let file_path = dir.path().join("multi.txt");
    tokio::fs::write(&file_path, "a\n").await?;

    let expected = "a\nb\n".to_string();
    let (base_url, handle) = spawn_mock_morph_server(expected.clone()).await?;
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
    eprintln!(
        "DEBUG multiedit result: success={}, output={}",
        result.success, result.output
    );
    let updated = tokio::fs::read_to_string(&file_path).await?;
    assert_eq!(updated, expected);

    restore_env(prev);
    handle.abort();
    // Ensure clean shutdown before next test
    tokio::time::sleep(std::time::Duration::from_millis(200)).await;
    Ok(())
}
