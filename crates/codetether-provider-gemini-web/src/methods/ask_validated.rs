//! Provider ask validated.
use crate::*;
impl GeminiWebProvider {
    pub(crate) async fn ask_validated(
        &self,
        prompt: &str,
        model: &str,
        tools: &[ToolDefinition],
        messages: &[Message],
    ) -> Result<(String, Vec<(String, String)>)> {
        let text = self.ask(prompt, model).await?;
        let first_error = match tool_validation::extract(&text, tools, messages) {
            Ok(parts) => return Ok(parts),
            Err(error) => error,
        };
        tracing::warn!(
            error = %format!("{first_error:#}"),
            "Gemini Web emitted invalid tool protocol; retrying once"
        );
        let corrected = prompt::retry(prompt, &format!("{first_error:#}"));
        let text = self.ask(&corrected, model).await?;
        tool_validation::extract(&text, tools, messages)
            .context("Gemini Web tool protocol retry failed")
    }
}
