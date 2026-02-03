//! A2A Worker - connects to an A2A server to process tasks

use crate::cli::A2aArgs;
use crate::session::Session;
use anyhow::Result;
use futures::StreamExt;
use reqwest::Client;
use std::collections::HashSet;
use std::sync::Arc;
use tokio::sync::Mutex;

/// Run the A2A worker
pub async fn run(args: A2aArgs) -> Result<()> {
    let server = args.server.trim_end_matches('/');
    let name = args.name.unwrap_or_else(|| format!("codetether-{}", std::process::id()));
    let worker_id = generate_worker_id();
    
    let codebases: Vec<String> = args
        .codebases
        .map(|c| c.split(',').map(|s| s.trim().to_string()).collect())
        .unwrap_or_else(|| vec![std::env::current_dir().unwrap().display().to_string()]);

    tracing::info!(
        "Starting A2A worker: {} ({})",
        name,
        worker_id
    );
    tracing::info!("Server: {}", server);
    tracing::info!("Codebases: {:?}", codebases);

    let client = Client::new();
    let processing = Arc::new(Mutex::new(HashSet::<String>::new()));
    
    let auto_approve = match args.auto_approve.as_str() {
        "all" => AutoApprove::All,
        "safe" => AutoApprove::Safe,
        _ => AutoApprove::None,
    };

    // Register worker
    register_worker(&client, server, &args.token, &worker_id, &name, &codebases).await?;

    // Fetch pending tasks
    fetch_pending_tasks(&client, server, &args.token, &worker_id, &processing, &auto_approve).await?;

    // Connect to SSE stream
    loop {
        match connect_stream(&client, server, &args.token, &worker_id, &name, &codebases, &processing, &auto_approve).await {
            Ok(()) => {
                tracing::warn!("Stream ended, reconnecting...");
            }
            Err(e) => {
                tracing::error!("Stream error: {}, reconnecting...", e);
            }
        }
        tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
    }
}

fn generate_worker_id() -> String {
    format!(
        "wrk_{}_{:x}",
        chrono::Utc::now().timestamp(),
        rand::random::<u64>()
    )
}

#[derive(Debug, Clone, Copy)]
enum AutoApprove {
    All,
    Safe,
    None,
}

async fn register_worker(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
) -> Result<()> {
    let mut req = client.put(format!("{}/v1/worker/codebases", server));
    
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req
        .json(&serde_json::json!({
            "codebases": codebases,
            "worker_id": worker_id,
            "agent_name": name,
        }))
        .send()
        .await?;

    if res.status().is_success() {
        tracing::info!("Worker registered successfully");
    } else {
        tracing::warn!("Failed to register worker: {}", res.status());
    }

    Ok(())
}

async fn fetch_pending_tasks(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    processing: &Arc<Mutex<HashSet<String>>>,
    auto_approve: &AutoApprove,
) -> Result<()> {
    tracing::info!("Checking for pending tasks...");
    
    let mut req = client.get(format!("{}/v1/opencode/tasks?status=pending", server));
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        return Ok(());
    }

    let data: serde_json::Value = res.json().await?;
    let tasks = data["tasks"].as_array().cloned().unwrap_or_default();
    
    tracing::info!("Found {} pending task(s)", tasks.len());

    for task in tasks {
        if let Some(id) = task["id"].as_str() {
            let mut proc = processing.lock().await;
            if !proc.contains(id) {
                proc.insert(id.to_string());
                drop(proc);
                
                let task_id = id.to_string();
                let client = client.clone();
                let server = server.to_string();
                let token = token.clone();
                let worker_id = worker_id.to_string();
                let auto_approve = *auto_approve;
                let processing = processing.clone();
                
                tokio::spawn(async move {
                    if let Err(e) = handle_task(&client, &server, &token, &worker_id, &task, auto_approve).await {
                        tracing::error!("Task {} failed: {}", task_id, e);
                    }
                    processing.lock().await.remove(&task_id);
                });
            }
        }
    }

    Ok(())
}

async fn connect_stream(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    name: &str,
    codebases: &[String],
    processing: &Arc<Mutex<HashSet<String>>>,
    auto_approve: &AutoApprove,
) -> Result<()> {
    let url = format!(
        "{}/v1/worker/tasks/stream?agent_name={}&worker_id={}",
        server,
        urlencoding::encode(name),
        urlencoding::encode(worker_id)
    );

    let mut req = client.get(&url)
        .header("Accept", "text/event-stream")
        .header("X-Worker-ID", worker_id)
        .header("X-Agent-Name", name)
        .header("X-Codebases", codebases.join(","));

    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req.send().await?;
    if !res.status().is_success() {
        anyhow::bail!("Failed to connect: {}", res.status());
    }

    tracing::info!("Connected to A2A server");

    let mut stream = res.bytes_stream();
    let mut buffer = String::new();

    while let Some(chunk) = stream.next().await {
        let chunk = chunk?;
        buffer.push_str(&String::from_utf8_lossy(&chunk));

        // Process SSE events
        while let Some(pos) = buffer.find("\n\n") {
            let event_str = buffer[..pos].to_string();
            buffer = buffer[pos + 2..].to_string();

            if let Some(data_line) = event_str.lines().find(|l| l.starts_with("data:")) {
                let data = data_line.trim_start_matches("data:").trim();
                if data == "[DONE]" || data.is_empty() {
                    continue;
                }

                if let Ok(task) = serde_json::from_str::<serde_json::Value>(data) {
                    if let Some(id) = task.get("task").and_then(|t| t["id"].as_str())
                        .or_else(|| task["id"].as_str())
                    {
                        let mut proc = processing.lock().await;
                        if !proc.contains(id) {
                            proc.insert(id.to_string());
                            drop(proc);

                            let task_id = id.to_string();
                            let client = client.clone();
                            let server = server.to_string();
                            let token = token.clone();
                            let worker_id = worker_id.to_string();
                            let auto_approve = *auto_approve;
                            let processing_clone = processing.clone();

                            tokio::spawn(async move {
                                if let Err(e) = handle_task(&client, &server, &token, &worker_id, &task, auto_approve).await {
                                    tracing::error!("Task {} failed: {}", task_id, e);
                                }
                                processing_clone.lock().await.remove(&task_id);
                            });
                        }
                    }
                }
            }
        }
    }

    Ok(())
}

async fn handle_task(
    client: &Client,
    server: &str,
    token: &Option<String>,
    worker_id: &str,
    task: &serde_json::Value,
    _auto_approve: AutoApprove,
) -> Result<()> {
    let task_id = task.get("task").and_then(|t| t["id"].as_str())
        .or_else(|| task["id"].as_str())
        .ok_or_else(|| anyhow::anyhow!("No task ID"))?;
    let title = task.get("task").and_then(|t| t["title"].as_str())
        .or_else(|| task["title"].as_str())
        .unwrap_or("Untitled");

    tracing::info!("Handling task: {} ({})", title, task_id);

    // Claim the task
    let mut req = client.post(format!("{}/v1/worker/tasks/claim", server))
        .header("X-Worker-ID", worker_id);
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    let res = req
        .json(&serde_json::json!({ "task_id": task_id }))
        .send()
        .await?;

    if !res.status().is_success() {
        let text = res.text().await?;
        tracing::warn!("Failed to claim task: {}", text);
        return Ok(());
    }

    tracing::info!("Claimed task: {}", task_id);

    // Create a session and process the task
    let session = Session::new().await?;
    let prompt = task.get("task").and_then(|t| t["prompt"].as_str())
        .or_else(|| task["prompt"].as_str())
        .or_else(|| task.get("task").and_then(|t| t["description"].as_str()))
        .or_else(|| task["description"].as_str())
        .unwrap_or(title);

    // TODO: Actually execute the agent here
    tracing::info!("Would execute prompt: {}", prompt);
    let result = format!("Task {} processed (session: {})", task_id, session.id);

    // Release the task
    let mut req = client.post(format!("{}/v1/worker/tasks/release", server))
        .header("X-Worker-ID", worker_id);
    if let Some(t) = token {
        req = req.bearer_auth(t);
    }

    req.json(&serde_json::json!({
        "task_id": task_id,
        "status": "completed",
        "result": result,
    }))
    .send()
    .await?;

    tracing::info!("Task completed: {}", task_id);
    Ok(())
}

// Add rand for worker ID generation
use rand;
