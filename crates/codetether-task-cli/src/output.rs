use crate::types::TaskTriggerResponse;
use anyhow::Result;
pub(crate) fn print(format: &str, workspace_id: &str, payload: &TaskTriggerResponse) -> Result<()> {
    if format == "json" {
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
