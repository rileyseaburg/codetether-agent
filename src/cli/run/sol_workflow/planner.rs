//! One-shot, tool-free completions from the Sol planning model.

use anyhow::Result;

use super::model::SOL_PLANNER_MODEL;
use crate::provider::{CompletionRequest, ContentPart, Message, ProviderRegistry, Role};

/// Ask Sol for a plan or diagnostic review without advertising any tools.
pub(super) async fn complete(system: String, user: String) -> Result<String> {
    let registry = ProviderRegistry::shared_from_vault().await?;
    let (provider, model) = registry.resolve_model(SOL_PLANNER_MODEL)?;
    let response = provider.complete(request(model, system, user)).await?;
    Ok(text(&response.message.content))
}

fn request(model: String, system: String, user: String) -> CompletionRequest {
    CompletionRequest {
        model,
        messages: vec![message(Role::System, system), message(Role::User, user)],
        tools: Vec::new(),
        temperature: None,
        top_p: None,
        max_tokens: Some(4_000),
        stop: Vec::new(),
    }
}

fn message(role: Role, text: String) -> Message {
    Message {
        role,
        content: vec![ContentPart::Text { text }],
    }
}

fn text(parts: &[ContentPart]) -> String {
    parts
        .iter()
        .filter_map(|part| match part {
            ContentPart::Text { text } => Some(text.as_str()),
            _ => None,
        })
        .collect::<Vec<_>>()
        .join("\n")
}

#[cfg(test)]
mod tests {
    #[test]
    fn planner_never_advertises_tools() {
        assert!(
            super::request("model".into(), "system".into(), "user".into())
                .tools
                .is_empty()
        );
    }
}
