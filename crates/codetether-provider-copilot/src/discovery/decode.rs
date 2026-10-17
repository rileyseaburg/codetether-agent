//! Bounded discovery body decoding.
use crate::*;
impl CopilotProvider {
    pub(super) async fn discovery_decode(
        &self,
        response: reqwest::Response,
    ) -> Option<CopilotModelsResponse> {
        let status = response.status();
        if !status.is_success() {
            let body = response.text().await.unwrap_or_default();
            tracing::warn!(
                provider = %self.provider_name,
                status = %status,
                body = %body.chars().take(200).collect::<String>(),
                "Copilot /models endpoint returned non-success"
            );
            return None;
        }

        let parsed: CopilotModelsResponse = match codetether_provider_core::body_cap::json_capped(
            response,
            codetether_provider_core::body_cap::PROVIDER_METADATA_BODY_CAP,
        )
        .await
        {
            Ok(p) => p,
            Err(e) => {
                tracing::warn!(provider = %self.provider_name, error = %e, "Failed to parse Copilot models response (or exceeded body cap)");
                return None;
            }
        };
        Some(parsed)
    }
}
