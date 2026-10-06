//! Persist recovered metadata and optionally execute the requested prompt.
use crate::session::Session;
use serde_json::{Value, json};

/// Save only the recovered identity, propagating storage failures at every step.
/// # Errors
/// Returns durable save failures; prompt failures are represented in the response.
pub(super) async fn finish(mut session: Session, prompt: Option<String>) -> anyhow::Result<Value> {
    session.save().await?;
    let mut response = json!({
        "session_id": session.id,
        "active_session_id": session.id,
        "status": "ready",
    });
    if let Some(prompt) = prompt.filter(|prompt| !prompt.is_empty()) {
        let result = session.prompt(&prompt).await;
        session.save().await?;
        match result {
            Ok(result) => {
                response["status"] = json!("completed");
                response["result"] = json!(result.text);
            }
            Err(error) => {
                response["status"] = json!("failed");
                response["error"] = json!(error.to_string());
            }
        }
    }
    Ok(response)
}
