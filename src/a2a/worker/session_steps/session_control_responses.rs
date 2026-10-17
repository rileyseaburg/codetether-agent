//! Build deterministic responses after applying or observing controls.

use super::Mock;
use crate::provider::*;
use std::sync::atomic::Ordering;

impl Mock {
    pub(super) async fn respond(
        &self,
        request: CompletionRequest,
    ) -> anyhow::Result<CompletionResponse> {
        let call = self.calls.fetch_add(1, Ordering::SeqCst);
        let mut content = vec![ContentPart::Text {
            text: "done".into(),
        }];
        if call == 0 {
            if self.first_action().await? {
                content.push(ContentPart::ToolCall {
                    id: "never".into(),
                    name: "must_not_run".into(),
                    arguments: "{}".into(),
                    thought_signature: None,
                });
            }
        } else {
            self.observe(&request).await?;
        }
        Ok(CompletionResponse {
            message: Message {
                role: Role::Assistant,
                content,
            },
            finish_reason: FinishReason::Stop,
            usage: Usage {
                prompt_tokens: 1,
                completion_tokens: 1,
                ..Usage::default()
            },
        })
    }
}
