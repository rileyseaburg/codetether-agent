//! Execute the model request with the legacy timeout and errors.
use super::*;
impl RlmExecutor {
    pub(super) async fn request_analysis(
        &self,
        messages: &[Message],
        tools: &[codetether_provider_core::ToolDefinition],
        llm_timeout_secs: u64,
    ) -> Result<codetether_provider_core::CompletionResponse> {
        tracing::debug!("Sending LLM request...");
        let response = match tokio::time::timeout(
            std::time::Duration::from_secs(llm_timeout_secs),
            self.provider.complete(CompletionRequest {
                messages: messages.to_vec(),
                tools: tools.to_vec(),
                model: self.model.clone(),
                temperature: Some(self.analysis_temperature),
                top_p: None,
                max_tokens: Some(2000),
                stop: vec![],
            }),
        )
        .await
        {
            Ok(Ok(r)) => {
                tracing::debug!("LLM response received");
                r
            }
            Ok(Err(e)) => return Err(e),
            Err(_) => {
                return Err(anyhow::anyhow!(
                    "LLM request timed out after {} seconds",
                    llm_timeout_secs
                ));
            }
        };

        Ok(response)
    }
}
