//! Completion text and API error handling.
use crate::*;
pub(super) async fn read(response: reqwest::Response) -> Result<String> {
    let status = response.status();
    let text = response
        .text()
        .await
        .context("Failed to read Copilot response")?;

    if !status.is_success() {
        if let Ok(err) = serde_json::from_str::<CopilotErrorResponse>(&text) {
            let message = err
                .error
                .and_then(|detail| {
                    detail.message.map(|msg| {
                        if let Some(code) = detail.code {
                            format!("{} ({})", msg, code)
                        } else {
                            msg
                        }
                    })
                })
                .or(err.message)
                .unwrap_or_else(|| "Unknown Copilot API error".to_string());
            anyhow::bail!("Copilot API error: {}", message);
        }
        anyhow::bail!("Copilot API error: {} {}", status, text);
    }
    Ok(text)
}
