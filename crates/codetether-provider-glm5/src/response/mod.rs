//! Buffered response interpretation.
use crate::*;
impl Glm5Provider {
    pub(crate) fn parse(text: &str) -> Result<CompletionResponse> {
        let parsed: Glm5Response = serde_json::from_str(text).context(format!(
            "Failed to parse GLM-5 FP8 response: {}",
            Self::preview_text(text, 200)
        ))?;

        let choice = parsed
            .choices
            .first()
            .ok_or_else(|| anyhow::anyhow!("No choices in GLM-5 FP8 response"))?;

        let (content, has_tool_calls) = content::content(choice);
        let finish_reason = finish::finish(choice, has_tool_calls);

        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            usage: parsed
                .usage
                .as_ref()
                .map(Glm5Usage::to_usage)
                .unwrap_or_default(),
            finish_reason,
        })
    }
}
mod content;
mod finish;
mod preview;
