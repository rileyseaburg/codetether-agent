//! Shared helpers: registry singleton, session creation, params, utilities.

use crate::provider::{ContentPart, Message, ProviderRegistry, Role};
use crate::session::Session;
use anyhow::{Context, Result};
use serde::Deserialize;
use std::sync::Arc;
use tokio::sync::OnceCell;

static PROVIDER_REGISTRY: OnceCell<Arc<ProviderRegistry>> = OnceCell::const_new();

pub(super) async fn get_registry() -> Result<Arc<ProviderRegistry>> {
    PROVIDER_REGISTRY
        .get_or_try_init(|| async { Ok(Arc::new(ProviderRegistry::from_vault().await?)) })
        .await
        .cloned()
}

pub(super) fn truncate_preview(input: &str, max_chars: usize) -> String {
    if max_chars == 0 {
        return String::new();
    }
    let preview: String = input.chars().take(max_chars).collect();
    if input.chars().nth(max_chars).is_some() {
        format!("{preview}...")
    } else {
        preview
    }
}

pub(super) async fn create_agent_session(
    name: &str,
    instructions: &str,
    model: &str,
) -> Result<Session> {
    let mut session = Session::new().await.context("Failed to create session")?;
    session.set_agent_name(name.to_string());
    session.metadata.model = Some(model.to_string());
    let system_msg = format!(
        "You are @{name}, a specialized sub-agent. {instructions}\n\n\
         You have access to all tools. Be thorough, focused, and concise. Complete the task fully."
    );
    session.add_message(Message {
        role: Role::System,
        content: vec![ContentPart::Text { text: system_msg }],
    });
    Ok(session)
}

#[derive(Deserialize)]
pub(super) struct Params {
    pub action: String,
    #[serde(default)]
    pub name: Option<String>,
    #[serde(default)]
    pub instructions: Option<String>,
    #[serde(default)]
    pub message: Option<String>,
    #[serde(default)]
    pub model: Option<String>,
    #[serde(default, rename = "__ct_current_model")]
    pub current_model: Option<String>,
}
