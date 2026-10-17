//! Application discovery transport and its existing failure diagnostics.
use reqwest::Client;
use serde_json::Value;
pub(super) async fn fetch(client: &Client, api_key: &str, url: &str) -> Option<Value> {
    let response = match client
        .get(url)
        .header("Authorization", format!("Bearer {}", api_key))
        .send()
        .await
    {
        Ok(r) => r,
        Err(e) => {
            tracing::debug!(
                url = %url,
                error = %e,
                "Z.AI /models discovery request failed"
            );
            return None;
        }
    };

    if !response.status().is_success() {
        tracing::debug!(
            url = %url,
            status = %response.status(),
            "Z.AI /models endpoint returned non-success"
        );
        return None;
    }

    let payload: Value = match response.json().await {
        Ok(p) => p,
        Err(e) => {
            tracing::debug!(
                url = %url,
                error = %e,
                "Failed to parse Z.AI /models response"
            );
            return None;
        }
    };

    Some(payload)
}
